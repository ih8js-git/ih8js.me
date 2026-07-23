use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn Knob(
    channel_a: RwSignal<bool>,
    channel_b: RwSignal<bool>,
    channel_c: RwSignal<bool>,
) -> impl IntoView {
    let (rotation, set_rotation) = signal(0.0);
    let (last_step_time, set_last_step_time) = signal(0.0);

    let on_wheel = move |ev: web_sys::WheelEvent| {
        ev.prevent_default();

        let now = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        let cooldown_ms = 450.0;

        if now - last_step_time.get_untracked() >= cooldown_ms {
            let step_size = 30.0;

            if ev.delta_y() > 0.0 {
                set_rotation.update(|r| *r -= step_size);
                set_last_step_time.set(now);

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
                set_rotation.update(|r| *r += step_size);
                set_last_step_time.set(now);

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

    view! {
        <div
            class="group w-20 h-20 cursor-pointer transition-transform duration-75 active:scale-95"
            on:wheel=on_wheel
            on:pointerdown=move |_| channel_c.set(true)
            on:pointerup=move |_| channel_c.set(false)
            on:pointerleave=move |_| channel_c.set(false)
        >
            <div
                class="relative w-full h-full rounded-full bg-slate-700 border border-slate-600
                       shadow-[inset_0_2px_4px_rgba(255,255,255,0.1),_0_6px_10px_rgba(0,0,0,0.4)]
                       transition-colors duration-200 
                       hover:bg-indigo-500 hover:border-indigo-400 hover:shadow-[0_0_15px_rgba(99,102,241,0.5)]
                       group-active:shadow-inner group-active:bg-slate-800"
                style=move || format!("transform: rotate({}deg);", rotation.get())
            >
                <div class="absolute top-2 left-1/2 -translate-x-1/2 w-2 h-2 rounded-full bg-white shadow-[0_0_5px_rgba(255,255,255,0.8)]"></div>
            </div>
        </div>
    }
}
