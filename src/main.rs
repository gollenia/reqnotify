#![windows_subsystem = "windows"]

extern crate winrt_notification;
use winrt_notification::{Toast};
use serde_json::{Value};
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
	let args: Vec<String> = env::args().collect();
	let mut notify = false;
	let mut url = "";

	for i in 0..args.len() {
        if args[i] == "-n" || &args[i] == "--notify" {
			notify = true;
			continue;
		}

		if args[i] == "-v" || &args[i] == "--version" {
			println!("Version: 0.1.0");
			return Ok(());
		}

		if args[i] == "-h" || &args[i] == "--help" {
			println!("Usage: url [-n | --notify] [-v | --version] [-h | --help]");
			return Ok(());
		}

		url = &args[i];
    }

    let resp = reqwest::get(url)
        .await
		.expect("Unable to get url")
		.text()
    	.await?;

	let result: Value = serde_json::from_str(&resp)?;

	if !notify {
		return Ok(());
	}

	Toast::new(Toast::POWERSHELL_APP_ID)
        .title(result["title"].as_str().unwrap_or("No title"))
        .text1(result["text"].as_str().unwrap_or("No text"))
        .show()
        .expect("unable to toast");

	Ok(())
}

