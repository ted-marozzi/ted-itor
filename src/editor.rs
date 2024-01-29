use gpui::{
    div, Context, InteractiveElement, IntoElement, Model, ParentElement, Render, ViewContext,
    VisualContext, WindowContext,
};

use crate::ui::{Background, Layout};

/* cspell:disable-next-line */
const INITIAL_EDITOR_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn get_editor_window(cx: &mut WindowContext<'_>) -> gpui::View<EditorView> {
    let text_model = cx.new_model(|_cx| Text {
        value: INITIAL_EDITOR_TEXT.to_owned(),
    });

    cx.new_view(|_cx| EditorView { text_model })
}

pub struct EditorView {
    pub text_model: Model<Text>,
}

impl Render for EditorView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Background::default().child(cx.new_view(|cx| {
            Layout::new(
                cx,
                Editor {
                    text_model: self.text_model.clone(),
                },
            )
        }))
    }
}

pub struct Editor {
    text_model: Model<Text>,
}

pub struct Text {
    pub value: String,
}

impl Render for Editor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = self.text_model.clone();
        div()
            .on_mouse_down(gpui::MouseButton::Left, move |_ev, cx| {
                text.update(cx, |text, _cx| {
                    text.value = "You clicked me".to_owned();
                });
            })
            .child(self.text_model.read(cx).value.clone())
    }
}
