use rand::{distributions::WeightedIndex, prelude::*};

/// Implement my functions, and I will create a `shuffle` method for you
/// which creates a list of weighted random variants
pub trait Shuffle: Copy + Sized + 'static {
    /// List of possible variants.
    /// There is no guarantee that every variant will be included in a shuffled list.
    fn pool() -> &'static [Self];
    /// The weight of this variant, used in a weighted random pick
    fn weight(self) -> f32;

    /// Get a list of weighted random variants
    fn shuffle<const COUNT: usize>() -> [Self; COUNT] {
        let mut rng = thread_rng();
        let weights =
            WeightedIndex::new(Self::pool().iter().map(|variant| variant.weight())).unwrap();
        [(); COUNT].map(|_| Self::pool()[weights.sample(&mut rng)])
    }
}
