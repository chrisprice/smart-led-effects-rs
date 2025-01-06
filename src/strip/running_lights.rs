use crate::strip::EffectIterator;
use crate::utils::{single_hsv_to_srgb, srgbu8_to_hsv};
use palette::{Hsv, Srgb};

pub struct RunningLights<const N: usize> {
    colour: Hsv,
    position: usize,
    reverse: bool,
}

impl<const N: usize> RunningLights<N> {
    pub fn new(colour: Option<Srgb<u8>>, reverse: bool) -> Self {
        RunningLights {
            colour: match colour {
                Some(colour) => srgbu8_to_hsv(colour),
                None => Hsv::new(0.0, 0.0, 1.0),
            },
            position: match reverse {
                true => N,
                false => 0,
            },
            reverse,
        }
    }
}

impl<const N: usize> EffectIterator<N> for RunningLights<N> {
    fn name(&self) -> &'static str {
        "RunningLights"
    }

    fn next(&mut self) -> Option<[Srgb<u8>; N]> {
        let mut out = [Srgb::<u8>::new(0, 0, 0); N];
        for (i, pixel) in out.iter_mut().enumerate() {
            let brightness = (i as f32 + self.position as f32).sin() / 2.0 + 0.5;
            let mut hsv = self.colour;
            hsv.value = brightness;
            *pixel = single_hsv_to_srgb(hsv);
        }
        if self.reverse {
            self.position -= 1;
            if self.position == 0 {
                self.position = N;
            }
        } else {
            self.position += 1;
            if self.position >= N {
                self.position = 0;
            }
        }

        Some(out)
    }
}
