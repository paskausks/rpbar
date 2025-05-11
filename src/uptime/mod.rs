use std::io::Read;
use std::fs::File;
use crate::plugin::{Plugin, Status};

const PLUGIN_NAME: &str = "uptime";
const UPTIME_FILE: &str = "/proc/uptime";

pub struct UptimePlugin;

impl Plugin for UptimePlugin {
    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn get_status(&self) -> Option<Status> {
        let file = File::open(UPTIME_FILE);

        if file.is_err() {
            return None;
        }

        let mut contents = String::new();

        let read_result = file.unwrap().read_to_string(&mut contents);

        if read_result.is_err() {
            return None;
        }

        let split = contents.split_once(' ');

        if split.is_none() {
            return None;
        }

        let time: f64 = match split.unwrap().0.parse::<f64>() {
            Ok(t) => t,
            Err(_) => 0.0,
        };

        let time_repr = repr_time(time);

        Some(Status {
            name: PLUGIN_NAME,
            full_text: time_repr.clone(),
            short_text: time_repr,
            markup: crate::plugin::Markup::None,
        })
    }
}

fn repr_time(sec: f64) -> String {
    let mut remainder = sec;

    let hours = sec / 3600.0;
    let hours_floored = hours.floor();

    remainder = remainder - hours_floored * 3600.0;

    let minutes: usize = (remainder / 60.0) as usize;

    if hours_floored != 0.0 {
        format!("UP {}h{}m", hours_floored, minutes)
    } else {
        format!("UP {}m", minutes)
    }
}
