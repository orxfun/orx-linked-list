use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use orx_linked_list::*;
use orx_selfref_col::MemoryPolicy;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone)]
enum Action {
    PushBack(String),
    PushFront(String),
    PopBack,
    PopFront,
}

fn get_test_data(n: usize) -> Vec<Action> {
    let mut rng = ChaCha8Rng::seed_from_u64(56456);
    let mut vec: Vec<_> = (0..n)
        .map(|_| match rng.random::<f32>() {
            x if x < 0.5 => Action::PushBack(rng.random_range(0..n).to_string()),
            _ => Action::PushFront(rng.random_range(0..n).to_string()),
        })
        .collect();
    for _ in 0..2 * n {
        let action = match rng.random::<f32>() {
            x if x < 0.25 => Action::PushBack(rng.random_range(0..n).to_string()),
            x if x < 0.50 => Action::PushFront(rng.random_range(0..n).to_string()),
            x if x < 0.75 => Action::PopBack,
            _ => Action::PopFront,
        };
        vec.push(action)
    }
    vec
}

// variants

fn fill_doubly_list<M: MemoryPolicy<Doubly<String>>>(
    actions: &[Action],
    list: &mut List<Doubly<String>, M>,
) {
    for action in actions {
        match action {
            Action::PushBack(x) => {
                list.push_back(x.clone());
            }
            Action::PushFront(x) => {
                list.push_front(x.clone());
            }
            Action::PopBack => {
                _ = list.pop_back();
            }
            Action::PopFront => {
                _ = list.pop_front();
            }
        }
    }
}

fn doubly_iter<M: MemoryPolicy<Doubly<String>>>(list: &List<Doubly<String>, M>) -> i64 {
    list.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| match i % 2 {
            0 => x.len() as i64,
            _ => -(x.len() as i64),
        })
        .sum()
}

fn fill_std_linked_list(actions: &[Action], list: &mut std::collections::LinkedList<String>) {
    for action in actions {
        match action {
            Action::PushBack(x) => {
                list.push_back(x.clone());
            }
            Action::PushFront(x) => {
                list.push_front(x.clone());
            }
            Action::PopBack => {
                _ = list.pop_back();
            }
            Action::PopFront => {
                _ = list.pop_front();
            }
        }
    }
}

fn std_linked_list(list: &std::collections::LinkedList<String>) -> i64 {
    list.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| match i % 2 {
            0 => x.len() as i64,
            _ => -(x.len() as i64),
        })
        .sum()
}

fn fill_vec_deque(actions: &[Action], list: &mut std::collections::VecDeque<String>) {
    for action in actions {
        match action {
            Action::PushBack(x) => {
                list.push_back(x.clone());
            }
            Action::PushFront(x) => {
                list.push_front(x.clone());
            }
            Action::PopBack => {
                _ = list.pop_back();
            }
            Action::PopFront => {
                _ = list.pop_front();
            }
        }
    }
}

fn std_vec_deque(list: &std::collections::VecDeque<String>) -> i64 {
    list.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| match i % 2 {
            0 => x.len() as i64,
            _ => -(x.len() as i64),
        })
        .sum()
}

fn bench(c: &mut Criterion) {
    // let treatments = vec![1_024, 1_024 * 16, 1_024 * 64, 1_024 * 64 * 4];
    let treatments = vec![1_024 * 64 * 4];

    let mut group = c.benchmark_group("doubly_iter_rev");

    for n in &treatments {
        let data = get_test_data(*n);

        let mut std_list = std::collections::LinkedList::new();
        fill_std_linked_list(&data, &mut std_list);
        let expected = std_linked_list(&std_list);

        group.bench_with_input(BenchmarkId::new("LinkedList", n), n, |b, _| {
            let mut std_list = std::collections::LinkedList::new();
            fill_std_linked_list(&data, &mut std_list);
            b.iter(|| {
                let result = std_linked_list(&std_list);
                assert_eq!(result, expected);
            })
        });

        group.bench_with_input(BenchmarkId::new("VecDeque", n), n, |b, _| {
            let mut list = std::collections::VecDeque::new();
            fill_vec_deque(&data, &mut list);
            b.iter(|| {
                let result = std_vec_deque(&list);
                assert_eq!(result, expected);
            })
        });

        group.bench_with_input(BenchmarkId::new("DoublyList", n), n, |b, _| {
            let mut list = DoublyList::new();
            fill_doubly_list(&data, &mut list);
            b.iter(|| {
                let result = doubly_iter(&list);
                assert_eq!(result, expected);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
