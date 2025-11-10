use crate::types::Todo;
use leptos::prelude::*;

/// Documentation for [`TodoRow`]
#[component]
pub fn TodoRow(todo: Todo) -> impl IntoView {
    let checked: RwSignal<bool> = todo.done;

    view! {
        <div class="todo-row-container">
            <input type="checkbox" class="checked-input" bind:checked=checked />
            <p>{move || format!("{} — done: {}", todo.title.get(),  checked.get())}</p>
        </div>
    }
}
