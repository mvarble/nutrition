//! Mount the environment.

use anyhow::{anyhow, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Environment {
    pub fdc_key: String,
}

struct PartialEnvironment {
    fdc_key: Option<String>,
}

pub fn get() -> Result<Environment> {
    dotenv::dotenv().ok();
    let penv = PartialEnvironment { fdc_key: None };
    let penv = std::env::vars().fold(penv, |penv, (key, value)| {
        if key == "FDC_KEY" {
            PartialEnvironment {
                fdc_key: Some(value),
                ..penv
            }
        } else {
            penv
        }
    });
    if penv.fdc_key.is_none() {
        Err(anyhow!("Environment needs FDC_KEY value"))
    } else {
        Ok(Environment {
            fdc_key: penv.fdc_key.unwrap(),
        })
    }
}
