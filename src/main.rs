mod views;

use views::login::widget::LoginView as Login;
use views::messages::widget::MessagesView as Message;

use relm4::adw;
use gtk::prelude::*;
use relm4::component::{AsyncComponentController, AsyncConnector};

use relm4::{
    component::{AsyncComponent, AsyncComponentParts, AsyncComponentSender},
    gtk, RelmApp,
};

struct App {
    login: AsyncConnector<Login>,
    messages: AsyncConnector<Message>,
}

#[relm4::component(async)]
impl AsyncComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::Window {
            set_default_size: (800, 300),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_vexpand: true,

                #[name = "content_header"]
                adw::HeaderBar {
                    #[name = "back_button"]
                    pack_start = &gtk::Button {
                        set_icon_name: "go-previous-symbolic",
                        // connect_clicked[leaflet] => move |_| {
                        //     leaflet.navigate(adw::NavigationDirection::Back);
                        // }
                    },

                    #[name = "switcher_title"]
                    #[wrap(Some)]
                    set_title_widget = &adw::ViewSwitcherTitle {
                        set_title: "Herald",
                        // set_view_switcher_enabled: true,
                    }
                },

                #[name = "stack"]
                adw::ViewStack {},
                
                #[name = "switcher_bar"]
                adw::ViewSwitcherBar {
                    // set_reveal: true,
                },
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>
    ) -> AsyncComponentParts<Self> {
        let login = views::login::widget::LoginView::builder().launch(());
        let messages = views::messages::widget::MessagesView::builder().launch(());

        let model = App {
            login,
            messages,
        };
        let widgets = view_output!();

        // widgets
        //     .leaflet
        //     .bind_property("folded", &widgets.sidebar_header, "show-end-title-buttons")
        //     .flags(glib::BindingFlags::SYNC_CREATE)
        //     .build();

        // widgets
        //     .leaflet
        //     .bind_property(
        //         "folded",
        //         &widgets.content_header,
        //         "show-start-title-buttons",
        //     )
        //     .flags(glib::BindingFlags::SYNC_CREATE)
        //     .build();

        // widgets
        //     .leaflet
        //     .bind_property("folded", &widgets.back_button, "visible")
        //     .flags(glib::BindingFlags::SYNC_CREATE)
        //     .build();

        widgets
            .stack
            .add_titled_with_icon(model.login.widget(), Some("login"), &"Login", &"go-home-symbolic");

        widgets
            .stack
            .add_titled_with_icon(model.messages.widget(), Some("messages"), &"Messages", &"go-home-symbolic");

        widgets
            .switcher_bar
            .set_stack(Some(&widgets.stack));

        widgets
            .switcher_title
            .set_stack(Some(&widgets.stack));

        widgets
            .switcher_title
            .bind_property("title-visible", &widgets.switcher_bar, "reveal")
            .flags(gtk::glib::BindingFlags::SYNC_CREATE)
            .build();

        // widgets
        //     .stack
        //     .set_visible_child_name("messages");

        // widgets
        //     .stack
        //     .remove(model.messages.widget());

        AsyncComponentParts { model, widgets }
    }
}

fn main() {
    env_logger::init();

    let app = RelmApp::new("io.github.zlendy.herald");
    app.run_async::<App>(());
}