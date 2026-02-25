use std::fs::OpenOptions;

use directories::BaseDirs;

fn main() {
    let base_dirs = BaseDirs::new().unwrap();
    let app_dir = base_dirs.data_local_dir().join("habit-rs");
    std::fs::create_dir_all(&app_dir).unwrap();

    let app_data = app_dir.join("data.json");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(app_data)
        .unwrap();
}


enum HabitStatus {
    Completed,
    Todo
}

struct Habit {
    color: u32,
    status: HabitStatus,
}