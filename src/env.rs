//! Mount the environment.

use anyhow::{anyhow, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Environment {
    pub database_url: String,
    pub database_name: String,
    pub ui_dir: PathBuf,
}

struct PartialEnvironment {
    database_url: Option<String>,
    database_name: Option<String>,
    ui_dir: Option<PathBuf>,
}

pub fn get() -> Result<Environment> {
    dotenv::dotenv().ok();
    let penv = PartialEnvironment {
        database_url: None,
        database_name: None,
        ui_dir: None,
    };
    let penv = std::env::vars().fold(penv, |penv, (key, value)| {
        if key == "DATABASE_URL" {
            PartialEnvironment {
                database_url: Some(value),
                ..penv
            }
        } else if key == "DATABASE_NAME" {
            PartialEnvironment {
                database_name: Some(value),
                ..penv
            }
        } else if key == "UI_DIR" {
            PartialEnvironment {
                ui_dir: value.parse().ok(),
                ..penv
            }
        } else {
            penv
        }
    });
    if penv.database_url.is_none() {
        Err(anyhow!("Environment needs DATABASE_URL value"))
    } else if penv.database_name.is_none() {
        Err(anyhow!("Environment needs DATABASE_NAME value"))
    } else if penv.ui_dir.is_none() {
        Err(anyhow!("Environment needs UI_DIR path-value"))
    } else {
        Ok(Environment {
            database_url: penv.database_url.unwrap(),
            database_name: penv.database_name.unwrap(),
            ui_dir: penv.ui_dir.unwrap(),
        })
    }
}
