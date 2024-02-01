use gpui::*;

use smallvec::SmallVec;

use crate::theme::Theme;
use gpui::prelude::FluentBuilder;

#[derive(IntoElement)]
pub struct Background {
    children: SmallVec<[AnyElement; 2]>,
}

impl Background {
    pub fn new() -> Self {
        Background {
            children: SmallVec::new(),
        }
    }
}

impl RenderOnce for Background {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .bg(theme.background_color)
            .text_color(theme.text_color)
            .size_full()
            .when(self.children.len() > 0, |this| this.children(self.children))
    }
}

impl ParentElement for Background {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

#[derive(IntoElement)]
pub struct Layout {
    toolbar: Toolbar,
    body: AnyElement,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            toolbar: Toolbar::new(),
            body: div().into_any_element(),
        }
    }

    pub fn body(mut self, body: impl IntoElement) -> Self {
        self.body = body.into_any_element();
        self
    }
}

impl RenderOnce for Layout {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .child(self.toolbar)
            .child(div().p_6().child(self.body))
    }
}

#[derive(IntoElement)]
struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Toolbar
    }
}

impl RenderOnce for Toolbar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .h_7()
            .flex()
            .items_center()
            .bg(theme.panel_color)
            .border_color(theme.border_color)
            .border_b()
    }
}

// TODO: Make a clear input button
// TODO: Create button variants
#[derive(IntoElement)]
pub struct Button {
    child: AnyElement,
    on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext)>,
}

impl Button {
    #[allow(dead_code)]
    pub fn new(
        child: impl IntoElement,
        on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext)>,
    ) -> Self {
        Self {
            child: child.into_any_element(),
            on_click,
        }
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div().flex().justify_center().items_center().child(
            div()
                .p_2()
                .rounded_md()
                .hover(|style| style.bg(red()))
                .flex()
                .bg(theme.danger_color)
                .on_mouse_down(MouseButton::Left, self.on_click)
                .child(self.child),
        )
    }
}

#[derive(IntoElement, Clone)]
pub struct TextInput {
    pub text_display_view: View<TextDisplay>,
    focus_handle: FocusHandle,
}

impl TextInput {
    pub fn new(cx: &mut WindowContext, initial_text: String) -> Self {
        Self {
            text_display_view: cx.new_view(|_cx| TextDisplay { text: initial_text }),
            focus_handle: cx.focus_handle(),
        }
    }
}

impl RenderOnce for TextInput {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let text_display_view = self.text_display_view.clone();
        div()
            .track_focus(&self.focus_handle)
            .on_key_down(move |ev, cx| {
                text_display_view.update(cx, |editor, cx| {
                    let keystroke = &ev.keystroke.key;

                    if let Some(ime_key) = &ev.keystroke.ime_key {
                        editor.text.push_str(ime_key);
                    } else {
                        match keystroke.as_str() {
                            "backspace" => {
                                editor.text.pop();
                            }
                            keystroke_str => {
                                eprintln!("Unhandled keystroke {keystroke_str}")
                            }
                        };
                    }

                    cx.notify();
                });
            })
            .p_4()
            .border_l()
            .border_color(transparent_black())
            .focus(|style| style.border_color(theme.border_color))
            .child(self.text_display_view)
    }
}

#[derive(Clone)]
pub struct TextDisplay {
    // TODO: Use Arc<String>? Other places we can reduce clones?
    pub text: String,
}

impl Render for TextDisplay {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.text.clone()
    }
}

#[allow(dead_code)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(IntoElement)]
pub struct Divider {
    orientation: Orientation,
}

impl Divider {
    pub fn horizontal() -> Self {
        Self {
            orientation: Orientation::Horizontal,
        }
    }

    #[allow(dead_code)]
    pub fn vertical() -> Self {
        Self {
            orientation: Orientation::Vertical,
        }
    }
}
impl RenderOnce for Divider {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let base = div()
            .flex()
            .border()
            .border_width(px(0.5))
            .border_color(theme.border_color);
        match self.orientation {
            Orientation::Horizontal => base.h_0(),
            Orientation::Vertical => div().w_0(),
        }
    }
}
