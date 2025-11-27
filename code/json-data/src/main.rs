use json_data::person::Person;
use std::fs::File;

use json_data::person::create_people;

fn save_to_json_file(people: &Vec<Person>, path: &str) -> () {
    // Serialize the vector of Person structs to JSON and save to a file
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, people).unwrap();
}

fn load_from_json_file(path: &str) -> Vec<Person> {
    let file = File::open(path).unwrap();
    let people: Vec<Person> = serde_json::from_reader(file).unwrap();
    people
}

mod with_anyhow {
    use anyhow::{Context, Error};
    use json_data::person::Person;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn save_to_json_file(people: &Vec<Person>, path: &str) -> anyhow::Result<()> {
        // Serialize the vector of Person structs to JSON and save to a file
        let file = File::create(path).context(format!("Error. Unable to create file: {}", path))?;
        serde_json::to_writer_pretty(file, people).context("Error. Saving to JSON failed")?;

        Ok(())
    }

    pub fn load_from_json_file(path: &str) -> anyhow::Result<Vec<Person>> {
        println!("trying to open {}", path);
        let file = File::open(path).with_context(|| format!("Failed to open file '{}'", path))?;
        let people: Vec<Person> = serde_json::from_reader(file)
            .with_context(|| format!("Failed to read person from '{}'", path))?;
        Ok(people)
    }
}

mod with_thiserror {
    use json_data::person::Person;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use thiserror::Error;

    #[derive(Debug, thiserror::Error)]
    pub enum JsonIoError {
        #[error("IO Error: {0}")]
        Io(#[from] std::io::Error),

        #[error("Serde JSON Error: {0}")]
        Json(#[from] serde_json::Error),
    }

    pub fn save_to_json_file(people: &Vec<Person>, path: &str) -> Result<(), JsonIoError> {
        // Serialize the vector of Person structs to JSON and save to a file
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, people)?;
        Ok(())
    }

    pub fn load_from_json_file(path: &str) -> Result<Vec<Person>, JsonIoError> {
        let file = File::open(path)?;
        let people: Vec<Person> = serde_json::from_reader(file)?;
        Ok(people)
    }
}

fn main() {
    let people = create_people();

    let file_path = "people.json";
    match with_thiserror::save_to_json_file(&people, file_path) {
        Ok(_) => println!("Save OK!"),
        Err(err) => panic!("Error: {err:?}"),
    }

    match with_anyhow::load_from_json_file(file_path) {
        Ok(loaded_people) => {
            for person in loaded_people {
                println!("{:?}", person);
            }
        }
        Err(err) => panic!("Error: {err:?}"),
    }
}
