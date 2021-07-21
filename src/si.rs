//! Parse Nutritionix servings to SI units.
//!
//! If we have a `NutritionixMeasure`, it may tell us something about the mass density of the food
//! or nominal servings that are not based off of physical units.
//! This module is designed to extract this information for us by providing a `MeasurementType`
//! enum and a converter.
//!
//! # Example
//! ```
//! use uom::si::{
//!    f32::{Mass, Volume},
//!    mass::gram,
//!    volume::cup,
//! };
//!
//! use nutrition::{
//!     nutritionix::NutritionixMeasure,
//!     si::MeasurementType,
//! };
//!
//! // this measure tells us about the mass density of the relevant food
//! let nixmeasure = NutritionixMeasure {
//!     serving_weight: 40.0,
//!     measure: "cup".to_string(),
//!     qty: 1.0
//! };
//! assert_eq!(
//!     MeasurementType::from(nixmeasure),
//!     MeasurementType::MassDensity(Mass::new::<gram>(40.0) / Volume::new::<cup>(1.0)),
//! );
//! // this measure doesn't really tell us anything new
//! let nixmeasure = NutritionixMeasure {
//!     serving_weight: 100.0,
//!     measure: "g".to_string(),
//!     qty: 100.0
//! };
//! assert_eq!(
//!     MeasurementType::from(nixmeasure),
//!     MeasurementType::Masslike,
//! );
//! // this measure tells us about a nominal serving that is not based on SI units
//! let nixmeasure = NutritionixMeasure {
//!     serving_weight: 15.75,
//!     measure: "packet".to_string(),
//!     qty: 1.0
//! };
//! assert_eq!(
//!     MeasurementType::from(nixmeasure),
//!     MeasurementType::Nominal("packet".to_string(), Mass::new::<gram>(15.75)),
//! );
//! ```
use uom::si::{
    f32::{Mass, MassDensity, Volume},
    mass::gram,
    volume::{
        cubic_inch, cup, fluid_ounce, gallon, liter, milliliter, pint_liquid, quart_liquid,
        tablespoon, teaspoon,
    },
};

#[derive(Debug, PartialEq)]
pub enum MeasurementType {
    Nominal(String, Mass),
    MassDensity(MassDensity),
    Masslike,
}

impl MeasurementType {
    fn is_masslike(string: &str) -> bool {
        string == "gram"
            || string == "kilogram"
            || string == "pound"
            || string == "ounce"
            || string == "wt. oz"
            || string == "oz"
            || string == "lb"
            || string == "g"
            || string == "kg"
    }
}

use crate::nutritionix::NutritionixMeasure;
impl From<NutritionixMeasure> for MeasurementType {
    fn from(nixmeasure: NutritionixMeasure) -> Self {
        let name = nixmeasure.measure;
        let mass = nixmeasure.serving_weight;
        let qty = nixmeasure.qty;
        if MeasurementType::is_masslike(&name) {
            MeasurementType::Masslike
        } else if name == "ml" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<milliliter>(qty))
        } else if name == "liter" || name == "l" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<liter>(qty))
        } else if name.contains("cup") {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<cup>(qty))
        } else if name == "pint" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<pint_liquid>(qty))
        } else if name == "quart" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<quart_liquid>(qty))
        } else if name == "gallon" || name == "gal" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<gallon>(qty))
        } else if name == "tablespoon" || name == "tbsp" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<tablespoon>(qty))
        } else if name == "teaspoon" || name == "tsp" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<teaspoon>(qty))
        } else if name == "cubic inch" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<cubic_inch>(qty))
        } else if name == "fl oz" {
            MeasurementType::MassDensity(Mass::new::<gram>(mass) / Volume::new::<fluid_ounce>(qty))
        } else if qty > 1.0 {
            MeasurementType::Nominal(format!("{} {}", qty as i32, name), Mass::new::<gram>(mass))
        } else {
            MeasurementType::Nominal(name, Mass::new::<gram>(mass))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::nutritionix::NutritionixFood;
    use crate::si::MeasurementType;
    use anyhow::Result;
    use uom::si::{
        f32::{Mass, Volume},
        mass::gram,
        volume::{cup, fluid_ounce, quart_liquid, tablespoon, teaspoon},
    };

    pub fn get_nutritionix_payload() -> Result<Vec<NutritionixFood>> {
        // open a file handle (assumes we are running at project root)
        let cwd = std::env::current_dir()?;
        let path = cwd.join("assets").join("nutritionix-example.json");
        let mut file = std::fs::File::open(path)?;

        // write file to string
        use std::io::Read;
        let mut string = String::new();
        file.read_to_string(&mut string)?;

        // parse string to Vec<NutritionixFood>
        let mut json: serde_json::Value = serde_json::from_str(&string)?;
        let foods = json["foods"].take();
        Ok(serde_json::from_value(foods)?)
    }

    #[test]
    fn measurement_type() {
        let foods = get_nutritionix_payload().unwrap();
        let mtypes = vec![
            MeasurementType::Nominal("small".to_string(), Mass::new::<gram>(38.0)),
            MeasurementType::Nominal("medium".to_string(), Mass::new::<gram>(44.0)),
            MeasurementType::Nominal("large".to_string(), Mass::new::<gram>(50.0)),
            MeasurementType::Nominal("jumbo".to_string(), Mass::new::<gram>(63.0)),
            MeasurementType::Nominal("extra large".to_string(), Mass::new::<gram>(56.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(243.0) / Volume::new::<cup>(1.0)),
            MeasurementType::Masslike,
            MeasurementType::Masslike,
            MeasurementType::Nominal("packet".to_string(), Mass::new::<gram>(10.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(220.0) / Volume::new::<cup>(1.0)),
            MeasurementType::Masslike,
            MeasurementType::Masslike,
            MeasurementType::MassDensity(Mass::new::<gram>(4.58) / Volume::new::<teaspoon>(1.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(13.75) / Volume::new::<tablespoon>(1.0)),
            MeasurementType::Nominal("packet (0.5 oz)".to_string(), Mass::new::<gram>(14.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(20.0) / Volume::new::<tablespoon>(1.0)),
            MeasurementType::Masslike,
            MeasurementType::Masslike,
            MeasurementType::MassDensity(Mass::new::<gram>(320.0) / Volume::new::<cup>(1.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(6.67) / Volume::new::<teaspoon>(1.0)),
            MeasurementType::Nominal("slice".to_string(), Mass::new::<gram>(25.0)),
            MeasurementType::Masslike,
            MeasurementType::Masslike,
            MeasurementType::Nominal("slice".to_string(), Mass::new::<gram>(11.5)),
            MeasurementType::Nominal("side".to_string(), Mass::new::<gram>(34.5)),
            MeasurementType::Nominal("3 slices".to_string(), Mass::new::<gram>(34.5)),
            MeasurementType::MassDensity(Mass::new::<gram>(80.0) / Volume::new::<cup>(1.0)),
            MeasurementType::Masslike,
            MeasurementType::Masslike,
            MeasurementType::MassDensity(Mass::new::<gram>(1.67) / Volume::new::<teaspoon>(1.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(5.0) / Volume::new::<tablespoon>(1.0)),
            MeasurementType::MassDensity(
                Mass::new::<gram>(980.0) / Volume::new::<quart_liquid>(1.0),
            ),
            MeasurementType::MassDensity(Mass::new::<gram>(245.0) / Volume::new::<cup>(1.0)),
            MeasurementType::Masslike,
            MeasurementType::Masslike,
            MeasurementType::MassDensity(Mass::new::<gram>(5.1) / Volume::new::<teaspoon>(1.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(15.31) / Volume::new::<tablespoon>(1.0)),
            MeasurementType::Nominal("fruit yields".to_string(), Mass::new::<gram>(86.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(31.0) / Volume::new::<fluid_ounce>(1.0)),
            MeasurementType::Masslike,
            MeasurementType::MassDensity(Mass::new::<gram>(248.0) / Volume::new::<cup>(1.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(5.17) / Volume::new::<teaspoon>(1.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(15.5) / Volume::new::<tablespoon>(1.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(180.0) / Volume::new::<cup>(1.0)),
            MeasurementType::Masslike,
            MeasurementType::Masslike,
            MeasurementType::MassDensity(Mass::new::<gram>(3.75) / Volume::new::<teaspoon>(1.0)),
            MeasurementType::MassDensity(Mass::new::<gram>(11.25) / Volume::new::<tablespoon>(1.0)),
        ];
        foods
            .into_iter()
            .flat_map(|food| food.alt_measures.unwrap_or_default())
            .zip(mtypes)
            .for_each(|(measure, mtype)| assert_eq!(MeasurementType::from(measure), mtype));
    }
}
