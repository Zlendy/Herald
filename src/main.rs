mod widgets;

use widgets::about_dialog::AboutDialog;
use widgets::login::widget::LoginWidget;
use widgets::message_container::widget::MessageContainerWidget;

use relm4::actions::{RelmActionGroup, RelmAction, ActionGroupName};
use relm4::ComponentController;

use relm4::{adw, Controller, ComponentBuilder};
use gtk::prelude::*;
use relm4::component::{AsyncComponentController, AsyncConnector};

use relm4::{
    component::{AsyncComponent, AsyncComponentParts, AsyncComponentSender},
    gtk, RelmApp,
};

struct App {
    login: AsyncConnector<LoginWidget>,
    messages: AsyncConnector<MessageContainerWidget>,
    about_dialog: Option<Controller<AboutDialog>>
}

#[relm4::component(async)]
impl AsyncComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        #[name = "main_window"]
        adw::Window {
            set_default_size: (800, 300),
            set_icon_name: Some("io.github.zlendy.herald"),

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
                    },

                    #[name = "menu"]
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        
                        #[wrap(Some)]
                        set_popover = &gtk::PopoverMenu::from_model(Some(&main_menu)) {
                            // add_child: (&popover_child, "my_widget"),
                        }
                    },
                },

                #[name = "stack"]
                adw::ViewStack {},
                
                #[name = "switcher_bar"]
                adw::ViewSwitcherBar {
                    // set_reveal: true,
                },

                // #[name = "popover_child"]
                // gtk::Spinner {
                //     set_spinning: true,
                // }
            }
        }
    }

    menu! {
        main_menu: {
            custom: "my_widget",
            "About Herald" => AboutAction,
            // "Example2" => AboutAction,
            // "Example toggle" => ExampleU8Action(1_u8),
            // section! {
            //     "Section example" => AboutAction,
            //     "Example toggle" => ExampleU8Action(1_u8),
            // },
            // section! {
            //     "Example" => AboutAction,
            //     "Example2" => AboutAction,
            //     "Example Value" => ExampleU8Action(1_u8),
            // },
            // "submenu1" {
            //     "Example" => AboutAction,
            //     "Example2" => AboutAction,
            //     "Example toggle" => ExampleU8Action(1_u8),
            //     "submenu2" {
            //         "Example" => AboutAction,
            //         "Example2" => AboutAction,
            //         "Example toggle" => ExampleU8Action(1_u8),
            //         "submenu3" {
            //             "Example" => AboutAction,
            //             "Example2" => AboutAction,
            //             "Example toggle" => ExampleU8Action(1_u8),
            //         }
            //     }
            // }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>
    ) -> AsyncComponentParts<Self> {
        let login = LoginWidget::builder().launch(());
        let messages = MessageContainerWidget::builder().launch(());

        let mut model = App {
            login,
            messages,
            about_dialog: None,
        };

        let widgets: AppWidgets = view_output!();

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

        let actions = RelmActionGroup::<WindowActionGroup>::new();

        let about_dialog = ComponentBuilder::default()
			.launch(widgets.main_window.upcast_ref::<gtk::Window>().clone())
			.detach();
        model.about_dialog = Some(about_dialog);

        let about_action = {
            let sender = model.about_dialog.as_ref().unwrap().sender().clone();
			RelmAction::<AboutAction>::new_stateless(move |_| {
				sender.send(()).unwrap_or_default();
			})
        };
        
        actions.add_action(&about_action);
        // widgets.main_window.set_accelerators_for_action::<ExampleAction>(&["<Control>q"]);
        // widgets.main_window.set_action_group(Some(&actions.into_action_group()));

        widgets
            .stack
            .add_titled_with_icon(model.login.widget(), Some("login"), "Login", "padlock2-symbolic");

        widgets
            .stack
            .add_titled_with_icon(model.messages.widget(), Some("messages"), "Messages", "chat-bubble-text-symbolic");

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

        widgets.main_window.insert_action_group(
			WindowActionGroup::NAME,
			Some(&actions.into_action_group()),
		);

        AsyncComponentParts { model, widgets }
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateful_action!(ExampleU8Action, WindowActionGroup, "example2", u8, u8);

fn main() {
    env_logger::init();

    let app = RelmApp::new("io.github.zlendy.herald");
    app.run_async::<App>(());
}
