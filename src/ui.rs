use gpui::*;

use smallvec::SmallVec;

use crate::theme::Theme;
use gpui::prelude::FluentBuilder;

#[derive(IntoElement)]
pub struct Background {
    children: SmallVec<[AnyElement; 2]>,
}

impl Background {
    pub fn default() -> Self {
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
            toolbar: Toolbar,
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
