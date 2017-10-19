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

use hyper::client::{Client, Response};
use serde_json;
use std::collections::HashMap;
use std::fmt::Write;
use ::{API_URL, Forecast, Options, Result};

/// The trait for implementations to different DarkSky routes.
pub trait DarkskyHyperRequester {
    /// Retrieve a [forecast][`Forecast`] for the given latitude and longitude.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate darksky;
    /// extern crate hyper;
    /// extern crate hyper_native_tls;
    ///
    /// # use std::error::Error;
    /// #
    /// use darksky::{DarkskyHyperRequester, Block};
    /// use hyper::net::HttpsConnector;
    /// use hyper::Client;
    /// use hyper_native_tls::NativeTlsClient;
    /// use std::env;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let tc = NativeTlsClient::new()?;
    /// let connector = HttpsConnector::new(tc);
    /// let client = Client::with_connector(connector);
    ///
    /// let token = env::var("FORECAST_TOKEN")?;
    /// let lat = 37.8267;
    /// let long = -122.423;
    ///
    /// match client.get_forecast(&token, lat, long) {
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
    fn get_forecast(&self, token: &str, latitude: f64, longitude: f64) -> Result<Forecast>;

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
    /// extern crate hyper;
    /// extern crate hyper_native_tls;
    ///
    /// # use std::error::Error;
    /// #
    /// use darksky::{DarkskyHyperRequester, Block};
    /// use hyper::net::HttpsConnector;
    /// use hyper::Client;
    /// use hyper_native_tls::NativeTlsClient;
    /// use std::env;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let tc = NativeTlsClient::new()?;
    /// let connector = HttpsConnector::new(tc);
    /// let client = Client::with_connector(connector);
    ///
    /// let token = env::var("FORECAST_TOKEN").expect("forecast token");
    /// let lat = 37.8267;
    /// let long = -122.423;
    ///
    /// let req = client.get_forecast_with_options(&token, lat, long, |o| o
    ///     .exclude(vec![Block::Minutely])
    ///     .extend_hourly());
    ///
    /// match req {
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
    fn get_forecast_with_options<F>(
        &self,
        token: &str,
        latitude: f64,
        longitude: f64,
        options: F
    ) -> Result<Forecast> where F: FnOnce(Options) -> Options;
}

impl DarkskyHyperRequester for Client {
    fn get_forecast(&self, token: &str, latitude: f64, longitude: f64)
        -> Result<Forecast> {
        let uri = format!("{}/forecast/{}/{},{}?units=auto", API_URL, token, latitude, longitude);

        let response = self.get(&uri).send()?;

        serde_json::from_reader::<Response, Forecast>(response).map_err(From::from)
    }

    fn get_forecast_with_options<F>(
        &self,
        token: &str,
        latitude: f64,
        longitude: f64,
        options: F,
    ) -> Result<Forecast> where F: FnOnce(Options) -> Options {
        let options = options(Options(HashMap::new())).0;

        let uri = {
            let mut uri = String::new();
            uri.push_str(API_URL);
            uri.push_str("/forecast/");
            uri.push_str(token);
            uri.push('/');
            write!(uri, "{}", latitude)?;
            uri.push(',');
            write!(uri, "{}", longitude)?;
            uri.push('?');

            for (k, v) in options {
                uri.push_str(k);
                uri.push('=');

                {
                    let v_bytes = v.into_bytes();

                    unsafe {
                        let bytes = uri.as_mut_vec();
                        bytes.extend(v_bytes);
                    }
                }

                uri.push('&');
            }

            uri
        };

        let response = self.get(&uri).send()?;

        serde_json::from_reader::<Response, Forecast>(response).map_err(From::from)
    }
}
