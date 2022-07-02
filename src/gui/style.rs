use iced::{button, container, Background, Color};
use iced_aw::card;
// use iced_aw::modal;

pub enum Button {
    SuccessAction,
    CancelAction,
    FilterActive,
    FilterSelected,
    Release,
    ReleaseSelected,
    PreRelease,
    PreReleaseSelected,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        let basic = button::Style {
            border_color: Color::from_rgb8(210, 210, 210),
            border_radius: 5.0,
            ..button::Style::default()
        };

        match self {
            Button::SuccessAction => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Color::from_rgb8(110, 196, 146),
                border_color: Color::from_rgb8(110, 196, 146),
                border_width: 1.0,
                ..basic
            },
            Button::CancelAction => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Color::from_rgb8(223, 84, 107),
                border_color: Color::from_rgb8(223, 84, 107),
                border_width: 1.0,
                ..basic
            },
            Button::FilterActive => button::Style {
                background: Some(Background::Color(Color::from_rgb8(240, 245, 250))),
                text_color: Color::from_rgb8(63, 112, 164),
                ..basic
            },
            Button::FilterSelected => button::Style {
                background: Some(Background::Color(Color::from_rgb8(84, 140, 203))),
                text_color: Color::WHITE,
                ..basic
            },
            Button::Release => button::Style {
                background: Some(Background::Color(Color::from_rgb8(239, 254, 252))),
                text_color: Color::from_rgb8(69, 151, 132),
                ..basic
            },
            Button::PreRelease => button::Style {
                background: Some(Background::Color(Color::from_rgb8(254, 250, 236))),
                text_color: Color::from_rgb8(142, 110, 34),
                ..basic
            },
            Button::ReleaseSelected => button::Style {
                background: Some(Background::Color(Color::from_rgb8(95, 206, 179))),
                text_color: Color::WHITE,
                ..basic
            },
            Button::PreReleaseSelected => button::Style {
                background: Some(Background::Color(Color::from_rgb8(250, 225, 149))),
                text_color: Color::from_rgb8(75, 67, 44),
                ..basic
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            text_color: match self {
                Button::SuccessAction | Button::CancelAction => Color::WHITE,
                _ => active.text_color,
            },
            background: match self {
                Button::SuccessAction => Some(Background::Color(Color::from_rgb8(110, 196, 146))),
                Button::CancelAction => Some(Background::Color(Color::from_rgb8(223, 84, 107))),
                Button::FilterActive => Some(Background::Color(Color::from_rgb8(230, 239, 248))),
                Button::Release => Some(Background::Color(Color::from_rgb8(228, 254, 250))),
                Button::PreRelease => Some(Background::Color(Color::from_rgb8(253, 246, 224))),
                _ => active.background,
            },
            ..active
        }
    }

    fn pressed(&self) -> button::Style {
        let hovered = self.hovered();

        button::Style {
            background: match self {
                Button::FilterActive => Some(Background::Color(Color::from_rgb8(221, 232, 245))),
                Button::Release => Some(Background::Color(Color::from_rgb8(218, 254, 248))),
                Button::PreRelease => Some(Background::Color(Color::from_rgb8(253, 243, 213))),
                _ => hovered.background,
            },
            ..hovered
        }
    }
}

pub enum Container {
    Error,
    // Test,
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        match self {
            Container::Error => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb8(223, 84, 107))),
                border_radius: 5.0,
                ..Default::default()
            },
            // Container::Test => container::Style {
            //     background: Some(Background::Color(Color::from_rgb8(210, 210, 210))),
            //     ..Default::default()
            // },
        }
    }
}

// pub enum Modal {}

// impl modal::StyleSheet for Modal {
//     fn active(&self) -> modal::Style {
//         modal::Style {
//             ..Default::default()
//         }
//     }
// }

pub enum Card {
    Modal,
}

impl card::StyleSheet for Card {
    fn active(&self) -> card::Style {
        match self {
            Card::Modal => card::Style {
                // background: Background,
                border_radius: 5.0,
                // border_width: f32,
                // border_color: Color,
                // head_background: Background,
                // head_text_color: Color,
                // body_background: Background,
                // body_text_color: Color,
                // foot_background: Background,
                // foot_text_color: Color,
                // close_color: Color,
                ..Default::default()
            },
        }
    }
}
