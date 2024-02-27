use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use orx_linked_list::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Copy)]
enum Action {
    PushBack(u32),
    PushFront(u32),
    PopBack,
    PopFront,
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
            x if x < 0.25 => Action::PushBack(rng.gen_range(0..n) as u32),
            x if x < 0.50 => Action::PushFront(rng.gen_range(0..n) as u32),
            x if x < 0.75 => Action::PopBack,
            _ => Action::PopFront,
        };
        vec.push(action)
    }
    vec
}

// variants
fn bench_orx_linked_list(group: &mut BenchmarkGroup<'_, WallTime>, data: &Vec<Action>, n: &usize) {
    fn run(actions: &[Action], list: &mut List<Doubly, u32>) -> u32 {
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
                Action::PopBack => list.pop_back(),
                Action::PopFront => list.pop_front(),
            };
            if let Some(x) = x {
                sum += x
            }
        }
        sum
    }

    group.bench_with_input(
        BenchmarkId::new("orx_linked_list::LinkedList", n),
        n,
        |b, _| {
            b.iter(|| {
                let mut list = List::new();
                run(black_box(data), black_box(&mut list))
            })
        },
    );
}

fn std_linked_list(actions: &[Action], list: &mut std::collections::LinkedList<u32>) -> u32 {
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
            Action::PopBack => list.pop_back(),
            Action::PopFront => list.pop_front(),
        };
        if let Some(x) = x {
            sum += x
        }
    }
    sum
}

fn std_vecdeque(actions: &[Action], list: &mut std::collections::VecDeque<u32>) -> u32 {
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
            Action::PopBack => list.pop_back(),
            Action::PopFront => list.pop_front(),
        };
        if let Some(x) = x {
            sum += x
        }
    }
    sum
}

fn bench(c: &mut Criterion) {
    let treatments = vec![1_024, 1_024 * 16, 1_024 * 64, 1_024 * 64 * 4];

    let mut group = c.benchmark_group("mutation_ends");

    for n in &treatments {
        let data = get_test_data(*n);

        bench_orx_linked_list(&mut group, &data, n);

        group.bench_with_input(
            BenchmarkId::new("std::collections::LinkedList", n),
            n,
            |b, _| {
                b.iter(|| {
                    let mut list = std::collections::LinkedList::new();
                    std_linked_list(black_box(&data), black_box(&mut list))
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("std::collections::VecDeque", n),
            n,
            |b, _| {
                b.iter(|| {
                    let mut list = std::collections::VecDeque::new();
                    std_vecdeque(black_box(&data), black_box(&mut list))
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
