mod widgets;

use widgets::app::App;
use relm4::RelmApp;

use dotenv::dotenv;

fn main() {
    env_logger::init();
    dotenv().ok();

    let app = RelmApp::new("io.github.zlendy.herald");
    app.run_async::<App>(());
}
