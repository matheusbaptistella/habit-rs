use std::collections::BTreeMap;

use iced::widget;
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Habit {
    #[serde(default = "Uuid::new_v4")]
    pub uuid: Uuid,
    title: String,
    archived: bool,

    #[serde(skip)]
    state: HabitState,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum HabitState {
    #[default]
    Idle,
    Editing,
}

pub enum HabitMessage {
    Archived(bool),
    Delete,
    Edit,
    TitleEdited(String),
    FinishEdition,
}

impl Habit {
    pub fn text_input_id(uuid: Uuid) -> widget::Id {
        widget::Id::from(format!("task-{uuid}"))
    }

    pub fn new(title: String) -> Self {
        Habit {
            uuid: Uuid::new_v4(),
            title,
            archived: false,
            state: HabitState::Idle,
        }
    }

    pub fn update(&mut self, msg: HabitMessage) {
        match msg {
            HabitMessage::Archived(archive) => {
                self.archived = archive;
            }
            HabitMessage::Edit => {
                self.state = HabitState::Editing;
            }
            HabitMessage::TitleEdited(title) => {
                self.title = title;
            }
            HabitMessage::FinishEdition => {
                if !self.title.is_empty() {
                    self.state = HabitState::Idle;
                }
            }
            HabitMessage::Delete => {}
        }
    }
}


pub struct HabitsManager {
    habits: Vec<Habit>,
    logs: BTreeMap<Date, Vec<Uuid>>,
}

pub enum HabitsManagerMessage {
    CreateHabit,
    ArchiveHabit,
    DeleteHabit,
}
