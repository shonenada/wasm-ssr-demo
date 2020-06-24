use std::cell::RefCell;
use std::rc::Rc;

use css_rs_macro::css;
use virtual_dom_rs::prelude::*;
use virtual_dom_rs::VirtualNode;
use web_sys;
use web_sys::MouseEvent;

use crate::state::State;
use crate::store::Store;

pub struct App {
    pub store: Rc<RefCell<Store>>,
}

pub fn print_msg(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

impl App {
    pub fn new() -> Self {
        let state = State::new();
        let store = Rc::new(RefCell::new(Store::new(state)));

        store
            .borrow_mut()
            .add_image("https://via.placeholder.com/300/09f/fff.png?text=hello");
        store
            .borrow_mut()
            .add_image("https://via.placeholder.com/300/09f/fff.png?text=world");

        Self { store }
    }

    pub fn render(&self) -> VirtualNode {
        let store = Rc::clone(&self.store);
        let image = html! {
            <img class=IMG_CSS src={store.borrow().cur_image()} />
        };
        let image_url = html! {
            <pre>{store.borrow().cur_image()}</pre>
        };
        html! {
            <div class=MAIN_CSS>
                Img URL: {image_url}
                <br />
                {image}
                <br />
                <button onclick=move |_e: MouseEvent| {
                    store.borrow_mut().next();
                    store.borrow_mut().update();
                } >Next</button>
            </div>
        }
    }
}

pub static MAIN_CSS: &'static str = css! {r#"
:host {
    text-align: center;
}
"#};

pub static IMG_CSS: &'static str = css! {r#"
:host {
    border: 1px solid #ff0000;
}
"#};

pub static _GLOBAL_CSS: &'static str = css! {r#"
body {
    margin: 0;
    padding: 0;
}
"#};
