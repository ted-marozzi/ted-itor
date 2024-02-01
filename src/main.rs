use app::run_app;
use gpui::App;

mod app;
mod editor;
mod theme;
mod ui;
mod window;
mod workspace;

fn main() {
    run_app(App::new())
}
