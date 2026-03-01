use std::fs;
use std::{collections::BTreeMap, fs::OpenOptions};
use std::io::BufReader;

use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

fn main() {
    let base_dirs = BaseDirs::new().unwrap();
    let app_dir = base_dirs.data_local_dir().join("habit-rs");
    std::fs::create_dir_all(&app_dir).unwrap();

    let app_data_dir = app_dir.join("data.json");

    if !app_data_dir.exists() {
        let now = OffsetDateTime::now_utc().date();
        let contents = serde_json::to_string(&now).unwrap();
        fs::write(&app_data_dir, contents).unwrap();
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(app_data_dir)
        .unwrap();
    let reader = BufReader::new(file);

    let data: Data = serde_json::from_reader(reader).unwrap();


    
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    created_at: Date,
    logs: BTreeMap<Date, Vec<Habit>>,
}

#[derive(Debug, Deserialize, Serialize)]
enum HabitStatus {
    Completed,
    Todo,
}

#[derive(Debug, Deserialize, Serialize)]
struct Habit {
    name: String,
    status: HabitStatus,
}