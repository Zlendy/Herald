use adw::prelude::ListBoxRowExt;
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::component::{AsyncComponent, AsyncComponentParts};

use relm4::factory::{
    AsyncFactoryComponent, AsyncFactorySender, AsyncFactoryVecDeque, DynamicIndex,
};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{adw, gtk, AsyncComponentSender, RelmWidgetExt};

use crate::models::gotify::message::MessageModel;
use crate::services::gotify::GotifyService;

#[derive(Debug)]
pub enum MessageComponentOutput {
    Remove(DynamicIndex),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for MessageModel {
    type Init = MessageModel;
    type Input = ();
    type Output = MessageComponentOutput;
    type CommandOutput = ();
    type ParentInput = FactoryMsg;
    type ParentWidget = adw::PreferencesPage;

    view! {
        root = adw::PreferencesGroup {
            set_halign: gtk::Align::Fill,
            set_hexpand: true,
            // set_homogeneous: true,

            adw::PreferencesRow {
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_hexpand: true,

                        gtk::Label {
                            set_can_focus: false,
                            set_wrap: true,
                            set_hexpand: true,

                            set_halign: gtk::Align::Start,

                            set_use_markup: true,
                            set_markup: {
                                let value = match &self.title {
                                    Some(title) => {
                                        title
                                    }
                                    None => {
                                        ""
                                    }
                                };

                                format!("<b>{value}</b>").as_str() // TODO: Sanitize input
                            }
                        },

                        gtk::Button {
                            set_icon_name: "user-trash-symbolic",
                            connect_clicked[sender, index] => move |_| {
                                sender.output(MessageComponentOutput::Remove(index.clone()))
                            }
                        },
                    },

                    gtk::Label {
                        set_can_focus: false,
                        set_wrap: true,
                        set_halign: gtk::Align::Start,

                        set_text: &self.message,
                    },
                },
            },
        }
    }

    fn output_to_parent_input(output: Self::Output) -> Option<FactoryMsg> {
        Some(match output {
            MessageComponentOutput::Remove(index) => FactoryMsg::Remove(index),
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
        log::debug!("Message with id {} was destroyed", self.id);
    }
}

pub struct MessageFactory {
    default_widget: MessageModel,
    messages: AsyncFactoryVecDeque<MessageModel>,
}

#[derive(Debug)]
pub enum FactoryMsg {
    AddDefaultMessage,
    AddMessageBack(MessageModel),
    RemoveMessage,
    Remove(DynamicIndex),
    RemoveLocalMessages,
    SetMessages,
}

#[relm4::component(pub async)]
impl AsyncComponent for MessageFactory {
    type Init = MessageModel;
    type Input = FactoryMsg;
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
                    set_label: "Add test notification",
                    connect_clicked => FactoryMsg::AddDefaultMessage,
                },

                gtk::Button {
                    set_label: "Remove test notification",
                    connect_clicked => FactoryMsg::RemoveMessage,
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

        let model = MessageFactory {
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
            FactoryMsg::AddDefaultMessage => {
                guard.push_front(self.default_widget.clone());
            }
            FactoryMsg::AddMessageBack(model) => {
                guard.push_back(model);
            }
            FactoryMsg::RemoveMessage => {
                guard.pop_front();
            }
            FactoryMsg::Remove(index) => {
                let Some(model) = guard.get(index.current_index()) else {
                    log::error!("Message with index {} not found", index.current_index());
                    return;
                };

                log::info!("Requested deletion of message with id {}", model.id);
                let response = GotifyService::instance().delete_message(model.id).await;

                let Ok(result) = response else { // There was an error in the request
                    return;
                };

                if result.is_some() && result.unwrap().error_code != 404 {
                    // Requested ID does not exist on the server (it is safe to delete the entry locally)
                    return;
                }

                guard.remove(index.current_index());
            }
            FactoryMsg::RemoveLocalMessages => {
                // NOTE: This is prone to panicking if it is called twice in a short time span
                guard.clear();
            }
            FactoryMsg::SetMessages => {
                let Ok(paged_messages) = GotifyService::instance().get_messages().await else {
                    return;
                };

                sender.input(FactoryMsg::RemoveLocalMessages);
                for message in paged_messages.messages {
                    log::debug!("{:#?}", message);
                    sender.input(FactoryMsg::AddMessageBack(message));
                }
            }
        }
    }
}
