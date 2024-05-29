use std::str::FromStr;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rocket::response::Redirect;
use std::collections::HashMap;

use crate::data::{DASH_MAP, KANBAN_MAP, RUNBOOK_MAP, SRCGRAPH_FLAGS, URLDATA};

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
    Kanban(&'static str),
    GitLab(&'static str),
    Jira(&'static str),
    OfficeSpace(&'static str),
    Proctor(&'static str),
    Runbook(&'static str),
    Sandbox(&'static str),
    SourceGraph(&'static str),
    Workday(&'static str),
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
            // This could also be generalized like the above... wonder if theyd be the same type
            HopType::Dashboard(w) => {
                if let Some(&fragment) = DASH_MAP.get(args) {
                    format!("{}dashboard/{}", w, fragment)
                } else {
                    let formatted = format!("{}lists?q={}", w, args);
                    utf8_percent_encode(&formatted, FRAGMENT).to_string()
                }
            }
            // This could probably be generalized to some "appendIf" kinda thing
            HopType::GitLab(w) => {
                if args.find('/').is_some() {
                    format!("{}{}", w, args)
                } else {
                    let formatted = format!("{}search?utf8=✓&search={}", w, args);
                    utf8_percent_encode(&formatted, FRAGMENT).to_string()
                }
            }
            HopType::Jira(w) => {
                if args == "create" || args == "new" {
                    format!("{}secure/CreateIssue!default.jspa", w)
                } else if args.chars().count() > 0 {
                    format!("{}browse/{}", w, args)
                } else {
                    format!("{}issues/?filter=-1", w)
                }
            }
            HopType::Kanban(w) => {
                if let Some(&fragment) = KANBAN_MAP.get(args) {
                    format!("{}secure/RapidBoard.jspa?rapidView={}", w, fragment)
                } else {
                    format!("{}secure/RapidBoard.jspa", w)
                }
            }
            HopType::OfficeSpace(w) => {
                if args.chars().count() > 0 {
                    let formatted = format!("{}visual-directory/search/{}", w, args);
                    utf8_percent_encode(&formatted, FRAGMENT).to_string()
                } else {
                    format!("{}portal", w)
                }
            }
            HopType::Proctor(w) => {
                if args.chars().count() > 0 {
                    let formatted = format!("{}definition/{}", w, args);
                    utf8_percent_encode(&formatted, FRAGMENT).to_string()
                } else {
                    w.to_string()
                }
            }
            HopType::Runbook(w) => {
                if let Some(&fragment) = RUNBOOK_MAP.get(args) {
                    format!("{}{}", w, fragment)
                } else {
                    w.to_string()
                }
            }
            // This would work in a format string/template kinda deal
            HopType::Sandbox(w) => {
                format!("https://{}.{}", args, w)
            }
            HopType::SourceGraph(w) => {
                // determine if this is a direct search or if there are flags to parse
                let offset = args.find(' ').unwrap_or(0);
                let formatted;
                if !SRCGRAPH_FLAGS.contains(args.get(0..offset).unwrap_or("")) {
                    formatted = format!("{}search?q={}&patternType=literal", w, args);
                } else {
                    let (flags, query) = args.split_at(offset);
                    let mut pattern_type = "literal"; // literal is the default case
                    match flags.get(0..2).unwrap().as_ref() {
                        "re" => pattern_type = "regexp",
                        "sl" => pattern_type = "literal",
                        "st" => pattern_type = "structural",
                        _ => (),
                    }
                    let case_sensitive = if (flags.len() == 3
                        && char::from(flags.as_bytes()[2]) == 'c')
                        || pattern_type == "structural"
                    {
                        "true"
                    } else {
                        "false"
                    };
                    formatted = format!(
                        "{}search?q={}&patternType={}&case={}",
                        w,
                        query.trim_start(),
                        pattern_type,
                        case_sensitive
                    );
                }
                utf8_percent_encode(&formatted, FRAGMENT).to_string()
            }
            HopType::Workday(w) => {
                if args.chars().count() > 0 {
                    let formatted = format!("{}search.htmld?q={}", w, args);
                    utf8_percent_encode(&formatted, FRAGMENT).to_string()
                } else {
                    format!("{}home.htmld", w)
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
    fn test_get_sandbox_uri() {
        assert_eq!(
            HopType::Sandbox("bar.test.com/").to_string("foo"),
            "https://foo.bar.test.com/"
        );
    }

    #[test]
    fn test_get_gitlab_uri_is_project() {
        assert_eq!(
            HopType::GitLab("https://code.bar.foo.com/").to_string("foo/bar"),
            "https://code.bar.foo.com/foo/bar"
        );
    }

    #[test]
    fn test_get_gitlab_uri_is_search() {
        assert_eq!(
            HopType::GitLab("https://code.bar.foo.com/").to_string("foobar"),
            "https://code.bar.foo.com/search?utf8=%E2%9C%93&search=foobar"
        );
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

    #[test]
    fn test_get_jira_uri_is_default() {
        assert_eq!(
            HopType::Jira("https://jira/").to_string(""),
            "https://jira/issues/?filter=-1"
        );
    }

    #[test]
    fn test_get_jira_uri_is_is_discreet() {
        assert_eq!(
            HopType::Jira("https://jira/").to_string("FOO-1234"),
            "https://jira/browse/FOO-1234"
        );
    }

    #[test]
    fn test_get_jira_uri_is_create() {
        assert_eq!(
            HopType::Jira("https://jira/").to_string("create"),
            "https://jira/secure/CreateIssue!default.jspa"
        );
    }

    #[test]
    fn test_get_jira_uri_is_create_alt() {
        assert_eq!(
            HopType::Jira("https://jira/").to_string("new"),
            "https://jira/secure/CreateIssue!default.jspa"
        );
    }

    #[test]
    fn test_get_bgs_uri_no_flags() {
        assert_eq!(
            HopType::SourceGraph("https://sg/").to_string("blahblahblah"),
            "https://sg/search?q=blahblahblah&patternType=literal"
        );
    }

    #[test]
    fn test_get_bgs_uri_string_literal_flags() {
        assert_eq!(
            HopType::SourceGraph("https://sg/").to_string("sl blahblahblah"),
            "https://sg/search?q=blahblahblah&patternType=literal&case=false"
        );
    }

    #[test]
    fn test_get_bgs_uri_regexp_flags() {
        assert_eq!(
            HopType::SourceGraph("https://sg/").to_string("re blahblahblah"),
            "https://sg/search?q=blahblahblah&patternType=regexp&case=false"
        );
    }
    #[test]
    fn test_get_bgs_uri_structural_flags() {
        assert_eq!(
            HopType::SourceGraph("https://sg/").to_string("st blahblahblah"),
            "https://sg/search?q=blahblahblah&patternType=structural&case=true"
        );
    }
    #[test]
    fn test_get_bgs_uri_case_insensitive() {
        assert_eq!(
            HopType::SourceGraph("https://sg/").to_string("rec blahblahblah"),
            "https://sg/search?q=blahblahblah&patternType=regexp&case=true"
        );
    }
    #[test]
    fn test_get_bgs_uri_with_spaces() {
        assert_eq!(
            HopType::SourceGraph("https://sg/").to_string("foo bar baz"),
            "https://sg/search?q=foo%20bar%20baz&patternType=literal"
        );
    }
}
