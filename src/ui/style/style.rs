use iced::{
    Color, Theme,
    widget::{button, pick_list},
};

pub fn tab_button_style(t: &Theme, s: button::Status, c: bool) -> button::Style {
    let mut t = button::primary(t, s);
    if c {
        t.text_color = Color::WHITE;
        t.background = Some(iced::Background::Color(Color::from_rgb8(120, 120, 118)));
    }
    t
}

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
