use gpui::*;

pub fn build_editor_view(cx: &mut WindowContext<'_>) -> gpui::View<Editor> {
    let editor_view = cx.new_view(|_cx| Editor {});

    editor_view
}

pub struct Editor;

impl Render for Editor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let view = cx.new_view(|_cx| Counter { count: 1 });

        CounterButton { child: view }
    }
}

#[derive(IntoElement)]
struct CounterButton {
    child: View<Counter>,
}

impl RenderOnce for CounterButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let child_clone = self.child.clone();
        Button {
            child: self.child.into(),
            on_click: Box::new(move |_ev, cx| {
                child_clone.update(cx, |counter, cx: &mut ViewContext<'_, Counter>| {
                    counter.count += 1;
                    println!("The count is {}", counter.count); // Always prints 2, no matter how many presses???
                    cx.notify();
                });
            }),
        }
    }
}

#[derive(IntoElement)]
pub struct Button {
    child: AnyView,
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

impl Render for Counter {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let current_count = self.count.to_string();
        div().child(current_count)
    }
}
