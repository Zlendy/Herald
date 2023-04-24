use std::time::Duration;

use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::{
    AsyncFactoryComponent, AsyncFactorySender, AsyncFactoryVecDeque, DynamicIndex,
};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{gtk, view, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
struct MessageComponent {
    value: u8,
}

#[derive(Debug)]
enum MessageComponentInput {
    Increment,
    Decrement,
}

#[derive(Debug)]
enum MessageComponentOutput {
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
    Remove(DynamicIndex),
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for MessageComponent {
    type Init = u8;
    type Input = MessageComponentInput;
    type Output = MessageComponentOutput;
    type CommandOutput = ();
    type ParentInput = FactoryMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_halign: gtk::Align::Center,

            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: &self.value.to_string(),
                set_width_chars: 3,
            },

            #[name(add_button)]
            gtk::Button {
                set_label: "+",
                connect_clicked => MessageComponentInput::Increment,
            },

            #[name(remove_button)]
            gtk::Button {
                set_label: "-",
                connect_clicked => MessageComponentInput::Decrement,
            },

            #[name(move_up_button)]
            gtk::Button {
                set_label: "Up",
                connect_clicked[sender, index] => move |_| {
                    sender.output(MessageComponentOutput::MoveUp(index.clone()))
                }
            },

            #[name(move_down_button)]
            gtk::Button {
                set_label: "Down",
                connect_clicked[sender, index] => move |_| {
                    sender.output(MessageComponentOutput::MoveDown(index.clone()))
                }
            },

            #[name(to_front_button)]
            gtk::Button {
                set_label: "To Start",
                connect_clicked[sender, index] => move |_| {
                    sender.output(MessageComponentOutput::SendFront(index.clone()))
                }
            },

            gtk::Button {
                set_label: "Remove",
                connect_clicked[sender, index] => move |_| {
                    sender.output(MessageComponentOutput::Remove(index.clone()))
                }
            }
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
        value: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Self { value }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncFactorySender<Self>) {
        tokio::time::sleep(Duration::from_secs(1)).await;
        match msg {
            MessageComponentInput::Increment => {
                self.value = self.value.wrapping_add(1);
            }
            MessageComponentInput::Decrement => {
                self.value = self.value.wrapping_sub(1);
            }
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        println!("Counter with value {} was destroyed", self.value);
    }
}

pub struct MessageFactory {
    created_widgets: u8,
    messages: AsyncFactoryVecDeque<MessageComponent>,
}

#[derive(Debug)]
pub enum FactoryMsg {
    AddCounter,
    RemoveCounter,
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
    Remove(DynamicIndex),
}

#[relm4::component(pub)]
impl SimpleComponent for MessageFactory {
    type Init = u8;
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
                    set_label: "Add counter",
                    connect_clicked => FactoryMsg::AddCounter,
                },

                gtk::Button {
                    set_label: "Remove counter",
                    connect_clicked => FactoryMsg::RemoveCounter,
                },

                #[local_ref]
                counter_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
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
            FactoryMsg::AddCounter => {
                counters_guard.push_back(self.created_widgets);
                self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            FactoryMsg::RemoveCounter => {
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
