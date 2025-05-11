mod matchers;

use core::str;
use matchers::MATCHERS;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{get_property, intern_atom, query_tree, AtomEnum};
use x11rb::rust_connection::RustConnection;
use crate::plugin::{Markup, Plugin, Status};

const PLUGIN_NAME: &str = "song_name";
const FRAMES: [char; 4] = ['|', '/', '-', '\\'];

pub struct SongTitlePlugin {
    x11_connection: Option<RustConnection>,
    root_window: u32,
    spinner_frame: usize,
}

impl Default for SongTitlePlugin {
    fn default() -> Self {
        let plugin = SongTitlePlugin {
            x11_connection: None,
            root_window: 0,
            spinner_frame: 0,
        };
        return plugin;
    }
}

impl Plugin for SongTitlePlugin {
    fn setup(&mut self) {
        let (conn, _screen_num) = x11rb::connect(None).unwrap();
        let setup = &conn.setup();

        if setup.roots_len() < 1 {
            return;
        }

        self.root_window = setup.roots[0].root;
        self.x11_connection = Some(conn);
    }

    fn update(&mut self) {
        self.spinner_frame = self.spinner_frame + 1;
        if self.spinner_frame >= FRAMES.len() {
            self.spinner_frame = 0;
        }
    }

    fn get_status(&self) -> Option<Status> {
        if let Some(conn) = &self.x11_connection {
            let title_match = crawl_titles(&conn, self.root_window);

            if title_match.is_none() {
                return None;
            }

            let title = title_match.unwrap();

            // for the short title, just use the track title
            let mut short_title = title.clone();
            let hyphen_index = short_title.find(" - ");

            if let Some(idx) = hyphen_index {
                let (head, _) = short_title.split_at(idx);
                short_title = String::from(head);
            }

            return Some(Status {
                name: PLUGIN_NAME,
                markup: Markup::Pango,
                full_text: wrap_in_tags(title, FRAMES[self.spinner_frame]),
                short_text: wrap_in_tags(short_title, FRAMES[self.spinner_frame]),
            });
        }

        None
    }
}

fn crawl_titles(conn: &RustConnection, window: u32) -> Option<String> {
    let tree_cookie = query_tree(&conn, window).unwrap();
    let tree = match tree_cookie.reply() {
        Ok(t) => t,
        Err(_) => return None,
    };

    // instead of AtomEnum::WM_NAME which is ASCII
    // https://specifications.freedesktop.org/wm-spec/1.3/ar01s05.html
    let net_wm_name_atom = intern_atom(&conn, true, b"_NET_WM_NAME")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    // instead of AtomEnum::STRING which is ASCII
    // https://specifications.freedesktop.org/wm-spec/1.3/ar01s05.html
    let utf8_string_atom = intern_atom(&conn, true, b"UTF8_STRING")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    for child in tree.children {
        let title_prop = get_property(
            &conn,
            false,
            child,
            net_wm_name_atom,
            utf8_string_atom,
            0,
            1024,
        )
        .unwrap()
        .reply();
        let class_prop = get_property(
            &conn,
            false,
            child,
            AtomEnum::WM_CLASS,
            AtomEnum::STRING,
            0,
            16,
        )
        .unwrap()
        .reply();

        let title = match title_prop {
            Ok(r) => r,
            Err(_) => continue,
        };

        if title.length > 0 {
            // class is application name and class separated by \0 (its a XClassHint struct)
            let class = class_prop.map(|res| res.value).unwrap_or_default();
            match str::from_utf8(&title.value) {
                Ok(t) => {
                    let title_match = match_title(t, str::from_utf8(&class).unwrap_or_default());
                    if title_match.is_some() {
                        return title_match;
                    }
                }
                Err(_) => continue,
            };
        }

        let child_res = crawl_titles(&conn, child);
        if child_res.is_some() {
            return child_res;
        }
    }

    return None;
}

fn match_title(title: &str, class: &str) -> Option<String> {
    for (match_fn, clean_fn) in MATCHERS {
        if match_fn(title, class) {
            return Some(clean_fn(title));
        }
    }
    return None;
}

fn wrap_in_tags(content: String, symbol: char) -> String {
    format!("🎵 ({}) <span foreground=\"#FF9900\" font_weight=\"bold\">{}</span> ({}) 🎵", symbol, content, symbol)
}
