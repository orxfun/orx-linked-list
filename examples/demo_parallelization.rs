// cargo run --release --features orx-parallel --example demo_parallelization

use orx_linked_list::*;

fn main() {
    let n = 12345;
    let input: DoublyList<_> = (0..n).map(|x| x.to_string()).collect();
    let expected_num_characters = 50615;

    // computation using iterators

    let total_num_characters: usize = input.iter_x().map(|x| x.len()).sum();
    assert_eq!(total_num_characters, expected_num_characters);

    #[cfg(feature = "orx-parallel")]
    {
        // computation using parallel iterator: replace `iter_x()` with `par_x()`

        let total_num_characters: usize = input.par_x().map(|x| x.len()).sum();
        assert_eq!(total_num_characters, expected_num_characters);

        // configure parallel computation
        let total_num_characters: usize = input
            .par_x()
            .num_threads(2)
            .chunk_size(64)
            .map(|x| x.len())
            .sum();
        assert_eq!(total_num_characters, expected_num_characters);

        // consuming parallel iterator: replace `into_iter_x` with `into_par_x`
        let total_num_characters: usize = input.into_par_x().map(|x| x.len()).sum();
        assert_eq!(total_num_characters, expected_num_characters);
    }
}
