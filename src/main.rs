mod widgets;
mod services;
mod models;

use std::sync::{Mutex, Arc};

use relm4::RelmApp;
use dotenv::dotenv;

use widgets::app::App;

fn main() {
    env_logger::init();
    dotenv().ok();

    let app = RelmApp::new("io.github.zlendy.herald");
    app.run_async::<App>(());
}
