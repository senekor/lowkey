use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

fn check_that_cwd_is_git_repo() -> Result<()> {
    let current_working_direcory =
        std::env::current_dir().context("Failed to get current directory")?;
    let git_dir_path = current_working_direcory.join(".git");
    if !git_dir_path.exists() {
        anyhow::bail!("Failed to find valid '.git' directory.\nlowkey must be run from the root of the repository.");
    }
    Ok(())
}

#[derive(Debug, Default, Serialize, Deserialize)]
enum JobsConfig {
    #[default]
    None,
    ScriptsDir(PathBuf),
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    jobs: JobsConfig,
}

fn get_config() -> Result<Config> {
    let config_string = std::fs::read_to_string("./config/lowkey.toml")
        .context("Failed to read the configuration file (config/lowkey.toml).")?;
    toml::from_str(&config_string).context("Failed to deserialize config file.")
}

fn main() -> Result<()> {
    check_that_cwd_is_git_repo()?;

    let config = get_config()?;
    match config.jobs {
        JobsConfig::None => println!("Nothing to do. Add some jobs to your config."),
        JobsConfig::ScriptsDir(dir) => {
            let entries = std::fs::read_dir(dir).context("Failed to read scripts directory.")?;
            let mut failed = false;
            for entry in entries {
                let entry = entry.context("Failed to read scripts directory entry.")?;
                let path = entry.path();
                let output = std::process::Command::new(&path)
                    .output()
                    .context("Failed to run job.")?;
                let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("?");

                if output.status.success() {
                    println!("✅ {}", name);
                } else {
                    println!("❌ {}", name);
                    failed = true;
                }
            }
            if failed {
                anyhow::bail!("Some jobs were not successful.");
            }
        }
    }

    Ok(())
}
