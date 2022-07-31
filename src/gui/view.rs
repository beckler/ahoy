use iced::{
    svg::Handle, Alignment, Color, Column, Container, Element, Length, Row, Rule, Space, Svg, Text,
};

use super::{
    style, Ahoy, Message, DEFAULT_HEADING_FONT_SIZE, DEFAULT_PADDING, IMAGE_BRIDGE_4_LIGHT,
    IMAGE_BRIDGE_6_LIGHT, IMAGE_USB_CABLE_LIGHT,
};

pub(crate) fn handle_view(ahoy: &mut Ahoy) -> Element<Message> {
    // BUILD PRIMARY VIEW
    let inner_content: Element<Message> = if let Some(conn) = &ahoy.connection {
        /* WHEN A DEVICE IS CONNECTED */
        if ahoy.installing {
            Column::new()
                .padding(DEFAULT_PADDING)
                .push(ahoy.status.view(conn))
                .push(Rule::horizontal(1))
                .push(ahoy.installer.view(&ahoy.dfu_connection))
                .into()
        } else {
            // selecting a release
            Column::new()
                .padding(DEFAULT_PADDING)
                .push(ahoy.status.view(conn))
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
        }
    } else {
        /* WHEN A DEVICE IS NOT CONNECTED */
        let usb_cable_image =
            Svg::new(Handle::from_memory(IMAGE_USB_CABLE_LIGHT.clone())).height(Length::Units(400));

        let bridge6 =
            Svg::new(Handle::from_memory(IMAGE_BRIDGE_6_LIGHT.clone())).width(Length::Units(100));

        let bridge4 =
            Svg::new(Handle::from_memory(IMAGE_BRIDGE_4_LIGHT.clone())).width(Length::Units(100));

        Column::new()
            .align_items(Alignment::Center)
            .spacing(DEFAULT_PADDING)
            .width(Length::Fill)
            .push(usb_cable_image)
            .push(Space::with_height(Length::Units(DEFAULT_PADDING * 2)))
            .push(Text::new("Please connect your").size(DEFAULT_HEADING_FONT_SIZE))
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(DEFAULT_PADDING * 2)
                    .push(bridge6)
                    .push(Text::new("or").size(DEFAULT_HEADING_FONT_SIZE))
                    .push(bridge4),
            )
            .into()
    };

    // wrap modal around the inner content
    let modal_wrapped_content = ahoy.confirm_modal.view(inner_content);

    // setup graphical debugging
    let output = if *&mut ahoy.debug {
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
