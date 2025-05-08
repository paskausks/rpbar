use std::str;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{get_property, intern_atom, query_tree, AtomEnum};
use x11rb::rust_connection::RustConnection;
use matchers::*;

pub mod matchers;

/// a tuple of a matcher function and a cleaner function
const MATCHERS: [(fn(&str, &str) -> bool, fn(&str) -> String); 2] = [
    (supersonic_matcher, supersonic_cleaner),
    (spotify_matcher, spotify_cleaner),
];

fn main() {
    let (conn, _screen_num) = x11rb::connect(None).unwrap();
    let setup = &conn.setup();

    if setup.roots_len() < 1 {
        return;
    }

    let root_window = setup.roots[0].root;

    crawl_titles(&conn, root_window);
}

fn crawl_titles(conn: &RustConnection, window: u32) {
    let tree_cookie = query_tree(&conn, window).unwrap();
    let tree = match tree_cookie.reply() {
        Ok(t) => t,
        Err(_) => return,
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
        let title_prop = get_property(&conn, false, child, net_wm_name_atom, utf8_string_atom, 0, 1024).unwrap().reply();
        let class_prop = get_property(&conn, false, child, AtomEnum::WM_CLASS, AtomEnum::STRING, 0, 16).unwrap().reply();

        let title = match title_prop {
            Ok(r) => r,
            Err(_) => continue,
        };

        if title.length > 0 {
            // class is application name and class separated by \0 (its a XClassHint struct)
            let class = class_prop.map(|res| res.value).unwrap_or_default();
            match str::from_utf8(&title.value) {
                Ok(t) => if match_title(t, str::from_utf8(&class).unwrap_or_default()) {
                    return;
                },
                Err(_) => continue,
            };
        }

        crawl_titles(&conn, child);
    }
}

fn match_title(title: &str, class: &str) -> bool {
    for (match_fn, clean_fn) in MATCHERS {
        if match_fn(title, class) {
            print!("{}", clean_fn(title));
            return true;
        }
    }
    return false;
}
