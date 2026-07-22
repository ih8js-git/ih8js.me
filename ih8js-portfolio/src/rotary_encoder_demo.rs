use leptos::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;

#[component]
pub fn RotaryEncoderDemo() -> impl IntoView {
    let (rotation, set_rotation) = signal(0.0);
    let (last_step_time, set_last_step_time) = signal(0.0);

    // Electrical pin states
    let channel_a = RwSignal::new(false);
    let channel_b = RwSignal::new(false);

    // Oscilloscope history buffer
    let history = RwSignal::new(VecDeque::new());

    // Continuous sampling loop (runs every 20ms)
    fn sample_signals(
        hist: RwSignal<VecDeque<(bool, bool)>>,
        a: RwSignal<bool>,
        b: RwSignal<bool>,
    ) {
        set_timeout(
            move || {
                hist.update(|h| {
                    if h.len() >= 100 {
                        h.pop_front();
                    }
                    h.push_back((a.get_untracked(), b.get_untracked()));
                });
                sample_signals(hist, a, b);
            },
            Duration::from_millis(20),
        );
    }
    sample_signals(history, channel_a, channel_b);

    let on_wheel = move |ev: web_sys::WheelEvent| {
        ev.prevent_default();

        let now = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        // Lockout to allow the slowed-down wave animation to complete (4 phases * 100ms = 400ms)
        let cooldown_ms = 450.0;

        if now - last_step_time.get_untracked() >= cooldown_ms {
            let step_size = 30.0;

            if ev.delta_y() > 0.0 {
                // CCW Turn
                set_rotation.update(|r| *r -= step_size);
                set_last_step_time.set(now);

                // CCW Sequence (Channel B leads Channel A)
                channel_b.set(true);
                set_timeout(
                    move || {
                        channel_a.set(true);
                        set_timeout(
                            move || {
                                channel_b.set(false);
                                set_timeout(
                                    move || {
                                        channel_a.set(false);
                                    },
                                    Duration::from_millis(100),
                                );
                            },
                            Duration::from_millis(100),
                        );
                    },
                    Duration::from_millis(100),
                );
            } else if ev.delta_y() < 0.0 {
                // CW Turn
                set_rotation.update(|r| *r += step_size);
                set_last_step_time.set(now);

                // CW Sequence (Channel A leads Channel B)
                channel_a.set(true);
                set_timeout(
                    move || {
                        channel_b.set(true);
                        set_timeout(
                            move || {
                                channel_a.set(false);
                                set_timeout(
                                    move || {
                                        channel_b.set(false);
                                    },
                                    Duration::from_millis(100),
                                );
                            },
                            Duration::from_millis(100),
                        );
                    },
                    Duration::from_millis(100),
                );
            }
        }
    };

    // Map history to SVG coordinates
    let points_a = move || {
        history
            .read()
            .iter()
            .enumerate()
            .map(|(i, (a, _))| format!("{},{}", i * 10, if *a { 10 } else { 50 }))
            .collect::<Vec<_>>()
            .join(" ")
    };

    let points_b = move || {
        history
            .read()
            .iter()
            .enumerate()
            .map(|(i, (_, b))| format!("{},{}", i * 10, if *b { 70 } else { 110 }))
            .collect::<Vec<_>>()
            .join(" ")
    };

    view! {
        <div class="flex flex-col items-center gap-12 w-full max-w-3xl">

            // The physical knob
            <div
                class="group w-24 h-24 cursor-pointer transition-transform duration-75 active:scale-95"
                on:wheel=on_wheel
            >
                <div
                    class="relative w-full h-full rounded-full bg-slate-700 border border-slate-600
                           shadow-[inset_0_2px_4px_rgba(255,255,255,0.1),_0_6px_10px_rgba(0,0,0,0.4)]
                           transition-colors duration-200 group-hover:bg-indigo-500 group-hover:border-indigo-400 group-active:shadow-inner"
                    style=move || format!("transform: rotate({}deg);", rotation.get())
                >
                    <div class="absolute top-2 left-1/2 -translate-x-1/2 w-2 h-2 rounded-full bg-white shadow-[0_0_5px_rgba(255,255,255,0.8)]"></div>
                </div>
            </div>

            // The Oscilloscope screen
            <div class="w-full bg-slate-950 border-2 border-slate-700 rounded-xl p-4 shadow-[inset_0_0_20px_rgba(0,0,0,0.8)] relative overflow-hidden">
                // Screen glare and grid lines
                <div class="absolute inset-0 opacity-10 bg-[radial-gradient(circle_at_center,_transparent_0%,_#000_100%)] pointer-events-none"></div>
                <div class="absolute inset-0 bg-[linear-gradient(to_right,#334155_1px,transparent_1px),linear-gradient(to_bottom,#334155_1px,transparent_1px)] bg-[size:40px_40px] opacity-20 pointer-events-none"></div>

                <div class="flex justify-between text-xs font-mono mb-2 relative z-10 font-bold">
                    <span class="text-indigo-400">"CH A"</span>
                    <span class="text-cyan-400">"CH B"</span>
                </div>

                <svg class="w-full h-32 relative z-10" viewBox="0 0 1000 120" preserveAspectRatio="none">
                    <polyline
                        points=points_a
                        fill="none"
                        stroke="#818cf8"
                        stroke-width="4"
                        stroke-linejoin="round"
                    />
                    <polyline
                        points=points_b
                        fill="none"
                        stroke="#22d3ee"
                        stroke-width="4"
                        stroke-linejoin="round"
                    />
                </svg>
            </div>

        </div>
    }
}
