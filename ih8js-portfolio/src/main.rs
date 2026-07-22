use leptos::mount::mount_to_body;
use leptos::prelude::*;

// This is your main App component
#[component]
fn App() -> impl IntoView {
    view! {
        <main style="padding: 2rem;">
            <h1>"ih8js.me"</h1>
            <p>"Kernel-v6.8.0-JS | WASM Target Acquired"</p>
        </main>
    }
}

fn main() {
    // This tells Leptos to mount the App component to the <body> tag
    mount_to_body(App)
}
