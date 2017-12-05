#![cfg(feature = "hyper")]

extern crate darksky;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use darksky::*;
use futures::Future;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use std::env;
use tokio_core::reactor::{Core, Handle};
// use tokio_core::Core;

#[inline]
fn client(handle: &Handle) -> Client<HttpsConnector<HttpConnector>, Body> {
	Client::configure()
		.connector(HttpsConnector::new(4, handle).unwrap())
		.build(handle)
}

#[ignore]
#[test]
fn test_get_forecast() {
	let token = env::var("FORECAST_TOKEN").expect("forecast token");

	let core = Core::new().unwrap();
	let client = client(&core.handle());

	client.get_forecast(&token[..], 37.8267, -122.423).wait().unwrap();
	client.get_forecast(&token[..], 39.9042, 116.4074).wait().unwrap();
	client.get_forecast(&token[..], 19.2465, -99.1013).wait().unwrap();
}

#[ignore]
#[test]
fn test_get_forecast_with_options() {
	let token = env::var("FORECAST_TOKEN").expect("forecast token");

	let core = Core::new().unwrap();
	let client = client(&core.handle());
	client.get_forecast_with_options(&token[..], 19.2465, -99.1013, |opt| {
		opt.exclude(vec![Block::Currently, Block::Daily])
		   .extend_hourly()
		   .language(Language::Es)
		   .unit(Unit::Si)
	}).wait().unwrap();
}
