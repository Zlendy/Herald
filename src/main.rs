mod widgets;
mod services;
mod models;

use relm4::RelmApp;
use dotenv::dotenv;

use widgets::app::App;
use services::gotify::GotifyService;

fn main() {
    env_logger::init();
    dotenv().ok();

    let mut gotify_service = GotifyService::new();

    let app = RelmApp::new("io.github.zlendy.herald");
    app.run_async::<App>(gotify_service);
}
