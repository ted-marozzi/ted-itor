use gpui::*;

use crate::ui::{Background, Layout};

/* cspell:disable-next-line */
const INITIAL_EDITOR_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn build_editor_view(cx: &mut WindowContext<'_>) -> gpui::View<Editor> {
    let editor_view = cx.new_view(|_cx| Editor {
        text: INITIAL_EDITOR_TEXT.to_owned(),
    });

    let keystroke_editor_view = editor_view.clone();

    cx.observe_keystrokes(move |ev, cx| {
        keystroke_editor_view.update(cx, |editor, cx| {
            let keystroke = &ev.keystroke.key;

            if let Some(ime_key) = &ev.keystroke.ime_key {
                editor.text.push_str(ime_key);
            } else {
                match keystroke.as_str() {
                    "backspace" => {
                        editor.text.pop();
                    }
                    keystroke_str => eprintln!("Unhandled keystroke {keystroke_str}"),
                };
            }

            cx.notify();
        });
    })
    .detach();

    editor_view
}

pub struct Editor {
    pub text: String,
}

impl Render for Editor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Background::new().child(Layout::new().body(self.text.clone()))
    }
}
