use leptos::mount::mount_to_body;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        // Added a sleek dark gradient that spans the entire background
        <main class="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-indigo-950 text-slate-300 font-sans">

            // Increased top/bottom padding significantly and enlarged the text for a massive hero presence
            <header class="max-w-5xl mx-auto px-8 pt-48 pb-32">
                <h1 class="text-6xl md:text-8xl font-extrabold tracking-tight text-white mb-6">
                    "I Hate JS "
                    <span class="text-transparent bg-clip-text bg-gradient-to-r from-indigo-400 to-cyan-400">
                        "(ih8js)"
                    </span>
                </h1>
                <p class="text-2xl md:text-3xl text-slate-400 max-w-3xl leading-relaxed font-light">
                    "This is my portfolio, written entirely using WASM."
                </p>
            </header>

            // Slightly taller cards with a subtle glass effect (backdrop-blur)
            <section class="max-w-5xl mx-auto px-8 pb-32 space-y-12">
                <div class="h-80 rounded-2xl border border-white/5 bg-white/5 backdrop-blur-sm flex items-center justify-center text-slate-500 shadow-xl">
                    "Project 1"
                </div>
                <div class="h-80 rounded-2xl border border-white/5 bg-white/5 backdrop-blur-sm flex items-center justify-center text-slate-500 shadow-xl">
                    "Project 2"
                </div>
            </section>

        </main>
    }
}

fn main() {
    mount_to_body(App)
}
