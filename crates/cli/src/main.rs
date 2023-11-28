use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use anyhow::{Context, Result};
use argh::FromArgs;
use serde::{Deserialize, Serialize};

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

    /// the subcommand: bench, new, open, run, or test
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
    Run,
    Test,
}

impl argh::FromArgValue for Command {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "bench" => Ok(Self::Bench),
            "new" => Ok(Self::New),
            "open" => Ok(Self::Open),
            "run" => Ok(Self::Run),
            "test" => Ok(Self::Test),
            _ => Err("expected one of: bench, new, open, run, test".into()),
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
        Command::Run => run(year, day, &args),
        Command::Test => test(year, day, &args),
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

    // Calculate bin and input file paths.
    let bin = workspace_dir.join(format!("{year:04}/{day:02}.rs"));
    let input = workspace_dir.join(format!("{year:04}/input/{day:02}.txt"));
    // Create directory if not exists
    fs::create_dir_all(input.parent().unwrap())?;

    // Download input
    let input_display = input
        .strip_prefix(&workspace_dir)
        .unwrap_or(&input)
        .display();
    if input.exists() {
        println!("• {input_display} already exists");
    } else {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let text = download(&url)?;
        fs::write(&input, text)?;
        println!("• {input_display} was downloaded");
    }

    // Add {year}/{day}.rs file
    const TEMPLATE: &str = include_str!("template.rs");
    let bin_display = bin.strip_prefix(&workspace_dir).unwrap_or(&bin).display();
    if bin.exists() {
        println!("• {bin_display} already exists");
    } else {
        let rendered = TEMPLATE
            .replace("{ year }", &format!("{year:04}"))
            .replace("{ day }", &format!("{day:02}"));
        fs::write(&bin, rendered)?;
        println!("• {bin_display} was created");
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
        println!("• {bin_name} binary added to Cargo manifest");
    } else {
        println!("• {bin_name} binary already exists in Cargo manifest");
    }

    println!("All done! Use `cargo advent -y {year} -d {day} run` to run");

    Ok(())
}

fn open(year: u32, day: u32, args: &[String]) -> Result<()> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    let browser = args.get(0).map(|s| s.as_str()).unwrap_or("firefox");
    open::with(url, browser)?;
    Ok(())
}

fn bench(year: u32, day: u32, args: &[String]) -> Result<()> {
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

fn run(year: u32, day: u32, args: &[String]) -> Result<()> {
    let bin_name = format!("{year:04}{day:02}");

    let status = process::Command::new(env!("CARGO"))
        .args(["run", "--release", "--bin", &bin_name])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap())
}

fn test(year: u32, day: u32, args: &[String]) -> Result<()> {
    let bin_name = format!("{year:04}{day:02}");

    let status = process::Command::new(env!("CARGO"))
        .args(["test", "--release", "--bin", &bin_name])
        .args(args)
        .status()?;

    process::exit(status.code().unwrap())
}
