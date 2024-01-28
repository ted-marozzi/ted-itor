use gpui::{
    div, rgb, App, AppContext, Context, CursorStyle, InteractiveElement, IntoElement, Model,
    ParentElement, Render, Styled, ViewContext, VisualContext, WindowOptions,
};

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let counter = cx.new_model(|_cx| Counter { count: 0 });
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| CounterView {
                counter_model: counter,
            })
        });
    });
}

struct Counter {
    count: usize,
}

struct CounterView {
    counter_model: Model<Counter>,
}

impl Render for CounterView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let counter = self.counter_model.clone();
        div()
            .flex()
            .justify_center()
            .items_center()
            .bg(rgb(0xeffdee))
            .size_full()
            .text_xl()
            .text_color(rgb(0x000000))
            .child(
                div()
                    .border()
                    .border_color(gpui::black())
                    .bg(rgb(0x90cf8e))
                    .p_2()
                    .cursor(CursorStyle::PointingHand)
                    .on_mouse_down(gpui::MouseButton::Left, move |_event, cx| {
                        counter.update(cx, |counter, _cx| {
                            counter.count += 1;
                        });
                    })
                    .child(format!("Count, {:?}!", &self.counter_model.read(cx).count)),
            )
    }
}
