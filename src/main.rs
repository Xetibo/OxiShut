use std::{path::PathBuf, process::Command};

use iced::widget::container;
use iced::{
    Alignment, Element, Length, Renderer, Subscription, Task, Theme, event,
    keyboard::{Modifiers, key::Named},
    widget::{button::Status, container::Style},
};
use iced_layershell::{
    Application,
    actions::LayershellCustomActions,
    reexport::{Anchor, KeyboardInteractivity, Layer},
    settings::{LayerShellSettings, Settings},
};
use oxiced::theme::theme::{get_derived_iced_theme, OXITHEME};
use oxiced::{
    widgets::{
        oxi_button::{self, ButtonVariant},
    },
};

#[derive(Debug, Default)]
struct OxiShut {
    theme: Theme,
    focused_action: Action,
}

#[derive(Debug, Clone)]
enum FocusDirection {
    Right,
    Left,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
enum Action {
    #[default]
    ShutDown,
    Reboot,
    Sleep,
}

impl Action {
    pub fn next(self) -> Self {
        match self {
            Action::ShutDown => Action::Reboot,
            Action::Reboot => Action::Sleep,
            Action::Sleep => Action::ShutDown,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Action::ShutDown => Action::Sleep,
            Action::Reboot => Action::ShutDown,
            Action::Sleep => Action::Reboot,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    MoveFocus(FocusDirection),
    Action(Action),
    CurrentAction,
}

impl TryInto<LayershellCustomActions> for Message {
    type Error = Self;
    fn try_into(self) -> Result<LayershellCustomActions, Self::Error> {
        Err(self)
    }
}

fn box_style(theme: &Theme) -> Style {
    let palette = OXITHEME;
    Style {
        background: Some(iced::Background::Color(
            palette.base
        )),
        border: iced::border::color(palette.primary).width(2.0).rounded(palette.border_radius),
        ..container::rounded_box(theme)
    }
}

fn run_action(action: &Action) -> Task<Message> {
    match action {
        Action::ShutDown => {
            Command::new("shutdown")
                .arg("now")
                .spawn()
                .expect("No shutdown process available?");
            std::process::exit(0)
        }
        Action::Reboot => {
            Command::new("reboot")
                .spawn()
                .expect("No reboot process available?");
            std::process::exit(0)
        }
        Action::Sleep => {
            Command::new("playerctl")
                .arg("-a")
                .arg("pause")
                .spawn()
                .expect("No playerctl available?");
            Command::new("hyprlock")
                .spawn()
                .expect("No hyprlock available?");
            Command::new("systemctl")
                .arg("suspend")
                .spawn()
                .expect("No soystemd available?");
            std::process::exit(0)
        }
    }
}

fn wrap_in_rounded_box<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message> {
    container(content)
        .style(box_style)
        .align_x(Alignment::Center)
        .padding(50)
        .width(Length::Fill)
        .into()
}

#[cfg(debug_assertions)]
fn svg_path(asset: &'static str) -> PathBuf {
    PathBuf::from(format!("./assets/{}", asset))
}

// TODO find a better way than this.
#[cfg(not(debug_assertions))]
fn svg_path(asset: &'static str) -> PathBuf {
    use std::env;
    use std::path::Path;
    match env::current_exe() {
        Ok(exe_path) => exe_path
            .parent()
            .unwrap_or(&Path::new("/"))
            .join(format!("../share/pixmaps/oxishut/{}", asset)),
        Err(_) => PathBuf::from(format!("./assets/{}", asset)),
    }
}

fn mk_button<'a>(
    asset: &'static str,
    action: Action,
    focused_action: &Action,
) -> Element<'a, Message> {
    let handle = iced::widget::svg::Handle::from_path(svg_path(asset));
    let svg = iced::widget::svg(handle).content_fit(iced::ContentFit::Contain).style(move |_, _| {
        let palette = OXITHEME;
        iced::widget::svg::Style { color: Some(palette.primary) }
    });
    let is_focused = focused_action == &action;
    oxiced::widgets::oxi_button::button(
        iced::widget::row!(svg)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_y(Alignment::Center),
        ButtonVariant::Primary,
    )
    .on_press(Message::Action(action))
    .height(Length::Fill)
    .width(Length::Fill)
    .style(move |theme, status| {
        let palette = OXITHEME;
        let default_style = oxi_button::row_entry(theme, status);
        let background = if status == Status::Hovered {
            Some(iced::Background::Color(palette.primary_bg_active))
        } else if status == Status::Pressed {
            Some(iced::Background::Color(palette.primary_bg_active))
        } else if is_focused {
            Some(iced::Background::Color(palette.primary_bg_hover))
        } else {
            Some(iced::Background::Color(palette.primary_bg))
        };
        iced::widget::button::Style {
            background,
            ..default_style
        }
    })
    .into()
}

impl Application for OxiShut {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Task<Message>) {
        (
            Self {
                theme: get_derived_iced_theme(),
                focused_action: Action::ShutDown,
            },
            Task::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("OxiShut")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Action(action) => run_action(&action),
            Message::CurrentAction => run_action(&self.focused_action),
            Message::Exit => std::process::exit(0),
            Message::MoveFocus(focus_direction) => {
                match focus_direction {
                    FocusDirection::Right => self.focused_action = self.focused_action.next(),
                    FocusDirection::Left => self.focused_action = self.focused_action.previous(),
                };
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let shutdown_button = mk_button("shutdown.svg", Action::ShutDown, &self.focused_action);
        let reboot_button = mk_button("reboot.svg", Action::Reboot, &self.focused_action);
        let sleep_button = mk_button("sleep.svg", Action::Sleep, &self.focused_action);
        wrap_in_rounded_box(
            iced::widget::row!(shutdown_button, reboot_button, sleep_button,)
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(35.0),
        )
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen_with(move |event, _status, _id| match event {
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                modifiers: modifier,
                key: iced::keyboard::key::Key::Named(key),
                modified_key: _,
                physical_key: _,
                location: _,
                text: _,
            }) => match key {
                Named::Escape => Some(Message::Exit),
                Named::Enter => Some(Message::CurrentAction),
                Named::Tab => match modifier {
                    Modifiers::SHIFT => Some(Message::MoveFocus(FocusDirection::Left)),
                    _ => Some(Message::MoveFocus(FocusDirection::Right)),
                },
                _ => None,
            },
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                modifiers: _,
                key: _,
                modified_key: _,
                physical_key: iced::keyboard::key::Physical::Code(code),
                location: _,
                text: _,
            }) => match code {
                iced::keyboard::key::Code::Digit1 => Some(Message::Action(Action::ShutDown)),
                iced::keyboard::key::Code::Digit2 => Some(Message::Action(Action::Reboot)),
                iced::keyboard::key::Code::Digit3 => Some(Message::Action(Action::Sleep)),
                _ => None,
            },
            _ => None,
        })
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    // remove the annoying background color
    fn style(&self, _: &Self::Theme) -> iced_layershell::Appearance {
        let palette = OXITHEME;
        iced_layershell::Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: palette.text
        }
    }

    fn scale_factor(&self) -> f64 {
        SCALE_FACTOR
    }
}

const SCALE_FACTOR: f64 = 1.0;
const WINDOW_SIZE: (u32, u32) = (800, 400);
const WINDOW_MARGINS: (i32, i32, i32, i32) = (100, 100, 100, 100);
const WINDOW_LAYER: Layer = Layer::Overlay;
const WINDOW_KEYBAORD_MODE: KeyboardInteractivity = KeyboardInteractivity::Exclusive;

pub fn main() -> Result<(), iced_layershell::Error> {
    let settings = Settings {
        layer_settings: LayerShellSettings {
            size: Some(WINDOW_SIZE),
            exclusive_zone: 0,
            anchor: Anchor::Left | Anchor::Right,
            layer: WINDOW_LAYER,
            margin: WINDOW_MARGINS,
            keyboard_interactivity: WINDOW_KEYBAORD_MODE,
            ..Default::default()
        },
        ..Default::default()
    };
    OxiShut::run(settings)
}
