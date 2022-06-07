mod bench;

use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use argh::FromArgs;
use serde::{Deserialize, Serialize};

use crate::bench::bench;

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
    let name = format!("{:04}{:02}", year, day);

    let workspace_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let manifest_path = workspace_dir.join("Cargo.toml");

    // Calculate bin and input file paths.
    let bin = workspace_dir
        .join(format!("{:04}", year))
        .join(format!("{:02}.rs", day));
    let input = workspace_dir
        .join(format!("{:04}", year))
        .join("input")
        .join(format!("{:02}.txt", day));

    // Create directory if not exists
    fs::create_dir_all(input.parent().unwrap())?;

    // Download input
    if input.exists() {
        println!(
            "{} already exists",
            input.strip_prefix(&workspace_dir)?.display()
        );
    } else {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let text = download(&url)?;
        fs::write(&input, text)?;
        println!(
            "Downloaded input to {}",
            input.strip_prefix(&workspace_dir)?.display()
        );
    }

    // Add {year}/{day}.rs file
    const TEMPLATE: &str = include_str!("template.rs");
    if bin.exists() {
        println!(
            "{} already exists",
            bin.strip_prefix(&workspace_dir)?.display()
        );
    } else {
        fs::write(&bin, TEMPLATE.replace("{day}", &format!("{:02}", day)))?;
        println!("Created {}", bin.strip_prefix(&workspace_dir)?.display());
    }

    // Update Cargo.toml
    let manifest = fs::read_to_string(&manifest_path)?;
    let index = manifest.find("[[bin]]").unwrap();
    let (main, binaries) = manifest.split_at(index);
    let mut bins: Binaries = toml::from_str(binaries)?;
    let to_add = Binary {
        name: name.clone(),
        path: bin.strip_prefix(&workspace_dir)?.to_owned(),
    };
    let added = !bins.bin.contains(&to_add);
    bins.bin.push(to_add);
    bins.bin.sort();
    bins.bin.dedup();
    let binaries = toml::to_string(&bins)?;
    fs::write(&manifest_path, main.to_owned() + &binaries)?;
    if added {
        println!("Added {} binary to Cargo manifest", name);
    } else {
        println!("{} binary already exists in Cargo manifest", name);
    }

    println!("All done!");
    println!("Use `cargo run --release --bin {}` to run", name);

    Ok(())
}

fn open(year: u32, day: u32) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    open::with(&url, "firefox")?;
    Ok(())
}

/// Add a puzzle template or open the puzzle description.
#[derive(Debug, FromArgs)]
struct Opt {
    /// the puzzle year.
    #[argh(option, short = 'y')]
    year: Option<u32>,

    /// the puzzle day.
    #[argh(option, short = 'd')]
    day: Option<u32>,

    /// the command.
    #[argh(positional, arg_name = "new|open|bench")]
    command: Command,
}

#[derive(Debug)]
enum Command {
    New,
    Open,
    Bench,
}

impl argh::FromArgValue for Command {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "new" => Ok(Self::New),
            "open" => Ok(Self::Open),
            "bench" => Ok(Self::Bench),
            _ => Err("expected `new`,`open`, or `bench`".into()),
        }
    }
}

fn main() -> Result<()> {
    let Opt { year, day, command } = argh::from_env();
    match command {
        Command::New => {
            let year = year.context("`year` is required")?;
            let day = day.context("`day` is required")?;
            new(year, day)
        }
        Command::Open => {
            let year = year.context("`year` is required")?;
            let day = day.context("`day` is required")?;
            open(year, day)
        }
        Command::Bench => bench(),
    }
}
