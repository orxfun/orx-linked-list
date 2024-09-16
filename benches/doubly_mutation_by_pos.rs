use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use orx_linked_list::*;
use orx_selfref_col::MemoryPolicy;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Copy)]
enum Action {
    PushBack(u32),
    PushFront(u32),
    Insert(usize, u32),
    Remove(usize),
}

fn get_test_data(n: usize) -> Vec<Action> {
    let mut rng = ChaCha8Rng::seed_from_u64(56456);
    let mut vec: Vec<_> = (0..n)
        .map(|_| match rng.gen::<f32>() {
            x if x < 0.5 => Action::PushBack(rng.gen_range(0..n) as u32),
            _ => Action::PushFront(rng.gen_range(0..n) as u32),
        })
        .collect();
    for _ in 0..2 * n {
        let action = match rng.gen::<f32>() {
            x if x < 0.50 => Action::Insert(rng.gen_range(0..n), rng.gen_range(0..n) as u32),
            _ => Action::Remove(rng.gen_range(0..n)),
        };
        vec.push(action)
    }
    vec
}

// variants

fn doubly_list<M: MemoryPolicy<Doubly<u32>>>(
    actions: &[Action],
    list: &mut List<Doubly<u32>, M>,
) -> u64 {
    let mut sum = 0;
    for action in actions {
        let x = match action {
            Action::PushBack(x) => {
                list.push_back(*x);
                None
            }
            Action::PushFront(x) => {
                list.push_front(*x);
                None
            }
            Action::Insert(pos, x) => {
                if *pos <= list.len() {
                    list.insert_at(*pos, *x);
                }
                Some(*x)
            }
            Action::Remove(pos) => {
                list.remove_at(*pos);
                None
            }
        };
        if let Some(x) = x {
            sum += x as u64;
        }
    }
    sum
}

fn std_vec_deque(actions: &[Action], list: &mut std::collections::VecDeque<u32>) -> u64 {
    let mut sum = 0;
    for action in actions {
        let x = match action {
            Action::PushBack(x) => {
                list.push_back(*x);
                None
            }
            Action::PushFront(x) => {
                list.push_front(*x);
                None
            }
            Action::Insert(pos, x) => {
                if *pos <= list.len() {
                    list.insert(*pos, *x);
                }
                Some(*x)
            }
            Action::Remove(pos) => {
                if *pos < list.len() {
                    list.remove(*pos);
                }
                None
            }
        };
        if let Some(x) = x {
            sum += x as u64;
        }
    }
    sum
}

fn bench(c: &mut Criterion) {
    // let treatments = vec![1_024, 1_024 * 16, 1_024 * 64, 1_024 * 64 * 4];
    // let treatments = vec![1_024 * 64 * 4];
    let treatments = vec![1_024 * 16];

    let mut group = c.benchmark_group("doubly_mutation_by_pos");

    for n in &treatments {
        let data = get_test_data(*n);
        let expected = std_vec_deque(&data, &mut std::collections::VecDeque::new());

        group.bench_with_input(BenchmarkId::new("VecDeque", n), n, |b, _| {
            b.iter(|| {
                let mut list = std::collections::VecDeque::new();
                let result = std_vec_deque(&data, &mut list);
                assert_eq!(result, expected);
            })
        });

        group.bench_with_input(BenchmarkId::new("DoublyList", n), n, |b, _| {
            b.iter(|| {
                let mut list = DoublyList::new();
                let result = doubly_list(&data, &mut list);
                assert_eq!(result, expected);
            })
        });

        group.bench_with_input(BenchmarkId::new("DoublyListLazy", n), n, |b, _| {
            b.iter(|| {
                let mut list = DoublyListLazy::new();
                let result = doubly_list(&data, &mut list);
                assert_eq!(result, expected);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
