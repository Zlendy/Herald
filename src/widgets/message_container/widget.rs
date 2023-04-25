use relm4::{adw, Component, factory::FactoryView, ComponentController, component::{Connector, AsyncComponent, AsyncComponentParts}, AsyncComponentSender};
use adw::{traits::PreferencesRowExt};
use gtk::prelude::*;
use relm4::gtk;
use crate::widgets::factory_async::{MessageFactory, MessageModel, FactoryMsg};

pub struct MessageContainerWidget {
    #[allow(dead_code)]
    current_section: u32, // Unused for now
    factory: Connector<MessageFactory>,
}

#[relm4::component(pub async)]
impl AsyncComponent for MessageContainerWidget {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        #[name = "leaflet"]
        adw::Leaflet {
            set_can_navigate_back: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,

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
        _sender: AsyncComponentSender<MessageContainerWidget>
    ) -> AsyncComponentParts<Self> {
        let factory = MessageFactory::builder().launch(MessageModel::default());

        factory.emit(FactoryMsg::AddMessage(MessageModel::new(0, "Test 1.1".to_string(), "Test 1.2".to_string())));
        factory.emit(FactoryMsg::AddMessage(MessageModel::new(0, "Test 2.1".to_string(), "Test 2.2".to_string())));
        factory.emit(FactoryMsg::AddMessage(MessageModel::new(0, "Test 3.1".to_string(), "Test 3.2".to_string())));

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

    async fn pre_view() {
        // widgets.leaflet.navigate(adw::NavigationDirection::Forward);
    }
}
