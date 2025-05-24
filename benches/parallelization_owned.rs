use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use orx_linked_list::*;
#[cfg(feature = "orx-parallel")]
use orx_parallel::ParIter;
use orx_selfref_col::MemoryPolicy;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

fn fibonacci(n: i64) -> i64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
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

fn doubly_iter<M: MemoryPolicy<Doubly<String>>>(list: List<Doubly<String>, M>) -> Vec<char> {
    list.into_iter()
        .filter(|x| fibonacci(x.parse::<i64>().unwrap() % 1000) % 2 == 0)
        .map(|x| x.chars().last().unwrap())
        .collect()
}

fn doubly_iter_x<M: MemoryPolicy<Doubly<String>>>(list: List<Doubly<String>, M>) -> Vec<char> {
    list.into_iter_x()
        .filter(|x| fibonacci(x.parse::<i64>().unwrap() % 1000) % 2 == 0)
        .map(|x| x.chars().last().unwrap())
        .collect()
}

#[cfg(feature = "orx-parallel")]
fn doubly_par_x<M: MemoryPolicy<Doubly<String>>>(list: List<Doubly<String>, M>) -> Vec<char> {
    list.into_par_x()
        .filter(|x| fibonacci(x.parse::<i64>().unwrap() % 1000) % 2 == 0)
        .map(|x| x.chars().last().unwrap())
        .collect()
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

fn std_linked_list(list: std::collections::LinkedList<String>) -> Vec<char> {
    list.into_iter()
        .filter(|x| fibonacci(x.parse::<i64>().unwrap() % 1000) % 2 == 0)
        .map(|x| x.chars().last().unwrap())
        .collect()
}

fn std_linked_list_rayon(list: std::collections::LinkedList<String>) -> Vec<char> {
    list.into_par_iter()
        .filter(|x| fibonacci(x.parse::<i64>().unwrap() % 1000) % 2 == 0)
        .map(|x| x.chars().last().unwrap())
        .collect()
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

fn std_vec_deque(list: std::collections::VecDeque<String>) -> Vec<char> {
    list.into_iter()
        .filter(|x| fibonacci(x.parse::<i64>().unwrap() % 1000) % 2 == 0)
        .map(|x| x.chars().last().unwrap())
        .collect()
}

fn std_vec_deque_rayon(list: std::collections::VecDeque<String>) -> Vec<char> {
    list.into_par_iter()
        .filter(|x| fibonacci(x.parse::<i64>().unwrap() % 1000) % 2 == 0)
        .map(|x| x.chars().last().unwrap())
        .collect()
}

fn bench(c: &mut Criterion) {
    let treatments = vec![1_024 * 64 * 4];

    let mut group = c.benchmark_group("parallelization_owned");

    for n in &treatments {
        let data = get_test_data(*n);

        let mut std_list = std::collections::LinkedList::new();
        fill_std_linked_list(&data, &mut std_list);
        let expected = std_linked_list(std_list.clone());

        group.bench_with_input(BenchmarkId::new("LinkedList::into_iter", n), n, |b, _| {
            let mut std_list = std::collections::LinkedList::new();
            fill_std_linked_list(&data, &mut std_list);
            let result = std_linked_list(std_list.clone());
            assert_eq!(result, expected);

            b.iter(|| std_linked_list(std_list.clone()))
        });

        group.bench_with_input(
            BenchmarkId::new("LinkedList::into_par_iter (rayon)", n),
            n,
            |b, _| {
                let mut std_list = std::collections::LinkedList::new();
                fill_std_linked_list(&data, &mut std_list);
                let result = std_linked_list_rayon(std_list.clone());
                assert_eq!(result, expected);

                b.iter(|| std_linked_list_rayon(std_list.clone()))
            },
        );

        group.bench_with_input(BenchmarkId::new("VecDeque::into_iter", n), n, |b, _| {
            let mut list = std::collections::VecDeque::new();
            fill_vec_deque(&data, &mut list);
            let result = std_vec_deque(list.clone());
            assert_eq!(result, expected);

            b.iter(|| std_vec_deque(list.clone()))
        });

        group.bench_with_input(
            BenchmarkId::new("VecDeque::into_par_iter (rayon)", n),
            n,
            |b, _| {
                let mut list = std::collections::VecDeque::new();
                fill_vec_deque(&data, &mut list);
                let result = std_vec_deque_rayon(list.clone());
                assert_eq!(result, expected);

                b.iter(|| std_vec_deque_rayon(list.clone()))
            },
        );

        group.bench_with_input(BenchmarkId::new("DoublyList::into_iter", n), n, |b, _| {
            let mut list = DoublyList::new();
            fill_doubly_list(&data, &mut list);
            let result = doubly_iter(list.clone());
            assert_eq!(result, expected);

            b.iter(|| doubly_iter(list.clone()))
        });

        group.bench_with_input(BenchmarkId::new("DoublyList::into_iter_x", n), n, |b, _| {
            let mut list = DoublyList::new();
            fill_doubly_list(&data, &mut list);
            let result = doubly_iter_x(list.clone());
            assert_eq!(result, expected);

            b.iter(|| doubly_iter_x(list.clone()))
        });

        #[cfg(feature = "orx-parallel")]
        group.bench_with_input(
            BenchmarkId::new("DoublyList::into_par_x (orx-parallel)", n),
            n,
            |b, _| {
                let mut list = DoublyList::new();
                fill_doubly_list(&data, &mut list);
                let result = doubly_par_x(list.clone());
                assert_eq!(result, expected);

                b.iter(|| doubly_par_x(list.clone()))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
