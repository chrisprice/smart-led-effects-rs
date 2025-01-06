use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, ShiftHue, Srgb};

pub struct Cycle<const N: usize> {
    last_state: Vec<Hsv>,
    step_size: f32,
}

impl<const N: usize> Cycle<N> {
    pub fn new(steps: Option<usize>) -> Self {
        let color = Hsv::new(0.0, 1.0, 1.0);
        let last_state = vec![color; N];

        let step = steps.unwrap_or(360);
        let step_size = 360.0 / step as f32;

        Cycle {
            last_state,
            step_size,
        }
    }
}

impl<const N: usize> EffectIterator<N> for Cycle<N> {
    fn name(&self) -> &'static str {
        "Cycle"
    }

    fn next(&mut self) -> Option<[Srgb<u8>; N]> {
        if let Some(pixel) = self.last_state.first() {
            self.last_state = vec![pixel.shift_hue(self.step_size); self.last_state.len()];
            Some(
                self.last_state
                    .iter()
                    .map(|x| Srgb::from_color(*x).into_format::<u8>())
                    .collect::<Vec<Srgb<u8>>>()
                    .try_into()
                    .unwrap_or_else(|v: Vec<Srgb<u8>>| {
                        panic!("Expected a Vec of length {} but it was {}", N, v.len())
                    }),
            )
        } else {
            None
        }
    }
}
