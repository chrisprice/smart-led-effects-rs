mod breathe;
pub use breathe::Breathe;
mod bounce;
pub use bounce::Bounce;
mod christmas;
pub use christmas::Christmas;
mod collision;
pub use collision::Collision;
mod cycle;
pub use cycle::Cycle;
mod cylon;
pub use cylon::Cylon;
mod fire;
pub use fire::Fire;
mod meteor;
pub use meteor::Meteor;
mod morse;
pub use morse::Morse;
mod progress;
pub use progress::ProgressBar;
mod rainbow;
pub use rainbow::Rainbow;
mod running_lights;
pub use running_lights::RunningLights;
mod strobe;
pub use strobe::Strobe;
mod timer;
pub use timer::Timer;
mod twinkle;
pub use twinkle::Twinkle;
mod snow_sparkle;
pub use snow_sparkle::SnowSparkle;
mod wipe;
pub use wipe::Wipe;

mod effects_trait;
pub use effects_trait::EffectIterator;

const LIST: &[&str] = &[
    "Breathe",
    "Bounce",
    "Collision",
    "Cycle",
    "Cylon",
    "Fire",
    "Meteor",
    "Morse",
    "ProgressBar",
    "Rainbow",
    "RunningLights",
    "SnowSparkle",
    "Strobe",
    "Timer",
    "Twinkle",
    "Wipe",
];

pub fn list() -> Vec<String> {
    LIST.iter().map(|s| s.to_string()).collect()
}

pub fn get_default_effect<const N: usize>(name: &str) -> Option<Box<dyn EffectIterator<N>>> {
    match name {
        "Breathe" => Some(Box::new(Breathe::new(None, None))),
        "Bounce" => Some(Box::new(Bounce::new(None, None, None, None, None))),
        "Collision" => Some(Box::new(Collision::new(Some(true)))),
        "Cycle" => Some(Box::new(Cycle::new(None))),
        "Cylon" => Some(Box::new(Cylon::new(
            palette::Srgb::<u8>::new(255, 0, 0),
            None,
            None,
        ))),
        "Fire" => Some(Box::new(Fire::new(None, None))),
        "Meteor" => Some(Box::new(Meteor::new(None, None, None))),
        "Morse" => Some(Box::new(Morse::new("Hello, world!", None, false))),
        "ProgressBar" => Some(Box::new(ProgressBar::new(None, None, None))),
        "Rainbow" => Some(Box::new(Rainbow::new(None))),
        "RunningLights" => Some(Box::new(RunningLights::new(None, false))),
        "Strobe" => Some(Box::new(Strobe::new(
            None,
            std::time::Duration::from_secs(1),
            None,
        ))),
        // "Timer" => Some(Box::new(Timer::new(None, None))),
        "Twinkle" => Some(Box::new(Twinkle::new(None, None, None, None))),
        "SnowSparkle" => Some(Box::new(SnowSparkle::new(None, None, None, None))),
        "Wipe" => Some(Box::new(Wipe::colour_wipe(None, false))),
        _ => None,
    }
}

pub fn get_all_default_effects<const N: usize>() -> Vec<Box<dyn EffectIterator<N>>> {
    let mut effects = Vec::new();
    for name in LIST {
        if let Some(effect) = get_default_effect::<N>(name) {
            effects.push(effect);
        }
    }
    effects
}
