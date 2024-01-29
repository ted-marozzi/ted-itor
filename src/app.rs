use crate::{editor::get_editor_window, theme::Theme, window::get_window_options};
use gpui::AppContext;

pub fn run_app(app: gpui::App) {
    app.run(|cx: &mut AppContext| {
        Theme::init(cx);

        cx.open_window(get_window_options(), get_editor_window);
    });
}
