// use crate::{helpers::build_conditional_class, store::Todo};
// use leptos::prelude::*;

// /// Documentation for [`TodoRow`]
// #[component]
// pub fn TodoRow(todo: Todo) -> impl IntoView {
//     let checked: RwSignal<bool> = todo.done;
//     view! {
//         <div class=move || { build_conditional_class(&[("todo-row-container", true), ("completed", checked.get())]) }>
//             <input class=move || { build_conditional_class(&[("checked-input", true)]) } type="checkbox" bind:checked=checked />
//             <p>{move || todo.title.get()}</p>
//         </div>
//     }
// }
