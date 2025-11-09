use crate::components::TodoRow;
use crate::types::Todo;
use leptos::logging::log;
use leptos::prelude::*;

/// Documentation for [`HomePage`]
#[component]
pub fn HomePage() -> impl IntoView {
    let todos = RwSignal::<Vec<RwSignal<Todo>>>::new(Vec::new());
    todos.update(|vec| {
        vec.push(RwSignal::new(Todo::new("Learn Leptos".to_string())));
        vec.push(RwSignal::new(Todo::new("Learn Leptos".to_string())));
        vec.push(RwSignal::new(Todo::new("Learn Leptos".to_string())));
        vec.push(RwSignal::new(Todo::new("Learn Leptos".to_string())));
    });

    Effect::new(move || {
        for todo in todos.get().iter() {
            log!("{:?}", todo.get());
        }
    });

    view! {
        <div>
            <div class="container">
                <div class="todos-container">
                    <For
                        each=move || todos.get()
                        key=|state| state.read_only()
                        let(child)
                    >
                        <TodoRow todo_map={child} />
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
