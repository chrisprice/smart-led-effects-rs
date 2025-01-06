use crate::strip::EffectIterator;
use palette::{Mix, Srgb};

pub struct ProgressBar<const N: usize> {
    start_colour: Srgb,
    end_colour: Srgb,
    gradient: bool,
    pixels_per_percent: f32,
    current_value: f32,
    last_value: f32,
    last_pixels: Option<[Srgb<u8>; N]>,
}

impl<const N: usize> ProgressBar<N> {
    const DEFAULT_START_COLOUR: Srgb = Srgb::new(0.0, 0.0, 1.0);
    const DEFAULT_END_COLOUR: Srgb = Srgb::new(1.0, 0.0, 0.0);
    pub fn new(
        start_colour: Option<Srgb>,
        end_colour: Option<Srgb>,
        gradient: Option<bool>,
    ) -> Self {
        ProgressBar {
            start_colour: start_colour.unwrap_or(Self::DEFAULT_START_COLOUR),
            end_colour: end_colour.unwrap_or(Self::DEFAULT_END_COLOUR),
            gradient: gradient.unwrap_or(false),
            pixels_per_percent: N as f32 / 100.0,
            current_value: 0.0,
            last_value: 0.0,
            last_pixels: None,
        }
    }

    pub fn set_percentage(&mut self, percentage: f32) {
        self.current_value = percentage;
    }

    pub fn get_output_for_value(&mut self, percentage: f32) -> [Srgb<u8>; N] {
        let percentage = percentage.clamp(0.0, 100.0);
        let pixels = N - (self.pixels_per_percent * (100.0 - percentage)) as usize;
        let mut out = [Srgb::new(0.0, 0.0, 0.0).into_format(); N];

        if self.gradient {
            for (i, pixel) in out.iter_mut().take(pixels).enumerate() {
                *pixel = self
                    .start_colour
                    .mix(self.end_colour, i as f32 / N as f32)
                    .into_format();
            }
        } else {
            for pixel in out.iter_mut().take(pixels) {
                *pixel = self
                    .start_colour
                    .mix(self.end_colour, percentage / 100.0)
                    .into_format();
            }
        }

        out
    }
}

impl<const N: usize> EffectIterator<N> for ProgressBar<N> {
    fn name(&self) -> &'static str {
        "ProgressBar"
    }

    fn next(&mut self) -> Option<[Srgb<u8>; N]> {
        if self.current_value == self.last_value {
            if let Some(pixels) = self.last_pixels.take() {
                return Some(pixels);
            }
        }
        let out = self.get_output_for_value(self.current_value);
        self.last_value = self.current_value;
        self.last_pixels = Some(out.clone());
        Some(out)
    }
}
