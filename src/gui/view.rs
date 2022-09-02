use iced::{
    alignment::Horizontal, Alignment, Button, Color, Column, Container, Element, Length, Row, Rule,
    Space, Svg, Text,
};

use super::{
    style, Ahoy, Message, DEFAULT_HEADING_FONT_SIZE, DEFAULT_PADDING, IMAGE_BRIDGE_4_DARK,
    IMAGE_BRIDGE_6_DARK, IMAGE_PIRATE_MIDI_LOGO, IMAGE_USB_CABLE_DARK,
};

pub(crate) fn handle_view(ahoy: &mut Ahoy) -> Element<Message> {
    /* WHEN A DEVICE IS NOT CONNECTED */
    let usb_cable_image = Svg::new(IMAGE_USB_CABLE_DARK.clone()).height(Length::Units(400));
    let bridge6 = Svg::new(IMAGE_BRIDGE_6_DARK.clone()).width(Length::Units(100));
    let bridge4 = Svg::new(IMAGE_BRIDGE_4_DARK.clone()).width(Length::Units(100));
    let pm_logo =
        iced_native::widget::Image::new(IMAGE_PIRATE_MIDI_LOGO.clone()).width(Length::Units(200));

    // BUILD PRIMARY VIEW
    let content: Element<Message> = match &ahoy.device {
        super::DeviceState::Disconnected => Column::new()
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
            .push(Space::with_height(Length::Fill))
            .push(pm_logo)
            .into(),
        super::DeviceState::Connected(details) => {
            // selecting a release
            let inner_content = Column::new()
                .padding(DEFAULT_PADDING)
                .push(ahoy.status.view(&details))
                .push(Rule::horizontal(1))
                .push(ahoy.controls.view(&ahoy.filter))
                .push(Rule::horizontal(1))
                .push(ahoy.versions.view(
                    &ahoy.error,
                    &ahoy.filter,
                    &ahoy.releases,
                    &details,
                    &ahoy.selected_version,
                ))
                .into();

            // wrap modal around the inner content
            ahoy.confirm_modal.view(inner_content)
        }
        // device is connected in DFU mode
        super::DeviceState::DFU(device, _, _) => Column::new()
            .padding(DEFAULT_PADDING)
            .align_items(Alignment::Center)
            // .push(ahoy.status.view(&details))
            // .push(Rule::horizontal(1))
            .push(Space::with_height(Length::Fill))
            .push(ahoy.installer.view(ahoy.install_progress, device))
            .push(Space::with_height(Length::Fill))
            .push(pm_logo)
            .into(),
        super::DeviceState::PostInstall => Column::new()
            .align_items(Alignment::Center)
            .spacing(DEFAULT_PADDING)
            .width(Length::Fill)
            .push(Space::with_height(Length::Fill))
            .push(Text::new("Installation Complete!").size(DEFAULT_HEADING_FONT_SIZE))
            .push(Text::new("Unplug your device and go forth brave explorer!"))
            .push(Space::with_height(Length::Units(DEFAULT_PADDING * 2)))
            .push(
                Button::new(
                    &mut ahoy.reset_button,
                    Text::new("Close").horizontal_alignment(Horizontal::Center),
                )
                .on_press(Message::AttemptReset)
                .padding(DEFAULT_PADDING)
                .width(Length::Units(130))
                .style(style::Button::SuccessAction),
            )
            .push(Space::with_height(Length::Fill))
            .push(pm_logo)
            .into(),
    };

    // setup graphical debugging
    let output = if ahoy.debug {
        content.explain(Color::BLACK)
    } else {
        content
    };

    // finally wrap everything in a container.
    Container::new(output)
        .height(Length::Fill)
        .width(Length::Fill)
        .style(style::Container::Default)
        .into()
}
