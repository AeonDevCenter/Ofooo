use leptos::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: RwSignal<String>,
    pub done: RwSignal<bool>,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: RwSignal::new(title),
            done: RwSignal::new(false),
        }
    }
    pub fn set_todo(&mut self,value: bool) {
        self.done.update(move |val| {
            *val = value;
        });
    }
}
