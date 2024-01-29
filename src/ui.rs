use gpui::{
    div, AnyElement, AnyView, IntoElement, ParentElement, Render, RenderOnce, Styled, View,
    ViewContext, VisualContext, WindowContext,
};
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

pub struct Layout {
    toolbar: View<Toolbar>,
    body: AnyView,
}

impl Layout {
    pub fn new<W: Render + 'static>(cx: &mut ViewContext<W>, body: impl Render) -> Self {
        Self {
            toolbar: cx.new_view(|_| Toolbar),
            body: cx.new_view(|_| body).into(),
        }
    }
}

impl Render for Layout {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        div().child(self.toolbar.clone()).child(self.body.clone())
    }
}

struct Toolbar;

impl Render for Toolbar {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
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
