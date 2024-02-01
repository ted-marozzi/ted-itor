use gpui::*;

#[derive(Debug)]
pub struct Theme {
    // TODO: Make hover colors, maybe with Hsla
    pub background_color: Rgba,
    pub danger_color: Rgba,
    pub panel_color: Rgba,
    pub border_color: Rgba,
    pub text_color: Rgba,
}

impl Theme {
    pub fn init(cx: &mut AppContext) {
        cx.set_global(Theme::default())
    }

    fn default() -> Self {
        Self {
            background_color: rgb(0xf4fbf5),
            danger_color: rgb(0xff6961),
            panel_color: rgb(0xdaeedb),
            border_color: rgb(0x8ea88e),
            text_color: black().into(),
        }
    }
}
