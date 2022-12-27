use super::MinoType;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

/// Chooses a mino with 7-bag randomizer
///
/// # Arguments
///
/// - `nth`: `n`th mino (0-indexed)
/// - `seed`: seed for RNG
pub(crate) fn rand_mino(nth: usize, seed: u64) -> MinoType {
    let nth_bag = nth / 7;
    let mut minoes = [
        MinoType::I,
        MinoType::O,
        MinoType::L,
        MinoType::J,
        MinoType::Z,
        MinoType::S,
        MinoType::T,
    ];
    let nth_in_bag = nth % 7;

    let mut rng = StdRng::seed_from_u64(seed + nth_bag as u64);
    minoes.shuffle(&mut rng);
    minoes[nth_in_bag]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_minoes_is_different_in_each_bags() {
        for nth_bag in 0..64 {
            // First nth of each bag
            let each_bag_first_nth = nth_bag * 7;
            let seed = 123456789;

            let first = rand_mino(each_bag_first_nth, seed);
            let second = rand_mino(each_bag_first_nth + 1, seed);
            let third = rand_mino(each_bag_first_nth + 2, seed);
            let fourth = rand_mino(each_bag_first_nth + 3, seed);
            let fifth = rand_mino(each_bag_first_nth + 4, seed);
            let sixth = rand_mino(each_bag_first_nth + 5, seed);
            let seventh = rand_mino(each_bag_first_nth + 6, seed);

            assert_ne!(first, second);
            assert_ne!(first, third);
            assert_ne!(first, fourth);
            assert_ne!(first, fifth);
            assert_ne!(first, sixth);
            assert_ne!(first, seventh);

            //assert_ne!(second, first); // already checked
            assert_ne!(second, third);
            assert_ne!(second, fourth);
            assert_ne!(second, fifth);
            assert_ne!(second, sixth);
            assert_ne!(second, seventh);

            //assert_ne!(third, first); // already checked
            //assert_ne!(third, second); // already checked
            assert_ne!(third, fourth);
            assert_ne!(third, fifth);
            assert_ne!(third, sixth);
            assert_ne!(third, seventh);

            //assert_ne!(fourth, first); // already checked
            //assert_ne!(fourth, second); // already checked
            //assert_ne!(fourth, third); // already checked
            assert_ne!(fourth, fifth);
            assert_ne!(fourth, sixth);
            assert_ne!(fourth, seventh);

            //assert_ne!(fifth, first); // already checked
            //assert_ne!(fifth, second); // already checked
            //assert_ne!(fifth, third); // already checked
            //assert_ne!(fifth, fourth); // already checked
            assert_ne!(fifth, sixth);
            assert_ne!(fifth, seventh);

            //assert_ne!(sixth, first); // already checked
            //assert_ne!(sixth, second); // already checked
            //assert_ne!(sixth, third); // already checked
            //assert_ne!(sixth, fourth); // already checked
            //assert_ne!(sixth, fifth); // already checked
            assert_ne!(sixth, seventh);

            //assert_ne!(seventh, first); // already checked
            //assert_ne!(seventh, second); // already checked
            //assert_ne!(seventh, third); // already checked
            //assert_ne!(seventh, fourth); // already checked
            //assert_ne!(seventh, fifth); // already checked
            //assert_ne!(seventh, sixth); // already checked

            println!("{}th bag checked", nth_bag + 1);
        }
    }
}
