use palette::Srgb;

pub trait EffectIterator<const N: usize> {
    fn name(&self) -> &'static str;
    fn next(&mut self) -> Option<[Srgb<u8>; N]>;
}
