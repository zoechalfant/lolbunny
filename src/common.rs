use std::str::FromStr;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rocket::response::Redirect;
use std::collections::HashMap;

use crate::data::{DASH_MAP, URLDATA};

pub const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

lazy_static::lazy_static! {
    pub static ref URL_MAP: HashMap<&'static str, (&'static HopType, &'static str)> = {
        let mut map = HashMap::new();
        for (shortcode, url, help) in URLDATA {
            map.insert(*shortcode, (url, *help));
        }
        map
    };
}

/// A type to represent a URL transformation scheme
///
/// Their names come from the sites they're applicable to, but
/// there's room for things like AppendIf, PrependIf, etc
#[derive(Clone, Copy)]
pub enum HopType {
    Basic(&'static str),
    Dashboard(&'static str),
}

impl FromStr for HopType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(*URL_MAP.get(s).ok_or(())?.0)
    }
}

impl HopType {
    /// Given a fragment of data as "args", generate the redirect URL
    pub fn to_string(&self, args: &str) -> String {
        match *self {
            HopType::Basic(w) => {
                let formatted = format!("{}{}", w, args);
                utf8_percent_encode(&formatted, FRAGMENT).to_string()
            }
            HopType::Dashboard(w) => {
                if let Some(&fragment) = DASH_MAP.get(args) {
                    format!("{}dashboard/{}", w, fragment)
                } else {
                    let formatted = format!("{}lists?q={}", w, args);
                    utf8_percent_encode(&formatted, FRAGMENT).to_string()
                }
            }
        }
    }

    /// Helper for generating a redirect directly from a hop
    pub fn to_redirect(&self, args: &str) -> Redirect {
        Redirect::to(self.to_string(args))
    }
}

/// Helper function to clean up and separate a "command" string
pub fn validate(args: &str) -> (&str, &str, &str) {
    let trimmed = args.trim();
    let strl = trimmed.len();
    let idx = trimmed.find(' ').unwrap_or(strl);
    let (cmd, argv) = trimmed.split_at(idx);
    (cmd, argv.trim(), trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_has_cmd_no_params() {
        assert_eq!(validate("cal"), ("cal", "", "cal"));
    }

    #[test]
    fn nobody_broke_validate() {
        assert_eq!(validate("cal"), ("cal", "", "cal"));
        let tricksters = [
            // (what are we testing, what broke)
            ("cal param1 param2", "the whole test"),
            (" cal param1 param2", "prepended whitespace"),
            ("cal param1 param2 ", "appended whitespace"),
        ];
        for (test_me, complaint) in tricksters.iter() {
            assert_eq!(
                validate(test_me),
                ("cal", "param1 param2", "cal param1 param2"),
                "failed because {}",
                complaint
            );
        }
    }

    #[test]
    fn test_get_dashboard_uri_is_alias() {
        let fail = "fail";
        assert_eq!(
            HopType::Dashboard("https://dash.board.foo.com/").to_string("ids"),
            format!(
                "https://dash.board.foo.com/dashboard/{}",
                DASH_MAP.get("ids").unwrap_or(&fail)
            )
        );
    }

    #[test]
    fn test_get_dashboard_uri_is_search() {
        assert_eq!(
            HopType::Dashboard("https://dash.board.foo.com/").to_string("search terms"),
            "https://dash.board.foo.com/lists?q=search%20terms"
        );
    }
}
