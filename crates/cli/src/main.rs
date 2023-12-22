use std::env;
use std::fmt::Display;
use std::fs;
use std::io;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process;

use anyhow::{Context, Result};
use argh::FromArgs;
use serde::{Deserialize, Serialize};
use yansi::Paint;

/// 🎄 Festive Advent of Code solution management
#[derive(Debug, FromArgs)]
#[argh(example = "cargo advent -y 2021 -d 17 run")]
struct Opt {
    /// the puzzle year
    #[argh(option, short = 'y')]
    year: u32,

    /// the puzzle day
    #[argh(option, short = 'd')]
    day: u32,

    /// the subcommand: bench, new, open, or a Cargo subcommand
    #[argh(positional)]
    command: Command,

    #[argh(positional, greedy)]
    args: Vec<String>,
}

#[derive(Debug)]
enum Command {
    Bench,
    New,
    Open,
    Cargo(String),
}

impl argh::FromArgValue for Command {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "new" => Ok(Self::New),
            "open" => Ok(Self::Open),
            "bench" => Ok(Self::Bench),
            "check" | "build" | "test" | "run" | "clippy" => Ok(Self::Cargo(value.into())),
            _ => Err("expected one of: bench, new, open, or a Cargo subcommand".into()),
        }
    }
}

fn main() -> Result<()> {
    let Opt {
        year,
        day,
        command,
        args,
    } = argh::from_env();

    match command {
        Command::Bench => bench(year, day, &args),
        Command::New => new(year, day),
        Command::Open => open(year, day, &args),
        Command::Cargo(cmd) => cargo(cmd, year, day, &args),
    }
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

fn download(url: &str) -> Result<String> {
    let mut buf = Vec::new();
    let mut easy = curl::easy::Easy::new();
    easy.fail_on_error(true)?;
    easy.follow_location(true)?;
    easy.useragent("github.com/rossmacarthur/advent by ross@macarthur.io")?;
    easy.cookie(&format!(
        "session={}",
        env::var("ADVENT_SESSION").context("`ADVENT_SESSION` must be set")?
    ))?;
    easy.url(url)?;
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    Ok(String::from_utf8(buf)?)
}

fn new(year: u32, day: u32) -> Result<()> {
    let bin_name = format!("{year:04}{day:02}");

    let workspace_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let manifest_path = workspace_dir.join("Cargo.toml");
    let bin = workspace_dir.join(format!("{year:04}/{day:02}.rs"));

    // Add {year}/{day}.rs file
    const TEMPLATE: &str = include_str!("template.rs");
    let bin_display = bin.strip_prefix(&workspace_dir).unwrap_or(&bin).display();
    if bin.exists() {
        print("Checked", format!("binary source `{bin_display}`"));
    } else {
        let rendered = TEMPLATE
            .replace("{ year }", &format!("{year:04}"))
            .replace("{ day }", &format!("{day:02}"));
        fs::create_dir_all(bin.parent().unwrap())?;
        fs::write(&bin, rendered)?;
        print(
            "Created",
            format!("binary source `{bin_display}` from template"),
        );
    }

    // Update Cargo.toml
    let manifest = fs::read_to_string(&manifest_path)?;
    let index = manifest.find("[[bin]]").unwrap();
    let (main, binaries) = manifest.split_at(index);
    let mut bins: Binaries = toml::from_str(binaries)?;
    let to_add = Binary {
        name: bin_name.clone(),
        path: bin.strip_prefix(&workspace_dir)?.to_owned(),
    };
    let added = !bins.bin.contains(&to_add);
    bins.bin.push(to_add);
    bins.bin.sort();
    bins.bin.dedup();
    let binaries = toml::to_string(&bins)?;
    fs::write(&manifest_path, main.to_owned() + &binaries)?;

    if added {
        print(
            "Updated",
            format!("Cargo manifest with `{bin_name}` binary"),
        );
    } else {
        print(
            "Checked",
            format!("Cargo manifest contains `{bin_name}` binary"),
        );
    }

    check_input(year, day)?;

    Ok(())
}

fn open(year: u32, day: u32, args: &[String]) -> Result<()> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    let browser = args.get(0).map(|s| s.as_str()).unwrap_or("firefox");
    open::with(url, browser)?;
    Ok(())
}

fn check_input(year: u32, day: u32) -> Result<()> {
    // Only try download if the puzzle has actually been released
    let now = time::OffsetDateTime::now_utc();
    let puzzle = time::PrimitiveDateTime::new(
        time::Date::from_calendar_date(year as i32, time::Month::December, day as u8)?,
        time::Time::from_hms(5, 0, 0)?,
    )
    .assume_utc();
    if now < puzzle {
        warning(
            "Unavailable",
            format!("puzzle input (year: {year:04}, day: {day:02})"),
        );
        return Ok(());
    }

    // Calculate input file path
    let workspace_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let input = workspace_dir.join(format!("input/{year:04}/{day:02}.txt"));

    // Create directory if not exists
    fs::create_dir_all(input.parent().unwrap())?;

    let input_display = input
        .strip_prefix(&workspace_dir)
        .unwrap_or(&input)
        .display();

    if !input.exists() {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let text = download(&url)?;
        fs::write(&input, text)?;
        print("Downloaded", format!("puzzle input `{input_display}`"));
    } else {
        print("Verified", format!("puzzle input `{input_display}`"));
    }

    Ok(())
}

fn bench(year: u32, day: u32, args: &[String]) -> Result<()> {
    check_input(year, day)?;

    let bin_name = format!("{year:04}{day:02}");

    let (cargo_args, bin_args) = match args.iter().position(|a| a == "--") {
        Some(i) => (&args[..i], &args[i + 1..]),
        None => (args, &[][..]),
    };

    let status = process::Command::new(env!("CARGO"))
        .args(["run", "--release", "--bin", &bin_name])
        .args(cargo_args)
        .args(["--", "--bench"])
        .args(bin_args)
        .status()?;

    process::exit(status.code().unwrap())
}

fn cargo(cmd: String, year: u32, day: u32, args: &[String]) -> Result<()> {
    check_input(year, day)?;

    let bin_name = format!("{year:04}{day:02}");

    let status = process::Command::new(env!("CARGO"))
        .args([&cmd, "--release", "--bin", &bin_name])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap())
}

fn print(header: &str, message: impl Display) {
    if io::stdout().is_terminal() {
        println!("{:>12} {}", Paint::new(&header).bold().green(), message);
    } else {
        println!("{:>12} {}", header, message);
    }
}

fn warning(header: &str, message: impl Display) {
    if io::stdout().is_terminal() {
        println!("{:>12} {}", Paint::new(&header).bold().yellow(), message);
    } else {
        println!("{:>12} {}", header, message);
    }
}
