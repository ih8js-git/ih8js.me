use leptos::mount::mount_to_body;
use leptos::prelude::*;
mod knob;
mod rotary_encoder;
use knob::*;
use rotary_encoder::*;

#[component]
fn App() -> impl IntoView {
    let channel_a = RwSignal::new(false);
    let channel_b = RwSignal::new(false);
    let channel_c = RwSignal::new(false);

    view! {
        <main class="h-screen w-full overflow-y-scroll snap-y snap-mandatory bg-gradient-to-br from-slate-950 via-slate-900 to-indigo-950 text-slate-300 font-sans">

            // Section 1: Hero
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

            // Section 2: Interactive Knobs
            <section class="h-screen w-full snap-start flex flex-col justify-center items-center gap-8 bg-slate-900/50">
                <div class="bg-slate-800/80 p-12 rounded-3xl border border-slate-700 shadow-2xl backdrop-blur-md">
                    <div class="grid grid-cols-3 gap-12">

                        { (0..6).map(|_| view! { <Knob channel_a channel_b channel_c /> }).collect_view() }

                    </div>
                </div>

                <RotaryEncoder channel_a channel_b channel_c />
            </section>

        </main>
    }
}

fn main() {
    mount_to_body(App)
}
