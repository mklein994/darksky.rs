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
//! A set of models representing data received by the API.

/// A safe representation of the indicated weather. This is useful for matching
/// and presenting an emoji or other weather symbol or representation.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Icon {
    /// The day's sky is clear.
    #[serde(rename = "clear-day")]
    ClearDay,
    /// The night sky is clear.
    #[serde(rename = "clear-night")]
    ClearNight,
    /// The sky is cloudy.
    #[serde(rename = "cloudy")]
    Cloudy,
    /// It is foggy.
    #[serde(rename = "fog")]
    Fog,
    /// Not actively in use
    #[serde(rename = "hail")]
    Hail,
    /// The day's sky is partly cloudy.
    #[serde(rename = "partly-cloudy-day")]
    PartlyCloudyDay,
    /// The night's sky is partly night.
    #[serde(rename = "partly-cloudy-night")]
    PartlyCloudyNight,
    /// The weather is rain.
    #[serde(rename = "rain")]
    Rain,
    /// The weather is sleet.
    #[serde(rename = "sleet")]
    Sleet,
    /// The weather is snow.
    #[serde(rename = "snow")]
    Snow,
    /// Not actively in use
    #[serde(rename = "thunderstorm")]
    Thunderstorm,
    /// Not actively in use
    #[serde(rename = "tornado")]
    Tornado,
    /// The weather is windy.
    #[serde(rename = "wind")]
    Wind,
}

/// The type of precipitation that is happening within a [`Datapoint`].
///
/// [`Datapoint`]: struct.Datapoint.html
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum PrecipitationType {
    /// Indicator that the type of precipitation is rain.
    #[serde(rename = "rain")]
    Rain,
    /// Indicator that the type of precipitation is sleet.
    #[serde(rename = "sleet")]
    Sleet,
    /// Indicator that the type of precipitation is snow.
    #[serde(rename = "snow")]
    Snow,
}

/// The severity of the weather alert.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// An individual should be aware of potentially severe weather.
    Advisory,
    /// An individual should prepare for potentially severe weather
    Watch,
    /// An individual should take immediate action to protect themselves and others from
    /// potentially severe weather.
    Warning,
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
    pub expires: u64,
    /// A detailed description of the alert.
    pub description: String,
    /// A short text summary.
    pub title: String,
    /// A URI that contains detailed information about the alert.
    pub uri: String,
    /// An array of strings representing the names of the regions covered by this weather alert.
    pub regions: Vec<String>,
    /// The UNIX time at which the alert was issued.
    pub time: u64,
    /// The severity of the weather alert.
    pub severity: Severity,
}

/// A block of data within a [`Forecast`], with potentially many [`Datapoint`]s.
///
/// [`Datapoint`]: struct.Datapoint.html
/// [`Forecast`]: struct.Forecast.html
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Datablock {
    /// The data for the datablock, if there is any data available.
    pub data: Option<Vec<Datapoint>>,
    /// The icon representing the weather type for the datablock.
    pub icon: Option<Icon>,
    /// A written summary of the datablock's expected weather.
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
#[serde(rename_all = "camelCase")]
pub struct Datapoint {
    /// The unix timestamp representing when the daytime high apparent
    /// temperature occurs.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub apparent_temperature_max_time: Option<u64>,
    /// The daytime high apparent temperature.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub apparent_temperature_max: Option<f64>,
    /// The unix timestamp representing when the overnight low apparent
    /// temperature occurs.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub apparent_temperature_min_time: Option<u64>,
    /// The overnight low apparent temperature.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub apparent_temperature_min: Option<f64>,
    /// The apparent (or "feels like") temperature in degrees Fahrenheit.
    ///
    /// **Note**: This is not present on `daily`.
    pub apparent_temperature: Option<f64>,
    /// The amount of error possible within the [`cloud_cover`] value.
    ///
    /// [`cloud_cover`]: #structfield.cloud_cover
    pub cloud_cover_error: Option<f64>,
    /// The percentage of sky occluded by clouds.
    ///
    /// This value is between `0` and `1`, inclusively.
    pub cloud_cover: Option<f64>,
    /// The amount of error possible within the [`dew_point`] value.
    ///
    /// [`dew_point`]: #structfield.dew_point
    pub dew_point_error: Option<f64>,
    /// The dew point in degrees Fahrenheit.
    pub dew_point: Option<f64>,
    /// The amount of error possible within the [`humidity`] value.
    ///
    /// [`humidity`]: #structfield.humidity
    pub humidity_error: Option<f64>,
    /// The relative humidity.
    ///
    /// This value is between `0` and `1`, inclusively.
    pub humidity: Option<f64>,
    /// A machine-readable summary of the datapoint, suitable for selecting an
    /// icon to display.
    pub icon: Option<Icon>,
    /// The fractional part of the [lunation number] during the given day.
    ///
    /// A value of `0` corresponds to a new moon, `0.25` to a first quarter
    /// moon, `0.5` to a full moon, `0.75` to a last quarter moon.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub moon_phase: Option<f64>,
    /// The approximate direction of the nearest storm in degrees, with true
    /// north at 0 degrees and progressing clockwise.
    ///
    /// If `nearestStormDistance` is `0`, then this value will not be present.
    ///
    /// **Note**: This is only present on the `currently` block.
    pub nearest_storm_bearing: Option<f64>,
    /// The approximate distance to the nearest storm in miles.
    ///
    /// A storm distance of `0` doesn't necessarily refer to a storm at the
    /// requested location, but rather a storm in the vicinity of that location.
    ///
    /// **Note**: This is only present on the `currently` block.
    pub nearest_storm_distance: Option<f64>,
    /// The amount of error possible within the [`ozone`] value.
    ///
    /// [`ozone`]: #structfield.ozone
    pub ozone_error: Option<f64>,
    /// The columnar density of total atmospheric ozone at the given time in
    /// Dobson units.
    pub ozone: Option<f64>,
    /// The amount of error possible within the [`precip_accumulation`] value.
    ///
    /// [`precip_accumulation`]: #structfield.precip_accumulation
    pub precip_accumulation_error: Option<f64>,
    /// The amount of snowfall accumulation expected to occur, in inches.
    ///
    /// If no snowfall is expected, this will be None.
    ///
    /// **Note**: This is only present on `hourly` and `daily` blocks.
    pub precip_accumulation: Option<f64>,
    /// The amount of error possible within the [`precip_intensity`] value.
    ///
    /// [`precip_intensity`]: #structfield.precip_intensity
    pub precip_intensity_error: Option<f64>,
    /// The amount of error possible within the [`precip_intensity_max`] value.
    ///
    /// [`precip_intensity_max`]: #structfield.precip_intensity_max
    pub precip_intensity_max_error: Option<f64>,
    /// The unix timestamp of when [`precip_intensity_max`] occurs during a
    /// given day.
    ///
    /// **Note**: This is only present on the `daily` block.
    ///
    /// [`precip_intensity_max`]: #structfield.precip_intensity_max
    pub precip_intensity_max_time: Option<u64>,
    /// The maximum value of [`precip_intensity`] during a given day.
    ///
    /// **Note**: This is only present on the `daily` block.
    ///
    /// [`precip_intensity`]: #structfield.precip_intensity
    pub precip_intensity_max: Option<f64>,
    /// The intensity (in inches of liquid water per hour) precipitation
    /// occurring at the given time.
    ///
    /// This value is conditional on probability (that is, assuming any
    /// precipitation occurs at all) for `minutely` datapoints, and
    /// unconditional otherwise.
    pub precip_intensity: Option<f64>,
    /// The amount of error possible within the [`precip_probability`] value.
    ///
    /// [`precip_probability`]: #structfield.precip_probability
    pub precip_probability_error: Option<f64>,
    /// The probably of precipitation occurring.
    ///
    /// This value is between `0` and `1`, inclusively.
    pub precip_probability: Option<f64>,
    /// The type of precipitation occurring at a given time.
    ///
    /// If [`precip_intensity`] is `0`, then this field will be `None`.
    ///
    /// Additionally, due to the lack of data in DarkSky sources, historical
    /// `precip_type` values is usually estimated, rather than observed.
    ///
    /// [`precip_intensity`]: #structfield.precip_intensity
    pub precip_type: Option<PrecipitationType>,
    /// The amount of error possible within the [`pressure`] value.
    ///
    /// [`pressure`]: #structfield.pressure
    pub pressure_error: Option<f64>,
    /// The sea-level air pressure in millibars.
    pub pressure: Option<f64>,
    /// A human-readable text summary of the datapoint.
    ///
    /// **Note**: Do not use this for automated icon display purposes, use the
    /// [`icon`] field instead.
    ///
    /// [`icon`]: #structfield.icon
    pub summary: Option<String>,
    /// The unix timestamp of when the sun will rise during a given day.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub sunrise_time: Option<u64>,
    /// The unix timestamp of when the sun will set during a given day.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub sunset_time: Option<u64>,
    /// The overnight low temperature.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub temperature_low: Option<f64>,
    /// The unix timestamp representing when the overnight low temperature
    /// occurs.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub temperature_low_time: Option<u64>,
    /// The daytime high temperature.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub temperature_high: Option<f64>,
    /// The unix timestamp representing when the daytime high temperature
    /// occurs.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub temperature_high_time: Option<u64>,
    /// The amount of error possible within the [`temperature_max`] value.
    ///
    /// [`temperature_max`]: #structfield.temperature_max
    pub temperature_max_error: Option<f64>,
    /// The unix timestamp representing when the maximum temperature during a
    /// given date occurs.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub temperature_max_time: Option<u64>,
    /// The maximum temperature during a given date.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub temperature_max: Option<f64>,
    /// The amount of error possible within the [`temperature_min`] value.
    ///
    /// [`temperature_min`]: #structfield.temperature_min
    pub temperature_min_error: Option<f64>,
    /// The unix timestamp representing when the minimum temperature during a
    /// given date occurs.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub temperature_min_time: Option<u64>,
    /// The minimum temperature during a given date.
    ///
    /// **Note**: This is only present on the `daily` block.
    pub temperature_min: Option<f64>,
    /// The amount of error possible within the [`temperature`] value.
    ///
    /// [`temperature`]: #structfield.temperature
    pub temperature_error: Option<f64>,
    /// The air temperature in degrees Fahrenheit.
    pub temperature: Option<f64>,
    /// The unix timestamp at which the datapoint begins.
    ///
    /// `minutely` datapoints are always aligned to the top of the minute.
    ///
    /// `hourly` datapoints align to the top of the hour.
    ///
    /// `daily` datapoints align to midnight of the day.
    ///
    /// All are according to the local timezone.
    pub time: u64,
    /// The UV index.
    pub uv_index: Option<u64>,
    /// The unix timestamp of when the maximum [`uv_index`] occurs during the
    /// given day.
    ///
    /// [`uv_index`]: #structfield.uv_index
    pub uv_index_time: Option<u64>,
    /// The amount of error possible within the [`visibility`] value.
    ///
    /// [`visibility`]: #structfield.visibility
    pub visibility_error: Option<f64>,
    /// The average visibility in miles, capped at 10 miles.
    pub visibility: Option<f64>,
    /// The amount of error possible within the [`wind_bearing`] value.
    ///
    /// [`wind_bearing`]: #structfield.wind_bearing
    pub wind_bearing_error: Option<f64>,
    /// The direction that the wind is coming from in degrees.
    ///
    /// True north is at 0 degrees, progressing clockwise.
    ///
    /// If [`wind_speed`] is `0`, then this value will not be defined.
    ///
    /// [`wind_speed`]: #structfield.wind_speed
    pub wind_bearing: Option<f64>,
    /// The wind gust speed in miles per hour.
    pub wind_gust: Option<f64>,
    /// The amount of time that the wind gust is expected to occur.
    pub wind_gust_time: Option<u64>,
    /// The amount of error possible within the [`wind_speed`] value.
    ///
    /// [`wind_speed`]: #structfield.wind_speed
    pub wind_speed_error: Option<f64>,
    /// The wind speed in miles per hour.
    pub wind_speed: Option<f64>,
}

/// A set of flags for a forecast, such as the [`Unit`]s specified or the vector
/// of [DarkSky] stations reporting.
///
/// [`Unit`]: ../enum.Unit.html
/// [DarkSky]: https://darksky.net
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
    /// [`Unit`]: ../enum.Unit.html
    pub units: Option<String>,
}

/// A full forecast returned from the `get_forecast` and
/// `get_forecast_with_options` functions.
///
/// The functions for hyper: [`get_forecast`][hyper `get_forecast`] and
/// [`get_forecast_with_options`][hyper `get_forecast_with_options`].
///
/// And for reqwest: [`get_forecast`][hyper `get_forecast`] and
/// [`get_forecast_with_options`][hyper `get_forecast_with_options`].
///
/// Most of the fields are optional, due to being able to be excluded via the
/// [`Options`] builder.
///
/// [`Options`]: ../struct.Options.html
/// [hyper `get_forecast`]:
///   ../bridge/hyper/trait.DarkskyHyperRequester.html#tymethod.get_forecast
/// [reqwest `get_forecast`]:
///   ../bridge/reqwest/trait.DarkskyReqwestRequester.html#tymethod.get_forecast
/// [hyper `get_forecast_with_options`]:
///   ../bridge/hyper/trait.DarkskyHyperRequester.html#tymethod.get_forecast_with_options
/// [reqwest `get_forecast_with_options`]:
///   ../bridge/reqwest/trait.DarkskyReqwestRequester.html#tymethod.get_forecast_with_options
/// [docs]: https://darksky.net/dev/docs/forecast

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Forecast {
    /// Contains any severe weather alerts pertinent to the location.
    #[serde(default)]
    pub alerts: Vec<Alert>,
    /// The current forecast.
    ///
    /// This may be excluded by passing the [`Block::Currently`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Currently`]: ../enum.Block.html#variant.Currently
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: ../struct.Options.html#method.exclude
    pub currently: Option<Datapoint>,
    /// Daily [`Datablock`]s within a forecast.
    ///
    /// This may be excluded by passing the [`Block::Daily`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Daily`]: ../enum.Block.html#variant.Daily
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: ../struct.Options.html#method.exclude
    pub daily: Option<Datablock>,
    /// A set of flags returned from the API.
    ///
    /// This may be excluded by passing the [`Block::Flags`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Flags`]: ../enum.Block.html#variant.Flags
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: ../struct.Options.html#method.exclude
    pub flags: Option<Flags>,
    /// Hourly [`Datablock`]s within a forecast.
    ///
    /// This may be excluded by passing the [`Block::Hourly`] variant to
    /// [`Options::exclude`].
    ///
    /// [`Block::Hourly`]: ../enum.Block.html#variant.Hourly
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: ../struct.Options.html#method.exclude
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
    /// [`Block::Minutely`]: ../enum.Block.html#variant.Minutely
    /// [`Datablock`]: struct.Datablock.html
    /// [`Options::exclude`]: ../struct.Options.html#method.exclude
    pub minutely: Option<Datablock>,
    /// The timezone offset of the forecast, relative to the UTC timezone.
    pub offset: Option<f64>,
    /// The name of the timezone.
    pub timezone: String,
}
