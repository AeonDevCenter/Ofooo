
#[derive(Debug, Clone)]
pub struct Todo {
    pub task: String,
    pub is_completed: bool,
}

impl Todo {
    pub fn new(task: String) -> Self {
        Self {
            task,
            is_completed: false,
        }
    }

    pub fn set_completed(&mut self) {
        self.is_completed = true;
    }
}
