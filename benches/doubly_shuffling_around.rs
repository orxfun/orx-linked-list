use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use orx_linked_list::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

fn get_cities(num_cities: usize) -> impl Iterator<Item = City> {
    (0..num_cities).map(|id| City {
        id,
        name: format!("city-{}", id),
        coordinates: [7, 42],
    })
}

fn get_moves(num_moves: usize, num_cities: usize) -> Vec<(usize, usize)> {
    let mut rng = ChaCha8Rng::seed_from_u64(56456);
    let mut moves = vec![];

    for _ in 0..num_moves {
        let a = rng.random_range(0..num_cities);
        let b = rng.random_range(0..num_cities);
        moves.push((a, b));
    }

    moves
}

#[derive(PartialEq, Eq)]
struct City {
    id: usize,
    name: String,
    coordinates: [i32; 2],
}

struct TourVecCities {
    cities: Vec<City>,
}

impl TourVecCities {
    fn insert_after(&mut self, city: usize, city_to_succeed: usize) {
        if city == city_to_succeed {
            return;
        }

        let mut pos_a = usize::MAX;
        let mut pos_b = usize::MAX;
        for (i, c) in self.cities.iter().enumerate() {
            match c.id {
                x if x == city => {
                    pos_a = i;
                    if pos_b < usize::MAX {
                        break;
                    }
                }
                x if x == city_to_succeed => {
                    pos_b = i;
                    if pos_a < usize::MAX {
                        break;
                    }
                }
                _ => {}
            }
        }

        let target_position = match pos_a < pos_b {
            true => pos_b,
            false => pos_b + 1,
        };

        let city = self.cities.remove(pos_a);
        self.cities.insert(target_position, city);
    }

    fn create_tour(num_cities: usize, moves: &[(usize, usize)]) -> Self {
        let cities = get_cities(num_cities).collect();
        let mut tour = TourVecCities { cities };

        for (city, city_to_succeed) in moves {
            tour.insert_after(*city, *city_to_succeed);
        }

        tour
    }
}

struct TourLinkedList {
    cities: DoublyList<City>,
    idx: Vec<DoublyIdx<City>>,
}

impl TourLinkedList {
    fn insert_after(&mut self, city: usize, city_to_succeed: usize) {
        let a = self.idx[city];
        let b = self.idx[city_to_succeed];
        self.cities.move_next_to(a, b);
    }

    fn create_tour(num_cities: usize, moves: &[(usize, usize)]) -> Self {
        let cities: DoublyList<_> = get_cities(num_cities).collect();
        let idx = cities.indices().collect();
        let mut tour = TourLinkedList { cities, idx };

        for (city, city_to_succeed) in moves {
            tour.insert_after(*city, *city_to_succeed);
        }

        tour
    }
}

fn bench(c: &mut Criterion) {
    let treatments = vec![
        (10, 10_000),
        (100, 10_000),
        (1_000, 10_000),
        (10_000, 10_000),
        (100_000, 10_000),
    ];

    let mut group = c.benchmark_group("doubly_shuffling_around");

    for (num_cities, num_moves) in &treatments {
        let n = &format!("num_cities:{};num_moves:{}", num_cities, num_moves);

        let moves = get_moves(*num_moves, *num_cities);

        let result_vec = TourVecCities::create_tour(*num_cities, &moves);
        let result_list = TourLinkedList::create_tour(*num_cities, &moves);
        assert!(result_list.cities.eq_to_iter_refs(&result_vec.cities));

        group.bench_with_input(BenchmarkId::new("TourLinkedList", n), n, |b, _| {
            b.iter(|| {
                let result = TourLinkedList::create_tour(*num_cities, &moves);
                assert_eq!(result.cities.len(), result_vec.cities.len());
            })
        });

        group.bench_with_input(BenchmarkId::new("TourVec", n), n, |b, _| {
            b.iter(|| {
                let result = TourVecCities::create_tour(*num_cities, &moves);
                assert_eq!(result.cities.len(), result_vec.cities.len());
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
