#![allow(unreachable_code)]
#![cfg(feature = "hyper")]

extern crate darksky;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use darksky::*;
use futures::{future, Future};
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use std::env;
use tokio_core::reactor::Core;

#[inline]
fn client() -> Client<HttpsConnector<HttpConnector>, Body> {
    Client::builder().build(HttpsConnector::new(4).unwrap())
}

#[ignore]
#[test]
fn test_get_forecast() {
    let token = env::var("FORECAST_TOKEN").expect("forecast token");

    let mut core = Core::new().unwrap();
    let client = client();

    let futures = vec![
        client.get_forecast(&token[..], 37.8267, -122.423),
        client.get_forecast(&token[..], 39.9042, 166.4074),
        client.get_forecast(&token[..], 19.2465, -99.1013),
    ];

    let done = future::join_all(futures)
        .and_then(|_| {
            assert!(true);

            Ok(())
        }).or_else(|_| {
            assert!(false);

            Err(())
        });

    core.run(done).expect("core err");
}

#[ignore]
#[test]
fn test_get_forecast_with_options() {
    let token = env::var("FORECAST_TOKEN").expect("forecast token");

    let mut core = Core::new().unwrap();
    let client = client();

    let done = client
        .get_forecast_with_options(&token[..], 19.2465, -99.1013, |opt| {
            opt.exclude(vec![Block::Currently, Block::Daily])
                .extend_hourly()
                .language(Language::Es)
                .unit(Unit::Si)
        }).and_then(|_| {
            assert!(true);

            Ok(())
        }).or_else(|_| {
            assert!(false);

            Err(())
        });

    core.run(done).expect("core err");
}

#[ignore]
#[test]
fn test_time_machine() {
    let token = env::var("FORECAST_TOKEN").expect("forecast token");

    let mut core = Core::new().unwrap();
    let client = client();

    let done = client
        .get_forecast_time_machine(&token[..], 19.2465, -99.1013, 1_450_000_000, |opt| {
            opt.exclude(vec![Block::Currently, Block::Daily])
                .extend_hourly()
                .language(Language::Es)
                .unit(Unit::Si)
        }).map(|_forecast| {
            assert!(true);

            ()
        }).map_err(|why| {
            panic!("{:?}", why);

            ()
        });

    core.run(done).expect("core err");
}
