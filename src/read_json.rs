use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub birth: String,
}

pub fn read_presidents() -> Result<Vec<Person>, Box<dyn Error>> {
    let file = File::open("data/presidents.json")?;
    let reader = BufReader::new(file);

    let presidents: Vec<Person> = serde_json::from_reader(reader)?;

    Ok(presidents)
}
