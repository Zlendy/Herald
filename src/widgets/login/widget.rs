use relm4::adw;
use adw::traits::PreferencesRowExt;
use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts, AsyncComponentSender},
    gtk, RelmWidgetExt,
};
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

use crate::{models::gotify::client::{ClientModel, CreateClientEnum}, services::gotify::GotifyService, widgets::app::App};

pub struct LoginWidget {
    #[allow(dead_code)]
    current_section: u32, // Unused for now
    username: String,
    password: String,
    token: String,
}

#[derive(Debug, PartialEq)]
pub enum LoginMsg {
    Login,
    SetUsername(String),
    SetPassword(String),
}

#[relm4::component(pub async)]
impl AsyncComponent for LoginWidget {
    type Init = ();
    type Input = LoginMsg;
    type Output = ();
    type CommandOutput = LoginMsg;

    view! {
        adw::Clamp {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
    
                adw::EntryRow {
                    set_title: "Username",
                    connect_changed[sender] => move |entry| {
                        let text = entry.text().to_string();
                        sender.input(LoginMsg::SetUsername(text));
                    },
                },
    
                adw::PasswordEntryRow {
                    set_title: "Password",
                    connect_changed[sender] => move |entry| {
                        let text = entry.text().to_string();
                        sender.input(LoginMsg::SetPassword(text));
                    },
                },
    
                gtk::Button {
                    set_label: "Login",
    
                    connect_clicked[sender] => move |_| {
                        sender.oneshot_command(async move {
                            LoginMsg::Login
                        })
                    },
                },
    
    
                gtk::Label {
                    set_margin_all: 5,
                    #[watch]
                    set_label: &format!("Token: \"{}\"", model.token),
                },
            },
        }
    }

    async fn update_cmd(
        &mut self,
        msg: LoginMsg,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        if msg != LoginMsg::Login { return; } // Only process "Login" events

        let possibilities = GotifyService::instance().create_client(self.username.as_str(), self.password.as_str()).await;
        match possibilities {
            CreateClientEnum::Success(model) => {
                match model.token {
                    Some(token) => {
                        self.token = token;
                    }
                    None => {
                        log::error!("Token is None");
                    }
                }
            },
            CreateClientEnum::Error(model) => {
                // TODO: Show toast
            },
            _ => {
                // TODO: Show toast
            }
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
            },
            LoginMsg::SetPassword(password) => {
                self.password = password;
            },
            _ => {}
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>
    ) -> AsyncComponentParts<Self> {
        let model = LoginWidget {
            current_section: 1,
            username: String::from(""),
            password: String::from(""),
            token: String::from(""),
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
