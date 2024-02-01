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
    title_bar: AnyElement,
    body: AnyElement,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            title_bar: TitleBar::new().into_any_element(),
            body: div().into_any_element(),
        }
    }

    pub fn title_bar(mut self, title_bar: impl IntoElement) -> Self {
        self.title_bar = title_bar.into_any_element();
        self
    }

    pub fn body(mut self, body: impl IntoElement) -> Self {
        self.body = body.into_any_element();
        self
    }
}

impl RenderOnce for Layout {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .child(self.title_bar)
            .child(div().p_6().child(self.body))
    }
}

#[derive(IntoElement)]
pub struct TitleBar {
    children: SmallVec<[AnyElement; 2]>,
}

impl TitleBar {
    pub fn new() -> Self {
        TitleBar {
            children: SmallVec::new(),
        }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .h_7()
            .flex()
            .items_center()
            .bg(theme.panel_color)
            .border_color(theme.border_color)
            .border_b()
            .when(self.children.len() > 0, |this| this.children(self.children))
    }
}

impl ParentElement for TitleBar {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

pub enum ButtonVariant {
    Primary,
    Danger,
}

// TODO: Make a clear input button
// TODO: Create button variants
#[derive(IntoElement)]
pub struct Button {
    base: Div,
    child: AnyElement,
    on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext)>,
    variant: ButtonVariant,
}

impl Button {
    #[allow(dead_code)]
    pub fn new(
        child: impl IntoElement,
        on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext)>,
    ) -> Self {
        Self {
            base: div(),
            child: child.into_any_element(),
            on_click,
            variant: ButtonVariant::Primary,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(&self, theme: &Theme) -> Hsla {
        match self.variant {
            ButtonVariant::Primary => theme.primary_color,
            ButtonVariant::Danger => theme.danger_color,
        }
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let color = self.color(theme);

        self.base
            .p_2()
            .rounded_md()
            .hover(|style| style.bg(red()))
            .flex()
            .justify_center()
            .items_center()
            .bg(color)
            .on_mouse_down(MouseButton::Left, self.on_click)
            .child(self.child)
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
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

    pub fn clear(self, cx: &mut WindowContext) {
        self.text_display_view.update(cx, |text_display, cx| {
            text_display.text = String::from("");
            cx.notify();
        })
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
