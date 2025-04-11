#![allow(dead_code)]

use orx_linked_list::*;
use orx_selfref_col::MemoryPolicy;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

type R = ChaCha8Rng;

#[derive(Clone, Copy, Debug)]
enum Actions {
    PushBack,
    PushFront,
    PopBack,
    PopFront,
    SwapBack,
    SwapFront,
    MutBack,
    MutFront,
    Insert,
    Remove,
}

const ACTIONS: [Actions; 10] = [
    Actions::PushBack,
    Actions::PushFront,
    Actions::PopBack,
    Actions::PopFront,
    Actions::SwapBack,
    Actions::SwapFront,
    Actions::MutBack,
    Actions::MutFront,
    Actions::Insert,
    Actions::Remove,
];

const GROW_ACTIONS: [Actions; 3] = [Actions::PushBack, Actions::PushFront, Actions::Insert];

fn val(r: &mut R) -> String {
    r.random_range(0..10000).to_string()
}

impl Actions {
    fn pick_grow(r: &mut R) -> Self {
        GROW_ACTIONS[r.random_range(0..GROW_ACTIONS.len())]
    }

    fn pick(r: &mut R) -> Self {
        ACTIONS[r.random_range(0..ACTIONS.len())]
    }

    fn apply<M>(self, r: &mut R, list: &mut List<Doubly<String>, M>)
    where
        M: MemoryPolicy<Doubly<String>>,
    {
        use Actions::*;
        match self {
            PushBack => {
                list.push_back(val(r));
            }
            PushFront => {
                list.push_front(val(r));
            }
            PopBack => {
                list.pop_back();
            }
            PopFront => {
                list.pop_front();
            }
            SwapBack => {
                list.swap_back(val(r));
            }
            SwapFront => {
                list.swap_front(val(r));
            }
            MutBack => {
                if let Some(x) = list.back_mut() {
                    *x = val(r);
                }
            }
            MutFront => {
                if let Some(x) = list.front_mut() {
                    *x = val(r);
                }
            }
            Insert => {
                let pos = match list.len() {
                    0 => 0,
                    _ => r.random_range(0..=list.len()),
                };
                list.insert_at(pos, val(r));
            }
            Remove => {
                let pos = match list.len() {
                    0 => 0,
                    _ => r.random_range(0..list.len()),
                };
                list.remove_at(pos);
            }
        }
    }

    fn apply_all<M>(
        r: &mut R,
        grow_len: usize,
        num_mutations: usize,
        list: &mut List<Doubly<String>, M>,
    ) where
        M: MemoryPolicy<Doubly<String>>,
    {
        for _ in 0..grow_len {
            Actions::pick_grow(r).apply(r, list);

            #[cfg(feature = "validation")]
            list.validate();
        }

        for _ in 0..num_mutations {
            Actions::pick(r).apply(r, list);

            #[cfg(feature = "validation")]
            list.validate();
        }
    }
}

pub fn rng() -> R {
    ChaCha8Rng::seed_from_u64(5678)
}

pub fn rng_with_seed(seed: u64) -> R {
    ChaCha8Rng::seed_from_u64(seed)
}

pub fn new_doubly(r: &mut R, grow_len: usize, num_mutations: usize) -> DoublyList<String> {
    let mut list = DoublyList::new();
    Actions::apply_all(r, grow_len, num_mutations, &mut list);
    list
}

pub fn new_doubly_with<const D: usize>(
    r: &mut R,
    grow_len: usize,
    num_mutations: usize,
) -> DoublyListThreshold<D, String> {
    let mut list = DoublyList::with_threshold_reclaimer::<D>();
    Actions::apply_all(r, grow_len, num_mutations, &mut list);
    list
}

pub fn new_doubly_lazy(r: &mut R, grow_len: usize, num_mutations: usize) -> DoublyListLazy<String> {
    let mut list = DoublyListLazy::new();
    Actions::apply_all(r, grow_len, num_mutations, &mut list);
    list
}
