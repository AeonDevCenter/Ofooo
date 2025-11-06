use crate::components::AppHeader;
use crate::components::TodoRow;
use crate::types::Todo;
use leptos::prelude::*;

/// Documentation for [`HomePage`]
#[component]
pub fn HomePage() -> impl IntoView {
    let (todos, set_todos) = signal::<Vec<Todo>>(Vec::new());
    view! {
        <div>
            <AppHeader />

            <div class="container">
                <div class="todos-container">
                    <TodoRow todo=Todo::new("Learn Leptos".to_owned())  />
                </div>

                <div class="summary-container">
                    "Summary content"
                </div>
            </div>
        </div>
    }
}
