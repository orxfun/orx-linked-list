use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use orx_linked_list::*;
use orx_selfref_col::MemoryPolicy;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Copy)]
enum Action {
    PushFront(u32),
    PopFront,
}

fn get_test_data(n: usize) -> Vec<Action> {
    let mut rng = ChaCha8Rng::seed_from_u64(56456);
    let mut vec: Vec<_> = (0..n)
        .map(|_| Action::PushFront(rng.gen_range(0..n) as u32))
        .collect();
    for _ in 0..2 * n {
        let action = match rng.gen::<f32>() {
            x if x < 0.50 => Action::PushFront(rng.gen_range(0..n) as u32),
            _ => Action::PopFront,
        };
        vec.push(action)
    }
    vec
}

// variants

fn orx_linked_list<M: MemoryPolicy<Singly<u32>>>(
    actions: &[Action],
    list: &mut List<Singly<u32>, M>,
) -> u64 {
    let mut sum = 0;
    for action in actions {
        let x = match action {
            Action::PushFront(x) => {
                list.push_front(*x);
                None
            }
            Action::PopFront => list.pop_front(),
        };
        if let Some(x) = x {
            sum += x as u64;
        }
    }
    sum
}

fn std_linked_list(actions: &[Action], list: &mut std::collections::LinkedList<u32>) -> u64 {
    let mut sum = 0;
    for action in actions {
        let x = match action {
            Action::PushFront(x) => {
                list.push_front(*x);
                None
            }
            Action::PopFront => list.pop_front(),
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
            Action::PushFront(x) => {
                list.push_front(*x);
                None
            }
            Action::PopFront => list.pop_front(),
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
    let treatments = vec![1_024 * 4];

    let mut group = c.benchmark_group("singly_mutation_ends");

    for n in &treatments {
        let data = get_test_data(*n);
        let expected = std_linked_list(&data, &mut std::collections::LinkedList::new());

        group.bench_with_input(BenchmarkId::new("LinkedList", n), n, |b, _| {
            b.iter(|| {
                let mut list = std::collections::LinkedList::new();
                let result = std_linked_list(&data, &mut list);
                assert_eq!(result, expected);
            })
        });

        group.bench_with_input(BenchmarkId::new("VecDeque", n), n, |b, _| {
            b.iter(|| {
                let mut list = std::collections::VecDeque::new();
                let result = std_vec_deque(&data, &mut list);
                assert_eq!(result, expected);
            })
        });

        group.bench_with_input(BenchmarkId::new("SinglyList", n), n, |b, _| {
            b.iter(|| {
                let mut list = SinglyList::new();
                let result = orx_linked_list(&data, &mut list);
                assert_eq!(result, expected);
            })
        });

        group.bench_with_input(BenchmarkId::new("SinglyListLazy", n), n, |b, _| {
            b.iter(|| {
                let mut list = SinglyListLazy::new();
                let result = orx_linked_list(&data, &mut list);
                assert_eq!(result, expected);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
