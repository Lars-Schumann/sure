use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;

static FEATURES: &[&str] = &[
    "u8-1_000",
    "u8-2_000",
    "u8-5_000",
    "u8-10_000",
    "u8-15_000",
    "u8-20_000",
    "u8-25_000",
    "u8-30_000",
    "u8-35_000",
    "u8-40_000",
    "u8-45_000",
    "u8-50_000",
    "u8-55_000",
    "u8-60_000",
    "u8-65_000",
    "u8-70_000",
    "u8-75_000",
    "u8-80_000",
    "u8-85_000",
    "u8-90_000",
    "u8-95_000",
    "u16-1_000",
    "u16-2_000",
    "u16-5_000",
    "u16-10_000",
    "u16-15_000",
    "u16-20_000",
    "u16-25_000",
    "u16-30_000",
    "u16-35_000",
    "u16-40_000",
    "u16-45_000",
    "u16-50_000",
    "u16-55_000",
    "u16-60_000",
    "u16-65_000",
    "u16-70_000",
    "u16-75_000",
    "u16-80_000",
    "u16-85_000",
    "u16-90_000",
    "u16-95_000",
    "u32-1_000",
    "u32-2_000",
    "u32-5_000",
    // "u32-10_000",
    // "u32-15_000",
    // "u32-20_000",
    // "u32-25_000",
    // "u32-30_000",
    // "u32-35_000",
    // "u32-40_000",
    // "u32-45_000",
    // "u32-50_000",
    // "u32-55_000",
    // "u32-60_000",
    // "u32-65_000",
    // "u32-70_000",
    // "u32-75_000",
    // "u32-80_000",
    // "u32-85_000",
    // "u32-90_000",
    // "u32-95_000",
];

fn main() {
    let one = multi_bench_round(1);
    let two = multi_bench_round(1);

    println!("diff:");

    for x in one.iter().zip(two) {
        assert_eq!(x.0.0, x.1.0, "unexpected feature");

        println!(
            "feature: {}, change: {:.3}",
            x.0.0,
            (x.0.1.as_secs_f32() - x.1.1.as_secs_f32()) / x.0.1.as_secs_f32()
        );
    }
}

fn multi_bench_round(count: usize) -> Vec<(&'static str, Duration)> {
    let mut sums: Vec<(&'static str, Duration)> =
        FEATURES.iter().map(|f| (*f, Duration::ZERO)).collect();

    for round in 0..count {
        println!("Round: {}/{count}", round + 1);
        let round = bench_round();
        for (sum, (name, took)) in sums.iter_mut().zip(round) {
            assert_eq!(sum.0, name, "unexpected feature");
            sum.1 += took;
        }
    }

    for (_, total) in sums.iter_mut() {
        *total /= count as u32;
    }

    sums
}

fn bench_round() -> Vec<(&'static str, Duration)> {
    run_cargo(&["build", "--package", "sure"]);
    run_cargo(&["clean", "--package", "sure_bench"]);

    let mut bench_round: Vec<(&'static str, Duration)> = vec![];

    for feature in FEATURES {
        print!("{feature:12}: ");
        let before = Instant::now();
        run_cargo(&["build", "--package", "sure_bench", "--features", feature]);
        let took = before.elapsed();
        println!("{:.3}s", took.as_secs_f32());
        bench_round.push((feature, took));
        run_cargo(&["clean", "--package", "sure_bench"]);
    }
    bench_round
}

fn run_cargo(args: &[&str]) {
    let status = Command::new("cargo")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("failed to run cargo");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
