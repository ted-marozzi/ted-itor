use crate::{editor::build_editor_view, theme::Theme, window::get_window_options};

pub fn run_app(app: gpui::App) {
    app.run(move |cx| {
        Theme::init(cx);

        cx.open_window(get_window_options(), build_editor_view);
    });
}
