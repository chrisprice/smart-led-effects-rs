use crate::strip::EffectIterator;
use palette::{Mix, Srgb};
use std::time::{Duration, Instant};

pub struct Timer<const N: usize> {
    duration: Duration,
    start_colour: Srgb,
    end_colour: Srgb,
    gradient: bool,
    pixels_per_second: f32,
    start: Instant,
    running: bool,
}

impl<const N: usize> Timer<N> {
    const DEFAULT_START_COLOUR: Srgb = Srgb::new(0.0, 0.0, 1.0);
    const DEFAULT_END_COLOUR: Srgb = Srgb::new(1.0, 0.0, 0.0);
    pub fn new(
        duration: Duration,
        start_colour: Option<Srgb>,
        end_colour: Option<Srgb>,
        gradient: Option<bool>,
        start: bool,
    ) -> Self {
        Timer {
            duration,
            start_colour: start_colour.unwrap_or(Self::DEFAULT_START_COLOUR),
            end_colour: end_colour.unwrap_or(Self::DEFAULT_END_COLOUR),
            gradient: gradient.unwrap_or(false),
            pixels_per_second: (N as f32) / (duration.as_millis() as f32 / 1000.0),
            start: Instant::now(),
            running: start,
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

impl<const N: usize> EffectIterator<N> for Timer<N> {
    fn name(&self) -> &'static str {
        "Timer"
    }

    fn next(&mut self) -> Option<[Srgb<u8>; N]> {
        let mut out = [Srgb::new(0u8, 0u8, 0u8); N];
        let elapsed = self.start.elapsed().as_secs();
        if elapsed >= self.duration.as_secs() {
            self.reset();
            return Some(out);
        }
        let pixels = N - (self.pixels_per_second * elapsed as f32).ceil() as usize;
        if self.gradient {
            for (i, pixel) in out.iter_mut().take(pixels).enumerate() {
                *pixel = self
                    .end_colour
                    .mix(self.start_colour, i as f32 / N as f32)
                    .into_format();
            }
        } else {
            for pixel in out.iter_mut().take(pixels) {
                let mix = elapsed as f32 / self.duration.as_secs() as f32;
                *pixel = self.start_colour.mix(self.end_colour, mix).into_format();
            }
        }
        Some(out)
    }
}
