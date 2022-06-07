use std::cmp::Reverse;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{ffi::OsStr, process::Output};

use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub fn bench() -> Result<()> {
    let f = fs::File::open("benches.json")?;
    let mut summary: Vec<SummaryOut> = serde_json::from_reader(f)?;

    summary.sort_by_key(|s| {
        let part2: f64 = s.benches.last().map(|b| b.mean).unwrap();
        Reverse((part2 * 1_000_000.) as i64)
    });

    summary_to_markdown(&summary);
    Ok(())
}

pub fn bench_all() -> Result<()> {
    let workspace_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let manifest_path = workspace_dir.join("Cargo.toml");

    let metadata: Metadata = {
        let mut cmd = Command::new(env!("CARGO"));
        cmd.arg("metadata");
        cmd.args([OsStr::new("--manifest-path"), manifest_path.as_os_str()]);
        cmd.output_json()?
    };

    let pkg = metadata
        .packages
        .into_iter()
        .find(|p| p.name == "advent-bins")
        .unwrap();

    let targets: Vec<_> = pkg
        .targets
        .into_iter()
        .filter(|target| target.name.chars().all(|c| matches!(c, '0'..='9')))
        .map(|target| target.name)
        .collect();

    let mut results = Vec::new();

    for target in &targets {
        eprintln!();
        let Summary { benches } = {
            let mut cmd = Command::new(env!("CARGO"));
            cmd.stderr(Stdio::inherit());
            cmd.arg("run");
            cmd.args([OsStr::new("--manifest-path"), manifest_path.as_os_str()]);
            cmd.args(&["--bin", &target, "--release", "--features", "json", "--"]);
            cmd.args(&["--bench", "--output", "json"]);
            cmd.output_json()?
        };

        let year = target[..4].parse().unwrap();
        let day = target[4..].parse().unwrap();

        results.push(SummaryOut { year, day, benches });
    }

    let f = fs::File::create("benches.json")?;
    serde_json::to_writer(f, &results)?;

    Ok(())
}

fn summary_to_markdown(summary: &[SummaryOut]) -> Result<()> {
    let mut engine = upon::Engine::new();
    engine.add_filter("find_part_1_mean", |v| {
        find_mean(v, upon::Value::from("Part 1"))
    });
    engine.add_filter("find_part_2_mean", |v| {
        find_mean(v, upon::Value::from("Part 2"))
    });

    let result = engine
        .compile(include_str!("BENCHES.md"))?
        .render(upon::value! { summaries: summary })?;

    fs::write("BENCHES.md", result)?;

    Ok(())
}

fn find_mean(v: &mut upon::Value, name: upon::Value) {
    let opt = match v {
        upon::Value::List(benches) => benches.iter().find_map(|b| match b {
            upon::Value::Map(summary) => (summary["name"] == name).then(|| summary["mean"].clone()),
            _ => panic!("expected map"),
        }),
        _ => panic!("expected list"),
    };
    *v = opt
        .map(|f| match f {
            upon::Value::Float(f) => {
                upon::Value::from(human::Time::with_scale(f, human::Scale::Milli).to_string())
            }
            _ => panic!("expected float"),
        })
        .unwrap_or(upon::Value::None);
}

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    targets: Vec<Target>,
}

#[derive(Debug, Deserialize)]
struct Target {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SummaryOut {
    year: u32,
    day: u32,
    benches: Vec<Bench>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Summary {
    benches: Vec<Bench>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bench {
    name: String,
    samples: usize,
    min: f64,
    max: f64,
    mean: f64,
    std_dev: f64,
}

pub trait CommandExt {
    /// Run the command and parse the standard output as T.
    fn output_json<T: DeserializeOwned>(&mut self) -> Result<T>;
}

impl CommandExt for Command {
    /// Run the command and parse the standard output as T.
    fn output_json<T: DeserializeOwned>(&mut self) -> Result<T> {
        let output = self
            .output()
            .with_context(|| format!("could not execute subprocess: `{:?}`", self))?;
        if !output.status.success() {
            bail!(
                "subprocess didn't exit successfully `{:?}` ({})",
                self,
                output.status
            );
        }
        Ok(serde_json::from_slice(&output.stdout).context("failed to parse stdout")?)
    }
}
