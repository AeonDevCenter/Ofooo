use leptos::prelude::*;
use crate::types::Todo;


/// Documentation for [`TodoRow`]
#[component]
pub fn TodoRow(todo: Todo) -> impl IntoView {
    view! {
      <div class="todo-row-container">
      { todo.task }
      </div>
    }
}
