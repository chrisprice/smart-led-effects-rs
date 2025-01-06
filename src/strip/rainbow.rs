use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, ShiftHueAssign, Srgb};

pub struct Rainbow<const N: usize> {
    last_state: [Hsv; N],
    step_size: f32,
}

impl<const N: usize> Rainbow<N> {
    pub fn new(steps: Option<usize>) -> Self {
        let mut last_state = [Hsv::new(0.0, 1.0, 1.0); N];
        let separation = 360.0 / N as f32;
        let step = steps.unwrap_or(360);
        let step_size = 360.0 / step as f32;

        for i in 1..N {
            last_state[i].shift_hue_assign(separation);
        }
        Rainbow {
            last_state,
            step_size,
        }
    }
}

impl<const N: usize> EffectIterator<N> for Rainbow<N> {
    fn name(&self) -> &'static str {
        "Rainbow"
    }

    fn next(&mut self) -> Option<[Srgb<u8>; N]> {
        for pixel in self.last_state.iter_mut() {
            pixel.shift_hue_assign(self.step_size);
        }

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
    }
}
