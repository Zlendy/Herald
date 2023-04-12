use relm4::adw;
use adw::traits::PreferencesRowExt;
use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts, AsyncComponentSender},
    gtk, RelmWidgetExt,
};
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

const GOTIFY: &str = "http://monitoring.beauvoir.local/gotify";

pub struct LoginView {
    #[allow(dead_code)]
    current_section: u32, // Unused for now
    username: String,
    password: String,
    token: String,
}

#[derive(Debug, PartialEq)]
pub enum LoginViewMsg {
    Login,
    SetUsername(String),
    SetPassword(String),
}

#[relm4::component(pub async)]
impl AsyncComponent for LoginView {
    type Init = ();
    type Input = LoginViewMsg;
    type Output = ();
    type CommandOutput = LoginViewMsg;

    view! {
        #[name = "leaflet"]
        adw::Leaflet {
            set_can_navigate_back: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::ListBox {
                    set_selection_mode: gtk::SelectionMode::Single,
                    add_css_class: "navigation-sidebar",

                    adw::ActionRow {
                        set_title: "App 1",
                    },

                    adw::ActionRow {
                        set_title: "App 2",
                    },

                    adw::ActionRow {
                        set_title: "App 3",
                    },

                    // connect_row_selected[sender] => move |_, row| {
                    //     if let Some(row) = row {
                    //         sender.input((row.index() + 1) as u32);
                    //     }
                    // }
                }
            },

            gtk::Box {
                // gtk::Label {
                //     add_css_class: "title-1",
                //     set_vexpand: true,

                //     #[watch]
                //     set_text: &format!("Page {}", model.current_section),
                // }

                adw::EntryRow {
                    set_title: "Username",
                    connect_changed[sender] => move |entry| {
                        let text = entry.text().to_string();
                        sender.input(LoginViewMsg::SetUsername(text));
                    },
                },

                adw::PasswordEntryRow {
                    set_title: "Password",
                    connect_changed[sender] => move |entry| {
                        let text = entry.text().to_string();
                        sender.input(LoginViewMsg::SetPassword(text));
                    },
                },

                gtk::Button {
                    set_label: "Login",

                    connect_clicked[sender] => move |_| {
                        sender.oneshot_command(async move {
                            LoginViewMsg::Login
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
        msg: LoginViewMsg,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        if msg != LoginViewMsg::Login { return; } // Only process "Login" events


        log::info!("Username: \"{}\". Password: \"{}\"", &self.username, &self.password);
        let response = LoginView::create_client(&self.username.as_str(), &self.password.as_str()).await.unwrap();
            log::info!("{:#?}", &response);

            match response.get("token").cloned() { // TODO: Fix extra brackets
                Some(token) => {
                    log::info!("{}", token);
                    self.token = token.to_string();
                },
                None => {
                    log::error!("Invalid credentials")
                }
            }
    }

    async fn update(
        &mut self,
        msg: LoginViewMsg,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            LoginViewMsg::SetUsername(username) => {
                self.username = username;
            },
            LoginViewMsg::SetPassword(password) => {
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
        let model = LoginView {
            current_section: 1,
            username: String::from(""),
            password: String::from(""),
            token: String::from(""),
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    fn pre_view() {
        // widgets.leaflet.navigate(adw::NavigationDirection::Forward);
    }
}

impl LoginView {
    pub async fn create_client(username: &str, password: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let body: ClientModel = ClientModel::new("Herald");

        let client = reqwest::Client::new()
            .post(format!("{}/client", GOTIFY))
            .basic_auth(username, Some(password))
            .json::<ClientModel>(&body);

        let resp = client.send()
            .await?
            .json::<Value>()
            .await?;

        Ok(resp)
    }
}

#[derive(Serialize, Deserialize)]
struct ClientModel {
    id: Option<i32>,
    name: String,
    token: Option<String>,
}

impl ClientModel {
    pub fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            token: None,
        }
    }
}