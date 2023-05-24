use super::about_dialog::AboutDialog;
use super::apps::container::AppsContainerWidget;
use super::clients::container::ClientContainerWidget;
use super::login::LoginWidget;
use super::messages::container::MessageContainerWidget;
use super::users::container::UsersContainerWidget;

use adw::{glib, Toast, Window};
use relm4::actions::{ActionGroupName, RelmAction, RelmActionGroup};
use relm4::{Component, ComponentController, ComponentParts, ComponentSender};

use gtk::prelude::*;
use relm4::component::{AsyncComponentController, AsyncConnector, AsyncController};
use relm4::{adw, ComponentBuilder, Controller};

use relm4::{component::AsyncComponent, gtk};

#[derive(Debug)]
pub enum GlobalActions {
    LoggedIn,
    LoggedOut,
    ShowToast(Toast),
}

pub struct App {
    login: AsyncController<LoginWidget>,
    message_container: AsyncConnector<MessageContainerWidget>,
    users_container: AsyncConnector<UsersContainerWidget>,
    client_container: AsyncConnector<ClientContainerWidget>,
    apps_container: AsyncConnector<AppsContainerWidget>,
    about_dialog: Option<Controller<AboutDialog>>,
}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = GlobalActions;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[name = "main_window"]
        adw::Window {
            set_default_size: (800, 600),
            set_icon_name: Some("io.github.zlendy.herald"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_vexpand: true,

                #[name = "content_header"]
                adw::HeaderBar {
                    #[name = "sidebar_button"]
                    pack_start = &gtk::ToggleButton {
                        set_icon_name: "sidebar-show-symbolic",
                        set_active: true,
                        // set_visible: false,

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
                adw::ViewStack {
                    // add_titled_with_icon: (model.login.widget(), Some("login"), "Login", "padlock2-symbolic"),
                    // add_titled_with_icon: (model.message_container.widget(), Some("messages"), "Messages", "chat-bubble-text-symbolic"),
                },

                #[name = "toast_overlay"]
                adw::ToastOverlay {

                },

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

    fn init(
        _init: Self::Init,
        root: &Window,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Main

        let login = LoginWidget::builder().launch(());
        let message_container = MessageContainerWidget::builder().launch(());
        let users_container = UsersContainerWidget::builder().launch(());
        let client_container = ClientContainerWidget::builder().launch(());
        let apps_container = AppsContainerWidget::builder().launch(());

        let input_sender = sender.input_sender().to_owned();
        let login_controller = login.connect_receiver(move |_, message| {
            input_sender.emit(message);
        });

        let mut model = App {
            login: login_controller,
            message_container,
            users_container,
            client_container,
            apps_container,
            about_dialog: None,
        };

        let widgets: AppWidgets = view_output!();

        // Actions

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

        // Stack

        widgets.switcher_bar.set_stack(Some(&widgets.stack));
        widgets.switcher_title.set_stack(Some(&widgets.stack));

        // Binding

        widgets
            .switcher_title
            .bind_property("title-visible", &widgets.switcher_bar, "reveal")
            .flags(gtk::glib::BindingFlags::SYNC_CREATE)
            .build();

        widgets
            .sidebar_button
            .bind_property("active", model.message_container.widget(), "reveal-flap")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .flags(gtk::glib::BindingFlags::BIDIRECTIONAL)
            .build();

        // Events

        sender.input(Self::Input::LoggedOut); // TODO: Log back in automatically (Token stored in SQLite)

        // Etc

        widgets.stack.connect_visible_child_notify(|stack| {
            let visible = stack.visible_child().unwrap();
            let name = stack.visible_child_name().unwrap();
            log::info!("Visible: {} ({})", name, visible);
            visible.emit_by_name("show", &[])
        });

        widgets
            .main_window
            .insert_action_group(WindowActionGroup::NAME, Some(&actions.into_action_group()));

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            GlobalActions::LoggedIn => {
                log::info!("Logged in.");

                widgets.stack.remove(self.login.widget());

                widgets.stack.add_titled_with_icon(
                    self.message_container.widget(),
                    Some("messages"),
                    "Messages",
                    "chat-bubble-text-symbolic",
                );
                widgets.stack.add_titled_with_icon(
                    self.users_container.widget(),
                    Some("users"),
                    "Users",
                    "org.gnome.Settings-users-symbolic",
                );
                widgets.stack.add_titled_with_icon(
                    self.apps_container.widget(),
                    Some("apps"),
                    "Apps",
                    "user-available-symbolic",
                );
                widgets.stack.add_titled_with_icon(
                    self.client_container.widget(),
                    Some("clients"),
                    "Clients",
                    "phonelink2-symbolic",
                );
            }
            GlobalActions::LoggedOut => {
                log::info!("Logged out.");

                widgets.stack.add_titled_with_icon(
                    self.login.widget(),
                    Some("login"),
                    "Login",
                    "padlock2-symbolic",
                );

                widgets.stack.remove(self.message_container.widget());
                widgets.stack.remove(self.users_container.widget());
                widgets.stack.remove(self.apps_container.widget());
                widgets.stack.remove(self.client_container.widget());
            }
            GlobalActions::ShowToast(toast) => {
                widgets.toast_overlay.add_toast(&toast);
            }
        }
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateful_action!(ExampleU8Action, WindowActionGroup, "example2", u8, u8);
