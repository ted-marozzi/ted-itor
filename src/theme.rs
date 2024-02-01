use gpui::*;

#[derive(Debug)]
pub struct Theme {
    // TODO: Make hover colors, maybe with Hsla
    pub background_color: Hsla,
    pub primary_color: Hsla,
    pub danger_color: Hsla,
    pub panel_color: Rgba,
    pub border_color: Rgba,
    pub text_color: Hsla,
}

impl Theme {
    pub fn init(cx: &mut AppContext) {
        cx.set_global(Theme::new())
    }

    fn new() -> Self {
        Self {
            primary_color: white(),
            background_color: hsla(129. / 360., 0.47, 0.96, 1.),
            danger_color: hsla(3. / 360., 100., 0.69, 1.),
            panel_color: rgb(0xdaeedb),
            border_color: rgb(0x8ea88e),
            text_color: black(),
        }
    }
}
