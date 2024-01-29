use gpui::{GlobalPixels, WindowOptions};

pub fn get_window_options() -> WindowOptions {
    return WindowOptions {
        bounds: gpui::WindowBounds::Fixed(gpui::Bounds {
            origin: gpui::Point {
                x: GlobalPixels::from(0.),
                y: GlobalPixels::from(0.),
            },
            size: gpui::Size {
                width: GlobalPixels::from(800.),
                height: GlobalPixels::from(400.),
            },
        }),
        titlebar: Some(gpui::TitlebarOptions {
            title: Some("Teditor".into()),
            appears_transparent: true,
            traffic_light_position: None,
        }),
        center: true,
        focus: true,
        show: true,
        kind: gpui::WindowKind::Normal,
        is_movable: true,
        display_id: None,
    };
}
