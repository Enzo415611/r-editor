use iced::{
    Theme,
    widget::{button, pick_list},
};

pub fn button_style(t: &Theme, s: button::Status) -> button::Style {
    let mut style = button::primary(t, s);
    style.border.radius = iced::border::Radius::new(10);
    style
}

pub fn pick_list_style(t: &Theme, s: pick_list::Status) -> pick_list::Style {
    let mut style = pick_list::default(t, s);
    style.border.radius = iced::border::Radius::new(10);
    style
}
