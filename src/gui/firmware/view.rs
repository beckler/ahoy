use iced::{Color, Column, Container, Element, Length, Rule};

use crate::gui::{style, DEFAULT_PADDING};

use super::{Event, Firmware};

pub(crate) fn handle_view(ahoy: &mut Firmware) -> Element<Event> {
    // BUILD PRIMARY VIEW
    let inner_content: Element<Event> = if ahoy.installing {
        Column::new()
            .padding(DEFAULT_PADDING)
            .push(Rule::horizontal(1))
            .push(ahoy.installer.view(&ahoy.dfu_connection))
            .into()
    } else {
        // selecting a release
        Column::new()
            .padding(DEFAULT_PADDING)
            .push(Rule::horizontal(1))
            // .push(error_message)
            .push(ahoy.controls.view(&ahoy.filter))
            .push(Rule::horizontal(1))
            .push(ahoy.versions.view(
                &ahoy.error,
                &ahoy.filter,
                &ahoy.releases,
                conn,
                &ahoy.selected_version,
            ))
            .into()
    };

    // wrap modal around the inner content
    let modal_wrapped_content = ahoy.confirm_modal.view(inner_content);

    // setup graphical debugging
    let output = if ahoy.debug {
        modal_wrapped_content.explain(Color::BLACK)
    } else {
        modal_wrapped_content
    };

    // finally wrap everything in a container.
    Container::new(output)
        .height(Length::Fill)
        .width(Length::Fill)
        .style(style::Container::Default)
        .into()
}
