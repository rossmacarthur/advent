use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Deserialize;
use syn::*;

#[derive(Debug, Clone, Deserialize)]
struct Binary {
    name: String,
    path: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct Manifest {
    bin: Vec<Binary>,
}

fn paths(ps: &mut Vec<String>, prefix: &str, tree: &UseTree) {
    match tree {
        UseTree::Path(path) => {
            let prefix = format!("{}::{}", prefix, path.ident);
            paths(ps, &prefix, &path.tree);
        }
        UseTree::Name(UseName { ident }) | UseTree::Rename(UseRename { ident, .. }) => {
            ps.push(format!("{}::{}", prefix, ident));
        }
        UseTree::Glob(_) => {
            ps.push(format!("{}::*", prefix));
        }
        UseTree::Group(UseGroup { items, .. }) => {
            for path in items {
                paths(ps, &prefix, &path);
            }
        }
    }
}

fn main() -> Result<()> {
    let workspace_dir = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let manifest_path = workspace_dir.join("Cargo.toml");
    let contents = fs::read_to_string(&manifest_path)?;
    let manifest: Manifest = toml::from_str(&contents)?;

    let mut imports = Vec::new();

    for bin in manifest.bin {
        if !bin.name.chars().all(|c| c.is_digit(10)) {
            continue;
        }
        let contents = fs::read_to_string(workspace_dir.join(&bin.path))?;
        let file = syn::parse_file(&contents)?;
        let mut ps = Vec::new();
        for item in &file.items {
            if let Item::Use(item) = item {
                paths(&mut ps, "", &item.tree);
            }
        }
        imports.push((bin.path, ps));
    }

    for (name, paths) in imports {
        let paths: Vec<_> = paths
            .iter()
            .filter(|p| p.as_str() != "::advent::prelude::*")
            .collect();
        if !paths.is_empty() {
            println!("-- {} --", name.display());
            for p in paths {
                println!("    {}", p);
            }
        }
    }

    Ok(())
}
