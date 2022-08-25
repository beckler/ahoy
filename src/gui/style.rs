use iced::{button, container, Background, Color};
use iced_aw::card;
use iced_aw::modal;

use super::DEFAULT_BORDER_RADIUS;
use super::DEFAULT_FONT_COLOR;

// COLORS
// Colors are between 0.0 and 1.0, but most color codes are in u8 with a max value of 255.
// So to translate from u8 to float, do this: {hex} / 255.0 = {decimal}

// #1F2528 - R31/G37/B40
pub static BLACK: Color = Color {
    r: 0.1216,
    g: 0.1450,
    b: 0.1568,
    a: 1.0,
};

// #EF5280 - R239/G82/B128
pub static PRIMARY: Color = Color {
    r: 0.9372,
    g: 0.3215,
    b: 0.5019,
    a: 1.0,
};

// #F15A5B - R241/G90/B91
// pub static PRIMARY_END: Color = Color {
//     r: 0.9450,
//     g: 0.3529,
//     b: 0.3568,
//     a: 1.0,
// };

// #5B8DCA - R91/G141/B202
pub static SECONDARY: Color = Color {
    r: 0.3568,
    g: 0.5529,
    b: 0.7921,
    a: 1.0,
};
pub static SECONDARY_HOVER: Color = Color {
    r: 0.3568,
    g: 0.5529,
    b: 0.7921,
    a: 0.8,
};
pub static SECONDARY_CLICK: Color = Color {
    r: 0.3568,
    g: 0.5529,
    b: 0.7921,
    a: 0.6,
};

// #85D1D4 - R133/G209/B212
pub static SECONDARY_END: Color = Color {
    r: 0.4784,
    g: 0.8196,
    b: 0.8313,
    a: 1.0,
};
pub static SECONDARY_END_HOVER: Color = Color {
    r: 0.4784,
    g: 0.8196,
    b: 0.8313,
    a: 0.8,
};
pub static SECONDARY_END_CLICK: Color = Color {
    r: 0.4784,
    g: 0.8196,
    b: 0.8313,
    a: 0.6,
};

// R248/G254/B167
pub static PRERELEASE: Color = Color {
    r: 0.9725,
    g: 0.9960,
    b: 0.6549,
    a: 1.0,
};
pub static PRERELEASE_HOVER: Color = Color {
    r: 0.9725,
    g: 0.9960,
    b: 0.6549,
    a: 0.7,
};
pub static PRERELEASE_CLICK: Color = Color {
    r: 0.9725,
    g: 0.9960,
    b: 0.6549,
    a: 0.5,
};

pub enum Button {
    SuccessAction,
    CancelAction,
    FilterOption,
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
            border_radius: DEFAULT_BORDER_RADIUS,
            ..button::Style::default()
        };

        match self {
            Button::SuccessAction => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: PRIMARY,
                border_color: PRIMARY,
                border_width: 1.0,
                ..basic
            },
            Button::CancelAction => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: SECONDARY,
                border_color: SECONDARY,
                border_width: 1.0,
                ..basic
            },
            Button::FilterOption => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: SECONDARY,
                border_color: SECONDARY,
                border_width: 1.0,
                ..basic
            },
            Button::FilterSelected => button::Style {
                background: Some(Background::Color(SECONDARY)),
                text_color: Color::WHITE,
                ..basic
            },
            Button::Release => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: SECONDARY_END,
                border_color: SECONDARY_END,
                border_width: 1.0,
                ..basic
            },
            Button::PreRelease => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: PRERELEASE,
                border_color: PRERELEASE,
                border_width: 1.0,
                ..basic
            },
            Button::ReleaseSelected => button::Style {
                background: Some(Background::Color(SECONDARY_END)),
                text_color: BLACK,
                ..basic
            },
            Button::PreReleaseSelected => button::Style {
                background: Some(Background::Color(PRERELEASE)),
                text_color: BLACK,
                ..basic
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            text_color: match self {
                Button::FilterSelected
                | Button::FilterOption
                | Button::SuccessAction
                | Button::CancelAction => Color::WHITE,
                _ => BLACK,
            },
            border_color: match self {
                Button::Release => SECONDARY_END_HOVER,
                Button::PreRelease => PRERELEASE_HOVER,
                Button::FilterOption => SECONDARY_HOVER,
                _ => active.border_color,
            },
            background: match self {
                Button::Release => Some(Background::Color(SECONDARY_END_HOVER)),
                Button::PreRelease => Some(Background::Color(PRERELEASE_HOVER)),
                Button::CancelAction => Some(Background::Color(SECONDARY_HOVER)),
                Button::FilterOption => Some(Background::Color(SECONDARY_HOVER)),
                Button::SuccessAction => Some(Background::Color(PRIMARY)),

                _ => active.background,
            },
            ..active
        }
    }

    fn pressed(&self) -> button::Style {
        let hovered = self.hovered();

        button::Style {
            border_color: match self {
                Button::Release | Button::ReleaseSelected => SECONDARY_END_CLICK,
                Button::PreRelease | Button::PreReleaseSelected => PRERELEASE_CLICK,
                Button::FilterOption | Button::FilterSelected => SECONDARY_CLICK,
                _ => hovered.border_color,
            },
            background: match self {
                Button::Release | Button::ReleaseSelected => {
                    Some(Background::Color(SECONDARY_END_CLICK))
                }
                Button::PreRelease | Button::PreReleaseSelected => {
                    Some(Background::Color(PRERELEASE_CLICK))
                }
                Button::FilterOption | Button::FilterSelected => {
                    Some(Background::Color(SECONDARY_CLICK))
                }
                _ => hovered.background,
            },
            ..hovered
        }
    }
}

pub enum Container {
    Error,
    Default,
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        let basic = container::Style {
            text_color: Some(DEFAULT_FONT_COLOR),
            border_radius: DEFAULT_BORDER_RADIUS,
            background: Some(Background::Color(BLACK)),
            ..container::Style::default()
        };

        match self {
            Container::Error => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb8(223, 84, 107))),
                ..basic
            },
            Container::Default => container::Style { ..basic },
        }
    }
}

pub enum Modal {
    Default,
}

impl modal::StyleSheet for Modal {
    fn active(&self) -> modal::Style {
        match self {
            Modal::Default => modal::Style {
                background: Background::Color(Color::from_rgba8(10, 10, 10, 0.86)),
            },
        }
    }
}

pub enum Card {
    Modal,
}

impl card::StyleSheet for Card {
    fn active(&self) -> card::Style {
        let basic = card::Style {
            background: Background::Color(Color::from_rgb8(245, 245, 245)),
            head_background: Color::TRANSPARENT.into(),
            border_radius: DEFAULT_BORDER_RADIUS,
            ..card::Style::default()
        };

        match self {
            Card::Modal => card::Style {
                // background: Background,
                // border_width: f32,
                // border_color: Color,
                // head_background: Background,
                // head_text_color: Color,
                // body_background: Background,
                // body_text_color: Color,
                // foot_background: Background,
                // foot_text_color: Color,
                // close_color: Color,
                ..basic
            },
        }
    }
}
