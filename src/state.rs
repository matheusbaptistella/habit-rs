use std::{collections::BTreeMap, path::PathBuf};

use iced::{Element, Task as Command, widget::{button, operation, row, text}};
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::{
    filter::{Filter, FilterState},
    habit::{Habit, HabitMessage},
};

pub enum HabitTracker {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
pub struct State {
    input_value: String,
    filter: Filter,
    habits_mgr: HabitsManager,
    // habits: Vec<Habit>,
    // logs: BTreeMap<Date, Vec<Uuid>>,
    dirty: bool,
    saving: bool,
}

pub enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    // CreateHabit,
    FilterChanged(FilterState),
    TabPressed { shift: bool },
    // HabitMessage(Uuid, HabitMessage),
}

impl HabitTracker {
    pub const ICON_FONT: &'static [u8] = include_bytes!("../fonts/icons.ttf");

    pub fn new() -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    pub fn title(&self) -> String {
        let dirty = match self {
            HabitTracker::Loading => false,
            HabitTracker::Loaded(state) => state.dirty,
        };

        format!("Todos{} - Iced", if dirty { "*" } else { "" })
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            HabitTracker::Loading => {
                match message {
                    Message::Loaded(Ok(saved_state)) => {
                        *self = HabitTracker::Loaded(saved_state.into());
                    }
                    // TODO! Possibly a skeleton or something like an error message?
                    // Message::Loaded(Err(_)) => {
                    //     *self = HabitTracker::Loaded(State::default());
                    // }
                    // _ => {}
                    _ => panic!("SOMETHING WENT VERY WRONG")
                }

                operation::focus("new-habit")
            }
            HabitTracker::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    Message::Loaded(_) => Command::none(),
                    Message::Saved(_result) => {
                        state.saving = false;
                        saved = true;

                        Command::none()
                    }
                    Message::InputChanged(value) => {
                        state.input_value = value;

                        Command::none()
                    }
                    Message::CreateHabit => {
                        if !state.input_value.is_empty() {
                            state.habits.push(Habit::new(state.input_value.clone()));
                            state.input_value.clear();
                        }

                        Command::none()
                    }
                    Message::FilterChanged(filter) => {
                        state.filter.update(filter);

                        Command::none()
                    }
                    Message::TabPressed { shift } => {
                        if shift {
                            operation::focus_previous()
                        } else {
                            operation::focus_next()
                        }
                    }
                    // No focus
                    Message::HabitMessage(uuid, HabitMessage::Delete) => {
                        state.habits.retain(|h| h.uuid != uuid);
                        state.logs.retain(|_, log| {
                            log.retain(|h| h != &uuid);
                            !log.is_empty()
                        });

                        Command::none()
                    }
                    Message::HabitMessage(uuid, msg) => {
                        if let Some(habit) = state.habits.iter_mut().find(|h| h.uuid == uuid) {
                            let should_focus = matches!(msg, HabitMessage::Edit);
                            habit.update(msg);

                            if should_focus {
                                let id = Habit::text_input_id(uuid);
                                return Command::batch(vec![
                                    operation::focus(id.clone()),
                                    operation::select_all(id),
                                ]);
                            }
                        }

                        Command::none()
                    }
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(SavedState::from(&*state).save(),Message::Saved,)
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SavedState {
    input_value: String,
    habits: Vec<Habit>,
    logs: BTreeMap<Date, Vec<Uuid>>,
}

pub enum LoadError {
    File,
    Format,
}

pub enum SaveError {
    Write,
    Format,
}

impl SavedState {
    fn path() -> PathBuf {
        let mut path =
            if let Some(project_dirs) = directories::ProjectDirs::from("rs", "Iced", "Habit") {
                project_dirs.data_dir().into()
            } else {
                std::env::current_dir().unwrap_or_default()
            };

        path.push("habit.json");

        path
    }

    pub async fn load() -> Result<SavedState, LoadError> {
        let contents = tokio::fs::read_to_string(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            tokio::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::Write)?;
        }

        tokio::fs::write(path, json.as_bytes())
            .await
            .map_err(|_| SaveError::Write)?;

        Ok(())
    }
}

// Consume SavedState since it could become invalid
impl From<SavedState> for State {
    fn from(saved: SavedState) -> Self {
        State {
            input_value: saved.input_value,
            filter: Filter::new(),
            habits: saved.habits,
            logs: saved.logs,
            dirty: false,
            saving: false,
        }
    }
}

// Do not consume State since it is still valid
impl From<&State> for SavedState {
    fn from(state: &State) -> Self {
        SavedState {
            input_value: state.input_value.clone(),
            habits: state.habits.clone(),
            logs: state.logs.clone(),
        }
    }
}
