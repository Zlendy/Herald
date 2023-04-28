use std::env;

use relm4::{adw, Component, factory::FactoryView, ComponentController, component::{Connector, AsyncComponent, AsyncComponentParts}, AsyncComponentSender};
use adw::{traits::PreferencesRowExt, gtk::ListBoxRow, glib};
use gtk::prelude::*;
use relm4::gtk;
use serde_json::Value;
use crate::widgets::{message_factory::widget::{MessageFactory, FactoryMsg}};
use crate::widgets::message_factory::models::MessageModel;
// use crate::widgets::app::App;
const GOTIFY: &str = "http://monitoring.beauvoir.local/gotify";

pub struct MessageContainerWidget {
    #[allow(dead_code)]
    current_section: u32,
    factory: Connector<MessageFactory>,
}

#[derive(Debug)]
pub enum MessageContainerSignals {
    GoBack,
    SelectRow(ListBoxRow),
}

#[relm4::component(pub async)]
impl AsyncComponent for MessageContainerWidget {
    type Init = ();
    type Input = MessageContainerSignals;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[name = "leaflet"]
        adw::Leaflet {
            set_can_navigate_back: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,

                // #[name = "sidebar_header"]
                //     adw::HeaderBar {
                //         #[wrap(Some)]
                //         set_title_widget = &adw::WindowTitle {
                //             set_title: "Sidebar",
                //     }
                // },

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

            #[name = "content"]
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                
                // Child
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

        widgets
            .listbox
            .bind_property("visible", &widgets.leaflet, "can-navigate-back")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .flags(glib::BindingFlags::INVERT_BOOLEAN)
            .build();

        AsyncComponentParts { model, widgets }
    }

    async fn pre_view() {
        // widgets.leaflet.navigate(adw::NavigationDirection::Forward);
    }

    async fn update_with_view(&mut self, widgets: &mut Self::Widgets, msg: Self::Input, _sender: AsyncComponentSender<Self>, _root: &Self::Root) {
        match msg {
            Self::Input::GoBack => {
                log::info!("Go Back");

                widgets.leaflet.navigate(adw::NavigationDirection::Back);
                widgets.listbox.set_visible(true);
                // widgets.leaflet.set_property("can-navigate-back", true);
            }

            Self::Input::SelectRow(row) => {
                log::info!("Select Row");

                widgets.listbox.select_row(Some(row).as_ref());
                widgets.listbox.set_visible(false);
                // widgets.leaflet.set_property("can-navigate-back", false);
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