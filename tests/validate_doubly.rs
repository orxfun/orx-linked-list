mod doubly;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use test_case::test_case;

#[cfg(miri)]
const NUM_RERUNS: usize = 1;
#[cfg(not(miri))]
const NUM_RERUNS: usize = 10;

#[test_case(0, 100)]
#[test_case(100, 100)]
fn validate_doubly_threshold_2(grow_len: usize, num_mutations: usize) {
    let mut rng = ChaCha8Rng::seed_from_u64(9874);
    let r = &mut rng;

    for _ in 0..NUM_RERUNS {
        doubly::new_doubly(r, grow_len, num_mutations);
    }
}

#[test_case(0, 100)]
#[test_case(100, 100)]
fn validate_doubly_threshold_4(grow_len: usize, num_mutations: usize) {
    let mut rng = ChaCha8Rng::seed_from_u64(9874);
    let r = &mut rng;

    for _ in 0..NUM_RERUNS {
        doubly::new_doubly_with::<4>(r, grow_len, num_mutations);
    }
}

#[test_case(0, 100)]
#[test_case(100, 100)]
fn validate_doubly_lazy(grow_len: usize, num_mutations: usize) {
    let mut rng = ChaCha8Rng::seed_from_u64(9874);
    let r = &mut rng;

    for _ in 0..NUM_RERUNS {
        doubly::new_doubly_lazy(r, grow_len, num_mutations);
    }
}
