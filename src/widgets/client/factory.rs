use adw::traits::PreferencesRowExt;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use gtk::prelude::*;
use libadwaita::prelude::EditableExt;
use relm4::component::{AsyncComponent, AsyncComponentParts};

use relm4::factory::{
    AsyncFactoryComponent, AsyncFactorySender, AsyncFactoryVecDeque, DynamicIndex,
};

use relm4::{adw, gtk, AsyncComponentSender, RelmWidgetExt};

use crate::models::gotify::client::ClientModel;
use crate::services::gotify::GotifyService;

#[derive(Debug)]
pub enum ClientModelOutput {
    Remove(DynamicIndex),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ClientModel {
    type Init = ClientModel;
    type Input = ();
    type Output = ClientModelOutput;
    type CommandOutput = ();
    type ParentInput = FactorySignal;
    type ParentWidget = adw::PreferencesPage; //gtk::Box;

    view! {
        root = adw::PreferencesGroup {
            // adw::PreferencesRow {
            //     #[wrap(Some)]
            //     set_child = &gtk::Label {
            //         set_text: &self.name,
            //     }
            // },
            adw::EntryRow {
                set_title: "Name",
                set_text: &self.name

            },
            adw::PasswordEntryRow {
                set_title: "Token",
                set_text: &self.token.to_owned().unwrap_or_default(),
                set_editable: false,

            },
            adw::PreferencesRow {
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_homogeneous: true,

                    gtk::Button {
                        set_label: "Save",
                    },
                    gtk::Button {
                        set_label: "Copy token",
                    //     connect_clicked => move |_| {
                    //         let mut ctx = ClipboardContext::new().unwrap();
                    //         match &self.token {
                    //             Some(token) => {
                    //                 ctx.set_contents(token.to_owned()).unwrap();
                    //             },
                    //             None => {
                    //                 log::error!("Could not copy to clipboard! Token is empty.");
                    //             }
                    //         }

                    //     },
                    },
                    gtk::Button {
                        set_label: "Delete",
                    }
                }
            },
        }

    }

    fn output_to_parent_input(output: Self::Output) -> Option<FactorySignal> {
        Some(match output {
            ClientModelOutput::Remove(index) => FactorySignal::Remove(index),
        })
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        // tokio::time::sleep(Duration::from_secs(1)).await;
        // Self { title: init.title.clone(), content: init.content.clone() }
        init
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        // log::debug!("Message with id {} was destroyed", self.id);
    }
}

pub struct ClientFactory {
    default_widget: ClientModel,
    messages: AsyncFactoryVecDeque<ClientModel>,
}

#[derive(Debug)]
pub enum FactorySignal {
    PushDefault,
    PushBack(ClientModel),
    Remove(DynamicIndex),
    ClearLocalData,
    SetData,
}

#[relm4::component(pub async)]
impl AsyncComponent for ClientFactory {
    type Init = ClientModel;
    type Input = FactorySignal;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_hexpand: true,

            gtk::Box {
                set_hexpand: true,
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Create client (TODO)",
                    connect_clicked => FactorySignal::PushDefault,
                },

                gtk::ScrolledWindow {
                    set_vexpand: true,

                    #[local_ref]
                    message_box -> adw::PreferencesPage {
                        // set_orientation: gtk::Orientation::Vertical,
                        // set_spacing: 5,
                    }
                }
            }
        }
    }

    async fn init(
        default_widget: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let messages =
            AsyncFactoryVecDeque::new(adw::PreferencesPage::default(), sender.input_sender());

        let model = ClientFactory {
            default_widget,
            messages,
        };

        let message_box = model.messages.widget();
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let mut guard = self.messages.guard();
        match msg {
            FactorySignal::PushDefault => {
                guard.push_front(self.default_widget.clone());
            }
            FactorySignal::PushBack(model) => {
                guard.push_back(model);
            }
            FactorySignal::Remove(index) => {
                // let Some(model) = guard.get(index.current_index()) else {
                //     log::error!("Message with index {} not found", index.current_index());
                //     return;
                // };

                // log::info!("Requested deletion of message with id {}", model.id);
                // let response = GotifyService::instance().delete_message(model.id).await;

                // let Ok(result) = response else { // There was an error in the request
                //     return;
                // };

                // if result.is_some() && result.unwrap().error_code != 404 {
                //     // Requested ID does not exist on the server (it is safe to delete the entry locally)
                //     return;
                // }

                // guard.remove(index.current_index());
            }
            FactorySignal::ClearLocalData => {
                guard.clear();
            }
            FactorySignal::SetData => {
                let Ok(clients) = GotifyService::instance().get_clients().await else {
                    return;
                };

                sender.input(FactorySignal::ClearLocalData);
                for client in clients {
                    log::debug!("{:#?}", client);
                    sender.input(FactorySignal::PushBack(client));
                }
            }
        }
    }
}
