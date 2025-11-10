use crate::components::TodoRow;
use crate::types::Todo;
use leptos::logging::log;
use leptos::prelude::*;

/// Documentation for [`HomePage`]
#[component]
pub fn HomePage() -> impl IntoView {
    let todos: RwSignal<Vec<Todo>> = RwSignal::new(Vec::new());
    todos.update(|vec| {
        vec.push(Todo::new("Learn Leptos".to_string()));
        vec.push(Todo::new("Learn Leptos".to_string()));
        vec.push(Todo::new("Learn Leptos".to_string()));
        vec.push(Todo::new("Learn Leptos".to_string()));
    });

    Effect::new(move || {
        for todo in todos.get().iter() {
            log!("{:?}", todo);
        }
    });

    view! {
        <div>
            <div class="container">
                <div class="todos-container">
                    <For
                        each=move || todos.get()
                        key=|todo| todo.id
                        let(child)
                    >
                        <TodoRow todo={child} />
                    </For>
                </div>

                <div class="summary-container">
                    {

                    }
                </div>
            </div>
        </div>
    }
}
