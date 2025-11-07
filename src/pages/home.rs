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
                {
                    todos.read().iter().map(|todo| {
                        "HEllo"
                    }).collect_view()
                }

                </div>

                <div class="summary-container">
                    {

                    }
                </div>
            </div>
        </div>
    }
}
