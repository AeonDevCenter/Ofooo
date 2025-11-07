use crate::types::Todo;
use leptos::prelude::*;

/// Documentation for [`TodoRow`]
#[component]
pub fn TodoRow(todo_map: Todo) -> impl IntoView {
    view! { <div class="todo-row-container">{todo_map.task}</div> }
}
