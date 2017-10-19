//! Utilities that provide some basic functionality that may be useful, but are
//! generally non-essential for usage of the library.

use std::collections::HashMap;
use std::fmt::Write;
use ::constants::API_URL;
use ::Result;

/// Formats a URI for retrieving a forecast without options.
///
/// Accepts the token to use, as well as the latitude and longitude of the
/// location being requested.
///
/// # Examples
///
/// Format a request URI using the token `"abc"`, a latitude of `-7.3`, and a
/// longitude of `8.17`:
///
/// ```rust
/// use darksky::utils;
///
/// let uri = utils::uri("abc", -7.3, 8.17);
/// let exp = "https://api.darksky.net/forecast/abc/-7.3,8.17?units=auto";
///
/// assert_eq!(uri, exp);
/// ```
#[inline]
pub fn uri(token: &str, lat: f64, long: f64) -> String {
    format!("{}/forecast/{}/{},{}?units=auto", API_URL, token, lat, long)
}

/// Formats a URI for retrieving a forecast with options.
///
/// Accepts the token to use, the latitude and longitude of the location being
/// requested, and additional options for the request.
///
/// # Examples
///
/// Format a request URI with the token `"def"`, a latitude of `-4.13`, a
/// longitude of `14.32`, and excluding the [`Hourly`][`Block::Hourly`] block:
///
/// ```rust
/// use darksky::{Block, Options, utils};
///
/// let options = Options::default().exclude(vec![Block::Hourly]).into_inner();
/// let uri = utils::uri_optioned("def", -4.13, 14.32, options).unwrap();
/// let exp = "https://api.darksky.net/forecast/def/-4.13,14.32?exclude=hourly&";
///
/// assert_eq!(uri, exp);
/// ```
///
/// [`Block::Hourly`]: ../enum.Block.html#variant.Hourly
#[inline]
pub fn uri_optioned(
    token: &str,
    lat: f64,
    long: f64,
    options: HashMap<&'static str, String>,
) -> Result<String> {
    let mut uri = String::new();
    uri.push_str(API_URL);
    uri.push_str("/forecast/");
    uri.push_str(token);
    uri.push('/');
    write!(uri, "{}", lat)?;
    uri.push(',');
    write!(uri, "{}", long)?;
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

    Ok(uri)
}
