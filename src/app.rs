use crate::{editor::build_editor_view, theme::Theme, window::get_window_options};
use gpui::AppContext;

pub fn run_app(app: gpui::App) {
    app.run(|cx: &mut AppContext| {
        Theme::init(cx);

        let _ = cx.observe_keystrokes(|_ev, _cx| println!("App Context keystroke"));

        cx.open_window(get_window_options(), build_editor_view);
    });
}
