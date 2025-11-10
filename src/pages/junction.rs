use leptos::prelude::*;

/// Documentation for [`Junction`]
#[component]
pub fn JunctionPage() -> impl IntoView {
    view! {
        <div class="input-container">
            <div class="input-box">
                <input placeholder="Todo...." />
                <button class="add-btn">Add Task</button>
            </div>
        </div>
    }
}
