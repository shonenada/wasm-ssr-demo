use virtual_dom_rs::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::App;

#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;
    pub static global_js: GlobalJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[wasm_bindgen]
pub struct Client {
    app: App,
    dom_updater: DomUpdater,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let app = App::new();

        app.store.borrow_mut().subscribe(Box::new(|| {
            global_js.update();
        }));

        let root_node = document().get_element_by_id("app").unwrap();
        let dom_updater = DomUpdater::new_replace_mount(app.render(), root_node);

        Self { app, dom_updater }
    }

    pub fn render(&mut self) {
        let vdom = self.app.render();
        self.dom_updater.update(vdom);
    }
}

fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

fn document() -> web_sys::Document {
    window().document().unwrap()
}

fn history() -> web_sys::History {
    window().history().unwrap()
}

fn location() -> web_sys::Location {
    document().location().unwrap()
}

fn hostname() -> String {
    location().hostname().unwrap()
}

fn port() -> String {
    location().port().unwrap()
}
