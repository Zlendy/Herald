use crate::models::gotify::application::ApplicationModel;
use crate::services::gotify::GotifyService;

use super::factory::{AppFactory, FactorySignal};
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

pub struct AppsContainerWidget {
    #[allow(dead_code)]
    current_section: u32,
    factory: AsyncConnector<AppFactory>,
}

#[derive(Debug)]
pub enum AppsContainerSignals {
    SelectRow(ListBoxRow),
    LoadData,
}

#[relm4::component(pub async)]
impl AsyncComponent for AppsContainerWidget {
    type Init = ();
    type Input = AppsContainerSignals;
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
        sender: AsyncComponentSender<AppsContainerWidget>,
    ) -> AsyncComponentParts<Self> {
        let factory = AppFactory::builder().launch(ApplicationModel::default());

        let model = AppsContainerWidget {
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
            show_sender.emit(AppsContainerSignals::LoadData);
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
