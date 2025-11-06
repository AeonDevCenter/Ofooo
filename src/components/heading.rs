use leptos::prelude::*;
use leptos_router::components::A as Link;
/// Documentation for [`AppHeader`]
#[component]
pub fn AppHeader() -> impl IntoView {
    view! {
        <div class:header >
            <h1>"ofooo"</h1>
            <nav>
                <Link href="/">
                    "Dashboard"
                </Link>
                <Link href="/junction">
                    "Junction"
                </Link>
            </nav>
        </div>
    }
}
