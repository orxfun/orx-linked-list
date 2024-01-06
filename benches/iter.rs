// use criterion::{
//     criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
// };
// use orx_linked_list::MemoryUtilization;
// use rand::prelude::*;
// use rand_chacha::ChaCha8Rng;

// #[derive(Clone, Copy)]
// enum Action {
//     PushBack(u32),
//     PushFront(u32),
// }

// fn get_test_data(n: usize) -> Vec<Action> {
//     let mut rng = ChaCha8Rng::seed_from_u64(6523);
//     let vec: Vec<_> = (0..n)
//         .map(|_| match rng.gen::<f32>() {
//             x if x < 0.5 => Action::PushBack(rng.gen()),
//             _ => Action::PushFront(rng.gen()),
//         })
//         .collect();
//     vec
// }
// fn get_orx_linked_list(actions: &[Action]) -> orx_linked_list::LinkedList<u32> {
//     let mut list = orx_linked_list::LinkedList::new();
//     for action in actions {
//         match action {
//             Action::PushBack(x) => list.push_back(*x),
//             Action::PushFront(x) => list.push_front(*x),
//         };
//     }
//     list
// }
// fn get_std_linked_list(actions: &[Action]) -> std::collections::LinkedList<u32> {
//     let mut list = std::collections::LinkedList::new();
//     for action in actions {
//         match action {
//             Action::PushBack(x) => list.push_back(*x),
//             Action::PushFront(x) => list.push_front(*x),
//         };
//     }
//     list
// }
// fn get_std_vecdeque(actions: &[Action]) -> std::collections::VecDeque<u32> {
//     let mut list = std::collections::VecDeque::new();
//     for action in actions {
//         match action {
//             Action::PushBack(x) => list.push_back(*x),
//             Action::PushFront(x) => list.push_front(*x),
//         };
//     }
//     list
// }

// // variants
// fn bench_orx_linked_list(
//     group: &mut BenchmarkGroup<'_, WallTime>,
//     data: &[Action],
//     n: &usize,
//     mem: MemoryUtilization,
//     use_iter_unordered: bool,
// ) {
//     fn run(list: &orx_linked_list::LinkedList<u32>, use_iter_unordered: bool) -> u32 {
//         if use_iter_unordered {
//             list.iter_unordered().sum()
//         } else {
//             list.iter().sum()
//         }
//     }

//     let iter = if use_iter_unordered {
//         ".iter_unordered()"
//     } else {
//         ".iter()"
//     };
//     group.bench_with_input(
//         BenchmarkId::new(format!("orx_linked_list::LinkedList({:?}){}", mem, iter), n),
//         n,
//         |b, _| {
//             let list = get_orx_linked_list(data);
//             b.iter(|| run(&list, use_iter_unordered))
//         },
//     );
// }

// fn std_linked_list(list: &std::collections::LinkedList<u32>) -> u32 {
//     list.iter().sum()
// }

// fn std_vecdeque(list: &std::collections::VecDeque<u32>) -> u32 {
//     list.iter().sum()
// }

// fn bench(c: &mut Criterion) {
//     let treatments = vec![1_024, 1_024 * 4, 1_024 * 16];

//     let mut group = c.benchmark_group("iter");

//     for n in &treatments {
//         let data = get_test_data(*n);

//         bench_orx_linked_list(&mut group, &data, n, MemoryUtilization::Lazy, false);
//         bench_orx_linked_list(
//             &mut group,
//             &data,
//             n,
//             MemoryUtilization::WithThreshold(0.75),
//             false,
//         );
//         bench_orx_linked_list(&mut group, &data, n, MemoryUtilization::Lazy, true);
//         bench_orx_linked_list(
//             &mut group,
//             &data,
//             n,
//             MemoryUtilization::WithThreshold(0.75),
//             true,
//         );

//         group.bench_with_input(
//             BenchmarkId::new("std::collections::LinkedList", n),
//             n,
//             |b, _| {
//                 let list = get_std_linked_list(&data);
//                 b.iter(|| std_linked_list(&list))
//             },
//         );
//         group.bench_with_input(
//             BenchmarkId::new("std::collections::VecDeque", n),
//             n,
//             |b, _| {
//                 let list = get_std_vecdeque(&data);
//                 b.iter(|| std_vecdeque(&list))
//             },
//         );
//     }

//     group.finish();
// }

// criterion_group!(benches, bench);
// criterion_main!(benches);
