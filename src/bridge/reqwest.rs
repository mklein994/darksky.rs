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
//! Bridge to provide DarkSky client implementation for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`DarkskyReqwestRequester`].
//!
//! [`DarkskyReqwestRequester`]: trait.DarkskyReqwestRequester.html

use reqwest::Client;
use ::models::Forecast;
use ::{Options, Result, internal, utils};

/// The trait for `reqwest` implementations to different DarkSky routes.
pub trait DarkskyReqwestRequester {
    /// Retrieve a [`Forecast`] for the given latitude and longitude.
    ///
    /// # Examples
    ///
    /// Retrieve a forecast for a location, taking a token from the environment:
    ///
    /// ```rust,no_run
    /// extern crate darksky;
    /// extern crate reqwest;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use darksky::DarkskyReqwestRequester;
    /// use reqwest::Client;
    /// use std::env;
    ///
    /// let token = env::var("FORECAST_TOKEN")?;
    /// let client = Client::new();
    ///
    /// let lat = 37.8267;
    /// let long = -122.423;
    ///
    /// let req = client.get_forecast(&token, lat, long)?;
    ///
    /// println!("Forecast: {:?}", req);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// [`Block::Minutely`]: ../enum.Block.html#variant.Minutely
    /// [`Forecast`]: ../models/struct.Forecast.html
    fn get_forecast(&self, token: &str, latitude: f64, longitude: f64)
        -> Result<Forecast>;

    /// Retrieve a [`Forecast`] for the given latitude and longitude, setting
    /// options where needed. For a full list of options, refer to the
    /// documentation for the [`Options`] builder.
    ///
    /// # Examples
    ///
    /// Retrieve an extended forecast, excluding the
    /// [minutely block][`Block::Minutely`], taking a token from the
    /// environment:
    ///
    /// ```rust,no_run
    /// extern crate darksky;
    /// extern crate reqwest;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use darksky::{Block, DarkskyReqwestRequester};
    /// use reqwest::Client;
    /// use std::env;
    ///
    /// let token = env::var("FORECAST_TOKEN")?;
    /// let client = Client::new();
    ///
    /// let lat = 37.8267;
    /// let long = -122.423;
    ///
    /// let req = client.get_forecast_with_options(&token, lat, long, |o| o
    ///     .exclude(vec![Block::Minutely]))?;
    ///
    /// println!("Forecast: {:?}", req);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// [`Block::Minutely`]: ../enum.Block.html#variant.Minutely
    /// [`Forecast`]: ../models/struct.Forecast.html
    /// [`Options`]: ../struct.Options.html
    fn get_forecast_with_options<F>(
        &self,
        token: &str,
        latitude: f64,
        longitude: f64,
        options: F,
    ) -> Result<Forecast> where F: FnOnce(Options) -> Options;
}

impl DarkskyReqwestRequester for Client {
    fn get_forecast(&self, token: &str, latitude: f64, longitude: f64)
        -> Result<Forecast> {
        let uri = utils::uri(token, latitude, longitude);

        internal::from_reader(self.get(&uri).send()?)
    }

    fn get_forecast_with_options<F>(
        &self,
        token: &str,
        latitude: f64,
        longitude: f64,
        options: F,
    ) -> Result<Forecast> where F: FnOnce(Options) -> Options {
        let options = options(Options::default()).0;
        let uri = utils::uri_optioned(token, latitude, longitude, options)?;

        internal::from_reader(self.get(&uri).send()?)
    }
}
