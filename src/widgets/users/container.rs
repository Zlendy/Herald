use crate::models::gotify::user::UserModel;

use super::factory::{FactorySignal, UserFactory};
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

pub struct UsersContainerWidget {
    #[allow(dead_code)]
    current_section: u32,
    factory: AsyncConnector<UserFactory>,
}

#[derive(Debug)]
pub enum UsersContainerSignals {
    SelectRow(ListBoxRow),
    LoadData,
}

#[relm4::component(pub async)]
impl AsyncComponent for UsersContainerWidget {
    type Init = ();
    type Input = UsersContainerSignals;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[name = "clamp"]
        adw::Clamp {

            #[name = "content"]
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,

                // Child
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<UsersContainerWidget>,
    ) -> AsyncComponentParts<Self> {
        let factory = UserFactory::builder().launch(UserModel::default());

        let model = UsersContainerWidget {
            current_section: 1,
            factory,
        };

        root.connect_show(|_test| {
            log::info!("SHOWN"); // TODO: Refresh data from server
        });

        let widgets = view_output!();

        widgets.content.factory_append(model.factory.widget(), &());

        let show_sender = sender.input_sender().to_owned();
        root.connect_show(move |_test| {
            log::info!("SHOWN"); // TODO: Refresh data from server
            show_sender.emit(UsersContainerSignals::LoadData);
        });

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
            Self::Input::LoadData => self.factory.emit(FactorySignal::SetData),
        }
    }
}
