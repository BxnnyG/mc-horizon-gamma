// Gamma Control for Horizon. Toggle a high gamma (G), raise or lower it in steps (Up, Down),
// toggle night vision (H) and change its strength (Right, Left), each
// change shown over the hotbar. This file is the module the bundle adds to
// the game (`crate::game::gammautils`).
use super::Game;
use std::cell::RefCell;
use std::time::{Duration, Instant};

pub const ID: &str = "gammautils";
pub const NAME: &str = "Gamma Control";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Toggle and change gamma and night vision with keys: G toggles gamma, Up/Down change it, H toggles night vision, Right/Left change its strength.";

/// The keys as Gamma Utils names them, with its defaults.
pub const KEYS: [(&str, &str, &str); 8] = [
    ("gamma_toggle", "Toggle Gamma", "key.keyboard.g"),
    ("gamma_up", "Increase Gamma", "key.keyboard.up"),
    ("gamma_down", "Decrease Gamma", "key.keyboard.down"),
    ("gamma_max", "Maximum Gamma", "key.keyboard.unknown"),
    ("gamma_min", "Minimum Gamma", "key.keyboard.unknown"),
    ("night_vision", "Toggle Night Vision", "key.keyboard.h"),
    ("night_vision_up", "Increase Night Vision", "key.keyboard.right"),
    ("night_vision_down", "Decrease Night Vision", "key.keyboard.left"),
];

/// Gamma Utils' limits, in percent.
const GAMMA_MIN: i32 = -750;
const GAMMA_MAX: i32 = 1500;
const STEPS: [i32; 4] = [5, 10, 25, 50];
const TOGGLED: [i32; 5] = [500, 800, 1000, 1500, 100];

/// What is kept between sessions (one file per key in the mod's folder).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The gamma in effect, in percent of the game's 0..1 slider.
    pub gamma: i32,
    /// The gamma before toggling, to go back to.
    pub restore: i32,
    pub toggled: bool,
    /// The gamma G toggles to.
    pub toggled_gamma: i32,
    pub step: i32,
    pub night_vision: bool,
    /// Night vision strength, percent.
    pub night_vision_strength: i32,
    pub smooth: bool,
    pub messages: bool,
}

impl Settings {
    fn read(game: &Game<'_>, base: i32) -> Self {
        let get = |k: &str| game.native_setting(ID, k);
        let int = |k: &str, d: i32| get(k).and_then(|v| v.parse().ok()).unwrap_or(d);
        let flag = |k: &str, d: bool| get(k).map_or(d, |v| v == "true");
        Self {
            gamma: int("gamma", base),
            restore: int("restore", base),
            toggled: flag("toggled", false),
            toggled_gamma: int("toggled_gamma", 1500),
            step: int("step", 10),
            night_vision: flag("night_vision", false),
            night_vision_strength: int("night_vision_strength", 100),
            smooth: flag("smooth", false),
            messages: flag("messages", true),
        }
    }
    fn write(&self, game: &Game<'_>) {
        let b = |v: bool| if v { "true" } else { "false" }.to_owned();
        for (k, v) in [
            ("gamma", self.gamma.to_string()),
            ("restore", self.restore.to_string()),
            ("toggled", b(self.toggled)),
            ("toggled_gamma", self.toggled_gamma.to_string()),
            ("step", self.step.to_string()),
            ("night_vision", b(self.night_vision)),
            ("night_vision_strength", self.night_vision_strength.to_string()),
            ("smooth", b(self.smooth)),
            ("messages", b(self.messages)),
        ] {
            game.native_set_setting(ID, k, &v);
        }
    }
}

struct State {
    settings: Option<Settings>,
    /// The gamma drawn now, easing towards the setting when smooth.
    shown: Option<f32>,
    last: Instant,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State { settings: None, shown: None, last: Instant::now() });
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

/// The game's own gamma option, in percent.
fn base_gamma(game: &Game<'_>) -> i32 {
    (game.settings.gamma * 100.0).round() as i32
}

fn settings(game: &Game<'_>) -> Settings {
    if let Some(s) = with(|s| s.settings) {
        return s;
    }
    let s = Settings::read(game, base_gamma(game));
    with(|st| st.settings = Some(s));
    s
}

fn store(game: &Game<'_>, s: Settings) {
    s.write(game);
    with(|st| st.settings = Some(s));
}

/// A line over the hotbar, as Gamma Utils' messages.
fn message(game: &mut Game<'_>, text: String) {
    if settings(game).messages {
        game.message = Some((text, Instant::now()));
    }
}

pub(super) fn labels(game: &mut Game<'_>) -> Vec<String> {
    let s = settings(game);
    let on = |b: bool| if b { "ON" } else { "OFF" };
    vec![
        format!("Toggled Gamma: {}%", s.toggled_gamma),
        format!("Gamma Step: {}%", s.step),
        format!("Smooth Transition: {}", on(s.smooth)),
        format!("Messages: {}", on(s.messages)),
        format!("Night Vision Strength: {}%", s.night_vision_strength),
        "Reset Gamma to the Game's".to_owned(),
    ]
}

pub(super) fn click(game: &mut Game<'_>, button: usize) {
    let mut s = settings(game);
    let next = |list: &[i32], v: i32| {
        let i = list.iter().position(|&x| x == v).map_or(0, |i| (i + 1) % list.len());
        list[i]
    };
    match button {
        0 => s.toggled_gamma = next(&TOGGLED, s.toggled_gamma),
        1 => s.step = next(&STEPS, s.step),
        2 => s.smooth = !s.smooth,
        3 => s.messages = !s.messages,
        4 => s.night_vision_strength = if s.night_vision_strength >= 100 { 10 } else { s.night_vision_strength + 10 },
        5 => {
            s.gamma = base_gamma(game);
            s.restore = s.gamma;
            s.toggled = false;
        }
        _ => {}
    }
    store(game, s);
}

fn bound(game: &Game<'_>, name: &str) -> String {
    let default = KEYS.iter().find(|k| k.0 == name).map_or("", |k| k.2);
    game.native_mod_key(ID, name, default)
}

pub(super) fn key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_on(ID) || !game.native_window() || !game.native_playing() {
        return;
    }
    let Some(action) = KEYS.iter().map(|k| k.0).find(|&k| key == bound(game, k)) else {
        return;
    };
    let mut s = settings(game);
    match action {
        "gamma_toggle" => {
            if s.toggled {
                s.gamma = s.restore;
                s.toggled = false;
            } else {
                s.restore = s.gamma;
                s.gamma = s.toggled_gamma;
                s.toggled = true;
            }
            message(game, format!("Gamma: {}%", s.gamma));
        }
        "gamma_up" | "gamma_down" => {
            let d = if action == "gamma_up" { s.step } else { -s.step };
            s.gamma = (s.gamma + d).clamp(GAMMA_MIN, GAMMA_MAX);
            s.toggled = false;
            message(game, format!("Gamma: {}%", s.gamma));
        }
        "gamma_max" => {
            s.gamma = GAMMA_MAX;
            message(game, format!("Gamma: {}%", s.gamma));
        }
        "gamma_min" => {
            s.gamma = GAMMA_MIN;
            message(game, format!("Gamma: {}%", s.gamma));
        }
        "night_vision" => {
            s.night_vision = !s.night_vision;
            let state = if s.night_vision { "ON" } else { "OFF" };
            message(game, format!("Night Vision: {state}"));
        }
        "night_vision_up" | "night_vision_down" => {
            let d = if action == "night_vision_up" { 10 } else { -10 };
            s.night_vision_strength = (s.night_vision_strength + d).clamp(0, 100);
            s.night_vision = true;
            message(game, format!("Night Vision: {}%", s.night_vision_strength));
        }
        _ => {}
    }
    store(game, s);
}

/// Once a frame: the gamma the renderer draws with (Gamma Utils overrides
/// the game's option; its own option screen still sets the base).
pub(super) fn frame(game: &mut Game<'_>, _elapsed: Duration) {
    if !game.native_window() {
        return;
    }
    if !game.native_on(ID) {
        return;
    }
    let s = settings(game);
    let target = s.gamma as f32 / 100.0;
    let shown = with(|st| {
        let dt = st.last.elapsed().as_secs_f32().min(0.25);
        st.last = Instant::now();
        let cur = st.shown.unwrap_or(target);
        let next = if s.smooth {
            let step = dt * 12.0;
            if (target - cur).abs() <= step { target } else { cur + step * (target - cur).signum() }
        } else {
            target
        };
        st.shown = Some(next);
        next
    });
    if let Some(r) = game.renderer.as_mut() {
        r.gamma = shown;
    }
}

/// The night vision strength Gamma Utils asks for, 0 when it is off.
pub(super) fn night_vision(game: &Game<'_>) -> f32 {
    if !game.native_on(ID) {
        return 0.0;
    }
    let s = with(|st| st.settings);
    match s {
        Some(s) if s.night_vision => s.night_vision_strength as f32 / 100.0,
        _ => 0.0,
    }
}
