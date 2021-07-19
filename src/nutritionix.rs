//! Interact with the [Nutritionix](https://trackapi.nutritionix.com/docs/) API.

use anyhow::Result;

/// The Nutritionix API returns a JSON payload of the form `{ foods: [...] }`, consisting of an array of
/// json values like `NutritionixFood`.
#[derive(Deserialize, Debug, PartialEq)]
pub struct NutritionixFood {
    pub food_name: String,
    pub brand_name: Option<String>,
    pub serving_qty: f32,
    pub serving_unit: String,
    pub serving_weight_grams: f32,
    pub full_nutrients: Vec<NutritionixNutrient>,
    pub upc: Option<String>,
    pub alt_measures: Option<Vec<NutritionixMeasure>>,
}

impl NutritionixFood {
    pub fn update_upc(&mut self, upc: String) {
        self.upc = Some(upc);
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct NutritionixNutrient {
    pub attr_id: i32,
    pub value: f32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct NutritionixMeasure {
    pub serving_weight: f32,
    pub measure: String,
    pub qty: f32,
}

/// This struct implements methods which make requests to the Nutritionix API.
#[derive(Debug)]
pub struct NutritionixService {
    pub app_id: String,
    pub app_key: String,
    pub client: reqwest::Client,
}

impl NutritionixService {
    pub fn new<S: Into<String>>(app_id: S, app_key: S) -> NutritionixService {
        NutritionixService {
            app_id: app_id.into(),
            app_key: app_key.into(),
            client: reqwest::Client::new(),
        }
    }

    /// Request a Nutritionix payload via the `/v2/natural/nutrients` endpoint
    pub async fn request_natural(&self, query: &str) -> Result<Vec<NutritionixFood>> {
        let body = serde_json::json!({ "query": query });
        let mut res = self
            .client
            .post("https://trackapi.nutritionix.com/v2/natural/nutrients")
            .json(&body)
            .header("x-app-id", &self.app_id)
            .header("x-app-key", &self.app_key)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(serde_json::from_value(res["foods"].take())?)
    }

    /// Request a Nutritionix payload via the `/v2/search/item` endpoint
    pub async fn request_upc(&self, upc: &str) -> Result<Vec<NutritionixFood>> {
        let url = "https://trackapi.nutritionix.com/v2/search/item";
        let url = format!(
            "{}?x-app-id={}&x-app-key={}&upc={}",
            url, self.app_id, self.app_key, upc
        );
        let mut res = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        let mut foods: Vec<NutritionixFood> = serde_json::from_value(res["foods"].take())?;
        foods
            .iter_mut()
            .for_each(|food| food.update_upc(upc.into()));
        Ok(foods)
    }
}

#[cfg(test)]
mod test {
    use crate::env;
    use crate::nutritionix::NutritionixService;

    fn get_service() -> NutritionixService {
        let environment = env::get().unwrap();
        NutritionixService::new(
            environment.nutritionix_app_id,
            environment.nutritionix_app_key,
        )
    }

    #[tokio::test]
    #[ignore]
    async fn request_natural() {
        let nixservice = get_service();
        let res = nixservice.request_natural("1 egg, 1 cup of spinach").await;
        if res.is_err() {
            panic!("A natural request `1 egg, 1 cup of spinach` did not work!");
        }
        res.unwrap().iter().for_each(|food| {
            if food.food_name == "egg" {
                assert_eq!(food.serving_weight_grams, 50.0);
                assert!(!food.full_nutrients.is_empty());
                assert!(
                    food.alt_measures.is_some() && !food.alt_measures.as_ref().unwrap().is_empty()
                );
            } else if food.food_name == "spinach" {
                assert_eq!(food.serving_unit, "cup");
                assert_eq!(food.serving_weight_grams, 180.0);
                assert!(food.full_nutrients.len() > 0);
                assert!(
                    food.alt_measures.is_some() && !food.alt_measures.as_ref().unwrap().is_empty()
                );
            } else {
            }
        })
    }

    #[tokio::test]
    #[ignore]
    async fn request_upc() {
        let nixservice = get_service();
        let res = nixservice.request_upc("024463061071").await;
        if res.is_err() {
            panic!("A upc request `024463061071` did not work!");
        }
        let res = res.unwrap();
        let item = res.get(0);
        if item.is_none() {
            panic!("A upc request `024463061071` did not work!");
        }
        let item = item.unwrap();
        assert_eq!(item.upc, Some("024463061071".to_string()));
        assert_eq!(
            item.food_name,
            "Chili  Paste, Chili Paste, Ground Fresh, Sambal Oelek"
        );
        assert_eq!(item.brand_name, Some("Huy Fong Foods".to_string()));
        assert_eq!(item.serving_weight_grams, 5.0);
    }
}
