// ISC License (ISC)
//
// Copyright (c) 2016, Zeyla Hellyer <zey@zey.moe>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
// RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
// CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//! Bridged support for the `hyper` library.

use futures::{future, Future, Stream};
use hyper::{
    body::Payload,
    client::{connect::Connect, Client},
};
use hyper::{Error as HyperError, Uri};
use models::Forecast;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use {internal, utils, Error, Options};

/// The trait for `hyper` implementations to different DarkSky routes.
pub trait DarkskyHyperRequester {
    /// Retrieve a [forecast][`Forecast`] for the given latitude and longitude.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate darksky;
    /// extern crate futures;
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate tokio_core;
    ///
    /// # use std::error::Error;
    /// #
    /// use darksky::{DarkskyHyperRequester, Block};
    /// use futures::Future;
    /// use hyper::{Body, client::{Client, HttpConnector}};
    /// use hyper_tls::HttpsConnector;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let core = Core::new()?;
    /// let client = Client::builder()
    ///     .build::<_, Body>(HttpsConnector::new(4).unwrap());
    ///
    /// let token = env::var("FORECAST_TOKEN")?;
    /// let lat = 37.8267;
    /// let long = -122.423;
    ///
    /// // We're waiting in this example, but you shouldn't in your code.
    /// match client.get_forecast(&token, lat, long).wait() {
    ///     Ok(forecast) => println!("{:?}", forecast),
    ///     Err(why) => println!("Error getting forecast: {:?}", why),
    /// }
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// [`Forecast`]: ../../models/struct.Forecast.html
    fn get_forecast<'a, 'b, T: AsRef<str>>(
        &'a self,
        token: T,
        latitude: f64,
        longitude: f64,
    ) -> Box<Future<Item = Forecast, Error = Error> + 'b>;

    /// Retrieve a [forecast][`Forecast`] for the given latitude and longitude,
    /// setting options where needed. For a full list of options, refer to the
    /// documentation for the [`Options`] builder.
    ///
    /// # Examples
    ///
    /// Retrieve an extended forecast, excluding the
    /// [minutely block][`Block::Minutely`].
    ///
    /// ```rust,no_run
    /// extern crate darksky;
    /// extern crate futures;
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate tokio_core;
    ///
    /// # use std::error::Error;
    /// #
    /// use darksky::{DarkskyHyperRequester, Block};
    /// use futures::Future;
    /// use hyper::{Body, client::{Client, HttpConnector}};
    /// use hyper_tls::HttpsConnector;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let core = Core::new()?;
    ///
    /// let client = Client::builder()
    ///     .build::<_, Body>(HttpsConnector::new(4).unwrap());
    ///
    /// let token = env::var("FORECAST_TOKEN").expect("forecast token");
    /// let lat = 37.8267;
    /// let long = -122.423;
    ///
    /// let req = client.get_forecast_with_options(&token, lat, long, |o| o
    ///     .exclude(vec![Block::Minutely])
    ///     .extend_hourly());
    ///
    /// // We're waiting in this example, but you shouldn't in your code.
    /// match req.wait() {
    ///     Ok(forecast) => println!("{:?}", forecast),
    ///     Err(why) => println!("Error getting forecast: {:?}", why),
    /// }
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// [`Block::Minutely`]: ../../enum.Block.html#variant.Minutely
    /// [`Forecast`]: ../../models/struct.Forecast.html
    /// [`Options`]: ../../struct.Options.html
    fn get_forecast_with_options<'a, 'b, F, T>(
        &'a self,
        token: T,
        latitude: f64,
        longitude: f64,
        options: F,
    ) -> Box<Future<Item = Forecast, Error = Error> + 'b>
    where
        F: FnOnce(Options) -> Options,
        T: AsRef<str>;

    /// Sets the time to request a forecast for by using DarkSky's Time Machine
    /// API.
    ///
    /// This accepts either a Unix timestamp or a string in the format of
    /// `[YYYY]-[MM]-[DD]T[HH]:[MM]:[SS][timezone`, where `timezone` should
    /// either be:
    ///
    /// - omitted (referring to the local time for the location being
    /// requested);
    /// - `Z` referring to GMT time;
    /// - or `-[HH][MM]` for an offset from GMT in hours and minutes.
    ///
    /// The timezone is only used for determining the time of the request. The
    /// response will always be relative to the local time zone.
    ///
    /// Refer to DarkSky's documentation on
    /// [Time Machine Request Parameters][docs]for information.
    ///
    /// This function accepts anything that implements the `Display` trait, so
    /// you should validate data beforehand. This is to avoid implementing a
    /// time validation scheme, while avoiding locking the parameter type to
    /// that of a time library (e.g. Chrono).
    ///
    /// [docs]: https://darksky.net/dev/docs#time-machine-request-parameters
    fn get_forecast_time_machine<D, F, T>(
        &self,
        token: T,
        latitude: f64,
        longitude: f64,
        time: D,
        options: F,
    ) -> Box<Future<Item = Forecast, Error = Error>>
    where
        D: Display,
        F: FnOnce(Options) -> Options,
        T: AsRef<str>;
}

impl<B, C> DarkskyHyperRequester for Client<C, B>
where
    C: Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
    B: Payload + Send + 'static + Default + Stream<Error = HyperError>,
    B::Data: Send,
    B::Item: AsRef<[u8]>,
{
    fn get_forecast<'a, 'b, T: AsRef<str>>(
        &'a self,
        token: T,
        latitude: f64,
        longitude: f64,
    ) -> Box<Future<Item = Forecast, Error = Error> + 'b> {
        let url = utils::uri(token.as_ref(), latitude, longitude);
        let uri = match Uri::from_str(&url) {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(Error::Uri(why))),
        };

        Box::new(
            self.get(uri)
                .and_then(|res| res.into_body().concat2())
                .from_err()
                .map(internal::from_chunk)
                .and_then(|x| x),
        )
    }

    fn get_forecast_with_options<'a, 'b, F, T>(
        &'a self,
        token: T,
        latitude: f64,
        longitude: f64,
        options: F,
    ) -> Box<Future<Item = Forecast, Error = Error> + 'b>
    where
        F: FnOnce(Options) -> Options,
        T: AsRef<str>,
    {
        let options = options(Options(HashMap::new())).0;
        let constructed = utils::uri_optioned(token.as_ref(), latitude, longitude, None, options);
        let url = match constructed {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(why)),
        };
        let uri = match Uri::from_str(&url) {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(Error::Uri(why))),
        };

        Box::new(
            self.get(uri)
                .and_then(|res| res.into_body().concat2())
                .from_err()
                .map(internal::from_chunk)
                .and_then(|x| x),
        )
    }

    fn get_forecast_time_machine<D, F, T>(
        &self,
        token: T,
        latitude: f64,
        longitude: f64,
        time: D,
        options: F,
    ) -> Box<Future<Item = Forecast, Error = Error>>
    where
        D: Display,
        F: FnOnce(Options) -> Options,
        T: AsRef<str>,
    {
        let time = time.to_string();

        forecast_optioned(self, token, latitude, longitude, Some(time), options)
    }
}

fn forecast_optioned<B, C, F, T>(
    client: &Client<C, B>,
    token: T,
    latitude: f64,
    longitude: f64,
    time: Option<String>,
    options: F,
) -> Box<Future<Item = Forecast, Error = Error>>
where
    B: Payload + Send + 'static + Default + Stream<Error = HyperError>,
    B::Item: AsRef<[u8]>,
    C: Connect + 'static,
    F: FnOnce(Options) -> Options,
    T: AsRef<str>,
{
    let options = options(Options(HashMap::new())).0;
    let constructed = utils::uri_optioned(token.as_ref(), latitude, longitude, time, options);
    let url = match constructed {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(why)),
    };
    let uri = match Uri::from_str(&url) {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(Error::Uri(why))),
    };

    Box::new(
        client
            .get(uri)
            .and_then(|res| res.into_body().concat2())
            .from_err()
            .map(internal::from_chunk)
            .and_then(|x| x),
    )
}
