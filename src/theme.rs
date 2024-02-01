use gpui::*;

#[derive(Debug)]
pub struct Theme {
    pub background_color: Hsla,
    pub primary_color: Hsla,
    pub danger_color: Hsla,
    pub panel_color: Hsla,
    pub border_color: Hsla,
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
            panel_color: hsla(123. / 360., 0.37, 0.89, 1.),
            border_color: hsla(120. / 360., 0.13, 0.61, 1.),
            text_color: black(),
        }
    }
}
