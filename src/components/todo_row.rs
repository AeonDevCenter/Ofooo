use crate::types::Todo;
use leptos::prelude::*;

/// Documentation for [`TodoRow`]
#[component]
pub fn TodoRow(todo_map: RwSignal<Todo>) -> impl IntoView {
    view! {
        <div class="todo-row-container">
            <button class=("completed", move || todo_map.get().done)  on:click=move |_| todo_map.get().toggle_done()></button>
            {move || todo_map.get().title}
        </div>
    }
}
