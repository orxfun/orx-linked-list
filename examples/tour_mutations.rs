use clap::Parser;
use orx_linked_list::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::{fmt::Debug, time::Instant, usize};

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
        let a = rng.gen_range(0..num_cities);
        let b = rng.gen_range(0..num_cities);
        moves.push((a, b));
    }

    moves
}

fn debug_cities<'a, I: Iterator<Item = &'a City>>(
    f: &mut std::fmt::Formatter<'_>,
    mut iter: I,
) -> std::fmt::Result {
    if let Some(x) = iter.next() {
        write!(f, "{}", x.id)?;
        for x in iter {
            write!(f, "-{}", x.id)?;
        }
    }
    Ok(())
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

impl Debug for TourVecCities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_cities(f, self.cities.iter())
    }
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

    fn create_tour(num_cities: usize, num_moves: usize) -> Self {
        let moves = get_moves(num_moves, num_cities);

        let cities = get_cities(num_cities).collect();
        let mut tour = TourVecCities { cities };

        for (city, city_to_succeed) in moves {
            tour.insert_after(city, city_to_succeed);
        }

        tour
    }
}

struct TourLinkedList {
    cities: DoublyList<City>,
    idx: Vec<DoublyIdx<City>>,
}

impl Debug for TourLinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_cities(f, self.cities.iter())
    }
}

impl TourLinkedList {
    fn insert_after(&mut self, city: usize, city_to_succeed: usize) {
        let a = &self.idx[city];
        let b = &self.idx[city_to_succeed];
        self.cities.move_next_to(&a, &b);
    }

    fn create_tour(num_cities: usize, num_moves: usize) -> Self {
        let moves = get_moves(num_moves, num_cities);

        let cities: DoublyList<_> = get_cities(num_cities).collect();
        let idx = cities.indices().collect();
        let mut tour = TourLinkedList { cities, idx };

        for (city, city_to_succeed) in moves {
            tour.insert_after(city, city_to_succeed);
        }

        tour
    }
}

fn timed<R, F: FnOnce() -> R>(run: F) -> (R, u128) {
    let now = Instant::now();
    let result = run();
    let ms = now.elapsed().as_millis();
    (result, ms)
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Length of the tour, or number of cities.
    #[arg(long, default_value_t = 10_000)]
    num_cities: usize,

    /// Number of insert moves to apply on the tour.
    #[arg(long, default_value_t = 10_000)]
    num_moves: usize,
}

const CMD: &str =
    "cargo run --release --example tour_mutations -- --num-cities 10000 --num-moves 10000";

fn main() {
    let args = Args::parse();

    let num_moves = args.num_moves;
    let num_cities = args.num_cities;

    println!("\n");
    println!("------ Tour Mutations Demo ------");
    println!("\n");
    println!(
        "A tour of length {} will be mutated {} times.",
        num_cities, num_moves
    );
    println!(
        "Each mutation takes a city from its position and insert into another position in the tour"
    );

    println!("\n");
    println!("A. Tour is represented as Vec<City>");
    let (tour_vec, ms_vec) = timed(|| TourVecCities::create_tour(num_cities, num_moves));

    println!("\n");
    println!(
        "B. Tour is represented as a combination of a LinkedList<City> and Vec<DoublyIdx<City>>"
    );
    println!("  DoublyIdx<_> can be considered as a reference to the city in the tour");
    println!("  which never changes due to the underlying PinnedVec storage.");
    let (tour_list, ms_list) = timed(|| TourLinkedList::create_tour(num_cities, num_moves));

    println!("\n");
    println!("Execution Times:");
    println!(
        "A. with Vec<_> in ms = {}\nB. with LinkedList<_> in ms = {}",
        ms_vec, ms_list
    );

    println!("\n");
    println!("\n");
    println!("Example usage:\n\n{}\n", CMD);
    assert!(tour_list.cities.eq_to_iter_refs(&tour_vec.cities));
}
