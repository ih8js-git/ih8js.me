use leptos::mount::mount_to_body;
use leptos::prelude::*;
use std::time::Duration;
mod icon_slider;
mod knob;
mod rotary_encoder;
use icon_slider::*;
use knob::*;
use rotary_encoder::*;

#[component]
fn Wires(
    channel_a: RwSignal<bool>,
    channel_b: RwSignal<bool>,
    channel_c: RwSignal<bool>,
) -> impl IntoView {
    let (spin_opacity, set_spin_opacity) = signal(0.0_f32);
    let (press_opacity, set_press_opacity) = signal(0.0_f32);
    let spin_gen = RwSignal::new(0_u32);
    let press_gen = RwSignal::new(0_u32);

    Effect::new(move |_| {
        let a = channel_a.get();
        let b = channel_b.get();
        if a || b {
            set_spin_opacity.set(1.0);
            spin_gen.update(|g| *g += 1);
        } else {
            let snapshot = spin_gen.get();
            set_timeout(
                move || {
                    if spin_gen.get_untracked() == snapshot {
                        set_spin_opacity.set(0.0);
                    }
                },
                Duration::from_millis(300),
            );
        }
    });

    Effect::new(move |_| {
        let c = channel_c.get();
        if c {
            set_press_opacity.set(1.0);
            press_gen.update(|g| *g += 1);
        } else {
            let snapshot = press_gen.get();
            set_timeout(
                move || {
                    if press_gen.get_untracked() == snapshot {
                        set_press_opacity.set(0.0);
                    }
                },
                Duration::from_millis(300),
            );
        }
    });

    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index:10;">
            <g style=move || format!("opacity:{};transition:opacity .3s ease", spin_opacity.get())>
                <path d="M 100 212 C 120 180, 0 80, 33 10" stroke="#818cf8" stroke-width="3" fill="none" stroke-linecap="round" />
                <path d="M 28 212 C 10 180, 18 80, 14 10" stroke="#22d3ee" stroke-width="3" fill="none" stroke-linecap="round" />
                <path d="M 64 212 C 64 170, 42 80, 50 10" stroke="#000" stroke-width="3" fill="none" stroke-linecap="round" />
            </g>
            <path
                d="M 64 292 C 64 310, 68 170, 68 10"
                stroke="#22c55e"
                stroke-width="3"
                fill="none"
                stroke-linecap="round"
                style=move || format!("opacity:{};transition:opacity .3s ease", press_opacity.get())
            />
        </svg>
    }
}

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
            <section class="h-screen w-full snap-start flex flex-col p-8 bg-slate-900/50">
                <h1 class="text-6xl font-bold text-white mb-8 text-center w-full">
                    Pico Mixer
                </h1>

                {/* Middle Content: Left and Right Split */}
                <div class="flex flex-row flex-1 justify-between items-center w-full">

                    <div class="relative flex flex-col gap-6 items-start w-1/2">
                        <div class="w-96 h-40" inner_html=PICO_SVG></div>

                        <div class="bg-slate-800/80 p-6 rounded-3xl border border-slate-700 shadow-2xl backdrop-blur-md">
                            <div class="grid grid-cols-3 gap-8">
                                { (0..6).map(|_| view! { <Knob channel_a channel_b channel_c /> }).collect_view() }
                            </div>
                        </div>

                        <Wires channel_a channel_b channel_c />
                    </div>

                    <div class="flex flex-col gap-6 items-end justify-center w-1/2">
                        <div class="bg-slate-800/80 p-6 rounded-3xl border border-slate-700 shadow-2xl backdrop-blur-md">
                            <div class="grid grid-cols-3 gap-8">
                                {icon_sliders()
                                    .into_iter()
                                    .map(|(svg, val)| {
                                        view! { <IconSlider icon=svg value=val /> }
                                    })
                                    .collect_view()}
                            </div>
                        </div>
                    </div>

                </div>

                {/* Bottom: Rotary Encoder Full Width */}
                <div class="w-full mt-auto flex justify-center pb-4">
                    <RotaryEncoder channel_a channel_b channel_c />
                </div>
            </section>

        </main>
    }
}

fn main() {
    mount_to_body(App)
}
