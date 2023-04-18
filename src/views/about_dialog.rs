// Adapted from https://github.com/done-devs/done/blob/84e636e94a9885502d56adcd22f978f118a940dc/src/widgets/about_dialog.rs

use gtk::prelude::GtkWindowExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

// use crate::{
// 	application::info::{APP_ID, VERSION},
// 	fl,
// };

pub struct AboutDialog {}

pub struct Widgets {
	main_window: gtk::Window,
}

impl SimpleComponent for AboutDialog {
	type Input = ();
	type Output = ();
	type Init = gtk::Window;
	type Root = ();
	type Widgets = Widgets;

	fn init_root() -> Self::Root {}

	fn init(
		main_window: Self::Init,
		_root: &Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {};

		let widgets = Widgets { main_window };

		ComponentParts { model, widgets }
	}

	fn update_view(
		&self,
		widgets: &mut Self::Widgets,
		_sender: ComponentSender<Self>,
	) {
        const APP_ID: &str = "io.github.zlendy.herald";
        const VERSION: &str = "0.1.0";

		let dialog = adw::AboutWindow::builder()
			.icon_name(APP_ID)
			.application_icon(APP_ID)
			.application_name("Herald")
			.developer_name("Zlendy")
			.website("Website")
			.copyright("© 2022 Zlendy")
			.license_type(gtk::License::Gpl30)
			// .website("https://done.edfloreshz.dev/")
			// .issue_url("https://github.com/done-devs/done/issues")
			.version(VERSION)
			// .translator_credits(fl!("translator-credits").replace("\\n", "\n"))
			.modal(true)
			.transient_for(&widgets.main_window)
			.developers(vec![
				"Zlendy <argutierrezg11@gmail.com>".to_string(),
			])
			// .artists(vec![
			// 	"Eduardo Flores <edfloreshz@gmail.com>",
			// 	"David Lapshin <ddaudix@gmail.com>"
			// ])
			// .documenters(vec!["Eduardo Flores <edfloreshz@gmail.com>"])
			.comments("A Gotify client app for GNOME.")
			.build();
		dialog.present();
	}
}
