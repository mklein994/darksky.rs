extern crate darksky;
extern crate hyper;
extern crate hyper_native_tls;

use darksky::*;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use std::env;

#[inline]
fn client() -> Client {
	let tc = NativeTlsClient::new().unwrap();
	let connector = HttpsConnector::new(tc);

	Client::with_connector(connector)
}

#[ignore]
#[test]
fn test_get_forecast() {
	let token = env::var("FORECAST_TOKEN").expect("forecast token");

	let client = client();
	client.get_forecast(&token[..], 37.8267, -122.423).unwrap();
	client.get_forecast(&token[..], 39.9042, 116.4074).unwrap();
	client.get_forecast(&token[..], 19.2465, -99.1013).unwrap();
}

#[ignore]
#[test]
fn test_get_forecast_with_options() {
	let token = env::var("FORECAST_TOKEN").expect("forecast token");

	let client = client();
	client.get_forecast_with_options(&token[..], 19.2465, -99.1013, |opt| {
		opt.exclude(vec![Block::Currently, Block::Daily])
		   .extend_hourly()
		   .language(Language::Es)
		   .unit(Unit::Si)
	}).unwrap();
}
