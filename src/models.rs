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

/// A safe representation of the indicated weather. This is useful for matching
/// and presenting an emoji or other weather symbol or representation.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum Icon {
    /// The day's sky is clear.
    ClearDay,
    /// The night sky is clear.
    ClearNight,
    /// The sky is cloudy.
    Cloudy,
    /// It is foggy.
    Fog,
    /// Not actively in use
    Hail,
    /// The day's sky is partly cloudy.
    PartlyCloudyDay,
    /// The night's sky is partly night.
    PartlyCloudyNight,
    /// The weather is rain.
    Rain,
    /// The weather is sleet.
    Sleet,
    /// The weather is snow.
    Snow,
    /// Not actively in use
    Thunderstorm,
    /// Not actively in use
    Tornado,
    /// The weather is windy.
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

/// The type of precipitation that is happening within a [`Datapoint`].
///
/// [`Datapoint`]: struct.Datapoint.html
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

/// A textual, expiring alert for a location. There may be multiple alerts per
/// [`Forecast`].
///
/// [`Forecast`]: struct.Forecast.html
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

/// A block of data within a [`Forecast`], with potentially many [`Datapoint`]s.
///
/// [`Datapoint`]: struct.Datapoint.html
/// [`Forecast`]: struct.Forecast.html
#[derive(Clone, Debug)]
pub struct Datablock {
    pub data: Option<Vec<Datapoint>>,
    pub icon: Option<Icon>,
    pub summary: Option<String>,
}

impl Datablock {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Datablock> {
        let mut value = try!(into_map(value));

        Ok(Datablock {
            data: try!(opt(&mut value, "data", |v| decode_array(v, Datapoint::decode))),
            icon: try!(opt(&mut value, "icon", Icon::decode)),
            summary: try!(opt(&mut value, "summary", into_string)),
        })
    }
}

/// A datapoint within a [`Datablock`], where there is usually multiple.
///
/// All fields are optional _except for [`time`]_, as some data may not be
/// available.
///
/// [`Datablock`]: struct.Datablock.html
/// [`time`]: #structfield.time
#[derive(Clone, Debug)]
pub struct Datapoint {
    pub apparent_temperature_max_time: Option<u64>,
    pub apparent_temperature_max: Option<f64>,
    pub apparent_temperature_min_time: Option<u64>,
    pub apparent_temperature_min: Option<f64>,
    pub apparent_temperature: Option<f64>,
    pub cloud_cover_error: Option<f64>,
    pub cloud_cover: Option<f64>,
    pub dew_point_error: Option<f64>,
    pub dew_point: Option<f64>,
    pub humidity_error: Option<f64>,
    pub humidity: Option<f64>,
    pub icon: Option<Icon>,
    pub moon_phase: Option<f64>,
    pub nearest_storm_bearing: Option<f64>,
    pub nearest_storm_distance: Option<f64>,
    pub ozone_error: Option<f64>,
    pub ozone: Option<f64>,
    pub precip_accumulation_error: Option<f64>,
    pub precip_accumulation: Option<f64>,
    pub precip_intensity_error: Option<f64>,
    pub precip_intensity_max_error: Option<f64>,
    pub precip_intensity_max_time: Option<u64>,
    pub precip_intensity_max: Option<f64>,
    pub precip_intensity: Option<f64>,
    pub precip_probability_error: Option<f64>,
    pub precip_probability: Option<f64>,
    pub precip_type: Option<PrecipitationType>,
    pub pressure_error: Option<f64>,
    pub pressure: Option<f64>,
    pub summary: Option<String>,
    pub sunrise_time: Option<u64>,
    pub sunset_time: Option<u64>,
    pub temperature_max_error: Option<f64>,
    pub temperature_max_time: Option<u64>,
    pub temperature_max: Option<f64>,
    pub temperature_min_error: Option<f64>,
    pub temperature_min_time: Option<u64>,
    pub temperature_min: Option<f64>,
    pub temperature_error: Option<f64>,
    pub temperature: Option<f64>,
    pub time: u64,
    pub visibility_error: Option<f64>,
    pub visibility: Option<f64>,
    pub wind_bearing_error: Option<f64>,
    pub wind_bearing: Option<f64>,
    pub wind_speed_error: Option<f64>,
    pub wind_speed: Option<f64>,
}

impl Datapoint {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Datapoint> {
        let mut map = try!(into_map(value));

        Ok(Datapoint {
            apparent_temperature_max_time: field!(map, int, "apparentTemperatureMaxTime"),
            apparent_temperature_max: field!(map, float, "apparentTemperatureMax"),
            apparent_temperature_min_time: field!(map, int, "apparentTemperatureMinTime"),
            apparent_temperature_min: field!(map, float, "apparentTemperatureMin"),
            apparent_temperature: field!(map, float, "apparentTemperature"),
            cloud_cover_error: field!(map, float, "cloudCoverError"),
            cloud_cover: field!(map, float, "cloudCover"),
            dew_point_error: field!(map, float, "dewPointError"),
            dew_point: field!(map, float, "dewPoint"),
            humidity_error: field!(map, float, "humidityError"),
            humidity: field!(map, float, "humidity"),
            icon: field!(map, O, "icon", Icon::decode),
            moon_phase: field!(map, float, "moonPhase"),
            nearest_storm_bearing: field!(map, float, "nearestStormBearing"),
            nearest_storm_distance: field!(map, float, "nearestStormDistance"),
            ozone_error: field!(map, float, "ozoneError"),
            ozone: field!(map, float, "ozone"),
            precip_accumulation_error: field!(map, float, "precipAccumulationError"),
            precip_accumulation: field!(map, float, "precipAccumulation"),
            precip_intensity_error: field!(map, float, "precipIntensityError"),
            precip_intensity_max_error: field!(map, float, "precipIntensityMaxError"),
            precip_intensity_max_time: field!(map, int, "precipIntensityMaxTime"),
            precip_intensity_max: field!(map, float, "precipIntensityMax"),
            precip_intensity: field!(map, float, "precipIntensity"),
            precip_probability_error: field!(map, float, "precipProbabilityError"),
            precip_probability: field!(map, float, "precipProbability"),
            precip_type: field!(map, O, "precipType", PrecipitationType::decode),
            pressure_error: field!(map, float, "pressureError"),
            pressure: field!(map, float, "pressure"),
            summary: field!(map, O, "summary", into_string),
            sunrise_time: field!(map, int, "sunriseTime"),
            sunset_time: field!(map, int, "sunsetTime"),
            temperature_error: field!(map, float, "temperatureError"),
            temperature_max_error: field!(map, float, "temperatureMaxError"),
            temperature_max_time: field!(map, int, "temperatureMaxTime"),
            temperature_max: field!(map, float, "temperatureMax"),
            temperature_min_error: field!(map, float, "temperatureMinError"),
            temperature_min_time: field!(map, int, "temperatureMinTime"),
            temperature_min: field!(map, float, "temperatureMin"),
            temperature: field!(map, float, "temperature"),
            time: field!(map, R, int, "time"),
            visibility_error: field!(map, float, "visibilityError"),
            visibility: field!(map, float, "visibility"),
            wind_bearing_error: field!(map, float, "windBearingError"),
            wind_bearing: field!(map, float, "windBearing"),
            wind_speed_error: field!(map, float, "windSpeedError"),
            wind_speed: field!(map, float, "windSpeed"),
        })
    }
}

/// A set of flags for a forecast, such as the [`Unit`]s specified or the vector
/// of [DarkSky] stations reporting.
///
/// [`Unit`]: enum.Unit.html
/// [DarkSky]: https://darksky.net
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

/// A full forecast returned from the [`get_forecast`] and
/// [`get_forecast_with_options`] functions.
///
/// Most of the fields are optional, due to being able to be excluded via the
/// [`Options`] builder.
///
/// [`Options`]: struct.Options.html
/// [`get_forecast`]: fn.get_forecast.html
/// [`get_forecast_with_options`]: fn.get_forecast_with_options.html
#[derive(Clone, Debug)]
pub struct Forecast {
    pub alerts: Vec<Alert>,
    pub currently: Option<Datapoint>,
    pub daily: Option<Datablock>,
    pub flags: Option<Flags>,
    pub hourly: Option<Datablock>,
    pub latitude: f64,
    pub longitude: f64,
    pub minutely: Option<Datablock>,
    pub offset: Option<f64>,
    pub timezone: String,
}

impl Forecast {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Forecast> {
        let mut map = try!(into_map(value));

        Ok(Forecast {
            alerts: try!(opt(&mut map, "alerts", |v| decode_array(v, Alert::decode))).unwrap_or(vec![]),
            currently: field!(map, O, "currently", Datapoint::decode),
            daily: field!(map, O, "daily", Datablock::decode),
            flags: field!(map, O, "flags", Flags::decode),
            hourly: field!(map, O, "hourly", Datablock::decode),
            latitude: field!(map, R, float, "latitude"),
            longitude: field!(map, R, float, "longitude"),
            minutely: field!(map, O, "minutely", Datablock::decode),
            offset: field!(map, float, "offset"),
            timezone: field!(map, R, "timezone", into_string),
        })
    }
}
