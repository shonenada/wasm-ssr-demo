use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Photo {
    image_url: String,
}

#[wasm_bindgen]
impl Photo {
    pub fn from_str(url: &str) -> Self {
        Self {
            image_url: url.to_string(),
        }
    }

    pub fn image_url(&self) -> String {
        self.image_url.clone()
    }
}

#[derive(Clone)]
pub struct State {
    cur: usize,
    photos: Vec<Photo>,
}

impl State {
    pub fn new() -> Self {
        let photos = Vec::new();
        Self { cur: 0, photos }
    }

    pub fn add_image(&mut self, image_url: &str) {
        self.photos.push(Photo::from_str(image_url));
    }

    pub fn next(&mut self) -> String {
        if let Some(p) = self.photos.get(self.cur) {
            self.cur = (self.cur + 1) % self.photos.len();
            p.image_url()
        } else {
            "".to_string()
        }
    }

    pub fn cur_image(&self) -> String {
        if let Some(p) = self.photos.get(self.cur) {
            p.image_url()
        } else {
            "".to_string()
        }
    }
}
