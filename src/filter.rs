use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug)]
pub struct Filter {
    years: Vec<i32>,
    state: FilterState,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FilterState {
    Weekly,
    Yearly(i32),
    Habits,
}

impl Filter {
    pub fn new() -> Self {
        let curr_year = OffsetDateTime::now_utc().year();
        Filter {
            years: vec![curr_year],
            state: FilterState::Yearly(curr_year),
        }
    }

    pub fn update(&mut self, filter: FilterState) {
        match filter {
            FilterState::Weekly => {
                self.state = filter;
            }
            FilterState::Habits => {
                self.state = filter;
            }
            FilterState::Yearly(year) => {
                if self.years.contains(&year) {
                    self.state = filter;
                }
            }
        }
    }
}