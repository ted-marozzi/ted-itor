use gpui::*;

use crate::ui::TextInput;

#[derive(IntoElement, Clone)]
pub struct Editor {
    pub text_input: TextInput,
}

impl RenderOnce for Editor {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.text_input
    }
}
