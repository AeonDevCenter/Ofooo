use leptos::logging::log;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            done: false,
        }
    }

    pub fn toggle_done(&mut self) {
        self.done = !self.done;
        log!("{} : {}", self.title, self.done);
    }
}
