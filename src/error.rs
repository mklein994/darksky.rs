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

use hyper::Error as HyperError;
use serde_json::{Error as JsonError, Value};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

/// A generic result type for all public-facing functions within the library.
pub type Result<T> = StdResult<T, Error>;

/// Common result type for the library's [`Result`] type. Includes errors for
/// JSON decoding, Io errors, etc.
///
/// [`Result`]: type.Result.html
#[derive(Debug)]
pub enum Error {
	/// A json decoding error, with a description and the value. This occurs
	/// when the received value type is not of the expected type.
	Decode(&'static str, Value),
	/// A `hyper` crate error
	Hyper(HyperError),
	/// A `serde_json` crate error
	Json(JsonError),
	/// A `std::io` module error
	Io(IoError),
}

impl From<HyperError> for Error {
	fn from(err: HyperError) -> Error {
		Error::Hyper(err)
	}
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Error {
		Error::Io(err)
	}
}

impl From<JsonError> for Error {
	fn from(err: JsonError) -> Error {
		Error::Json(err)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match *self {
			Error::Hyper(ref inner) => inner.fmt(f),
			Error::Json(ref inner) => inner.fmt(f),
			Error::Io(ref inner) => inner.fmt(f),
			_ => f.write_str(self.description()),
		}
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Decode(msg, _) => msg,
			Error::Hyper(ref inner) => inner.description(),
			Error::Json(ref inner) => inner.description(),
			Error::Io(ref inner) => inner.description(),
		}
	}

	fn cause(&self) -> Option<&StdError> {
		match *self {
			Error::Hyper(ref inner) => Some(inner),
			Error::Json(ref inner) => Some(inner),
			Error::Io(ref inner) => Some(inner),
			_ => None,
		}
	}
}
