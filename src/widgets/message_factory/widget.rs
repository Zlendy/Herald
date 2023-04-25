use std::time::Duration;
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};

use relm4::factory::{
    AsyncFactoryComponent, AsyncFactorySender, AsyncFactoryVecDeque, DynamicIndex,
};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{gtk, view, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};
use super::models::MessageModel;

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
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_halign: gtk::Align::Fill,
            set_hexpand: true,
            set_homogeneous: true,

            gtk::ListBoxRow {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_hexpand: true,

                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Start,
                            
                            set_text: match &self.title {
                                Some(title) => {
                                    title
                                }
                                None => {
                                    ""
                                }
                            },
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

    fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local_ref]
            root {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                #[name(spinner)]
                gtk::Spinner {
                    start: (),
                    set_hexpand: true,
                    set_halign: gtk::Align::Center,
                    // Reserve vertical space
                    set_height_request: 34,
                }
            }
        }
        Some(LoadingWidgets::new(root, spinner))
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
        tokio::time::sleep(Duration::from_secs(1)).await;
        // Self { title: init.title.clone(), content: init.content.clone() }
        init
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        println!("Counter with value {} was destroyed", self.message);
    }
}

pub struct MessageFactory {
    created_widgets: MessageModel,
    messages: AsyncFactoryVecDeque<MessageModel>,
}

#[derive(Debug)]
pub enum FactoryMsg {
    AddDefaultMessage,
    AddMessage(MessageModel),
    RemoveMessage,
    Remove(DynamicIndex),
}

#[relm4::component(pub)]
impl SimpleComponent for MessageFactory {
    type Init = MessageModel;
    type Input = FactoryMsg;
    type Output = ();

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
                    message_box -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 5,
                    }
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let messages = AsyncFactoryVecDeque::new(gtk::Box::default(), sender.input_sender());

        let model = MessageFactory {
            created_widgets: counter,
            messages,
        };

        let message_box = model.messages.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        let mut guard = self.messages.guard();
        match msg {
            FactoryMsg::AddDefaultMessage => {
                guard.push_back(self.created_widgets.clone());
            }
            FactoryMsg::AddMessage(model) => {
                guard.push_back(model);
            }
            FactoryMsg::RemoveMessage => {
                guard.pop_back();
            }
            FactoryMsg::Remove(index) => {
                guard.remove(index.current_index());
            }
        }
    }
}
