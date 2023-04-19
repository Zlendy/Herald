use relm4::{adw, Component, factory::FactoryView, ComponentController, component::Connector};
use adw::traits::PreferencesRowExt;
use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts, AsyncComponentSender},
    gtk, RelmWidgetExt,
};
use crate::views::factory_async::MessageFactory;

pub struct MessagesView {
    #[allow(dead_code)]
    current_section: u32, // Unused for now
    factory: Connector<MessageFactory>,
}

#[derive(Debug, PartialEq)]
pub enum MessageType {}

#[relm4::component(pub async)]
impl AsyncComponent for MessagesView {
    type Init = ();
    type Input = MessageType;
    type Output = ();
    type CommandOutput = MessageType;

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

    async fn update_cmd(
        &mut self,
        msg: MessageType,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
    }

    async fn update(
        &mut self,
        msg: MessageType,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>
    ) -> AsyncComponentParts<Self> {
        let factory = MessageFactory::builder().launch(0);

        let model = MessagesView {
            current_section: 1,
            factory,
        };

        let widgets = view_output!();

        widgets
            .content
            .factory_append(model.factory.widget(), &());


        AsyncComponentParts { model, widgets }
    }

    fn pre_view() {
        // widgets.leaflet.navigate(adw::NavigationDirection::Forward);
    }
}
