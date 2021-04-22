pub struct ItemState {
    pub text: String,
    pub done: bool,
}

impl ItemState {
    pub fn new(text: String) -> ItemState {
        ItemState { text, done: false }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn done(&self) -> bool {
        self.done
    }
}
