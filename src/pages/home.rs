use crate::components::TodoRow;
use leptos::prelude::*;
use crate::types::Todo;

/// Documentation for [`HomePage`]
#[component]
pub fn HomePage() -> impl IntoView {
    let (todos, set_todos) = signal::<Vec<Todo>>(Vec::new());
    set_todos.update(|vec| {
        vec.push(Todo::new("Learn Leptos".to_string()));
        vec.push(Todo::new("Learn Leptos".to_string()));
        vec.push(Todo::new("Learn Leptos".to_string()));
        vec.push(Todo::new("Learn Leptos".to_string()));
    });

    view! {
        <div>
            <div class="container">
                <div class="todos-container">
                {
                    todos.read().iter().map(|todo| {
                        view! {
                            <TodoRow todo_map={todo.clone()} />
                        }
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
