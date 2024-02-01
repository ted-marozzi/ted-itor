use crate::{theme::Theme, window::get_window_options, workspace::build_workspace_view};

pub fn run_app(app: gpui::App) {
    app.run(move |cx| {
        Theme::init(cx);

        cx.open_window(get_window_options(), build_workspace_view);
    });
}
