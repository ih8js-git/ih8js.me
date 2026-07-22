use leptos::mount::mount_to_body;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        // Parent: Fixed height, handles scrolling, enforces strict vertical snapping
        <main class="h-screen w-full overflow-y-scroll snap-y snap-mandatory bg-gradient-to-br from-slate-950 via-slate-900 to-indigo-950 text-slate-300 font-sans">

            // Section 1: Hero (fills screen, snaps to top, centers content)
            <section class="h-screen w-full snap-start flex flex-col justify-center items-center px-8 text-center">
                <header class="max-w-5xl mx-auto">
                    <h1 class="text-6xl md:text-8xl font-extrabold tracking-tight text-white mb-6">
                        "I Hate JS "
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-indigo-400 to-cyan-400">
                            "(ih8js)"
                        </span>
                    </h1>
                    <p class="text-2xl md:text-3xl text-slate-400 max-w-3xl leading-relaxed font-light mx-auto">
                        "This is my portfolio, written entirely using WASM."
                    </p>
                </header>
            </section>

            // Section 2: Projects/About (snaps immediately into place when you scroll down)
            <section class="h-screen w-full snap-start flex flex-col justify-center items-center px-8 text-center bg-slate-900/50">
                <h2 class="text-5xl font-bold text-white mb-4">"Pure CSS Snapping"</h2>
                <p class="text-xl text-slate-400 max-w-2xl">
                    "Scroll down, and this section locks perfectly into place."
                </p>
            </section>

        </main>
    }
}

fn main() {
    mount_to_body(App)
}
