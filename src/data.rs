use super::common::HopType;
use phf::{self, phf_map, phf_set};

type UrlEntry = (&'static str, HopType, &'static str);

pub static DASH_MAP: phf::Map<&str, &str> = phf_map! {
    "ids" => "9bk-z3t-ikj/identity-service-slo-dashboard",
    "passp" => "xzb-gkq-rst/passport-webapp-slo-dashboard",
};
pub static KANBAN_MAP: phf::Map<&str, &str> = phf_map! {
    "passp" => "1640&quickFilter=24126",
};
pub static RUNBOOK_MAP: phf::Map<&str, &str> = phf_map! {
    "passp" => "display/PASS/Passport+Runbook",
};
pub static SRCGRAPH_FLAGS: phf::Set<&str> = phf_set! {
    "re",
    "rec",
    "sl",
    "slc",
    "st",
    "stc",
};
// nobody wants each item in its own 4 line block
#[rustfmt::skip]
pub static URLDATA: &[UrlEntry] = &[

    // simple links or searches
    //   use HopType::Basic if the workflow is always the same regardless of arguments
    ("bgs", HopType::SourceGraph("https://sourcegraph.indeed.tech/"), "Big grep search/Sourcegraph"),
    ("cal", HopType::Basic("https://calendar.google.com/"), "Open Google calendar"),
    ("expense", HopType::Basic("https://system.nexonia.com/assistant/logon.do"), "Buy things with company money!"),
    ("feedback", HopType::Basic("https://feedback.sanbox.indeed.net/dashboard"), "Openthe feedback tool"),
    ("g", HopType::Basic("https://google.com/search?q="), "Google search"),
    ("grok", HopType::Basic("https://opengrok.dev.indeed.net/source/search?project=!all!&q=%s&defs=&refs=&path=&hist=&type="), "OpenGrok"),
    ("interviews", HopType::Basic("https://palp2.sandbox.indeed.net/"), "Interviews tool"),
    ("iql", HopType::Basic("https://squall.indeed.com/iqlweb/"), "IQL shortcut"),
    ("iw", HopType::Basic("https://wiki.indeed.com/dosearchsite.action?queryString="), "Wiki search"),
    ("login", HopType::Basic("https://indeed.onelogin.com/"), "OneLogin"),
    ("marv", HopType::Basic("https://marvin2.ausprod.indeed.net/projects/"), "Marvin"),
    ("nexonia", HopType::Basic("https://system.nexonia.com/assistant/logon.do"), "Buy things with company money!"),
    ("opengrok", HopType::Basic("https://opengrok.dev.indeed.net/source/search?project=!all!&q=%s&defs=&refs=&path=&hist=&type="), "OpenGrok, full codebase search"),
    ("w", HopType::Basic("https://wiki.indeed.com/dosearchsite.action?queryString="), "Wiki search"),


    // complex workflows
    //   create and handle a custom HopType if the workflow depends on its arguments
    ("dash", HopType::Dashboard("https://app.datadoghq.com/"), "Go to or search for dashboard"),
    ("code", HopType::GitLab("https://code.corp.indeed.com/"), "Go to or search in GitLab"),
    ("jira", HopType::Jira("https://bugs.indeed.com/"), "JIRA shortcut"),
    ("kanban", HopType::Jira("https://bugs.indeed.com/"), "Go to `kanban <team>`"),
    ("officespace", HopType::OfficeSpace("https://indeed.officespacesoftware.com/"), "It's a jump-to-conclusions mat."),
    ("os", HopType::OfficeSpace("https://indeed.officespacesoftware.com/"), "It's a jump-to-conclusions mat."),
    ("proc", HopType::Proctor("https://proctor.sandbox.indeed.net/proctor/"), "Go to or search in Proctor"),
    ("proctor", HopType::Proctor("https://proctor.sandbox.indeed.net/proctor/"), "Go to or search in Proctor"),
    ("rb", HopType::Runbook("https://wiki.indeed.com/"), "Open a runbook for team, in form 'runbook <team name>'"),
    ("runbook", HopType::Runbook("https://wiki.indeed.com/"), "Open a runbook for team, in form 'runbook <team name>'"),
    ("sb", HopType::Sandbox("sandbox.indeed.net"), "Shortcut to a sandbox"),
    ("sg", HopType::SourceGraph("https://sourcegraph.indeed.tech/"), "Big grep search/Sourcegraph"),
    ("wd", HopType::Workday("https://www.myworkday.com/indeed/d/"), "Go to or search workday"),
    ("workday", HopType::Basic("https://www.myworkday.com/indeed/d/"), "Go to or search workday"),

];

lazy_static::lazy_static! {
    pub static ref HELP_PAGE: String = {
        let top_half = r#"
        <html>
            <head>
                <link rel="search"
                      type="application/opensearchdescription+xml"
                      title="lol"
                      href="https://lolbunny.sandbox.indeed.net/opensearch.xml">
                <title>lolbunny</title>
                <!-- it really whips the llama's ass -->
            </head>
            <style>
            </style>
            <body>
                <h1>Help</h1>
                <table>
                    <tr><th>Shortcut</th><th>Description</th><th>Example of where you can end up</th></td>
                "#;
        let bottom_half = "
                </table>
            </body>
        </html>";
        let mut out = top_half.to_string();
        use std::fmt::Write;
        for (short, hop, help) in URLDATA {
            // this will panic if we run out of memory to allocate the string... we should make this const
            write!(&mut out, "<tr><td>{}</td><td>{}</td><td><a href=\"{2}\">{2}</a></td></tr>", short, help, hop.to_string("EXAMPLE")).unwrap();
        }
        out += bottom_half;
        out
    };
}
