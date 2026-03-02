use habit_rs::state::HabitTracker;

fn main() -> iced::Result {
    iced::application(HabitTracker::new, HabitTracker::update, HabitTracker::view)
        .subscription(HabitTracker::subscription)
        .title(HabitTracker::title)
        .font(HabitTracker::ICON_FONT)
        .window_size((500.0, 800.0))
        .presets(presets())
        .run()
}
