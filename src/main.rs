mod constants;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use http::StatusCode;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::error::Error;
use std::process;
use url::{ParseError, Url};

fn main() -> Result<(), Box<dyn Error>> {
    let theme = &ColorfulTheme::default();
    let mut resource_prompt = Select::with_theme(theme);
    for resource in constants::RESOURCES {
        resource_prompt.item(resource);
    }

    println!("Choose the resource you'd like to lookup");
    let resource_response: usize = resource_prompt.interact()?;
    let resource = constants::RESOURCES[resource_response];
    println!("{}", resource.green());

    // construct the filter prompt
    let mut resource_data_filter_prompt = Select::with_theme(theme);

    for data_filter in constants::RESOURCES_DATA_FILTER {
        resource_data_filter_prompt.item(data_filter);
    }
    println!("Choose the filter you'd like to use.");
    let data_filter_response: usize = resource_data_filter_prompt.interact()?;
    println!(
        "{}",
        constants::RESOURCES_DATA_FILTER[data_filter_response].green()
    );

    let path_and_arguments = if data_filter_response == 0 {
        [resource, "/"].concat()
    } else if data_filter_response == 1 {
        let id = Input::<String>::new()
            .with_prompt("What id would you like to look up?")
            .interact()?;
        [resource, "/", &id].concat()
    } else if data_filter_response == 2 {
        [resource, "/schema"].concat()
    } else if data_filter_response == 3 {
        // if they want to search, ask for specific input
        if resource_response == 0 {
            // title for films
            let title = Input::<String>::new()
                .with_prompt("What title would you like to lookup?")
                .interact()?;

            [resource, "/?search=", &title].concat()
        } else if resource_response == 1 || resource_response == 2 || resource_response == 3 {
            // name for species and planets
            let name = Input::<String>::new()
                .with_prompt("What name would you like to lookup?")
                .interact()?;

            [resource, "/?search=", &name].concat()
        } else if resource_response == 4 || resource_response == 5 {
            // name and model for starships and vehicles
            let name_or_model = Input::<String>::new()
                .with_prompt("What name or model would you like to lookup?")
                .interact()?;

            [resource, "/?search=", &name_or_model].concat()
        } else {
            String::from("")
        }
    } else {
        String::from("")
    };

    // wookie prompt was working, but the response is inconsistent
    // with different requests. Seems unstable
    // let mut wookiee_prompt = Select::with_theme(theme);
    // wookiee_prompt.item("No thanks");
    // wookiee_prompt.item("huurh... means yes 😉");

    // println!("You want your response, in wookiee?");
    // let wookiee_prompt_response: usize = wookiee_prompt.interact()?;

    // if wookiee_prompt_response == 1 {
    //     path_and_arguments = [path_and_arguments, String::from("?format=wookiee")].concat();
    // }

    // https://github.com/rust-lang/rust/issues/46871#issuecomment-618186642

    let url = build_swapi_url(&path_and_arguments)?;
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(constants::TICK_STRINGS)
            .template("{spinner:.green} {msg}"),
    );
    let response = reqwest::blocking::get(url)?;
    match response.status() {
        StatusCode::NOT_FOUND => {
            pb.finish_with_message("Received a 404. Resource was not found.");
            process::exit(exitcode::NOINPUT);
        }
        StatusCode::OK => {
            pb.finish_with_message("Found!");
            let body: Value = response.json()?;
            // https://www.reddit.com/r/rust/comments/3ceaui/psa_produces_prettyprinted_debug_output/
            // https://users.rust-lang.org/t/parsing-json-response-to-just-json/47092/2?u=maxuuell
            println!("{:#}", body);
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
    let url = Url::parse(constants::SWAPI_BASE_PATH)?;
    let url = url.join(path)?;
    Ok(url)
}
