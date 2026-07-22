use leptos::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;

#[component]
pub fn RotaryEncoder(channel_a: RwSignal<bool>, channel_b: RwSignal<bool>) -> impl IntoView {
    let history = RwSignal::new(VecDeque::new());

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
        <div class="w-full bg-slate-950 border-2 border-slate-700 rounded-xl p-4 shadow-[inset_0_0_20px_rgba(0,0,0,0.8)] relative overflow-hidden">
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
    }
}
