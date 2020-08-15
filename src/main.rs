use exitcode;
use http::StatusCode;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use serde_json::Value;
use std::error::Error;
use std::process;
use structopt::StructOpt;
use url::{ParseError, Url};

#[derive(StructOpt, Debug)]
struct Cli {
    attributes: String,
    id: String,
}

fn main() -> Result<(), Box<Error>> {
    let args = Cli::from_args();

    let paths = [&args.attributes, "/", &args.id].concat();
    let url = build_swapi_url(&paths)?;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "🌑 T ",
                "🌒 Th ",
                "🌓 Tha ",
                "🌔 That ",
                "🌕 That's ",
                "🌖 That's n ",
                "🌗 That's no ",
                "🌘 That's no m ",
                "🌑 That's no mo ",
                "🌒 That's no moon ",
                "🌓 That's no moon. ",
                "🌔 That's no moon.. ",
                "🌕 That's no moon... ",
                "🌖 That's no moon.... ",
                "🌗 That's no moon..... ",
                "🌘 That's no moon...... ",
            ])
            .template("{spinner:.green} {msg}"),
    );
    // https://docs.rs/reqwest/0.10.7/reqwest/blocking/struct.Response.html
    let response = reqwest::blocking::get(url)?;
    match response.status() {
        StatusCode::NOT_FOUND => {
            pb.finish_with_message("Received a 404. Resource was not found.");
            process::exit(exitcode::NOINPUT);
        }
        StatusCode::OK => {
            pb.finish_with_message("Found!");
            // https://docs.rs/reqwest/0.10.7/reqwest/blocking/struct.Response.html#optional-1
            // Error: reqwest::Error { kind: Decode, source: Error("invalid type: map, expected unit", line: 1, column: 0) }
            // This requires the optional json feature enabled
            let body: Value = response.json()?;
            // https://www.reddit.com/r/rust/comments/3ceaui/psa_produces_prettyprinted_debug_output/
            // https://users.rust-lang.org/t/parsing-json-response-to-just-json/47092/2?u=maxuuell
            println!("{:#}", body)
        }
        _ => {
            pb.finish_with_message("Something went wrong");
            println!("Status Code: {}", response.status());
            process::exit(exitcode::UNAVAILABLE);
        }
    };
    Ok(())
}

fn build_swapi_url(path: &str) -> Result<Url, ParseError> {
    const SWAPIBASEPATH: &'static str = "http://swapi.dev/api/";
    let url = Url::parse(SWAPIBASEPATH)?;
    let url = url.join(path)?;

    Ok(url)
}
