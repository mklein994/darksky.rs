// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
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

extern crate hyper;
extern crate serde_json;

#[macro_use]
mod utils;

mod error;
mod models;

pub use error::{Error, Result};
pub use models::*;

use hyper::Client;

static API_URL: &'static str = "https://api.forecast.io";

pub fn get_forecast<S: Into<String>>(token: S,
                                     latitude: f64,
                                     longitude: f64)
                                     -> Result<Forecast> {
    let response = try!(Client::new()
        .get(&format!("{}/forecast/{}/{},{}?units=auto",
                      API_URL,
                      token.into(),
                      latitude,
                      longitude))
        .send());

    Forecast::decode(try!(serde_json::from_reader(response)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_forecast() {
        let token = ::std::env::var("FORECAST_TOKEN").expect("forecast token");

        if let Err(why) = ::get_forecast(&token[..],
                                         37.8267,
                                         -122.423) {
            panic!("{:?}", why);
        }

        if let Err(why) = ::get_forecast(&token[..],
                                         39.9042,
                                         116.4074) {
            panic!("{:?}", why);
        }

        if let Err(why) = ::get_forecast(&token[..],
                                         19.2465,
                                         -99.1013) {
            panic!("{:?}", why);
        }
    }
}
