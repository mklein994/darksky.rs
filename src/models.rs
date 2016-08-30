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

use serde_json::Value;
use ::error::{Error, Result};
use ::utils::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum Icon {
    ClearDay,
    ClearNight,
    Cloudy,
    Fog,
    /// Not actively in use
    Hail,
    PartlyCloudyDay,
    PartlyCloudyNight,
    Rain,
    Sleet,
    Snow,
    /// Not actively in use
    Thunderstorm,
    /// Not actively in use
    Tornado,
    Wind,
}

map_names! { Icon;
    ClearDay, "clear-day";
    ClearNight, "clear-night";
    Cloudy, "cloudy";
    Fog, "fog";
    Hail, "hail";
    PartlyCloudyDay, "partly-cloudy-day";
    PartlyCloudyNight, "partly-cloudy-night";
    Rain, "rain";
    Sleet, "sleet";
    Snow, "snow";
    Thunderstorm, "thunderstorm";
    Tornado, "tornado";
    Wind, "wind";
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum PrecipitationType {
    Rain,
    Sleet,
    Snow,
}

map_names! { PrecipitationType;
    Rain, "rain";
    Sleet, "sleet";
    Snow, "snow";
}

#[derive(Clone, Debug)]
pub struct Alert {
    pub expires: u64,
    pub description: String,
    pub title: String,
    pub uri: String,
}

impl Alert {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Alert> {
        let mut value = try!(into_map(value));

        Ok(Alert {
            description: try!(remove(&mut value, "description").and_then(into_string)),
            expires: req!(try!(remove(&mut value, "expires")).as_u64()),
            title: try!(remove(&mut value, "title").and_then(into_string)),
            uri: try!(remove(&mut value, "uri").and_then(into_string)),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Current {
    pub apparent_temperature: f64,
    pub cloud_cover: f64,
    pub dew_point: f64,
    pub humidity: f64,
    pub icon: Icon,
    pub nearest_storm_bearing: Option<u64>,
    pub nearest_storm_distance: Option<u64>,
    pub ozone: f64,
    pub precip_intensity: u64,
    pub precip_probability: u64,
    pub pressure: f64,
    pub summary: String,
    pub temperature: f64,
    pub time: u64,
    pub visibility: Option<f64>,
    pub wind_bearing: u64,
    pub wind_speed: f64,
}

impl Current {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Current> {
        let mut value = try!(into_map(value));

        Ok(Current {
            apparent_temperature: req!(try!(remove(&mut value, "apparentTemperature")).as_f64()),
            cloud_cover: req!(try!(remove(&mut value, "cloudCover")).as_f64()),
            dew_point: req!(try!(remove(&mut value, "dewPoint")).as_f64()),
            humidity: req!(try!(remove(&mut value, "humidity")).as_f64()),
            icon: try!(remove(&mut value, "icon").and_then(Icon::decode)),
            nearest_storm_bearing: remove(&mut value, "nearestStormBearing").ok().and_then(|v| v.as_u64()),
            nearest_storm_distance: remove(&mut value, "nearestStormDistance").ok().and_then(|v| v.as_u64()),
            ozone: req!(try!(remove(&mut value, "ozone")).as_f64()),
            precip_intensity: req!(try!(remove(&mut value, "precipIntensity")).as_u64()),
            precip_probability: req!(try!(remove(&mut value, "precipProbability")).as_u64()),
            pressure: req!(try!(remove(&mut value, "pressure")).as_f64()),
            summary: try!(remove(&mut value, "summary").and_then(into_string)),
            temperature: req!(try!(remove(&mut value, "temperature")).as_f64()),
            time: req!(try!(remove(&mut value, "time")).as_u64()),
            visibility: remove(&mut value, "visibility").ok().and_then(|v| v.as_f64()),
            wind_bearing: req!(try!(remove(&mut value, "windBearing")).as_u64()),
            wind_speed: req!(try!(remove(&mut value, "windSpeed")).as_f64()),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Daily {
    pub data: Vec<DailyData>,
    pub icon: Icon,
    pub summary: String,
}

impl Daily {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Daily> {
        let mut value = try!(into_map(value));

        Ok(Daily {
            data: try!(decode_array(try!(remove(&mut value, "data")), DailyData::decode)),
            icon: try!(remove(&mut value, "icon").and_then(Icon::decode)),
            summary: try!(remove(&mut value, "summary").and_then(into_string)),
        })
    }
}

#[derive(Clone, Debug)]
pub struct DailyData {
    pub apparent_temperature_max_time: u64,
    pub apparent_temperature_max: f64,
    pub apparent_temperature_min_time: u64,
    pub apparent_temperature_min: f64,
    pub cloud_cover: f64,
    pub dew_point: f64,
    pub humidity: f64,
    pub icon: Icon,
    pub moon_phase: f64,
    pub ozone: f64,
    pub precip_intensity_max: f64,
    pub precip_intensity: f64,
    pub precip_probability: f64,
    pub precip_type: Option<PrecipitationType>,
    pub pressure: f64,
    pub summary: String,
    pub sunrise_time: u64,
    pub sunset_time: u64,
    pub temperature_max_time: u64,
    pub temperature_max: f64,
    pub temperature_min_time: u64,
    pub temperature_min: f64,
    pub time: u64,
    pub visibility: Option<f64>,
    pub wind_bearing: f64,
    pub wind_speed: f64,
}

impl DailyData {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<DailyData> {
        let mut value = try!(into_map(value));

        Ok(DailyData {
            apparent_temperature_max_time: req!(try!(remove(&mut value, "apparentTemperatureMaxTime")).as_u64()),
            apparent_temperature_max: req!(try!(remove(&mut value, "apparentTemperatureMax")).as_f64()),
            apparent_temperature_min_time: req!(try!(remove(&mut value, "apparentTemperatureMinTime")).as_u64()),
            apparent_temperature_min: req!(try!(remove(&mut value, "apparentTemperatureMin")).as_f64()),
            cloud_cover: req!(try!(remove(&mut value, "cloudCover")).as_f64()),
            dew_point: req!(try!(remove(&mut value, "dewPoint")).as_f64()),
            humidity: req!(try!(remove(&mut value, "humidity")).as_f64()),
            icon: try!(remove(&mut value, "icon").and_then(Icon::decode)),
            moon_phase: req!(try!(remove(&mut value, "moonPhase")).as_f64()),
            ozone: req!(try!(remove(&mut value, "ozone")).as_f64()),
            precip_intensity_max: req!(try!(remove(&mut value, "precipIntensityMax")).as_f64()),
            precip_intensity: req!(try!(remove(&mut value, "precipIntensity")).as_f64()),
            precip_probability: req!(try!(remove(&mut value, "precipProbability")).as_f64()),
            precip_type: try!(opt(&mut value, "precipType", PrecipitationType::decode)),
            pressure: req!(try!(remove(&mut value, "pressure")).as_f64()),
            summary: try!(remove(&mut value, "summary").and_then(into_string)),
            sunrise_time: req!(try!(remove(&mut value, "sunriseTime")).as_u64()),
            sunset_time: req!(try!(remove(&mut value, "sunsetTime")).as_u64()),
            temperature_max_time: req!(try!(remove(&mut value, "temperatureMaxTime")).as_u64()),
            temperature_max: req!(try!(remove(&mut value, "temperatureMax")).as_f64()),
            temperature_min_time: req!(try!(remove(&mut value, "temperatureMinTime")).as_u64()),
            temperature_min: req!(try!(remove(&mut value, "temperatureMin")).as_f64()),
            time: req!(try!(remove(&mut value, "time")).as_u64()),
            visibility: remove(&mut value, "visibility").ok().and_then(|v| v.as_f64()),
            wind_bearing: req!(try!(remove(&mut value, "windBearing")).as_f64()),
            wind_speed: req!(try!(remove(&mut value, "windSpeed")).as_f64()),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub darksky_stations: Option<Vec<String>>,
    pub darksky_unavailable: Option<String>,
    pub datapoint_stations: Option<Vec<String>>,
    pub isd_stations: Option<Vec<String>>,
    pub lamp_stations: Option<Vec<String>>,
    pub metar_stations: Option<Vec<String>>,
    pub metno_license: Option<String>,
    pub sources: Option<Vec<String>>,
    pub units: Option<String>,
}

impl Flags {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Flags> {
        let mut value = try!(into_map(value));

        Ok(Flags {
            darksky_stations: try!(opt(&mut value, "darksky-stations", |v| decode_array(v, into_string))),
            darksky_unavailable: try!(opt(&mut value, "darksky-unavailable", into_string)),
            datapoint_stations: try!(opt(&mut value, "datapoint-stations", |v| decode_array(v, into_string))),
            isd_stations: try!(opt(&mut value, "isd-stations", |v| decode_array(v, into_string))),
            lamp_stations: try!(opt(&mut value, "lamp-stations", |v| decode_array(v, into_string))),
            metar_stations: try!(opt(&mut value, "metar-stations", |v| decode_array(v, into_string))),
            metno_license: try!(opt(&mut value, "metno-license", into_string)),
            sources: try!(opt(&mut value, "sources", |v| decode_array(v, into_string))),
            units: try!(opt(&mut value, "units", into_string)),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Forecast {
    pub alerts: Vec<Alert>,
    pub currently: Current,
    pub daily: Daily,
    pub flags: Flags,
    pub hourly: Hourly,
    pub latitude: f64,
    pub longitude: f64,
    pub minutely: Option<Minutely>,
    pub offset: f64,
    pub timezone: String,
}

impl Forecast {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Forecast> {
        let mut value = try!(into_map(value));

        Ok(Forecast {
            alerts: try!(opt(&mut value, "alerts", |v| decode_array(v, Alert::decode))).unwrap_or(vec![]),
            currently: try!(remove(&mut value, "currently").and_then(Current::decode)),
            daily: try!(remove(&mut value, "daily").and_then(Daily::decode)),
            flags: try!(remove(&mut value, "flags").and_then(Flags::decode)),
            hourly: try!(remove(&mut value, "hourly").and_then(Hourly::decode)),
            latitude: req!(try!(remove(&mut value, "latitude")).as_f64()),
            longitude: req!(try!(remove(&mut value, "longitude")).as_f64()),
            minutely: try!(opt(&mut value, "minutely", Minutely::decode)),
            offset: req!(try!(remove(&mut value, "offset")).as_f64()),
            timezone: try!(remove(&mut value, "timezone").and_then(into_string)),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Hourly {
    pub data: Vec<HourlyData>,
    pub icon: Icon,
    pub summary: String,
}

impl Hourly {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Hourly> {
        let mut value = try!(into_map(value));

        Ok(Hourly {
            data: try!(decode_array(try!(remove(&mut value, "data")), HourlyData::decode)),
            icon: try!(remove(&mut value, "icon").and_then(Icon::decode)),
            summary: try!(remove(&mut value, "summary").and_then(into_string)),
        })
    }
}

#[derive(Clone, Debug)]
pub struct HourlyData {
    pub apparent_temperature: f64,
    pub cloud_cover: f64,
    pub dew_point: f64,
    pub humidity: f64,
    pub icon: Icon,
    pub ozone: f64,
    pub precip_intensity: f64,
    pub precip_probability: f64,
    pub precip_type: Option<PrecipitationType>,
    pub pressure: f64,
    pub summary: String,
    pub temperature: f64,
    pub time: u64,
    pub visibility: Option<f64>,
    pub wind_bearing: f64,
    pub wind_speed: f64,
}

impl HourlyData {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<HourlyData> {
        let mut value = try!(into_map(value));

        Ok(HourlyData {
            apparent_temperature: req!(try!(remove(&mut value, "apparentTemperature")).as_f64()),
            cloud_cover: req!(try!(remove(&mut value, "cloudCover")).as_f64()),
            dew_point: req!(try!(remove(&mut value, "dewPoint")).as_f64()),
            humidity: req!(try!(remove(&mut value, "humidity")).as_f64()),
            icon: try!(remove(&mut value, "icon").and_then(Icon::decode)),
            ozone: req!(try!(remove(&mut value, "ozone")).as_f64()),
            precip_intensity: req!(try!(remove(&mut value, "precipIntensity")).as_f64()),
            precip_probability: req!(try!(remove(&mut value, "precipProbability")).as_f64()),
            precip_type: try!(opt(&mut value, "precipType", PrecipitationType::decode)),
            pressure: req!(try!(remove(&mut value, "pressure")).as_f64()),
            summary: try!(remove(&mut value, "summary").and_then(into_string)),
            temperature: req!(try!(remove(&mut value, "temperature")).as_f64()),
            time: req!(try!(remove(&mut value, "time")).as_u64()),
            visibility: remove(&mut value, "visibility").ok().and_then(|v| v.as_f64()),
            wind_bearing: req!(try!(remove(&mut value, "windBearing")).as_f64()),
            wind_speed: req!(try!(remove(&mut value, "windSpeed")).as_f64()),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Minutely {
    pub data: Vec<MinutelyData>,
    pub icon: Icon,
    pub summary: String,
}

impl Minutely {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Minutely> {
        let mut value = try!(into_map(value));

        Ok(Minutely {
            data: try!(decode_array(try!(remove(&mut value, "data")), MinutelyData::decode)),
            icon: try!(remove(&mut value, "icon").and_then(Icon::decode)),
            summary: try!(remove(&mut value, "summary").and_then(into_string)),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MinutelyData {
    pub precip_intensity: f64,
    pub precip_probability: f64,
    pub time: u64,
}

impl MinutelyData {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<MinutelyData> {
        let mut value = try!(into_map(value));

        Ok(MinutelyData {
            precip_intensity: req!(try!(remove(&mut value, "precipIntensity")).as_f64()),
            precip_probability: req!(try!(remove(&mut value, "precipProbability")).as_f64()),
            time: req!(try!(remove(&mut value, "time")).as_u64()),
        })
    }
}
