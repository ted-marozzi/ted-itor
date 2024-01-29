use std::{cell::RefCell, rc::Rc};

use crate::{editor::build_editor_view, theme::Theme, window::get_window_options};
use gpui::Subscription;

pub fn run_app(app: gpui::App) {
    let subscription: Rc<RefCell<Option<Subscription>>> = Rc::new(RefCell::new(None));
    let clone = subscription.clone();

    app.run(move |cx| {
        Theme::init(cx);

        let mut reference = clone.borrow_mut();

        *reference = Some(cx.observe_keystrokes(|_ev, _cx| println!("App Context keystroke")));

        cx.open_window(get_window_options(), build_editor_view);
    });

    if subscription.borrow_mut().is_some() {
        println!("Subscription alive")
    }

    println!("Subscription about to be dropped")
}
