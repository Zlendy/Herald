use crate::models::gotify::message::MessageModel;

use crate::widgets::message::factory::{FactoryMsg, MessageFactory};
use adw::{gtk::ListBoxRow, traits::PreferencesRowExt};
use gtk::prelude::*;
use relm4::component::{AsyncComponentController, AsyncConnector};
use relm4::gtk;
use relm4::{
    adw,
    component::{AsyncComponent, AsyncComponentParts},
    factory::FactoryView,
    AsyncComponentSender,
};

pub struct MessageContainerWidget {
    #[allow(dead_code)]
    current_section: u32,
    factory: AsyncConnector<MessageFactory>,
}

#[derive(Debug)]
pub enum MessageContainerSignals {
    SelectRow(ListBoxRow),
    LoadMessages,
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
        sender: AsyncComponentSender<MessageContainerWidget>,
    ) -> AsyncComponentParts<Self> {
        let factory = MessageFactory::builder().launch(MessageModel::default());

        let model = MessageContainerWidget {
            current_section: 1,
            factory,
        };

        let widgets = view_output!();

        widgets.content.factory_append(model.factory.widget(), &());

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        _widgets: &mut Self::Widgets,
        msg: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            Self::Input::SelectRow(row) => {
                log::info!("Select Row: \"{}\"", row.index());
            }
            Self::Input::LoadMessages => self.factory.emit(FactoryMsg::SetMessages),
        }
    }
}
