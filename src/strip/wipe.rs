use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, Srgb};
use rand::Rng;

pub struct Wipe<const N: usize> {
    position: usize,
    buffer: Vec<Srgb<u8>>,
    reverse: bool,
    end: usize,
    randomize: bool,
}

impl<const N: usize> Wipe<N> {
    pub fn new(data: Vec<Srgb<u8>>, reverse: bool) -> Self {
        let mut buffer = vec![Srgb::<u8>::new(0, 0, 0); N];
        buffer.extend(data);
        buffer.extend(vec![Srgb::<u8>::new(0, 0, 0); N]);

        let end = buffer.len() - N;

        Wipe {
            position: match reverse {
                true => end,
                false => 0,
            },
            buffer,
            reverse,
            end,
            randomize: false,
        }
    }

    pub fn colour_wipe(colour: Option<Srgb<u8>>, reverse: bool) -> Self {
        let mut s = Wipe::new(vec![Srgb::new(0, 0, 0); N], reverse);
        match colour {
            Some(colour) => s.fill_wipe(colour),
            None => s.randomize_colour_wipe(),
        }
        s
    }

    fn fill_wipe(&mut self, colour: Srgb<u8>) {
        let mut buffer = vec![Srgb::<u8>::new(0, 0, 0); N];
        buffer.extend(vec![colour; N]);
        buffer.extend(vec![Srgb::<u8>::new(0, 0, 0); N]);
        self.buffer = buffer;
    }

    fn randomize_colour_wipe(&mut self) {
        let mut rng = rand::thread_rng();
        let colour: Srgb<u8> =
            Srgb::from_color(Hsv::new(rng.gen_range(0.0..360.0), 1.0, 1.0)).into_format();
        self.fill_wipe(colour);
        self.randomize = true;
    }
}

impl<const N: usize> EffectIterator<N> for Wipe<N> {
    fn name(&self) -> &'static str {
        "Wipe"
    }

    fn next(&mut self) -> Option<[Srgb<u8>; N]> {
        let out = self
            .buffer
            .iter()
            .skip(self.position)
            .take(N)
            .copied()
            .collect::<Vec<Srgb<u8>>>()
            .try_into()
            .unwrap_or_else(|v: Vec<Srgb<u8>>| {
                panic!("Expected a Vec of length {} but it was {}", N, v.len())
            });

        if self.reverse {
            self.position -= 1;
            if self.position == 0 {
                self.position = self.end;
                if self.randomize {
                    self.randomize_colour_wipe();
                }
            }
        } else {
            self.position += 1;
            if self.position >= self.end {
                self.position = 0;
                if self.randomize {
                    self.randomize_colour_wipe();
                }
            }
        }
        Some(out)
    }
}
