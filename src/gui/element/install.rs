use iced::{Element, Row};

use crate::gui::Message;

pub struct InstallBar {}

impl InstallBar {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        Row::new().into()
    }
}
