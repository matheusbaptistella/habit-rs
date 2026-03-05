use std::collections::HashMap;

use iced::{
    Alignment, Element, Font, Length,
    widget::{Text, button, column, row, scrollable, text},
};

type HabitId = u64;

fn main() -> iced::Result {
    iced::application(App::mock, App::update, App::view)
        .font(App::ICON_FONT)
        .window_size((700.0, 300.0))
        .run()
}

#[derive(Debug, Default)]
struct App {
    inc_id: u64,
    habits: HashMap<HabitId, Habit>,
    create_habit_modal: Option<String>,
    delete_habit_modal: Option<HabitId>,
}

#[derive(Clone, Debug)]
enum AppMessage {
    OpenCreateHabitModal,
    CancelCreateHabitModal,
    UpdateTitleCreateHabitModal { new_title: String },
    ConfirmCreateHabitModal { title: String },

    OpenDeleteHabitModal(HabitId),
    CancelDeleteHabitModal,
    ConfirmDeleteHabitModal(HabitId),
}

impl App {
    const ICON_FONT: &'static [u8] = include_bytes!("../fonts/icons.ttf");

    fn mock() -> Self {
        let mut app = App::default();

        for i in 1..=10 {
            app.habits.insert(
                i,
                Habit {
                    id: i,
                    title: format!("Habit #{i}"),
                },
            );
        }

        app
    }

    fn update(&mut self, msg: AppMessage) {
        match msg {
            AppMessage::OpenCreateHabitModal => {
                self.create_habit_modal = Some(String::new());
            }
            AppMessage::CancelCreateHabitModal => {
                self.create_habit_modal = None;
            }
            AppMessage::UpdateTitleCreateHabitModal { new_title } => {
                self.create_habit_modal = Some(new_title);
            }
            AppMessage::ConfirmCreateHabitModal { title } => {
                if !self.habits.contains_key(&self.inc_id) {
                    let h = Habit::new(self.inc_id, title);

                    self.habits.insert(self.inc_id, h);
                    self.inc_id += 1;
                }
                self.create_habit_modal = None;
            }

            AppMessage::OpenDeleteHabitModal(id) => {
                self.delete_habit_modal = Some(id);
            }
            AppMessage::ConfirmDeleteHabitModal(id) => {
                self.habits.remove(&id);
                self.delete_habit_modal = None;
            }
            AppMessage::CancelDeleteHabitModal => {
                self.delete_habit_modal = None;
            }
        }
    }

    fn view(&self) -> Element<'_, AppMessage> {
        let mut ids: Vec<HabitId> = self.habits.keys().copied().collect();
        ids.sort_unstable();
        ids.reverse();

        let habits = column(ids.iter().map(|id| {
            let habit = &self.habits[id];
            row![
                text(&habit.title),
                button(delete_icon()).on_press(AppMessage::OpenDeleteHabitModal(habit.id)).padding(10).style(button::danger)
            ]
            .into()
        }))
        .spacing(8);

        scrollable(habits).height(Length::Fill).into()
    }
}

#[derive(Debug)]
struct Habit {
    id: HabitId,
    title: String,
}

impl Habit {
    fn new(id: HabitId, title: String) -> Self {
        Habit { id, title }
    }
}

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(Font::with_name("Iced-Todos-Icons"))
        .width(20)
        .align_x(Alignment::Center)
        .shaping(text::Shaping::Basic)
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}
