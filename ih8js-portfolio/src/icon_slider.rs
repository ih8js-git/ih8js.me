use leptos::prelude::*;

const ICON_DISCORD: &str = include_str!("../assets/icons/discord.svg");
const ICON_LIBREWOLF: &str = include_str!("../assets/icons/librewolf.svg");
const ICON_SPEAKER: &str = include_str!("../assets/icons/speaker.svg");
const ICON_SPOTIFY: &str = include_str!("../assets/icons/spotify.svg");
const ICON_STEAM: &str = include_str!("../assets/icons/steam.svg");
const ICON_VLC: &str = include_str!("../assets/icons/vlc.svg");

#[component]
pub fn IconSlider(icon: &'static str, value: u32) -> impl IntoView {
    let fill_pct = (value as f64 / 150.0) * 100.0;
    let fill_style = format!("height: {}%;", fill_pct);

    let track_marker_style = format!("bottom: {}%;", 100.0 / 150.0 * 100.0);

    view! {
        <div class="flex flex-col items-center gap-1.5">
            <div class="w-8 h-8" inner_html=icon></div>

            <div class="relative w-3 h-28 bg-slate-900 rounded-full border border-slate-600 shadow-[inset_0_1px_3px_rgba(0,0,0,0.6)]">
                <div
                    class="absolute bottom-0 left-0 w-full rounded-full bg-gradient-to-t from-indigo-500 to-cyan-400 transition-all duration-300 shadow-[0_0_8px_rgba(99,102,241,0.4)]"
                    style=fill_style
                ></div>
                <div
                    class="absolute left-0 w-full h-px bg-slate-400 opacity-60"
                    style=track_marker_style
                ></div>
            </div>

            <span class="text-xs font-mono text-slate-300 font-bold">
                {value}"%"
            </span>
        </div>
    }
}

pub fn icon_sliders() -> Vec<(&'static str, u32)> {
    vec![
        (ICON_SPEAKER, 50),
        (ICON_LIBREWOLF, 85),
        (ICON_DISCORD, 75),
        (ICON_SPOTIFY, 45),
        (ICON_STEAM, 65),
        (ICON_VLC, 10),
    ]
}
