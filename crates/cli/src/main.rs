use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{AppSettings, Clap};
use serde::{Deserialize, Serialize};

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
    let bin = PathBuf::from(format!("{:04}/{:02}.rs", year, day));
    let input = PathBuf::from(format!("{:04}/input/{:02}.txt", year, day));

    // Download input
    if input.exists() {
        println!("{} already exists", input.display());
    } else {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let text = download(&url)?;
        fs::write(&input, text)?;
        println!("Downloaded input to {}", input.display());
    }

    // Add {year}/{day}.rs file
    const TEMPLATE: &str = include_str!("template.rs");
    if bin.exists() {
        println!("{} already exists", bin.display());
    } else {
        fs::write(&bin, TEMPLATE.replace("{day}", &format!("{:02}", day)))?;
        println!("Created {}", bin.display());
    }

    // Update Cargo.toml
    let manifest = fs::read_to_string("Cargo.toml")?;
    let index = manifest.find("[[bin]]").unwrap();
    let (main, binaries) = manifest.split_at(index);
    let mut bins: Binaries = toml::from_str(binaries)?;
    let to_add = Binary {
        name: name.clone(),
        path: bin,
    };
    let added = !bins.bin.contains(&to_add);
    bins.bin.push(to_add);
    bins.bin.sort();
    bins.bin.dedup();
    let binaries = toml::to_string(&bins)?;
    fs::write("Cargo.toml", main.to_owned() + &binaries)?;
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
    open::that(&url)?;
    Ok(())
}

#[derive(Debug, Clap)]
#[clap(
    author,
    about,
    global_setting = AppSettings::DeriveDisplayOrder,
    global_setting = AppSettings::DisableHelpSubcommand,
    global_setting = AppSettings::GlobalVersion,
    global_setting = AppSettings::VersionlessSubcommands,
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
enum Opt {
    /// Create a new blank template for the given day and year.
    New {
        #[clap(long, short, value_name = "YEAR")]
        year: u32,

        #[clap(long, short, value_name = "DAY")]
        day: u32,
    },

    /// Open the browser for the given problem.
    Open {
        #[clap(long, short, value_name = "YEAR")]
        year: u32,

        #[clap(long, short, value_name = "DAY")]
        day: u32,
    },
}

fn main() -> Result<()> {
    match Opt::parse() {
        Opt::New { year, day } => new(year, day),
        Opt::Open { year, day } => open(year, day),
    }
}
