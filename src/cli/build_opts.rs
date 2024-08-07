use std::{
    collections::BTreeSet,
    env,
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use glob::glob;

use crate::run_bundle;

use super::CmdExector;

#[derive(Debug, Parser)]
pub struct BuildOpts {}

impl CmdExector for BuildOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let path = env::current_dir()?;
        build_project(&path)?;
        Ok(())
    }
}

const BUILD_DIR_NAME: &str = "build";
const ENTRY_FILE_NAME: &str = "main.ts";
const EXTS: [&str; 3] = ["ts", "js", "json"];

fn build_project(path: &Path) -> Result<String> {
    let build_path = path.join(BUILD_DIR_NAME);
    if !build_path.exists() || !build_path.is_dir() {
        fs::create_dir_all(&build_path)?;
    }

    let main_ts = path.join(ENTRY_FILE_NAME);
    let build_file_name = generate_build_file_name(path, &build_path)?;
    let build_file = build_path.join(build_file_name);

    // if the file already exists, skip building
    if build_file.exists() {
        return Ok(build_file.display().to_string());
    }

    fs::write(
        &build_file,
        run_bundle(&main_ts.display().to_string(), &Default::default())?,
    )?;
    Ok(build_file.display().to_string())
}

fn generate_build_file_name(path: &Path, build_path: &Path) -> Result<String> {
    let mut files: BTreeSet<PathBuf> = BTreeSet::new();
    for ext in EXTS {
        let tmps: BTreeSet<_> = glob(&format!("{}/**/*.{ext}", path.display()))?
            .filter_map(|f| f.ok())
            .filter(|f| !f.starts_with(build_path.display().to_string()))
            .collect();
        files.extend(tmps);
    }
    let mut hasher = blake3::Hasher::new();
    for file in files {
        hasher.update_reader(File::open(file)?)?;
    }
    let hash = format!("{}.js", hasher.finalize().to_string());
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;

    use super::build_project;

    #[test]
    fn build_project_should_work() -> Result<()> {
        let build = build_project(Path::new("/root/workspace/rust/dino/demo"))?;
        println!("{build}");
        Ok(())
    }
}
