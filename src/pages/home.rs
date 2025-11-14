// use leptos::logging::log;
use leptos::prelude::*;

/// Documentation for [`HomePage`]
#[component]
pub fn HomePage() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));

    view! {
        <label>
            "Type an integer (or not!)"
            <input type="number" on:input:target=move |ev| {
              // when input changes, try to parse a number from the input
              set_value.set(ev.target().value().parse::<i32>())
            }/>
            <p>
                "You entered "
                <strong>{value}</strong>
            </p>
        </label>
    }
}
