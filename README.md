# rpbar

A wrapper around [i3status](https://i3wm.org/docs/i3status.html) to allow easy extension of i3bar with extra data.

## Features

* show the current playing song - supports Spotify and Supersonic.
* show the current weather via MeteoSource.

## Usage

In your i3 config update the `bar` section so i3status is piped into rpbar:

```
bar {
        status_command i3status | METEOSOURCE_POINT=some_city METEOSOURCE_API_KEY=yourapikey /path/to/rpbar
}
```
