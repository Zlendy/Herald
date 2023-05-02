use std::env;

use relm4::{adw, Component, factory::FactoryView, ComponentController, component::{Connector, AsyncComponent, AsyncComponentParts}, AsyncComponentSender};
use adw::{traits::PreferencesRowExt, gtk::ListBoxRow};
use gtk::prelude::*;
use relm4::gtk;
use serde_json::Value;
use crate::widgets::{message_factory::widget::{MessageFactory, FactoryMsg}};
use crate::models::gotify::message::MessageModel;
// use crate::widgets::app::App;
const GOTIFY: &str = "http://monitoring.beauvoir.local/gotify";

pub struct MessageContainerWidget {
    #[allow(dead_code)]
    current_section: u32,
    factory: Connector<MessageFactory>,
}

#[derive(Debug)]
pub enum MessageContainerSignals {
    SelectRow(ListBoxRow),
}

#[relm4::component(pub async)]
impl AsyncComponent for MessageContainerWidget {
    type Init = ();
    type Input = MessageContainerSignals;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[name = "flap"]
        adw::Flap {
            set_swipe_to_open: true,
            set_swipe_to_close: true,

            #[wrap(Some)]
            set_flap = &gtk::Box {
                set_width_request: 200,
                set_css_classes: &["background"],
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,

                #[name = "listbox"]
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

                    connect_row_selected[sender] => move |_, row| {
                        if let Some(row) = row {
                            sender.input(Self::Input::SelectRow(row.to_owned()));
                        }
                    }
                },
            },

            #[wrap(Some)]
            set_content = &adw::Clamp {

                #[name = "content"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    
                    // Child
                }
            },
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<MessageContainerWidget>
    ) -> AsyncComponentParts<Self> {
        let factory = MessageFactory::builder().launch(MessageModel::default());

        let response = get_messages().await.unwrap(); // TODO: Move to Gotify struct
        let messages: Vec<MessageModel> = serde_json::from_str(&response["messages"].to_string()).unwrap();

        for message in messages {
            // log::info!("{:#?}", message);
            factory.emit(FactoryMsg::AddMessageBack(message));
        }

        let model = MessageContainerWidget {
            current_section: 1,
            factory,
        };

        let widgets = view_output!();

        widgets
            .content
            .factory_append(model.factory.widget(), &());

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(&mut self, _widgets: &mut Self::Widgets, msg: Self::Input, _sender: AsyncComponentSender<Self>, _root: &Self::Root) {
        match msg {
            Self::Input::SelectRow(row) => {
                log::info!("Select Row: \"{}\"", row.index());
            }
        }
    }
}

async fn get_messages() -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new()
        .get(format!("{}/message", GOTIFY))
        .header("X-Gotify-Key", env::var("TOKEN").unwrap());

    let resp = client.send().await?.json::<Value>().await?;

    Ok(resp)
}