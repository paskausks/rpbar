use std::{env, f64, time::{SystemTime, UNIX_EPOCH}};
use serde::Deserialize;
use ureq::{http::Response, Body};

use crate::plugin::{Plugin, Status};

const PLUGIN_NAME: &str = "meteosource_weather";

/// How long to wait between calls. 400 requests per day.
const INTERVAL_SEC: u64 = 216;

#[derive(Deserialize)]
struct MeteoSourcePointResponse {
    current: MeteoSourceCurrentResponse
}

#[derive(Deserialize)]
struct MeteoSourceCurrentResponse {
    temperature: f64,
    icon: String,
    icon_num: usize,
    summary: String,
}

pub struct WeatherPlugin {
    api_key: Option<String>,
    point: String,
    last_request_time: u64,
    last_response: Option<MeteoSourceCurrentResponse>,
}

impl Default for WeatherPlugin {
    fn default() -> Self {
        WeatherPlugin {
            api_key: None,
            last_request_time: 0,
            last_response: None,
            point: String::new(),
        }
    }
}

impl Plugin for WeatherPlugin {
    fn setup(&mut self) {
        self.api_key = match env::var("METEOSOURCE_API_KEY") {
            Ok(val) => Some(val),
            Err(_) => None,
        };

        self.point = match env::var("METEOSOURCE_POINT") {
            Ok(val) => val.trim().to_string(),
            Err(_) => "".to_string(),
        };
    }

    fn update(&mut self) {
        if self.api_key.is_none() {
            return;
        }

        if self.point.len() == 0 {
            return;
        }

        if get_unix_time_sec() - self.last_request_time <= INTERVAL_SEC {
            return;
        }

        let res = ureq::get(format!("https://www.meteosource.com/api/v1/free/point?place_id={}&language=en&sections=current&units=metric", self.point))
            .header("Accept", "application/json")
            .header("X-API-Key", self.api_key.as_ref().unwrap())
            .call()
            .and_then(|mut res| parse_response(&mut res));

        self.last_request_time = get_unix_time_sec();

        if res.is_err() {
            return;
        }

        self.last_response = Some(res.unwrap().current);
    }

    fn get_status(&self) -> Option<Status> {
        if self.last_response.is_none() {
            return None;
        }

        let res = self.last_response.as_ref().unwrap();

        let text = format!("{} C° ({})", res.temperature, res.summary);

        Some(Status {
            name: PLUGIN_NAME,
            full_text: text.clone(),
            short_text: text,
            markup: crate::plugin::Markup::None,
        })
    }
}

fn get_unix_time_sec() -> u64 {
    let start = SystemTime::now();
    match start.duration_since(UNIX_EPOCH) {
        Ok(i) => i.as_secs(),
        Err(_) => 0,
    }
}

/// Full response looks like this:
/// 
/// {
///     "lat": "56.946N",
///     "lon": "24.10589E",
///     "elevation": 6,
///     "timezone": "Europe/Riga",
///     "units": "metric",
///     "current": {
///         "icon": "sunny",
///         "icon_num": 2,
///         "summary": "Sunny",
///         "temperature": 9.0,
///         "wind": {
///             "speed": 4.0,
///             "angle": 325,
///             "dir": "NW"
///         },
///         "precipitation": {
///             "total": 0.0,
///             "type": "none"
///         },
///         "cloud_cover": 1
///     },
///     "hourly": null,
///     "daily": null
/// }
fn parse_response(response: &mut Response<Body>) -> Result<MeteoSourcePointResponse, ureq::Error> {
    response.body_mut().read_json::<MeteoSourcePointResponse>()
}
