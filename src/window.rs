use cosmic::app::Core;
use cosmic::applet::padded_control;
use cosmic::iced::alignment::Horizontal;
use cosmic::iced::wayland::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::iced::{Command, Length, Limits};
use cosmic::iced_runtime::core::window;
use cosmic::iced_style::application;
use cosmic::iced_widget::{row, Column};
use cosmic::widget::{button, icon, slider, text};
use cosmic::{Element, Theme};

use crate::monitor::Monitor;

const ID: &str = "com.maciekk64.CosmicAppletExternalMonitorBrightness";
const ICON_HIGH: &str = "cosmic-applet-battery-display-brightness-high-symbolic";
const ICON_MEDIUM: &str = "cosmic-applet-battery-display-brightness-medium-symbolic";
const ICON_LOW: &str = "cosmic-applet-battery-display-brightness-low-symbolic";
const ICON_OFF: &str = "cosmic-applet-battery-display-brightness-off-symbolic";

#[derive(Default)]
pub struct Window {
    core: Core,
    popup: Option<Id>,
    monitors: Vec<Monitor>,
}

#[derive(Clone, Debug)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    SetScreenBrightness(usize, u16),
    ToggleMinMaxBrightness(usize),
}

impl cosmic::Application for Window {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(
        core: Core,
        _flags: Self::Flags,
    ) -> (Self, Command<cosmic::app::Message<Self::Message>>) {
        let monitors = Monitor::new_vec();
        let window = Window {
            core,
            monitors,
            ..Default::default()
        };

        (window, Command::none())
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Self::Message) -> Command<cosmic::app::Message<Self::Message>> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    for monitor in &mut self.monitors {
                        monitor.update_brightness();
                    }

                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings =
                        self.core
                            .applet
                            .get_popup_settings(Id::MAIN, new_id, None, None, None);
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        // .min_width(300.0)
                        // .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::SetScreenBrightness(id, brightness) => {
                self.monitors[id].set_screen_brightness(brightness);
            }
            Message::ToggleMinMaxBrightness(id) => {
                let monitor = &mut self.monitors[id];
                monitor.set_screen_brightness(match monitor.brightness {
                    0 => 100,
                    _ => 0,
                });
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        self.core
            .applet
            .icon_button(
                self.monitors
                    .first()
                    .map(|v| brightness_icon(v.brightness))
                    .unwrap_or(ICON_HIGH),
            )
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        let mut content = vec![];
        for monitor in &self.monitors {
            content.push(
                padded_control(
                    row![
                        button::icon(
                            icon::from_name(brightness_icon(monitor.brightness))
                                .size(24)
                                .symbolic(true)
                        )
                        .on_press(Message::ToggleMinMaxBrightness(monitor.id)),
                        slider(0..=100, monitor.brightness, |brightness| {
                            Message::SetScreenBrightness(monitor.id, brightness)
                        }),
                        text(format!("{:.0}%", monitor.brightness))
                            .size(16)
                            .width(Length::Fixed(40.0))
                            .horizontal_alignment(Horizontal::Right)
                    ]
                    .spacing(12),
                )
                .into(),
            )
        }

        self.core
            .applet
            .popup_container(Column::with_children(content))
            .into()
    }

    fn style(&self) -> Option<<Theme as application::StyleSheet>::Style> {
        Some(cosmic::applet::style())
    }
}

fn brightness_icon(brightness: u16) -> &'static str {
    if brightness > 66 {
        ICON_HIGH
    } else if brightness > 33 {
        ICON_MEDIUM
    } else if brightness > 0 {
        ICON_LOW
    } else {
        ICON_OFF
    }
}
