use std::fs;
use std::path::PathBuf;
use std::process;

use anyhow::{bail, Context, Result};
use argh::FromArgs;
use serde::{Deserialize, Serialize};
use some::Some;

use advent::summary::{Bench, Summary};

/// benchmark an entire year
#[derive(Debug, FromArgs)]
struct Opt {
    /// render markdown file with all the benchmarks
    #[argh(switch)]
    markdown: bool,

    /// the year to benchmark
    #[argh(option, short = 'y')]
    year: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Day {
    name: String,
    bin: String,
    benches: Vec<Bench>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
struct Binary {
    name: String,
    path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Binaries {
    bin: Vec<Binary>,
}

fn main() -> Result<()> {
    let opt: Opt = argh::from_env();

    match (opt.year, opt.markdown) {
        (None, false) => bail!("one of --year or --markdown required"),
        (None, true) => markdown(),
        (Some(year), false) => bench(year),
        (Some(_), true) => bail!("can't provide both --year and --markdown"),
    }
}

fn bench(year: u32) -> Result<()> {
    let workspace_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let manifest_path = workspace_dir.join("Cargo.toml");

    let manifest = fs::read_to_string(&manifest_path)?;
    let index = manifest.find("[[bin]]").unwrap();
    let bins: Binaries = toml::from_str(&manifest[index..]).context("failed to load manifest")?;
    let year = format!("{:04}", year);
    let bins: Vec<_> = bins
        .bin
        .into_iter()
        .filter_map(|bin| {
            let ok = bin.name.starts_with(&year) && bin.name.len() == 6;
            ok.some(bin.name)
        })
        .collect();

    let mut days = Vec::new();

    for bin in bins.into_iter() {
        eprintln!("Benchmarking {bin}");
        let output = process::Command::new(env!("CARGO"))
            .args(["run", "--release", "--bin", &bin, "--features=json"])
            .args(["--", "--bench", "--output", "json"])
            .output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("process did not exit successfully: {stderr}");
        }
        let summary: Summary = serde_json::from_slice(&output.stdout).with_context(|| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            format!("failed to load JSON from {stdout}",)
        })?;
        let benches = match summary {
            Summary::Bench(benches) => benches,
            _ => unreachable!(),
        };
        eprintln!(" {:?}", benches);
        let name = bin[4..6].to_owned();
        days.push(Day { name, bin, benches });
    }

    let path = workspace_dir.join(year).join("benches.json");
    fs::write(path, serde_json::to_string(&days)?)?;

    Ok(())
}

fn markdown() -> Result<()> {
    let workspace_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));

    #[derive(Debug, Serialize)]
    struct ContextYear {
        name: String,
        days: Vec<ContextDay>,
    }

    #[derive(Debug, Serialize)]
    struct ContextDay {
        name: String,
        parse: Option<String>,
        part1: Option<String>,
        part2: Option<String>,
    }

    let mut years = Vec::new();

    for year in 2019..=2021 {
        let name = format!("{:04}", year);
        let path = workspace_dir.join(&name).join("benches.json");

        let s = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let days: Vec<Day> = serde_json::from_str(&s)?;

        let format_time = |b: &Bench| {
            if b.data.mean > 0.5 {
                format!("**{}**", b.human.mean)
            } else {
                format!("{}", b.human.mean)
            }
        };

        let days = days
            .into_iter()
            .map(|day| {
                let parse = day
                    .benches
                    .iter()
                    .find(|b| b.name == "Parse")
                    .map(format_time);
                let part1 = day
                    .benches
                    .iter()
                    .find(|b| b.name == "Part 1")
                    .map(format_time);
                let part2 = day
                    .benches
                    .iter()
                    .find(|b| b.name == "Part 2")
                    .map(format_time);

                ContextDay {
                    name: day.name,
                    parse,
                    part1,
                    part2,
                }
            })
            .collect();

        years.push(ContextYear { name, days });
    }

    let output = upon::Engine::new()
        .compile(include_str!("BENCHES_TEMPLATE.md"))?
        .render(upon::value! { years: years })
        .map_err(|err| anyhow::anyhow!("{:?}", err))?;

    fs::write(workspace_dir.join("BENCHES.md"), output)?;

    Ok(())
}
