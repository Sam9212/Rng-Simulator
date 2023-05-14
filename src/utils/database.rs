use std::collections::HashMap;
use postcard::{from_bytes, to_allocvec};
use serde::{Serialize, Deserialize};
use std::fs::{read, write};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub balance: f64,
    pub experience: u32,
    pub cluck_level: u32,
    pub rluck_level: u32,
    pub inventory: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub rarity: u32,
    pub class: u32,
    pub quality: f64,
}

impl Default for User {
    fn default() -> Self {
        User {
            balance: 0.0,
            experience: 0,
            cluck_level: 0,
            rluck_level: 0,
            inventory: vec![],
        }
    }
}

impl User {
    pub fn level(&self, additional_xp: f64) -> u32 {
        ((self.experience as f64 + additional_xp)/300.0).log(1.03).floor() as u32
    }

    pub fn inventory_value(&self) -> f64 {
        let mut total = 0f64;
        for item in &self.inventory {
            total += item.value()
        }
        total
    }
}

impl Item {
    pub fn value(&self) -> f64 {
        500_000_000_000_000_000f64 / (self.class as f64).powf(1.5) / (self.rarity as f64).powf(1.5)
    }
}

pub fn get_exp_requirement(level: u32) -> u32 {
    (1.03f64.powi(level as i32) * 300f64) as u32
}

pub fn retrieve_database(db_path: &str) -> HashMap<u64, User> {
    let database = read(db_path).expect("Issue reading the file into a vector");
    from_bytes(&database).expect("Issue deserializing the database")
}

pub fn write_database(db_path: &str, new_data: HashMap<u64, User>) {
    let db_vec = to_allocvec(&new_data).expect("Issue serializing the database");
    write(db_path, db_vec).expect("There was an issue writing the database to the file");
}