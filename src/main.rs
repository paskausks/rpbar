use plugin::Plugin;
use song_title::SongTitlePlugin;
use weather::WeatherPlugin;
use std::io;

pub mod song_title;
pub mod weather;
pub mod plugin;

type PluginVec = Vec<Box<dyn Plugin>>;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut plugins: PluginVec = vec![
        Box::new(SongTitlePlugin::default()),
        Box::new(WeatherPlugin::default()),
    ];

    for plugin in plugins.iter_mut() {
        plugin.setup();
    }

    loop {
        let bytes = match stdin.read_line(&mut buffer) {
            Ok(n) => n,
            Err(_) => 0,
        };

        if bytes == 0 {
            continue;
        }

        if buffer.starts_with("{\"version\":") || buffer.trim() == "[" {
            // opening headers, just echo them forwards
            print!("{}", buffer);
            buffer.clear();
            continue;
        }

        // since this is a json stream, we remove a prefixing comma
        let mut clean_buffer = buffer.trim();
        let mut had_prefix: bool = false;
        if clean_buffer.starts_with(",") {
            had_prefix = true;
            clean_buffer = &clean_buffer[1..];
        }

        process_i3status(&clean_buffer, had_prefix, &mut plugins);

        buffer.clear();
    }
}

fn process_i3status(buffer: &str, had_prefix: bool, plugins: &mut PluginVec) {
    let mut plugin_json: Vec<String> = Vec::with_capacity(plugins.len());

    for plugin in plugins.iter_mut() {
        plugin.update();

        let status = plugin.get_status();

        if status.is_none() {
            continue;
        }

        plugin_json.push(status.unwrap().to_json());
    }

    let json: String;

    if plugin_json.len() > 0 {
        // get the existing json without the opening brace
        json = format!("[{},{}", plugin_json.join(","), &buffer[1..]);
    } else {
        // no status from plugins, so we pass the buffer untouched
        json = String::from(buffer);
    }

    if had_prefix {
        println!(",{}", json);
    } else {
        println!("{}", json);
    }
}
