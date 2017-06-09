extern crate darksky;

use darksky::*;
use std::env;

#[test]
fn test_get_forecast() {
    let token = env::var("FORECAST_TOKEN").expect("forecast token");

    get_forecast(&token[..], 37.8267, -122.423).unwrap();
    get_forecast(&token[..], 39.9042, 116.4074).unwrap();
    get_forecast(&token[..], 19.2465, -99.1013).unwrap();
}

#[test]
fn test_get_forecast_with_options() {
    let token = env::var("FORECAST_TOKEN").expect("forecast token");

    get_forecast_with_options(&token[..], 19.2465, -99.1013, |opt| {
        opt.exclude(vec![Block::Currently, Block::Daily])
           .extend_hourly()
           .language(Language::Es)
           .unit(Unit::Si)
    }).unwrap();
}
