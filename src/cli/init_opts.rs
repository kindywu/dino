use anyhow::Result;
use askama::Template;
use clap::Parser;
use dialoguer::Input;
use git2::Repository;
use std::{fs, path::Path};

use crate::CmdExector;

#[derive(Debug, Parser)]
pub struct InitOpts {}

impl CmdExector for InitOpts {
    async fn execute(self) -> Result<()> {
        let name: String = Input::new().with_prompt("Project name").interact_text()?;

        let current_dir = Path::new(".");
        if fs::read_dir(current_dir)?.next().is_none() {
            init_project(name, current_dir)?;
        } else {
            init_project(name.clone(), &current_dir.join(name))?;
        }
        Ok(())
    }
}

fn init_project(name: String, path: &Path) -> Result<()> {
    if !path.exists() || !path.is_dir() {
        fs::create_dir_all(path)?;
    }
    Repository::init(path)?;

    fs::write(path.join("config.yml"), ConfigFile { name }.render()?)?;
    fs::write(path.join("main.ts"), MainTsFile {}.render()?)?;
    fs::write(path.join(".gitignore"), GitIgnoreFile {}.render()?)?;

    Ok(())
}

#[derive(Template)]
#[template(path = "../templates/config.yml.j2")]
struct ConfigFile {
    name: String,
}

#[derive(Template)]
#[template(path = "../templates/main.ts.j2")]
struct MainTsFile {}

#[derive(Template)]
#[template(path = "../templates/.gitignore.j2")]
struct GitIgnoreFile {}
