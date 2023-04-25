use std::time::Duration;

use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::{
    AsyncFactoryComponent, AsyncFactorySender, AsyncFactoryVecDeque, DynamicIndex,
};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{gtk, view, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug, Clone)]
pub struct MessageComponent {
    title: String,
    content: String,
}

impl Default for MessageComponent {
    fn default() -> Self {
        Self {
            title: "Message title".to_string(),
            content: "Message content".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum MessageComponentInput {
    Increment,
    Decrement,
}

#[derive(Debug)]
pub enum MessageComponentOutput {
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
    Remove(DynamicIndex),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for MessageComponent {
    type Init = MessageComponent;
    type Input = MessageComponentInput;
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
                            set_text: &self.title,
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
                        set_text: &self.content,
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
            MessageComponentOutput::SendFront(index) => FactoryMsg::SendFront(index),
            MessageComponentOutput::MoveUp(index) => FactoryMsg::MoveUp(index),
            MessageComponentOutput::MoveDown(index) => FactoryMsg::MoveDown(index),
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

    async fn update(&mut self, msg: Self::Input, _sender: AsyncFactorySender<Self>) {
        tokio::time::sleep(Duration::from_secs(1)).await;
        match msg {
            MessageComponentInput::Increment => {
                // self.value = self.value.wrapping_add(1);
            }
            MessageComponentInput::Decrement => {
                // self.value = self.value.wrapping_sub(1);
            }
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        println!("Counter with value {} was destroyed", self.content);
    }
}

pub struct MessageFactory {
    created_widgets: MessageComponent,
    messages: AsyncFactoryVecDeque<MessageComponent>,
}

#[derive(Debug)]
pub enum FactoryMsg {
    AddMessage,
    RemoveMessage,
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
    Remove(DynamicIndex),
}

#[relm4::component(pub)]
impl SimpleComponent for MessageFactory {
    type Init = MessageComponent;
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
                    connect_clicked => FactoryMsg::AddMessage,
                },

                gtk::Button {
                    set_label: "Remove test notification",
                    connect_clicked => FactoryMsg::RemoveMessage,
                },

                gtk::ScrolledWindow {
                    set_vexpand: true,

                    #[local_ref]
                    counter_box -> gtk::Box {
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

        let counter_box = model.messages.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        let mut counters_guard = self.messages.guard();
        match msg {
            FactoryMsg::AddMessage => {
                counters_guard.push_back(self.created_widgets.clone());
                // self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            FactoryMsg::RemoveMessage => {
                counters_guard.pop_back();
            }
            FactoryMsg::SendFront(index) => {
                counters_guard.move_front(index.current_index());
            }
            FactoryMsg::MoveDown(index) => {
                let index = index.current_index();
                let new_index = index + 1;
                // Already at the end?
                if new_index < counters_guard.len() {
                    counters_guard.move_to(index, new_index);
                }
            }
            FactoryMsg::MoveUp(index) => {
                let index = index.current_index();
                // Already at the start?
                if index != 0 {
                    counters_guard.move_to(index, index - 1);
                }
            }
            FactoryMsg::Remove(index) => {
                counters_guard.remove(index.current_index());
            }
        }
    }
}
