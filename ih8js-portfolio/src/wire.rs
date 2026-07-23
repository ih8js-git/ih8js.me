use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn Wires(
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
                d="M 78 292 C 78 310, 80 170, 70 10"
                stroke="#22c55e"
                stroke-width="3"
                fill="none"
                stroke-linecap="round"
                style=move || format!("opacity:{};transition:opacity .3s ease", press_opacity.get())
            />
            <path
                d="M 50 292 C 50 310, 50 170, 50 10"
                stroke="#000"
                stroke-width="3"
                fill="none"
                stroke-linecap="round"
                style=move || format!("opacity:{};transition:opacity .3s ease", press_opacity.get())
            />
        </svg>
    }
}
