mod models;
mod services;
mod widgets;

use dotenv::dotenv;
use relm4::RelmApp;

use widgets::root::App;

fn main() {
    env_logger::init();
    dotenv().ok();

    let app = RelmApp::new("io.github.zlendy.herald");
    app.run::<App>(());
}
