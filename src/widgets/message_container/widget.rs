use relm4::{adw, Component, factory::FactoryView, ComponentController, component::Connector, SimpleComponent, ComponentSender};
use adw::{traits::PreferencesRowExt, Leaflet};
use gtk::prelude::*;
use relm4::{
    component::ComponentParts,
    gtk,
};
use crate::widgets::factory_async::{MessageFactory, MessageComponent};

pub struct MessageContainerWidget {
    #[allow(dead_code)]
    current_section: u32, // Unused for now
    factory: Connector<MessageFactory>,
}

#[relm4::component(pub)]
impl SimpleComponent for MessageContainerWidget {
    type Init = ();
    type Input = ();
    type Output = ();

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

    fn init(
        _init: Self::Init,
        root: &Leaflet,
        _sender: ComponentSender<MessageContainerWidget>
    ) -> ComponentParts<Self> {
        let factory = MessageFactory::builder().launch(MessageComponent::default());

        let model = MessageContainerWidget {
            current_section: 1,
            factory,
        };

        let widgets = view_output!();

        widgets
            .content
            .factory_append(model.factory.widget(), &());


        ComponentParts { model, widgets }
    }

    fn pre_view() {
        // widgets.leaflet.navigate(adw::NavigationDirection::Forward);
    }
}
