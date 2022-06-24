use iced::{
    button, pick_list, Alignment, Button, Column, Container, Element, Length, PickList, Row, Space,
    Text,
};

use crate::gui::{Error, Filter, Message, DEFAULT_PADDING};

// use super::{};

pub fn menu_bar<'a>(
    error: &Option<Error>,
    fetch: &'a mut button::State,
    filter: &'a mut pick_list::State<Filter>,
    selected_filter: Option<Filter>,
) -> Container<'a, Message> {
    // Create settings row
    let mut settings_row = Row::new()
        .height(Length::Units(50))
        .align_items(Alignment::Center);
    // .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)));

    /* FETCH BUTTON */
    let fetch_button: Element<Message> = Button::new(fetch, Text::new("Refresh"))
        .on_press(Message::Fetch)
        .into();

    /* VERSION FILTER */
    // left most element - release filter
    let version_filter = PickList::new(
        filter,
        &Filter::ALL[..],
        selected_filter,
        Message::FilterChanged,
    );
    /* END VERSION FILTER */

    /* VERSION DISPLAY */
    let version_text = Text::new("v0.0.0");
    let version_container: Element<Message> = Container::new(version_text).center_y().into();
    // .style(style::NormalForegroundContainer(color_palette));
    /* END VERSION DISPLAY */

    /* BUILD OUR MENU BAR! */
    settings_row = settings_row
        .push(fetch_button)
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
        .push(version_filter)
        .push(Space::new(Length::Fill, Length::Units(0)))
        .push(version_container);

    // Wrap with a column so errors can go on below the menu bar
    let mut settings_column = Column::new().push(settings_row);

    /* ERROR DISPLAY */
    settings_column = if let Some(error) = error {
        let error_container = Container::new(Text::new(error.to_string()))
            .center_y()
            .center_x()
            .padding(DEFAULT_PADDING)
            .width(Length::Fill);
        // .style(style::NormalErrorForegroundContainer(color_palette));
        settings_column.push(error_container)
    } else {
        settings_column
    };
    /* END ERROR DISPLAY */

    // Wrap everything in a container.
    Container::new(settings_column) //.style(style::BrightForegroundContainer(color_palette))
}
