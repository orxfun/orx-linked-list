// cargo run --release --features orx-parallel --example bench_parallelization
// cargo run --release --features orx-parallel --example bench_parallelization -- --help
// cargo run --release --features orx-parallel --example bench_parallelization -- --len 50000 --num-repetitions 20

mod utils;

use clap::Parser;
use orx_linked_list::*;
use utils::timed_collect_all;

#[derive(Parser, Debug)]
struct Args {
    /// Number of items in the input iterator.
    #[arg(long, default_value_t = 1_000_000)]
    len: usize,
    /// Number of repetitions to measure time; total time will be reported.
    #[arg(long, default_value_t = 100)]
    num_repetitions: usize,
}

fn fibonacci(n: usize) -> usize {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

fn main() {
    let args = Args::parse();

    let expected_output = {
        let list: DoublyList<_> = (0..args.len as usize).collect();

        list.iter()
            .filter(|x| *x % 3 != 0)
            .map(|x| x + fibonacci(x % 1000))
            .filter_map(|x| (x % 2 == 0).then(|| x.to_string()))
            .collect::<Vec<_>>()
    };

    let computations: Vec<(&str, Box<dyn Fn() -> Vec<String>>)> = vec![
        (
            "Sequential computation over std::collections::LinkedList",
            Box::new(move || {
                let list: std::collections::LinkedList<_> = (0..args.len as usize).collect();

                list.iter()
                    .filter(|x| *x % 3 != 0)
                    .map(|x| x + fibonacci(x % 1000))
                    .filter_map(|x| (x % 2 == 0).then(|| x.to_string()))
                    .collect::<Vec<_>>()
            }),
        ),
        #[cfg(feature = "orx-parallel")]
        (
            "Sequential computation over DoublyList",
            Box::new(move || {
                let list: DoublyList<_> = (0..args.len as usize).collect();

                list.iter_x()
                    .filter(|x| *x % 3 != 0)
                    .map(|x| x + fibonacci(x % 1000))
                    .filter_map(|x| (x % 2 == 0).then(|| x.to_string()))
                    .collect::<Vec<_>>()
            }),
        ),
        #[cfg(feature = "orx-parallel")]
        (
            "Parallelized over DoublyList using orx_parallel",
            Box::new(move || {
                let list: DoublyList<_> = (0..args.len as usize).collect();

                list.par_x() // replace iter_x (into_iter_x) with par_x (into_par_x) to parallelize !
                    .filter(|x| *x % 3 != 0)
                    .map(|x| x + fibonacci(x % 1000))
                    .filter(|x| x % 2 == 0)
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            }),
        ),
    ];

    timed_collect_all(
        "benchmark_parallelization",
        args.num_repetitions,
        &expected_output,
        &computations,
    );
}
