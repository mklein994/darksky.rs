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

use futures::{Future, Stream, future};
use hyper::client::{Client, Connect};
use hyper::{Error as HyperError, Uri};
use std::collections::HashMap;
use std::str::FromStr;
use ::models::Forecast;
use ::{Error, Options, internal, utils};

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
    /// use hyper::client::{Client, HttpConnector};
    /// use hyper_tls::HttpsConnector;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let core = Core::new()?;
    /// let handle = core.handle();
    ///
    /// let client = Client::configure()
    ///     .connector(HttpsConnector::new(4, &handle)?)
    ///     .build(&handle);
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
    /// [`Forecast`]: struct.Forecast.html
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
    /// use hyper::client::{Client, HttpConnector};
    /// use hyper_tls::HttpsConnector;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let core = Core::new()?;
    /// let handle = core.handle();
    ///
    /// let client = Client::configure()
    ///     .connector(HttpsConnector::new(4, &handle)?)
    ///     .build(&handle);
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
    /// [`Block::Minutely`]: enum.Block.html#variant.Minutely
    /// [`Forecast`]: struct.Forecast.html
    /// [`Options`]: struct.Options.html
    fn get_forecast_with_options<'a, 'b, F, T>(
        &'a self,
        token: T,
        latitude: f64,
        longitude: f64,
        options: F,
    ) -> Box<Future<Item = Forecast, Error = Error> + 'b>
        where F: FnOnce(Options) -> Options, T: AsRef<str>;
}

impl<B, C: Connect> DarkskyHyperRequester for Client<C, B>
    where B: Stream<Error = HyperError> + 'static, B::Item: AsRef<[u8]> {
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

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .from_err()
            .map(internal::from_chunk)
            .and_then(|x| x))
    }

    fn get_forecast_with_options<'a, 'b, F, T>(
        &'a self,
        token: T,
        latitude: f64,
        longitude: f64,
        options: F,
    ) -> Box<Future<Item = Forecast, Error = Error> + 'b>
        where F: FnOnce(Options) -> Options, T: AsRef<str> {
        let options = options(Options(HashMap::new())).0;
        let url = match utils::uri_optioned(token.as_ref(), latitude, longitude, options) {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(why)),
        };
        let uri = match Uri::from_str(&url) {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(Error::Uri(why))),
        };

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .from_err()
            .map(internal::from_chunk)
            .and_then(|x| x))
    }
}
