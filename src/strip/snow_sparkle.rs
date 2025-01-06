use crate::strip::EffectIterator;
use palette::{Hsv, Srgb};
use rand::{thread_rng, Rng};

use crate::utils::{hsv_to_srgb, srgbu8_to_hsv};

pub struct SnowSparkle<const N: usize> {
    frequency: u8,
    probability: f32,
    fade: f32,
    colour: Hsv,
    current: [Hsv; N],
}

impl<const N: usize> SnowSparkle<N> {
    const DEFAULT_FREQUENCY: u8 = 0x04;
    const DEFAULT_PROBABILITY: f32 = 0.1;
    const DEFAULT_FADE: f32 = 0.4;
    const BASE_BRIGHTNESS: f32 = 0.2;
    pub fn new(
        colour: Option<Srgb<u8>>,
        sparkle: Option<u8>,
        probability: Option<f32>,
        fade: Option<f32>,
    ) -> Self {
        let mut colour = match colour {
            Some(colour) => srgbu8_to_hsv(colour),
            None => Hsv::new(0.0, 0.0, 1.0),
        };

        colour.value = SnowSparkle::BASE_BRIGHTNESS;

        SnowSparkle {
            frequency: sparkle.unwrap_or(SnowSparkle::DEFAULT_FREQUENCY),
            fade: fade.unwrap_or(SnowSparkle::DEFAULT_FADE),
            probability: probability.unwrap_or(SnowSparkle::DEFAULT_PROBABILITY),
            current: [colour; N],
            colour,
        }
    }

    pub fn sparkle(colour: Option<Srgb<u8>>) -> Self {
        let colour = match colour {
            Some(colour) => Some(colour),
            None => Some(Srgb::<u8>::new(255, 255, 255)),
        };
        SnowSparkle::new(colour, Some(20), Some(0.4), Some(1.0))
    }
}

impl<const N: usize> SnowSparkle<N> {
    fn generate_sparkle(&mut self) {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.current.len());

        let mut sparkle = self.colour;
        sparkle.value = rng.gen_range(0.5..1.0);

        let chance = rng.gen_range(0.0..1.0);
        if chance < self.probability {
            self.current[index] = sparkle;
        }
    }

    fn fade_sparkles(&mut self) {
        for pixel in self.current.iter_mut() {
            pixel.value = (pixel.value - self.fade).max(Self::BASE_BRIGHTNESS);
        }
    }
}

impl<const N: usize> EffectIterator<N> for SnowSparkle<N> {
    fn name(&self) -> &'static str {
        "SnowSparkle"
    }

    fn next(&mut self) -> Option<[Srgb<u8>; N]> {
        self.fade_sparkles();

        let chances = thread_rng().gen_range(0..self.frequency);
        for _ in 0..chances {
            self.generate_sparkle();
        }

        Some(hsv_to_srgb(self.current.clone()))
    }
}
