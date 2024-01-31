use gpui::*;

pub fn build_editor_view(cx: &mut WindowContext<'_>) -> View<Editor> {
    cx.new_view(|_cx| Editor {})
}

pub struct Editor;

impl Render for Editor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.new_view(|cx| CounterButton {
            child: Counter { count: 1 },
        })
    }
}

pub struct CounterButton {
    child: Counter,
}

impl Render for CounterButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let view = cx.view().clone();
        Button {
            child: div()
                .child("Button")
                .child(self.child.count.to_string())
                .into_any_element(),
            on_click: Box::new(move |ev, cx| {
                println!("click");
                view.update(cx, |this, cx| {
                    this.child.count += 1;
                    println!("click");

                    cx.notify();
                });
            }),
        }
    }
}

#[derive(IntoElement)]
pub struct Button {
    child: AnyElement,
    on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext)>,
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            .bg(red())
            .on_mouse_down(MouseButton::Left, self.on_click)
            .child(self.child)
    }
}

pub struct Counter {
    count: usize,
}
