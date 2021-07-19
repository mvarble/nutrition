//! Types which we use in our REST API.

use itertools::Itertools;
use std::collections::HashMap;
use uom::si::{
    f32::{MassDensity, Volume},
    mass::{gram, kilogram, milligram},
    mass_density::kilogram_per_cubic_meter,
    volume::{
        cup, fluid_ounce, gallon, liter, milliliter, pint_liquid, quart_liquid, tablespoon,
        teaspoon, Unit,
    },
};

use super::pg;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub img: Option<String>,
    pub brand: Option<String>,
    pub upc: Option<String>,
    pub mass: f32,
    pub nutrition: HashMap<String, f32>,
    pub measurements: Vec<Measurement>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Measurement {
    pub measurement_type: MeasurementType,
    pub name: String,
    pub mass: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MeasurementType {
    Mass,
    Volume,
    Nominal,
}

lazy_static! {
    /// The CSV contents of the nutrition codes are compiled to this HashMap.
    static ref NUTRITION: HashMap<i32, String> = {
        // open a file handle (assumes we are running at project root)
        let cwd = std::env::current_dir().unwrap();
        let path = cwd.join("assets").join("nutrition.csv");
        let file = std::fs::File::open(path).unwrap();

        // use the csv reader to create a HashMap from the file contents
        let mut reader = csv::Reader::from_reader(file);
        let mut map: HashMap<i32, String> = HashMap::new();
        reader.records().for_each(|res| {
            let row = res.unwrap();
            let id = row.get(0).unwrap().parse::<i32>().unwrap();
            let name = row.get(2).unwrap().to_string();
            map.insert(id, name);
        });
        map
    };
}

/// Map mass density to (volume, mass) data via a `Measurement` struct.
fn from_volume<V: std::fmt::Debug + Unit + uom::Conversion<f32, T = f32>>(
    md: MassDensity,
) -> Measurement {
    Measurement {
        measurement_type: MeasurementType::Volume,
        name: V::singular().to_string(),
        mass: (md * Volume::new::<V>(1.0)).get::<gram>(),
    }
}

/// Each Food payload in our REST API is built from joining data from the `foods` and `servings`
/// tables. This is a helper function which does such a build
pub fn parsedb(food: pg::Food, nominals: Vec<pg::Serving>) -> Food {
    // create all of the measurements
    let mut measurements = vec![
        Measurement {
            measurement_type: MeasurementType::Mass,
            name: format!("{:?}", milligram),
            mass: 0.001,
        },
        Measurement {
            measurement_type: MeasurementType::Mass,
            name: format!("{:?}", gram),
            mass: 1.0,
        },
        Measurement {
            measurement_type: MeasurementType::Mass,
            name: format!("{:?}", kilogram),
            mass: 1000.0,
        },
    ];
    if let Some(g2l) = food.g2l_density {
        let md = MassDensity::new::<kilogram_per_cubic_meter>(g2l);
        measurements.push(from_volume::<milliliter>(md));
        measurements.push(from_volume::<liter>(md));
        measurements.push(from_volume::<cup>(md));
        measurements.push(from_volume::<pint_liquid>(md));
        measurements.push(from_volume::<quart_liquid>(md));
        measurements.push(from_volume::<gallon>(md));
        measurements.push(from_volume::<tablespoon>(md));
        measurements.push(from_volume::<teaspoon>(md));
        measurements.push(from_volume::<fluid_ounce>(md));
    }
    nominals.into_iter().for_each(|serving| {
        measurements.push(Measurement {
            measurement_type: MeasurementType::Nominal,
            name: serving.name,
            mass: serving.mass,
        });
    });

    // create the nutrition
    let nutrition: HashMap<String, f32> =
        food.nutrition
            .iter()
            .tuples()
            .fold(HashMap::new(), |mut nutrition, (key, value)| {
                let index = *key as i32;
                match NUTRITION.get(&index) {
                    None => nutrition,
                    Some(name) => {
                        nutrition.insert(name.to_string(), *value);
                        nutrition
                    }
                }
            });

    // return the food
    Food {
        id: food.id,
        name: food.name,
        img: food.img,
        brand: food.brand,
        upc: food.upc,
        mass: food.mass,
        nutrition,
        measurements,
    }
}
