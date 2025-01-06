use crate::strip::EffectIterator;
use palette::{Darken, Srgb};
use rand::{thread_rng, Rng};

pub struct Meteor<const N: usize> {
    colour: Srgb,
    size: usize,
    position: usize,
    fade: f32,
    current: [Srgb; N],
    random_colour: bool,
}

impl<const N: usize> Meteor<N> {
    const DEFAULT_SIZE: usize = 4;
    const DEFAULT_FADE: f32 = 0.3;
    const DEFAULT_COLOUR: Srgb<u8> = Srgb::<u8>::new(255, 255, 255);

    pub fn new(
        colour: Option<Srgb<u8>>,
        size: Option<usize>,
        fade: Option<f32>,
    ) -> Self {
        Meteor {
            colour: colour.unwrap_or(Self::DEFAULT_COLOUR).into_format(),
            size: size.unwrap_or(Self::DEFAULT_SIZE),
            position: 0,
            fade: fade.unwrap_or(Self::DEFAULT_FADE),
            current: [Srgb::new(0.0, 0.0, 0.0); N],
            random_colour: colour.is_none(),
        }
    }
}

impl<const N: usize> EffectIterator<N> for Meteor<N> {
    fn name(&self) -> &'static str {
        "Meteor"
    }

    fn next(&mut self) -> Option<[Srgb<u8>; N]> {
        let mut rng = thread_rng();
        for pixel in self.current.iter_mut() {
            if rng.gen_range(0.0..1.0) < 0.5 {
                *pixel = pixel.darken(self.fade);
            }
        }

        for i in 0..self.size {
            if (self.position.saturating_sub(i) < N) && (self.position >= i) {
                self.current[self.position - i] = self.colour;
            }
        }
        self.position += 1;
        if self.position > 2 * N {
            if self.random_colour {
                self.colour = Srgb::new(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                );
            }
            self.position = 0;
        }

        Some(self.current)
    }
}
