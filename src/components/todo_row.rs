use crate::types::Todo;
use leptos::prelude::*;

/// Documentation for [`TodoRow`]
#[component]
pub fn TodoRow(todo: Todo) -> impl IntoView {
    let mut check_todo = todo.clone();
    view! {
        <div class="todo-row-container">
            <input type="checkbox"
            class="checked-input"
            prop:checked= move || todo.done.get()
             on:change=move |ev| {
                    let checked = event_target_checked(&ev);
                    check_todo.set_todo(checked);
                } />
            {move || todo.title.get()}
        </div>
    }
}
