use super::common::HopType;
use phf::{self, phf_map};

type UrlEntry = (&'static str, HopType, &'static str);

pub static DASH_MAP: phf::Map<&str, &str> = phf_map! {
    "ids" => "9bk-z3t-ikj/identity-service-slo-dashboard",
    "passp" => "xzb-gkq-rst/passport-webapp-slo-dashboard",
};

// nobody wants each item in its own 4 line block
#[rustfmt::skip]
pub static URLDATA: &[UrlEntry] = &[

    // simple links or searches
    //   use HopType::Basic if the workflow is always the same regardless of arguments
    ("cal", HopType::Basic("https://calendar.google.com/"), "Open Google calendar"),
    ("g", HopType::Basic("https://google.com/search?q="), "Google search"),


    // complex workflows
    //   create and handle a custom HopType if the workflow depends on its arguments
    ("dash", HopType::Dashboard("https://app.datadoghq.com/"), "Go to or search for dashboard, e.g. `dash yourdashboardname`"),

];

lazy_static::lazy_static! {
    pub static ref HELP_PAGE: String = {
        let top_half = r#"
        <html>
            <head>
                <link rel="search"
                      type="application/opensearchdescription+xml"
                      title="lol"
                      href="https://<YOUR_HOSTNAME_HERE>/opensearch.xml">
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
