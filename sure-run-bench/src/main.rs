use colored::Colorize;
use std::collections::HashMap as Map;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;

static _FEATURES: &[&str] = &[
    "u8-95_000",
    "i8-95_000",
    "u16-95_000",
    "i16-95_000",
    "u32-95_000",
    "i32-95_000",
    "u64-95_000",
    "i64-95_000",
    "u128-95_000",
    "i128-95_000",
    "usize-95_000",
    "isize-95_000",
];

static FEATURES: &[&str] = &[
    "u8-1_000",
    "u8-2_000",
    "u8-5_000",
    "u8-10_000",
    "u8-20_000",
    "u8-50_000",
    "u8-95_000",
    "u16-1_000",
    "u16-2_000",
    "u16-5_000",
    "u16-10_000",
    "u16-20_000",
    "u16-50_000",
    "u16-95_000",
    "u32-1_000",
    "u32-2_000",
    "u32-5_000",
    "u32-10_000",
    "u32-20_000",
    "u32-50_000",
    "u32-95_000",
    "u64-1_000",
    "u64-2_000",
    "u64-5_000",
    "u64-10_000",
    "u64-20_000",
    "u64-50_000",
    "u64-95_000",
    "u128-1_000",
    "u128-2_000",
    "u128-5_000",
    "u128-10_000",
    "u128-20_000",
    "u128-50_000",
    "u128-95_000",
    "usize-1_000",
    "usize-2_000",
    "usize-5_000",
    "usize-10_000",
    "usize-20_000",
    "usize-50_000",
    "usize-95_000",
    "i8-1_000",
    "i8-2_000",
    "i8-5_000",
    "i8-10_000",
    "i8-20_000",
    "i8-50_000",
    "i8-95_000",
    "i16-1_000",
    "i16-2_000",
    "i16-5_000",
    "i16-10_000",
    "i16-20_000",
    "i16-50_000",
    "i16-95_000",
    "i32-1_000",
    "i32-2_000",
    "i32-5_000",
    "i32-10_000",
    "i32-20_000",
    "i32-50_000",
    "i32-95_000",
    "i64-1_000",
    "i64-2_000",
    "i64-5_000",
    "i64-10_000",
    "i64-20_000",
    "i64-50_000",
    "i64-95_000",
    "i128-1_000",
    "i128-2_000",
    "i128-5_000",
    "i128-10_000",
    "i128-20_000",
    "i128-50_000",
    "i128-95_000",
    "isize-1_000",
    "isize-2_000",
    "isize-5_000",
    "isize-10_000",
    "isize-20_000",
    "isize-50_000",
    "isize-95_000",
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
    let before = multi_bench_round(1);

    run_git(&["stash", "pop"]);
    let after = multi_bench_round(1);

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
    run_cargo(&["clean", "--package", "sure-bench"]);

    let mut bench_round: Map<&'static str, Duration> = Map::new();

    for feature in FEATURES {
        print!("{feature:12}: ");
        let before = Instant::now();
        run_cargo(&["build", "--package", "sure-bench", "--features", feature]);
        let took = before.elapsed();
        println!("{:.3}s", took.as_secs_f32());
        bench_round.insert(feature, took);
        run_cargo(&["clean", "--package", "sure-bench"]);
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
