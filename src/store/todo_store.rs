use leptos::prelude::*;
use uuid::Uuid;

// TODO Data struture
pub struct Todo {
    id: Uuid,
    title: RwSignal<String>,
    done: RwSignal<bool>,
}

// TODOSTORE data struture
pub struct TodoStore {
    todos: RwSignal<Vec<Todo>>,
}

// IMPLEMENTATIONS Todo data struct
impl Todo {
    pub(crate) fn new(title: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: RwSignal::new(title.to_string()),
            done: RwSignal::new(false),
        }
    }

    pub fn get_title(&self) -> String {
        self.title.get()
    }

    pub fn bind_done(&self) -> RwSignal<bool> {
        self.done.clone()
    }

    pub fn toggle_done(&mut self) {
        self.done.update(move |done| {
            *done = !*done;
        });
    }
}

// IMPLEMENTATIONS TodoStore data struct
impl TodoStore {
    pub fn new() -> Self {
        Self {
            todos: RwSignal::new(Vec::new()),
        }
    }

    pub fn read(&self) -> Vec<Todo> {

    }
}
