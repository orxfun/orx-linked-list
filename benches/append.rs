use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Copy)]
enum Action {
    PushBack(u32),
    PushFront(u32),
}

fn get_test_data(n: usize) -> Vec<Action> {
    let mut rng = ChaCha8Rng::seed_from_u64(6523);
    let vec: Vec<_> = (0..n)
        .map(|_| match rng.gen::<f32>() {
            x if x < 0.5 => Action::PushBack(rng.gen()),
            _ => Action::PushFront(rng.gen()),
        })
        .collect();
    vec
}
fn get_orx_linked_list(actions: &[Action]) -> orx_linked_list::DoublyLinkedList<u32> {
    let mut list = orx_linked_list::DoublyLinkedList::new();
    for action in actions {
        match action {
            Action::PushBack(x) => list.push_back(*x),
            Action::PushFront(x) => list.push_front(*x),
        };
    }
    list
}
fn get_std_linked_list(actions: &[Action]) -> std::collections::LinkedList<u32> {
    let mut list = std::collections::LinkedList::new();
    for action in actions {
        match action {
            Action::PushBack(x) => list.push_back(*x),
            Action::PushFront(x) => list.push_front(*x),
        };
    }
    list
}
fn get_std_vecdeque(actions: &[Action]) -> std::collections::VecDeque<u32> {
    let mut list = std::collections::VecDeque::new();
    for action in actions {
        match action {
            Action::PushBack(x) => list.push_back(*x),
            Action::PushFront(x) => list.push_front(*x),
        };
    }
    list
}

// variants
fn bench_orx_linked_list(group: &mut BenchmarkGroup<'_, WallTime>, data: &[Action], n: &usize) {
    group.bench_with_input(
        BenchmarkId::new("orx_linked_list::DoublyLinkedList", n),
        n,
        |b, _| {
            let mut list = get_orx_linked_list(data);
            b.iter(|| list.append_back(get_orx_linked_list(data)))
        },
    );
}

fn bench(c: &mut Criterion) {
    let treatments = vec![
        1_024,
        1_024 * 4,
        1_024 * 16,
        1_024 * 16 * 4,
        1_024 * 16 * 4 * 4,
    ];

    let mut group = c.benchmark_group("append");

    for n in &treatments {
        let data = get_test_data(*n);

        bench_orx_linked_list(&mut group, &data, n);

        group.bench_with_input(
            BenchmarkId::new("std::collections::LinkedList", n),
            n,
            |b, _| {
                let mut list = get_std_linked_list(&data);
                b.iter(|| list.append(&mut get_std_linked_list(&data)))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("std::collections::VecDeque", n),
            n,
            |b, _| {
                let mut list = get_std_vecdeque(&data);
                b.iter(|| list.append(&mut get_std_vecdeque(&data)))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
