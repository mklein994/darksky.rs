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
//! An unofficial Rust wrapper for the [DarkSky] API.
//!
//! While this documentation tries to stay as up-to-date as possible, refer to
//! the [official documentation][docs] for the latest, sanctioned information.
//!
//! See the [developer portal][devportal] to sign up and obtain your token.
//!
//! DarkSky has a status page [here][status] if you need to check its uptime.
//!
//! **Note**: This package was previously named `forecast_io`. Due to a
//! [change in name], this package has been renamed to `darksky`, and can be
//! found on [crates.io] by the same name.
//!
//! ### Installation
//!
//! Add the following dependency to your `Cargo.toml`:
//!
//! ```toml
//! darksky = "0.8"
//! ```
//!
//! And include it in your project:
//!
//! ```rust,no_run
//! extern crate darksky;
//! ```
//!
//! ### Examples
//!
//! Retrieve a [forecast][`Forecast`] for the given latitude and longitude,
//! using a hyper client with a `hyper_native_tls` connector:
//!
//! ```rust,no_run
//! extern crate darksky;
//! extern crate futures;
//! extern crate hyper;
//! extern crate hyper_tls;
//! extern crate tokio_core;
//!
//! # use std::error::Error;
//! #
//! use darksky::DarkskyHyperRequester;
//! use futures::Future;
//! use hyper::{Body, client::{Client, HttpConnector}};
//! use hyper_tls::HttpsConnector;
//! use std::env;
//! use tokio_core::reactor::Core;
//!
//! # fn try_main() -> Result<(), Box<Error>> {
//! let core = Core::new()?;
//! let handle = core.handle();
//!
//! let client = Client::builder()
//!     .build::<_, Body>(HttpsConnector::new(4).unwrap());
//!
//! let token = env::var("FORECAST_TOKEN")?;
//! let lat = 37.8267;
//! let long = -122.423;
//!
//! // We're waiting in this example, but you shouldn't in your code.
//! match client.get_forecast(&token, lat, long).wait() {
//!     Ok(forecast) => println!("{:?}", forecast),
//!     Err(why) => println!("Error getting forecast: {:?}", why),
//! }
//! #     Ok(())
//! # }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! ```
//!
//! ### Features
//!
//! **hyper**: Enables an implementation of [`DarkskyHyperRequester`] on hyper's
//! `Client` (enabled by default).
//!
//! **reqwest**: Enables an implementation of [`DarkskyReqwestRequester`] on
//! reqwest's `Client`.
//!
//! [`DarkskyHyperRequester`]: bridge/hyper/trait.DarkskyHyperRequester.html
//! [`DarkskyReqwestRequester`]: bridge/reqwest/trait.DarkskyReqwestRequester.html
//! [`Forecast`]: models/struct.Forecast.html
//! [DarkSky]: https://darksky.net
//! [change in name]: http://status.darksky.net/2016/09/20/forecast-api-is-now-dark-sky-api.html
//! [crates.io]: https://crates.io
//! [devportal]: https://darksky.net/dev
//! [docs]: https://darksky.net/dev/docs
//! [status]: http://status.darksky.net
#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]
#![deny(missing_docs)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "futures")]
extern crate futures;
#[cfg(feature = "hyper")]
extern crate http;
#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "reqwest")]
extern crate reqwest;

pub mod constants;
pub mod models;
pub mod utils;

#[cfg(any(feature = "hyper", feature = "reqwest"))]
pub mod bridge;

mod error;
mod internal;

pub use error::{Error, Result};

#[cfg(feature = "hyper")]
pub use bridge::DarkskyHyperRequester;
#[cfg(feature = "reqwest")]
pub use bridge::DarkskyReqwestRequester;

use std::collections::HashMap;

/// A block is a name of a [`Datablock`] returned from the API. This can be used
/// to exclude datablocks from being returned from the API, to reduce bandwidth.
///
/// [`Datablock`]: models/struct.Datablock.html
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Block {
    /// Indicator to retrieve the current weather in a request.
    #[serde(rename = "currently")]
    Currently,
    /// Indicator to retrieve the daily weather in a request.
    #[serde(rename = "daily")]
    Daily,
    /// Indicator to retrieve miscellaneous metadata in a request.
    #[serde(rename = "flags")]
    Flags,
    /// Indicator to retrieve hour-by-hour data over the next two days in a
    /// request.
    #[serde(rename = "hourly")]
    Hourly,
    /// Indicator to retrieve minute-by-minute data for the next hour.
    #[serde(rename = "minutely")]
    Minutely,
}

impl Block {
    fn name(&self) -> &str {
        use Block::*;

        match *self {
            Currently => "currently",
            Daily => "daily",
            Flags => "flags",
            Hourly => "hourly",
            Minutely => "minutely",
        }
    }
}

/// The language to return from the API for the [`summary`] field.
///
/// The language is automatically [English][`Language::En`], so specifying
/// English is not required.
///
/// [`Language::En`]: #variant.En
/// [`summary`]: models/struct.Datapoint.html#structfield.summary
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Language {
    /// Arabic
    #[serde(rename = "ar")]
    Ar,
    /// Azerbaijani
    #[serde(rename = "az")]
    Az,
    /// Belarusian
    #[serde(rename = "be")]
    Be,
    /// Bosnian
    #[serde(rename = "bs")]
    Bs,
    /// Czech
    #[serde(rename = "cs")]
    Cs,
    /// German
    #[serde(rename = "de")]
    De,
    /// Greek
    #[serde(rename = "el")]
    El,
    /// English
    #[serde(rename = "en")]
    En,
    /// Spanish
    #[serde(rename = "es")]
    Es,
    /// French
    #[serde(rename = "fr")]
    Fr,
    /// Croatian
    #[serde(rename = "hr")]
    Hr,
    /// Hungarian
    #[serde(rename = "hu")]
    Hu,
    /// Indonesian
    #[serde(rename = "id")]
    Id,
    /// Italian
    #[serde(rename = "it")]
    It,
    /// Icelandic
    #[serde(rename = "is")]
    Is,
    /// Cornish
    #[serde(rename = "kw")]
    Kw,
    /// Norwegian Bokmål
    #[serde(rename = "nb")]
    Nb,
    /// Dutch
    #[serde(rename = "nl")]
    Nl,
    /// Polish
    #[serde(rename = "pl")]
    Pl,
    /// Portuguese
    #[serde(rename = "pt")]
    Pt,
    /// Russian
    #[serde(rename = "ru")]
    Ru,
    /// Slovak
    #[serde(rename = "sk")]
    Sk,
    /// Serbian
    #[serde(rename = "sr")]
    Sr,
    /// Swedish
    #[serde(rename = "sv")]
    Sv,
    /// Tetum
    #[serde(rename = "tet")]
    Tet,
    /// Turkish
    #[serde(rename = "tr")]
    Tr,
    /// Ukrainian
    #[serde(rename = "uk")]
    Uk,
    /// Igpay Atinlay
    #[serde(rename = "x-pig-latin")]
    XPigLatin,
    /// Simplified Chinese
    #[serde(rename = "zh")]
    Zh,
    /// Traditional Chinese
    #[serde(rename = "zh-tw")]
    ZhTw,
}

impl Language {
    fn name(&self) -> &str {
        use Language::*;

        match *self {
            Ar => "ar",
            Az => "az",
            Be => "be",
            Bs => "bs",
            Cs => "cs",
            De => "de",
            El => "el",
            En => "en",
            Es => "es",
            Fr => "fr",
            Hr => "hr",
            Hu => "hu",
            Id => "id",
            It => "it",
            Is => "is",
            Kw => "kw",
            Nb => "nb",
            Nl => "nl",
            Pl => "pl",
            Pt => "pt",
            Ru => "ru",
            Sk => "sk",
            Sr => "sr",
            Sv => "sv",
            Tet => "tet",
            Tr => "tr",
            Uk => "uk",
            XPigLatin => "x-pig-latin",
            Zh => "zh",
            ZhTw => "zh-tw",
        }
    }
}

/// The type of units that the API should send back. `us` is the default value,
/// and does not need to be specified in that case.
///
/// The values are explained under `Options` and then `units=[setting]` in the
/// [documentation][docs].
///
/// Used in conjunction with the [`Options::unit`] method, which is a builder
/// for an argument of `get_forecast_with_options`, which exists on both, the
/// hyper client [`get_forecast_with_options`][hyper `get_forecast_with_options`]
/// and on the reqwest client
/// [`get_forecast_with_options`][reqwest `get_forecast_with_options`].
///
/// [`Options::unit`]: struct.Options.html#method.unit
/// [hyper `get_forecast_with_options`]:
///   bridge/hyper/trait.DarkskyHyperRequester.html#tymethod.get_forecast_with_options
/// [reqwest `get_forecast_with_options`]:
///   bridge/reqwest/trait.DarkskyReqwestRequester.html#tymethod.get_forecast_with_options
/// [docs]: https://darksky.net/dev/docs/forecast
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Unit {
    /// Automatically select units based on geographic location.
    #[serde(rename = "auto")]
    Auto,
    /// Same as [Si][`Unit::Si`], except that [`wind_speed`] is in kilometers
    /// per hour.
    ///
    /// [`wind_speed`]: models/struct.Datapoint.html#structfield.wind_speed
    /// [`Unit::Si`]: #variant.Si
    #[serde(rename = "ca")]
    Ca,
    /// Si units.
    #[serde(rename = "si")]
    Si,
    /// Same as [Si][`Unit::Si`], except that [`nearest_storm_distance`] and
    /// [`visibility`] are in miles and [`wind_speed`] is in miles per hour.
    ///
    /// [`nearest_storm_distance`]: models/struct.Datapoint.html#structfield.nearest_storm_distance
    /// [`visibility`]: models/struct.Datapoint.html#structfield.visibility
    /// [`wind_speed`]: models/struct.Datapoint.html#structfield.wind_speed
    /// [`Unit::Si`]: #variant.Si
    #[serde(rename = "uk2")]
    Uk2,
    /// Imperial units (the default).
    #[serde(rename = "us")]
    Us,
}

impl Unit {
    fn name(&self) -> &str {
        use Unit::*;

        match *self {
            Auto => "auto",
            Ca => "ca",
            Si => "si",
            Uk2 => "uk2",
            Us => "us",
        }
    }
}

/// Build a list of options to send in the request, including the type of
/// [unit][`Unit`]s that the API should return, the [block][`Block`]s to
/// exclude, whether to [extend the hourly][`Options::extend_hourly`]
/// [forecast][`Forecast`], and the [language][`Language`] for the
/// [summary][`Datapoint::summary`].
///
/// Refer to the documentation for `get_forecast_with_options` on how to use
/// this.
/// The documentation for the hyper client can be found here:
/// [`get_forecast_with_options`][hyper `get_forecast_with_options`], and for
/// the reqwest client here:
/// [`get_forecast_with_options`][reqwest `get_forecast_with_options`]
///
/// [`Block`]: enum.Block.html
/// [`Datapoint::summary`]: models/struct.Datapoint.html#structfield.summary
/// [`Forecast`]: models/struct.Forecast.html
/// [`Language`]: enum.Language.html
/// [`Options::extend_hourly`]: struct.Options.html#method.extend_hourly
/// [`Unit`]: enum.Unit.html
/// [hyper `get_forecast_with_options`]:
///   bridge/hyper/trait.DarkskyHyperRequester.html#tymethod.get_forecast_with_options
/// [reqwest `get_forecast_with_options`]:
///   bridge/reqwest/trait.DarkskyReqwestRequester.html#tymethod.get_forecast_with_options
#[derive(Clone, Debug, Default)]
pub struct Options(HashMap<&'static str, String>);

impl Options {
    /// Set the list of [`Datablock`]s to exclude. For a full list of potential
    /// datablocks to exclude, refer to [`Block`].
    ///
    /// [`Block`]: enum.Block.html
    /// [`Datablock`]: models/struct.Datablock.html
    pub fn exclude(mut self, blocks: Vec<Block>) -> Self {
        let block_names = blocks.iter().map(|b| b.name()).collect::<Vec<_>>();

        let list = block_names.join(",");

        self.0.insert("exclude", list.to_owned());

        self
    }

    /// Extends the hourly [forecast][`Forecast`] to the full `7` days ahead,
    /// rather than only the first `2` days.
    ///
    /// [`Forecast`]: models/struct.Forecast.html
    pub fn extend_hourly(mut self) -> Self {
        self.0.insert("extend", "hourly".to_owned());

        self
    }

    /// Gets a mutable reference to the underlying HashMap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use darksky::Options;
    ///
    /// let mut options = Options::default().extend_hourly();
    ///
    /// // Note: you probably shouldn't add keys yourself. If there is a key the
    /// // library does not support, please submit a PR.
    /// let mut inner = options.get_mut();
    /// inner.insert("foo", "bar".to_owned());
    ///
    /// assert_eq!(inner.len(), 2);
    /// ```
    pub fn get_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.0
    }

    /// Gets an immuatble reference to the underlying HashMap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use darksky::{Block, Options};
    ///
    /// let options = Options::default().exclude(vec![Block::Hourly]);
    ///
    /// assert_eq!(options.get_ref().len(), 1);
    /// ```
    pub fn get_ref(&self) -> &HashMap<&'static str, String> {
        &self.0
    }

    /// Unwraps the struct, returning the underlying HashMap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use darksky::{Block, Options};
    ///
    /// let options = Options::default().exclude(vec![Block::Hourly]);
    /// let map = options.into_inner();
    ///
    /// assert!(map.get("exclude").is_some());
    /// ```
    pub fn into_inner(self) -> HashMap<&'static str, String> {
        self.0
    }

    /// Set the language of the [`summary`] provided.
    ///
    /// [`summary`]: models/struct.Datapoint.html#structfield.summary
    pub fn language(mut self, language: Language) -> Self {
        self.0.insert("lang", language.name().to_owned());

        self
    }

    /// Sets the unit type returned from the API. Refer to the
    /// [DarkSky documentation][docs] or the [`Unit`] docs for more info.
    ///
    /// [`Unit`]: enum.Unit.html
    /// [docs]: https://darksky.net/dev/docs
    pub fn unit(mut self, unit: Unit) -> Self {
        self.0.insert("units", unit.name().to_owned());

        self
    }
}
