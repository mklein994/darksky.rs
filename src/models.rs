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

/// A safe representation of the indicated weather. This is useful for matching
/// and presenting an emoji or other weather symbol or representation.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Icon {
    /// The day's sky is clear.
    #[serde(rename="clear-day")]
    ClearDay,
    /// The night sky is clear.
    #[serde(rename="clear-night")]
    ClearNight,
    /// The sky is cloudy.
    #[serde(rename="cloudy")]
    Cloudy,
    /// It is foggy.
    #[serde(rename="fog")]
    Fog,
    /// Not actively in use
    #[serde(rename="hail")]
    Hail,
    /// The day's sky is partly cloudy.
    #[serde(rename="partly-cloudy-day")]
    PartlyCloudyDay,
    /// The night's sky is partly night.
    #[serde(rename="partly-cloudy-night")]
    PartlyCloudyNight,
    /// The weather is rain.
    #[serde(rename="rain")]
    Rain,
    /// The weather is sleet.
    #[serde(rename="sleet")]
    Sleet,
    /// The weather is snow.
    #[serde(rename="snow")]
    Snow,
    /// Not actively in use
    #[serde(rename="thunderstorm")]
    Thunderstorm,
    /// Not actively in use
    #[serde(rename="tornado")]
    Tornado,
    /// The weather is windy.
    #[serde(rename="wind")]
    Wind,
}

/// The type of precipitation that is happening within a [`Datapoint`].
///
/// [`Datapoint`]: struct.Datapoint.html
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum PrecipitationType {
    #[serde(rename="rain")]
    Rain,
    #[serde(rename="sleet")]
    Sleet,
    #[serde(rename="snow")]
    Snow,
}

/// A textual, expiring severe weather warning issued for a location. There may
/// be multiple alerts per [`Forecast`].
///
/// [`Forecast`]: struct.Forecast.html
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Alert {
    /// [Unix timestamp][unixtime] of when the alert expires.
    ///
    /// [unixtime]: https://en.wikipedia.org/wiki/Unix_time
    pub expires: Option<u64>,
    /// A detailed description of the alert.
    pub description: String,
    /// A short text summary.
    pub title: String,
    /// A URI that contains detailed information about the alert.
    pub uri: String,
}

/// A block of data within a [`Forecast`], with potentially many [`Datapoint`]s.
///
/// [`Datapoint`]: struct.Datapoint.html
/// [`Forecast`]: struct.Forecast.html
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Datablock {
    pub data: Option<Vec<Datapoint>>,
    pub icon: Option<Icon>,
    pub summary: Option<String>,
}

/// A datapoint within a [`Datablock`], where there is usually multiple.
///
/// All fields are optional _except for [`time`]_, as some data may not be
/// available for a location at a given point in time.
///
/// All of the data oriented fields may have associated `error` fields,
/// representing the confidence in a prediction or value. An example is
/// [`precip_accumulation`], which has an associated error field of
/// [`precip_accumulation_error`]. Those fields represent standard deviations of
/// the value of the associated field. Smaller error values represent greater
/// confidence levels, while larger error values represent less confidence.
/// These fields are omitted where the confidence is not precisely known.
///
/// [`Datablock`]: struct.Datablock.html
/// [`time`]: #structfield.time
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
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
    pub temperature_low: Option<f64>,
    pub temperature_low_time: Option<u64>,
    pub temperature_high: Option<f64>,
    pub temperature_high_time: Option<u64>,
    pub temperature_max_error: Option<f64>,
    pub temperature_max_time: Option<u64>,
    pub temperature_max: Option<f64>,
    pub temperature_min_error: Option<f64>,
    pub temperature_min_time: Option<u64>,
    pub temperature_min: Option<f64>,
    pub temperature_error: Option<f64>,
    pub temperature: Option<f64>,
    pub time: u64,
    pub uv_index: Option<u64>,
    pub uv_index_time: Option<u64>,
    pub visibility_error: Option<f64>,
    pub visibility: Option<f64>,
    pub wind_bearing_error: Option<f64>,
    pub wind_bearing: Option<f64>,
    pub wind_gust: Option<f64>,
    pub wind_gust_time: Option<u64>,
    pub wind_speed_error: Option<f64>,
    pub wind_speed: Option<f64>,
}

/// A set of flags for a forecast, such as the [`Unit`]s specified or the vector
/// of [DarkSky] stations reporting.
///
/// [`Unit`]: enum.Unit.html
/// [DarkSky]: https://darksky.net
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all="kebab-case")]
pub struct Flags {
    /// A list of DarkSky stations used for the [`Forecast`].
    ///
    /// [`Forecast`]: struct.Forecast.html
    pub darksky_stations: Option<Vec<String>>,
    /// A list of the unavailable DarkSky stations.
    pub darksky_unavailable: Option<String>,
    /// A list of the
    pub datapoint_stations: Option<Vec<String>>,
    /// A list of [ISD] stations used.
    ///
    /// [ISD]: https://www.ncdc.noaa.gov/isd
    pub isd_stations: Option<Vec<String>>,
    /// A list of [LAMP] stations used to obtain the information.
    ///
    /// [LAMP]: http://www.nws.noaa.gov/mdl/lamp/lamp_info.shtml
    pub lamp_stations: Option<Vec<String>>,
    /// A list of [METAR] stations used to obtain the information.
    ///
    /// [METAR]: https://www.aviationweather.gov/metar
    pub metar_stations: Option<Vec<String>>,
    /// The [METNO license] used.
    ///
    /// [METNO license]: http://www.met.no/
    pub metno_license: Option<String>,
    /// A list of sources used to obtain the information.
    pub sources: Option<Vec<String>>,
    /// The [`Unit`]s used to format the data.
    ///
    /// [`Unit`]: enum.Unit.html
    pub units: Option<String>,
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Forecast {
    #[serde(default)]
    pub alerts: Vec<Alert>,
    /// The current forecast.
    ///
    /// This may be excluded by passing the [`Block::Currently`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Currently`]: enum.Block.html#variant.Currently
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: struct.Options.html#method.exclude
    pub currently: Option<Datapoint>,
    /// Daily [`Datablock`]s within a forecast.
    ///
    /// This may be excluded by passing the [`Block::Daily`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Daily`]: enum.Block.html#variant.Daily
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: struct.Options.html#method.exclude
    pub daily: Option<Datablock>,
    /// A set of flags returned from the API.
    ///
    /// This may be excluded by passing the [`Block::Flags`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Flags`]: enum.Block.html#variant.Flags
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: struct.Options.html#method.exclude
    pub flags: Option<Flags>,
    /// Hourly [`Datablock`]s within a forecast.
    ///
    /// This may be excluded by passing the [`Block::Hourly`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Hourly`]: enum.Block.html#variant.Hourly
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: struct.Options.html#method.exclude
    pub hourly: Option<Datablock>,
    /// The latitude of the forecast's location.
    pub latitude: f64,
    /// The longitude of the forecast's location.
    pub longitude: f64,
    /// Minutely [`Datablock`]s within a forecast.
    ///
    /// This may be excluded by passing the [`Block::Minutely`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Minutely`]: enum.Block.html#variant.Minutely
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: struct.Options.html#method.exclude
    pub minutely: Option<Datablock>,
    /// The timezone offset of the forecast, relative to the UTC timezone.
    pub offset: Option<f64>,
    /// The name of the timezone.
    pub timezone: String,
}
