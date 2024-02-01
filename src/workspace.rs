use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;

use crate::{
    editor::Editor,
    ui::{Background, Divider, Layout, TextInput},
};

/* cspell:disable-next-line */
const INITIAL_EDITOR_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn build_workspace_view(cx: &mut WindowContext<'_>) -> View<Workspace> {
    cx.new_view(|cx| Workspace {
        editors: SmallVec::from_iter([
            Editor {
                text_input: TextInput::new(cx, INITIAL_EDITOR_TEXT.to_owned()),
            },
            Editor {
                text_input: TextInput::new(cx, INITIAL_EDITOR_TEXT.to_owned()),
            },
        ]),
    })
}

pub struct Workspace {
    editors: SmallVec<[Editor; 2]>,
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Background::new().child(
            Layout::new().body(div().when(self.editors.len() > 0, |this| {
                let mut children: Vec<AnyElement> = vec![];

                for i in 0..self.editors.len() {
                    let editor = self.editors[i].clone();
                    children.push(editor.into_any_element());
                    if i != self.editors.len() - 1 {
                        children.push(Divider::horizontal().into_any_element())
                    }
                }

                this.children(children)
            })),
        )
    }
}
