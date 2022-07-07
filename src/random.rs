use rand::{distributions::WeightedIndex, prelude::*};

pub trait Shuffle: Copy + Sized + 'static {
    fn pool() -> &'static [Self];
    fn weight(self) -> f32;

    fn shuffle<const COUNT: usize>() -> [Self; COUNT] {
        let mut rng = thread_rng();
        let weights =
            WeightedIndex::new(Self::pool().iter().map(|variant| variant.weight())).unwrap();
        [(); COUNT].map(|_| Self::pool()[weights.sample(&mut rng)])
    }
}
