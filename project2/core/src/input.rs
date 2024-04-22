use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
pub struct UserInput {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
}

#[wasm_bindgen]
pub struct UserInputEvent {
    key: String,
    pub pressed: bool,
}

#[wasm_bindgen]
impl UserInputEvent {
    #[wasm_bindgen(constructor)]
    pub fn new(key: String, pressed: bool) -> Self {
        Self { key, pressed }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }
}

pub trait UserInputEventReciever {
    fn update(&mut self, user_input_event: &UserInputEvent);
}