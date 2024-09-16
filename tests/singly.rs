#![allow(dead_code)]

use orx_linked_list::*;
use orx_selfref_col::MemoryPolicy;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

type R = ChaCha8Rng;

#[derive(Clone, Copy, Debug)]
enum Actions {
    PushFront,
    PopFront,
    SwapFront,
    MutFront,
    Insert,
    Remove,
}

const ACTIONS: [Actions; 6] = [
    Actions::PushFront,
    Actions::PopFront,
    Actions::SwapFront,
    Actions::MutFront,
    Actions::Insert,
    Actions::Remove,
];

const GROW_ACTIONS: [Actions; 2] = [Actions::PushFront, Actions::Insert];

fn val(r: &mut R) -> String {
    r.gen_range(0..10000).to_string()
}

impl Actions {
    fn pick_grow(r: &mut R) -> Self {
        GROW_ACTIONS[r.gen_range(0..GROW_ACTIONS.len())]
    }

    fn pick(r: &mut R) -> Self {
        ACTIONS[r.gen_range(0..ACTIONS.len())]
    }

    fn apply<M>(self, r: &mut R, list: &mut List<Singly<String>, M>)
    where
        M: MemoryPolicy<Singly<String>>,
    {
        use Actions::*;
        match self {
            PushFront => {
                list.push_front(val(r));
            }
            PopFront => {
                list.pop_front();
            }
            SwapFront => {
                list.swap_front(val(r));
            }
            MutFront => {
                if let Some(x) = list.front_mut() {
                    *x = val(r);
                }
            }
            Insert => {
                let pos = match list.len() {
                    0 => 0,
                    _ => r.gen_range(0..=list.len()),
                };
                list.insert_at(pos, val(r));
            }
            Remove => {
                let pos = match list.len() {
                    0 => 0,
                    _ => r.gen_range(0..list.len()),
                };
                list.remove_at(pos);
            }
        }
    }

    fn apply_all<M>(
        r: &mut R,
        grow_len: usize,
        num_mutations: usize,
        list: &mut List<Singly<String>, M>,
    ) where
        M: MemoryPolicy<Singly<String>>,
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

pub fn new_singly(r: &mut R, grow_len: usize, num_mutations: usize) -> SinglyList<String> {
    let mut list = SinglyList::new();
    Actions::apply_all(r, grow_len, num_mutations, &mut list);
    list
}

pub fn new_singly_with<const D: usize>(
    r: &mut R,
    grow_len: usize,
    num_mutations: usize,
) -> SinglyListThreshold<D, String> {
    let mut list = SinglyList::with_threshold_reclaimer::<D>();
    Actions::apply_all(r, grow_len, num_mutations, &mut list);
    list
}

pub fn new_singly_lazy(r: &mut R, grow_len: usize, num_mutations: usize) -> SinglyListLazy<String> {
    let mut list = SinglyListLazy::new();
    Actions::apply_all(r, grow_len, num_mutations, &mut list);
    list
}
