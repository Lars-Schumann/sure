use colored::Colorize;
use std::collections::HashMap as Map;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;

static _FEATURES: &[&str] = &[
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
    "u32-10_000",
    "u32-15_000",
    "u32-20_000",
    "u32-25_000",
    "u32-30_000",
    "u32-35_000",
    "u32-40_000",
    "u32-45_000",
    "u32-50_000",
    "u32-55_000",
    "u32-60_000",
    "u32-65_000",
    "u32-70_000",
    "u32-75_000",
    "u32-80_000",
    "u32-85_000",
    "u32-90_000",
    "u32-95_000",
    "u64-1_000",
    "u64-2_000",
    "u64-5_000",
    "u64-10_000",
    "u64-15_000",
    "u64-20_000",
    "u64-25_000",
    "u64-30_000",
    "u64-35_000",
    "u64-40_000",
    "u64-45_000",
    "u64-50_000",
    "u64-55_000",
    "u64-60_000",
    "u64-65_000",
    "u64-70_000",
    "u64-75_000",
    "u64-80_000",
    "u64-85_000",
    "u64-90_000",
    "u64-95_000",
    "u128-1_000",
    "u128-2_000",
    "u128-5_000",
    "u128-10_000",
    "u128-15_000",
    "u128-20_000",
    "u128-25_000",
    "u128-30_000",
    "u128-35_000",
    "u128-40_000",
    "u128-45_000",
    "u128-50_000",
    "u128-55_000",
    "u128-60_000",
    "u128-65_000",
    "u128-70_000",
    "u128-75_000",
    "u128-80_000",
    "u128-85_000",
    "u128-90_000",
    "u128-95_000",
    "i32-cartesian-1_000",
    "i32-cartesian-2_000",
    "i32-cartesian-5_000",
    "i32-cartesian-10_000",
    "i32-cartesian-15_000",
    "i32-cartesian-20_000",
    "i32-cartesian-25_000",
    "i32-cartesian-30_000",
    "i32-cartesian-35_000",
    "i32-cartesian-40_000",
    "i32-cartesian-45_000",
    "i32-cartesian-50_000",
    "i32-cartesian-55_000",
    "i32-cartesian-60_000",
    "i32-cartesian-65_000",
    "i32-cartesian-70_000",
    "i32-cartesian-75_000",
    "i32-cartesian-80_000",
    "i32-cartesian-85_000",
    "i32-cartesian-90_000",
    "i32-cartesian-95_000",
];

static FEATURES: &[&str] = &[
    // "u8-1_000",
    // "u8-2_000",
    // "u8-5_000",
    // "u8-10_000",
    // "u8-20_000",
    // "u8-50_000",
    // "u8-95_000",
    // "u16-1_000",
    // "u16-2_000",
    // "u16-5_000",
    // "u16-10_000",
    // "u16-20_000",
    // "u16-50_000",
    // "u16-95_000",
    // "u32-1_000",
    // "u32-2_000",
    // "u32-5_000",
    // "u32-10_000",
    // "u32-20_000",
    // "u32-50_000",
    // "u32-95_000",
    // "u64-1_000",
    // "u64-2_000",
    // "u64-5_000",
    // "u64-10_000",
    // "u64-20_000",
    // "u64-50_000",
    // "u64-95_000",
    // "u128-1_000",
    // "u128-2_000",
    // "u128-5_000",
    // "u128-10_000",
    // "u128-20_000",
    // "u128-50_000",
    // "u128-95_000",
    "i32-cartesian-1_000",
    "i32-cartesian-2_000",
    "i32-cartesian-5_000",
    "i32-cartesian-10_000",
    "i32-cartesian-20_000",
    "i32-cartesian-50_000",
    "i32-cartesian-95_000",
];

fn main() {
    run_git(&["stash", "push"]);
    let before = multi_bench_round(2);

    run_git(&["stash", "pop"]);
    let after = multi_bench_round(2);

    println!("diff:");

    for feature in FEATURES {
        let change_percent = (after[feature].as_secs_f32() - before[feature].as_secs_f32())
            / before[feature].as_secs_f32()
            * 100.0;

        let change_percent_str = format!("{change_percent:.2}");

        let change_pecercent_str = match change_percent {
            ..-1.0 => change_percent_str.green(),
            -1.0..=1.0 => change_percent_str.bright_black(),
            1.0.. => change_percent_str.red(),
            _ => change_percent_str.blink(),
        };
        println!("feature: {feature:12}, change: {change_pecercent_str:>6}%",);
    }
}

fn multi_bench_round(count: usize) -> Map<&'static str, Duration> {
    let mut multi_bench_round: Map<&'static str, Vec<Duration>> =
        FEATURES.iter().map(|f| (*f, vec![])).collect();

    for round in 0..count {
        println!("Round: {}/{count}", round + 1);

        let bench_round = bench_round();

        for (feature, took) in bench_round {
            multi_bench_round
                .entry(feature)
                .and_modify(|e| e.push(took));
        }
    }

    multi_bench_round
        .iter()
        .map(|(feature, vec)| (*feature, average(vec)))
        .collect()
}

fn bench_round() -> Map<&'static str, Duration> {
    run_cargo(&["build", "--package", "sure"]);
    run_cargo(&["clean", "--package", "sure_bench"]);

    let mut bench_round: Map<&'static str, Duration> = Map::new();

    for feature in FEATURES {
        print!("{feature:12}: ");
        let before = Instant::now();
        run_cargo(&["build", "--package", "sure_bench", "--features", feature]);
        let took = before.elapsed();
        println!("{:.3}s", took.as_secs_f32());
        bench_round.insert(feature, took);
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
        panic!(
            "aborting, `cargo` failed with status code: {:?}",
            status.code()
        );
    }
}

fn run_git(args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("failed to run git");

    if !status.success() {
        panic!(
            "aborting, `git` failed with status code: {:?}",
            status.code()
        );
    }
}

fn average(durations: &[Duration]) -> Duration {
    let count: u32 = durations.len().try_into().expect("can't put len in u32");
    durations.iter().sum::<Duration>() / count
}
