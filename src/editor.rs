use gpui::*;

use crate::ui::TextInput;

#[derive(IntoElement, Clone)]
pub struct Editor {
    pub text_input: TextInput,
}

impl Editor {
    pub fn clear(self, cx: &mut WindowContext) {
        self.text_input.clear(cx);
    }
}

impl RenderOnce for Editor {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.text_input
    }
}
