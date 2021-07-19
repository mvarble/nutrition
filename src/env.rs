//! Mount the environment.

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Environment {
    pub nutritionix_app_id: String,
    pub nutritionix_app_key: String,
}

struct PartialEnvironment {
    nutritionix_app_id: Option<String>,
    nutritionix_app_key: Option<String>,
}

pub fn get() -> Result<Environment> {
    dotenv::dotenv().ok();
    let penv = PartialEnvironment {
        nutritionix_app_id: None,
        nutritionix_app_key: None,
    };
    let penv = std::env::vars().fold(penv, |penv, (key, value)| {
        if key == "NUTRITIONIX_APP_ID" {
            PartialEnvironment {
                nutritionix_app_id: Some(value),
                ..penv
            }
        } else if key == "NUTRITIONIX_APP_KEY" {
            PartialEnvironment {
                nutritionix_app_key: Some(value),
                ..penv
            }
        } else {
            penv
        }
    });
    if penv.nutritionix_app_id.is_none() {
        Err(anyhow!("Environment needs NUTRITIONIX_APP_ID value"))
    } else if penv.nutritionix_app_key.is_none() {
        Err(anyhow!("Environment needs NUTRITIONIX_APP_KEY value"))
    } else {
        Ok(Environment {
            nutritionix_app_id: penv.nutritionix_app_id.unwrap(),
            nutritionix_app_key: penv.nutritionix_app_key.unwrap(),
        })
    }
}
