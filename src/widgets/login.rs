use std::env;

use adw::builders::ToastBuilder;
use adw::traits::{PreferencesPageExt, PreferencesRowExt};
use gtk::prelude::*;
use relm4::adw;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts, AsyncComponentSender},
    gtk,
};

use crate::widgets::app::GlobalActions;
use crate::{models::gotify::client::CreateClientEnum, services::gotify::GotifyService};

pub struct LoginWidget {
    #[allow(dead_code)]
    current_section: u32, // Unused for now

    server_url: String,
    username: String,
    password: String,
    token: String,
}

#[derive(Debug, PartialEq)]
pub enum LoginMsg {
    Login,
    SetServerUrl(String),
    SetUsername(String),
    SetPassword(String),
}

#[relm4::component(pub async)]
impl AsyncComponent for LoginWidget {
    type Init = ();
    type Input = LoginMsg;
    type Output = GlobalActions;
    type CommandOutput = LoginMsg;

    view! {
        adw::Clamp {
            adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    set_hexpand: true,

                    adw::EntryRow {
                        set_title: "Server URL",
                        set_text: model.server_url.as_str(),
                        connect_changed[sender] => move |entry| {
                            let text = entry.text().to_string();
                            sender.input(LoginMsg::SetServerUrl(text));
                        },
                    },

                    adw::EntryRow {
                        set_title: "Username",
                        set_text: model.username.as_str(),
                        connect_changed[sender] => move |entry| {
                            let text = entry.text().to_string();
                            sender.input(LoginMsg::SetUsername(text));
                        },
                    },

                    adw::PasswordEntryRow {
                        set_title: "Password",
                        set_text: model.password.as_str(),
                        connect_changed[sender] => move |entry| {
                            let text = entry.text().to_string();
                            sender.input(LoginMsg::SetPassword(text));
                        },
                    },

                    adw::PreferencesRow {

                        #[wrap(Some)]
                        set_child = &gtk::Button {
                            set_label: "Login",

                            connect_clicked[sender] => move |_| {
                                sender.oneshot_command(async move {
                                    LoginMsg::Login
                                })
                            },
                        },
                    }
                },
            }
        }
    }

    async fn update_cmd(
        &mut self,
        msg: LoginMsg,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        if msg != LoginMsg::Login {
            return;
        } // Only process "Login" events

        let Ok(_) = GotifyService::instance().set_base_url(self.server_url.clone()).await else {
            return; // Stop code execution
        };

        let possibilities = GotifyService::instance()
            .create_client(self.username.as_str(), self.password.as_str())
            .await;

        match possibilities {
            CreateClientEnum::Success(model) => match model.token {
                Some(token) => {
                    self.token = token;
                    // let window = root.toplevel_window()
                    // MESSAGE_BROKER
                    //     .sender()
                    //     .send(GlobalActions::ViewStackLoggedIn)
                    //     .unwrap();

                    sender.output_sender().emit(GlobalActions::LoggedIn);
                    // sender.input_sender()
                    // sender.input(message)
                }
                None => {
                    log::error!("Token is None");
                }
            },
            CreateClientEnum::Error(model) => {
                let toast = ToastBuilder::new().title(&model.error_descripton).build();
                sender.output(GlobalActions::ShowToast(toast)).unwrap();
            }
            _ => {}
        }
    }

    async fn update(
        &mut self,
        msg: LoginMsg,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            LoginMsg::SetUsername(username) => {
                self.username = username;
            }
            LoginMsg::SetPassword(password) => {
                self.password = password;
            }
            LoginMsg::SetServerUrl(server_url) => {
                self.server_url = server_url;
            }
            _ => {}
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = LoginWidget {
            current_section: 1,
            server_url: env::var("BASE_URL").unwrap_or_default(),
            username: env::var("USERNAME").unwrap_or_default(),
            password: env::var("PASSWORD").unwrap_or_default(),
            token: "".to_string(),
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
