use adw::prelude::*;
use gtk::glib::timeout_add_seconds;
use gtk::glib::{self, ControlFlow};
use relm4::prelude::*;

use crate::components::header::HeaderModel;
use crate::utils::format_seconds::format_seconds;

#[tracker::track]
pub struct FlowFast {
    focus_time: u32,
    break_time: u32,
    #[tracker::no_eq]
    header: Controller<HeaderModel>,
    state: FlowFastState,
    timer_state: FlowFastTimerState,
    timer_id: Option<glib::SourceId>,
}

#[derive(PartialEq, Debug)]
pub enum FlowFastState {
    Focused,
    Idle,
    Break,
}

#[derive(PartialEq, Debug)]
pub enum FlowFastTimerState {
    Paused,
    Playing,
}

#[derive(Debug)]
pub enum Msg {
    DecrementBreak,
    IncrementFocus,
    Pause,

    ToggleState,
    HandleBreak,

    SetFocus,
    SetBreak,

    StartBreak,
    StartFocus,
}

#[relm4::component(pub)]
impl SimpleComponent for FlowFast {
    type Init = FlowFastState;
    type Input = Msg;
    type Output = ();

    view! {
        #[root]
        adw::ApplicationWindow {
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                append = model.header.widget(),

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 10,
                    set_spacing: 10,
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,


                    gtk::Label {
                        #[watch]
                        set_label: if model.state == FlowFastState::Break {
                            "Take a break"
                        } else {
                            "Focus"
                        }
                    },

                    gtk::Label {
                        #[watch]
                        set_visible: model.state == FlowFastState::Break,


                        #[watch]
                        set_label: &format!("{}", format_seconds(model.break_time)),
                    },

                    gtk::Label {
                        #[watch]
                        set_visible: model.state == FlowFastState::Focused || model.state == FlowFastState::Idle,

                        #[watch]
                        set_label: &format!("{}", format_seconds(model.focus_time)),
                    },



                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_halign: gtk::Align::Center,

                        gtk::Button {
                            #[watch]
                            set_visible: model.state == FlowFastState::Focused,
                            set_icon_name: "media-playback-stop-symbolic",

                            connect_clicked => Msg::SetBreak,
                        },

                        gtk::Button {
                            #[watch]
                            set_icon_name: if model.timer_state == FlowFastTimerState::Paused || model.state == FlowFastState::Idle {
                                "media-playback-start-symbolic"
                            } else {
                                "media-playback-pause-symbolic"
                            },

                            #[watch]
                            set_visible: model.state != FlowFastState::Break,

                            connect_clicked => Msg::ToggleState,
                        },

                        gtk::Button {
                            #[watch]
                            set_icon_name: if model.timer_state == FlowFastTimerState::Paused {
                                "media-playback-start-symbolic"
                            } else {
                                "media-playback-stop-symbolic"
                            },

                            #[watch]
                            set_visible: model.state == FlowFastState::Break,


                            connect_clicked => Msg::HandleBreak
                        },
                    }
                }
            }

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header = HeaderModel::builder()
            .launch(String::from("FlowFast"))
            .detach();

        let model = FlowFast {
            focus_time: 0,
            break_time: 0,
            header,
            timer_id: None,
            timer_state: FlowFastTimerState::Paused,
            state: FlowFastState::Idle,
            tracker: 0,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Msg::IncrementFocus => {
                self.focus_time += 1;
            }
            Msg::SetFocus => {
                if let Some(id) = self.timer_id.take() {
                    id.remove();
                }
                self.state = FlowFastState::Focused;
                self.timer_state = FlowFastTimerState::Paused;
            }
            Msg::StartFocus => {
                self.timer_state = FlowFastTimerState::Playing;

                let sender_clone = sender.clone();
                let timer_id = timeout_add_seconds(1, move || {
                    sender_clone.input(Msg::IncrementFocus);
                    ControlFlow::Continue
                });

                self.timer_id = Some(timer_id);
            }
            Msg::DecrementBreak => {
                if self.break_time <= 0 {
                    if let Some(id) = self.timer_id.take() {
                        id.remove();
                    }

                    let sender_clone = sender.clone();
                    sender_clone.input(Msg::SetFocus);
                    self.timer_state = FlowFastTimerState::Paused;
                } else {
                    self.break_time -= 1;
                }
            }

            Msg::HandleBreak => {
                let sender_clone = sender.clone();

                if self.timer_state == FlowFastTimerState::Paused
                    && self.state == FlowFastState::Break
                {
                    sender_clone.input(Msg::StartBreak);
                    self.timer_state = FlowFastTimerState::Playing;
                } else if self.timer_state == FlowFastTimerState::Playing
                    && self.state == FlowFastState::Break
                {
                    sender_clone.input(Msg::SetFocus);
                }
            }

            Msg::SetBreak => {
                if let Some(id) = self.timer_id.take() {
                    id.remove();
                }
                self.state = FlowFastState::Break;
                self.timer_state = FlowFastTimerState::Paused;

                self.break_time = self.focus_time / 5;

                self.focus_time = 0;
            }

            Msg::StartBreak => {
                let sender_clone = sender.clone();
                let timer_id = timeout_add_seconds(1, move || {
                    sender_clone.input(Msg::DecrementBreak);

                    ControlFlow::Continue
                });

                self.timer_id = Some(timer_id);
            }

            Msg::ToggleState => {
                if let Some(id) = self.timer_id.take() {
                    id.remove();
                }

                let sender_clone = sender.clone();

                if self.timer_state == FlowFastTimerState::Paused
                    && self.state != FlowFastState::Break
                    || self.state == FlowFastState::Idle
                {
                    sender_clone.input(Msg::SetFocus);
                    sender_clone.input(Msg::StartFocus);
                } else {
                    self.timer_state = FlowFastTimerState::Paused;
                }
            }

            Msg::Pause => {
                self.timer_state = FlowFastTimerState::Paused;
            }
        }
    }
}
