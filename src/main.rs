use chrono::{Local, Datelike, DateTime, Duration};
use reqwest::blocking::get;
use regex::Regex;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current date

    // Fetch the webpage HTML
    let url = "https://packages.microsoft.com/repos/edge/pool/main/m/microsoft-edge-stable/"; // Replace with the desired webpage URL
    let response = get(url)?;
    let html = response.text()?;

    // Check if the current date is present in the webpage HTML
    let mut local: DateTime<Local> = Local::now();
    local = local - Duration::days(1);
    let pattern = local.format("%d-%b-%Y").to_string();
    println!("yesterday's date is {}",pattern);
    // let pattern = "21-Aug-2023";

    // let pattern = format!(r"\b{}-{}-{}\b", year, month, day);
    let regex = Regex::new(&pattern)?;
    let contains_date = regex.is_match(&html);

    println!("Contains yesterday's date: {}", contains_date);
    // println!("Contains today's date: {}", pattern);

    Ok(())
}