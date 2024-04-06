mod brc;
pub mod common;
use common::Timer;

fn solution(path: &str) -> String {
    unsafe { brc::solution(path) }
}

fn main() {
    let expect_output = std::fs::read_to_string(common::OUTPUT_PATH).unwrap();

    let timer = Timer::new();
    let got = solution(common::MEASUREMENTS_PATH);
    println!("Elapsed: {}ms", timer.elapsed_as_millis());

    assert_eq!(expect_output, got);
}
