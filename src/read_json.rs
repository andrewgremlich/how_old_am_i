use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[derive(Deserialize, Debug)]
pub struct DaysToMonth {
    pub days: f32,
    pub value: u8,
    pub name: String,
}

// read days-to-months.json file
pub fn days_to_months() -> Result<Vec<DaysToMonth>, Box<dyn Error>> {
    let file = File::open("data/days-to-months.json")?;
    let reader = BufReader::new(file);

    let days_to_months: Vec<DaysToMonth> = serde_json::from_reader(reader)?;

    Ok(days_to_months)
}

// read presidents.json file
pub fn presidents() -> Result<Vec<Person>, Box<dyn Error>> {
    let file = File::open("data/presidents.json")?;
    let reader = BufReader::new(file);

    let presidents: Vec<Person> = serde_json::from_reader(reader)?;

    Ok(presidents)
}