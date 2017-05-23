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
//! darksky = "0.5"
//! ```
//!
//! And include it in your project:
//!
//! ```rust
//! extern crate darksky;
//! ```
//!
//! ### Examples
//!
//! Retrieve a [forecast][`Forecast`] for the given latitude and longitude:
//!
//! ```rust
//! use darksky::Block;
//! use std::env;
//!
//! let token = env::var("FORECAST_TOKEN").expect("forecast token");
//! let lat = 37.8267;
//! let long = -122.423;
//!
//! let req = darksky::get_forecast(&token, lat, long);
//!
//! let _forecast = match req {
//!     Ok(forecast) => forecast,
//!     Err(why) => {
//!         println!("Error getting forecast: {:?}", why);
//!
//!         return;
//!     },
//! };
//! ```
//!
//! [`Forecast`]: struct.Forecast.html
//! [DarkSky]: https://darksky.net
//! [change in name]: http://status.darksky.net/2016/09/20/forecast-api-is-now-dark-sky-api.html
//! [crates.io]: https://crates.io
//! [devportal]: https://darksky.net/dev
//! [docs]: https://darksky.net/dev/docs
//! [status]: http://status.darksky.net

#[macro_use] extern crate serde_derive;

extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;

mod error;
mod models;

pub use error::{Error, Result};
pub use models::*;

use hyper::{Client, Error as HyperError};
use hyper::client::Response as HyperResponse;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::collections::HashMap;

static API_URL: &'static str = "https://api.darksky.net";

/// A block is a name of a [`Datablock`] returned from the API. This can be used
/// to exclude datablocks from being returned from the API, to reduce bandwidth.
///
/// [`Datablock`]: struct.Datablock.html
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Block {
    #[serde(rename="currently")]
    Currently,
    #[serde(rename="daily")]
    Daily,
    #[serde(rename="flags")]
    Flags,
    #[serde(rename="hourly")]
    Hourly,
    #[serde(rename="minutely")]
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
/// [`summary`]: struct.Datapoint.html#structfield.summary
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Language {
    /// Arabic
    #[serde(rename="ar")]
    Ar,
    /// Azerbaijani
    #[serde(rename="az")]
    Az,
    /// Belarusian
    #[serde(rename="be")]
    Be,
    /// Bosnian
    #[serde(rename="bs")]
    Bs,
    /// Czech
    #[serde(rename="cs")]
    Cs,
    /// German
    #[serde(rename="de")]
    De,
    /// Greek
    #[serde(rename="el")]
    El,
    /// English
    #[serde(rename="en")]
    En,
    /// Spanish
    #[serde(rename="es")]
    Es,
    /// French
    #[serde(rename="fr")]
    Fr,
    /// Croatian
    #[serde(rename="hr")]
    Hr,
    /// Hungarian
    #[serde(rename="hu")]
    Hu,
    /// Indonesian
    #[serde(rename="id")]
    Id,
    /// Italian
    #[serde(rename="it")]
    It,
    /// Icelandic
    #[serde(rename="is")]
    Is,
    /// Cornish
    #[serde(rename="kw")]
    Kw,
    /// Norwegian Bokmål
    #[serde(rename="nb")]
    Nb,
    /// Dutch
    #[serde(rename="nl")]
    Nl,
    /// Polish
    #[serde(rename="pl")]
    Pl,
    /// Portuguese
    #[serde(rename="pt")]
    Pt,
    /// Russian
    #[serde(rename="ru")]
    Ru,
    /// Slovak
    #[serde(rename="sk")]
    Sk,
    /// Serbian
    #[serde(rename="sr")]
    Sr,
    /// Swedish
    #[serde(rename="sv")]
    Sv,
    /// Tetum
    #[serde(rename="tet")]
    Tet,
    /// Turkish
    #[serde(rename="tr")]
    Tr,
    /// Ukrainian
    #[serde(rename="uk")]
    Uk,
    /// Igpay Atinlay
    #[serde(rename="x-pig-latin")]
    XPigLatin,
    /// Simplified Chinese
    #[serde(rename="zh")]
    Zh,
    /// Traditional Chinese
    #[serde(rename="zh-tw")]
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
/// for an argument of [`get_forecast_with_options`].
///
/// [`Options::unit`]: struct.Options.html#method.unit
/// [`get_forecast_with_options`]: fn.get_forecast_with_options.html
/// [docs]: https://darksky.net/dev/docs/forecast
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Unit {
    /// Automatically select units based on geographic location.
    #[serde(rename="auto")]
    Auto,
    /// Same as [Si][`Unit::Si`], except that [`wind_speed`] is in kilometers
    /// per hour.
    ///
    /// [`wind_speed`]: struct.Datapoint.html#structfield.wind_speed
    /// [`Unit::Si`]: #variant.Si
    #[serde(rename="ca")]
    Ca,
    /// Imperial units (the default).
    #[serde(rename="si")]
    Si,
    /// Same as [Si][`Unit::Si`], except that [`nearest_storm_distance`] and
    /// [`visibility`] are in miles and [`wind_speed`] is in miles per hour.
    ///
    /// [`nearest_storm_distance`]: struct.Datapoint.html#structfield.nearest_storm_distance
    /// [`visibility`]: struct.Datapoint.html#structfield.visibility
    /// [`wind_speed`]: struct.Datapoint.html#structfield.wind_speed
    /// [`Unit::Si`]: #variant.Si
    #[serde(rename="uk2")]
    Uk2,
    /// SI units.
    #[serde(rename="us")]
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
/// Refer to the documentation for [`get_forecast_with_options`] on how to use
/// this.
///
/// [`Block`]: enum.Block.html
/// [`Datapoint::summary`]: struct.Datapoint.html#structfield.summary
/// [`Forecast`]: struct.Forecast.html
/// [`Language`]: enum.Language.html
/// [`Options::extend_hourly`]: struct.Options.html#method.extend_hourly
/// [`Unit`]: enum.Unit.html
/// [`get_forecast_with_options`]: fn.get_forecast_with_options.html
pub struct Options(HashMap<String, String>);

impl Options {
    /// Set the list of [`Datablock`]s to exclude. For a full list of potential
    /// datablocks to exclude, refer to [`Block`].
    ///
    /// [`Block`]: enum.Block.html
    /// [`Datablock`]: struct.Datablock.html
    pub fn exclude(mut self, blocks: Vec<Block>) -> Self {
        let block_names: Vec<&str> = blocks.iter()
            .map(|block| block.name())
            .collect();

        let list = block_names.join(",");

        self.0.insert("exclude".to_owned(), list.to_owned());

        self
    }

    /// Extends the hourly [forecast][`Forecast`] to the full `7` days ahead,
    /// rather than only the first `2` days.
    ///
    /// [`Forecast`]: struct.Forecast.html
    pub fn extend_hourly(mut self) -> Self {
        self.0.insert("extend".to_owned(), "hourly".to_owned());

        self
    }

    /// Set the language of the [`summary`] provided.
    ///
    /// [`summary`]: struct.Datapoint.html#structfield.summary
    pub fn language(mut self, language: Language) -> Self {
        self.0.insert("lang".to_owned(), language.name().to_owned());

        self
    }

    /// Sets the unit type returned from the API. Refer to the
    /// [DarkSky documentation][docs] or the [`Unit`] docs for more info.
    ///
    /// [`Unit`]: enum.Unit.html
    /// [docs]: https://darksky.net/dev/docs
    pub fn unit(mut self, unit: Unit) -> Self {
        self.0.insert("units".to_owned(), unit.name().to_owned());

        self
    }
}

fn get_client() -> Result<Client> {
    let ssl = NativeTlsClient::new().map_err(|e| HyperError::Ssl(Box::new(e)))?;
    let connector = HttpsConnector::new(ssl);

    Ok(Client::with_connector(connector))
}

/// Retrieve a [forecast][`Forecast`] for the given latitude and longitude.
///
/// # Examples
///
/// ```rust
/// use darksky::Block;
/// use std::env;
///
/// let token = env::var("FORECAST_TOKEN").expect("forecast token");
/// let lat = 37.8267;
/// let long = -122.423;
///
/// let req = darksky::get_forecast(&token, lat, long);
///
/// let _forecast = match req {
///     Ok(forecast) => forecast,
///     Err(why) => {
///         println!("Error getting forecast: {:?}", why);
///
///         return;
///     },
/// };
/// ```
///
/// [`Forecast`]: struct.Forecast.html
pub fn get_forecast(token: &str, latitude: f64, longitude: f64)
    -> Result<Forecast> {
    let client = get_client()?;
    let uri = format!("{}/forecast/{}/{},{}?units=auto", API_URL, token, latitude, longitude);

    let response = client.get(&uri).send()?;

    serde_json::from_reader::<HyperResponse, Forecast>(response).map_err(From::from)
}

/// Retrieve a [forecast][`Forecast`] for the given latitude and longitude,
/// setting options where needed. For a full list of options, refer to the
/// documentation for the [`Options`] builder.
///
/// # Examples
///
/// Retrieve an extended forecast, excluding the
/// [minutely block][`Block::Minutely`].
///
/// ```rust
/// use darksky::Block;
/// use std::env;
///
/// let token = env::var("FORECAST_TOKEN").expect("forecast token");
/// let lat = 37.8267;
/// let long = -122.423;
///
/// let req = darksky::get_forecast_with_options(&token, lat, long, |o| o
///     .exclude(vec![Block::Minutely])
///     .extend_hourly());
///
/// let _forecast = match req {
///     Ok(forecast) => forecast,
///     Err(why) => {
///         println!("Error getting forecast: {:?}", why);
///
///         return;
///     },
/// };
/// ```
///
/// [`Block::Minutely`]: enum.Block.html#variant.Minutely
/// [`Forecast`]: struct.Forecast.html
/// [`Options`]: struct.Options.html
pub fn get_forecast_with_options<F>(token: &str,
                                    latitude: f64,
                                    longitude: f64,
                                    options: F)
                                    -> Result<Forecast>
                                    where F: FnOnce(Options) -> Options {
    let client = get_client()?;
    let items: Vec<String> = options(Options(HashMap::new()))
        .0
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    let built = items.join("&");
    let uri = format!("{}/forecast/{}/{},{}?{}", API_URL, token, latitude, longitude, built);

    let response = client.get(&uri).send()?;

    serde_json::from_reader::<HyperResponse, Forecast>(response).map_err(From::from)
}

#[cfg(test)]
mod tests {
    use ::std::env;
    use super::*;

    #[test]
    fn test_get_forecast() {
        let token = env::var("FORECAST_TOKEN").expect("forecast token");

        if let Err(why) = get_forecast(&token[..], 37.8267, -122.423) {
            panic!("{:?}", why);
        }

        if let Err(why) = get_forecast(&token[..], 39.9042, 116.4074) {
            panic!("{:?}", why);
        }

        if let Err(why) = get_forecast(&token[..], 19.2465, -99.1013) {
            panic!("{:?}", why);
        }
    }

    #[test]
    fn test_get_forecast_with_options() {
        let token = env::var("FORECAST_TOKEN").expect("forecast token");

        match get_forecast_with_options(&token[..], 19.2465, -99.1013, |opt| {
            opt.exclude(vec![Block::Currently, Block::Daily])
               .extend_hourly()
               .language(Language::Es)
               .unit(Unit::Si)
        }) {
            Ok(forecast) => {
                assert!(forecast.currently.is_none());
                assert!(forecast.daily.is_none());
                assert!(forecast.flags.is_some());
            },
            Err(why) => {
                panic!("{:?}", why);
            },
        }
    }
}
