//! This module declares the [`Quantity`] type to type different servings a food might have, along
//! with its associated string parsers.

use uom::si::f32::{Mass, Volume};

/// Serving quantities are either measured in volume/mass SI units or nominally.
#[derive(Debug, PartialEq)]
pub enum Quantity {
    Volume(Volume),
    Mass(Mass),
    Nominal(f32, String),
}

/// This module provides the parsing functionality for serving quantities.
pub mod parse {
    use super::Quantity;

    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, digit1, multispace0, multispace1};
    use nom::combinator::map_opt;
    use nom::number::complete::float;
    use nom::sequence::{delimited, terminated, tuple};
    use nom::{IResult, Parser};

    /// Parse a fraction string like `"1/2"` to the corresponding float.
    fn fraction(input: &str) -> IResult<&str, f32> {
        let digit_pair = tuple((
            digit1,
            delimited(multispace0, char('/'), multispace0),
            terminated(digit1, multispace0),
        ));
        map_opt(digit_pair, |(str0, _, str1): (&str, char, &str)| {
            let num0 = str0.parse::<f32>().ok()?;
            let num1 = str1.parse::<f32>().ok()?;
            Some(num0 / num1)
        })(input)
    }

    /// Parse a compound fraction string like `"1 1/2"` to the corresponding float.
    fn compound_fraction(input: &str) -> IResult<&str, f32> {
        map_opt(
            tuple((digit1, multispace1, fraction)),
            |(whole, _, frac): (&str, &str, f32)| whole.parse::<f32>().ok().map(|n| n + frac),
        )(input)
    }

    /// Parse any numeric string like `"3/2"`, `"1 1/2"`, or `"1.5"` to the corresponding float.
    pub fn number(input: &str) -> IResult<&str, f32> {
        alt((compound_fraction, fraction, float))(input)
    }

    /// This module simply holds static variables which are used for parsing units
    mod units {
        use super::*;
        use uom::si::{
            f32::{Mass, Volume},
            mass::{self, centigram, gram, kilogram, milligram, ounce, pound},
            volume::{
                self, centiliter, cubic_centimeter, cubic_inch, cup, fluid_ounce, gallon, liter,
                milliliter, pint_liquid, quart_liquid, tablespoon, teaspoon,
            },
        };

        /// We store all the units in an enum to ensure each one is matched against
        #[allow(non_camel_case_types)]
        enum Units {
            CENTILITER,
            CUBIC_CENTIMETER,
            CUBIC_INCH,
            CUP,
            FLUID_OUNCE,
            GALLON,
            LITER,
            MILLILITER,
            PINT,
            QUART,
            TABLESPOON,
            TEASPOON,
            CENTIGRAM,
            GRAM,
            KILOGRAM,
            MILLIGRAM,
            OUNCE,
            POUND,
            NONE,
        }

        /// Map various names associated to a unit to a normalized static candidate. A [`None`]
        /// variant corresponds to the input string slice not being associated to a unit.
        fn normalize_unit<'a>(input: &'a str) -> Units {
            match &input.to_lowercase()[..] {
                // volumes
                "centiliter" | "centiliters" | "cl" => Units::CENTILITER,
                "cubic centimeter" | "cubic centimeters" => Units::CUBIC_CENTIMETER,
                "cubic inch" | "cubic inches" => Units::CUBIC_INCH,
                "cup" | "cups" => Units::CUP,
                "fl.oz." | "fl. oz." | "fl oz" | "fluid ounce" | "fluid oz" | "fluid ounces"
                | "oza" => Units::FLUID_OUNCE,
                "gallon" | "gallons" | "gals" | "gal" => Units::GALLON,
                "l" | "liter" | "liters" => Units::LITER,
                "ml" | "milliliter" | "milliliters" => Units::MILLILITER,
                "pint" | "pints" => Units::PINT,
                "quart" | "quarts" => Units::QUART,
                "tbsp" | "tablespoon" | "tablespoons" => Units::TABLESPOON,
                "tsp" | "teaspoon" | "teaspoons" => Units::TEASPOON,
                // masses
                "centigram" | "centigrams" | "cg" => Units::CENTIGRAM,
                "gram" | "grams" | "g" | "grm" | "gr" => Units::GRAM,
                "kilogram" | "kilograms" | "kg" => Units::KILOGRAM,
                "milligram" | "milligrams" | "mg" => Units::MILLIGRAM,
                "ounce" | "onz" | "ounces" | "oz" | "oz." | "wt. oz." | "wt oz" | "wt.oz." => {
                    Units::OUNCE
                }
                "pound" | "pounds" | "lb" | "lbs" => Units::POUND,
                // no match
                &_ => Units::NONE,
            }
        }

        /// helper function which creates volume quantities
        fn v<U>(amount: f32) -> Quantity
        where
            U: volume::Unit + volume::Conversion<f32, T = f32>,
        {
            Quantity::Volume(Volume::new::<U>(amount))
        }

        /// helper function which creates mass quantities
        fn m<U>(amount: f32) -> Quantity
        where
            U: mass::Unit + mass::Conversion<f32, T = f32>,
        {
            Quantity::Mass(Mass::new::<U>(amount))
        }

        impl Units {
            /// helper function which creates quantities
            fn si_quantity(&self, amount: f32) -> Option<Quantity> {
                match self {
                    Units::CENTILITER => Some(v::<centiliter>(amount)),
                    Units::CUBIC_CENTIMETER => Some(v::<cubic_centimeter>(amount)),
                    Units::CUBIC_INCH => Some(v::<cubic_inch>(amount)),
                    Units::CUP => Some(v::<cup>(amount)),
                    Units::FLUID_OUNCE => Some(v::<fluid_ounce>(amount)),
                    Units::GALLON => Some(v::<gallon>(amount)),
                    Units::LITER => Some(v::<liter>(amount)),
                    Units::MILLILITER => Some(v::<milliliter>(amount)),
                    Units::PINT => Some(v::<pint_liquid>(amount)),
                    Units::QUART => Some(v::<quart_liquid>(amount)),
                    Units::TABLESPOON => Some(v::<tablespoon>(amount)),
                    Units::TEASPOON => Some(v::<teaspoon>(amount)),
                    Units::CENTIGRAM => Some(m::<centigram>(amount)),
                    Units::GRAM => Some(m::<gram>(amount)),
                    Units::KILOGRAM => Some(m::<kilogram>(amount)),
                    Units::MILLIGRAM => Some(m::<milligram>(amount)),
                    Units::OUNCE => Some(m::<ounce>(amount)),
                    Units::POUND => Some(m::<pound>(amount)),
                    Units::NONE => None,
                }
            }
        }

        /// helper function which creates si quantities
        pub fn si_quantity(input: &str, amount: f32) -> Option<Quantity> {
            normalize_unit(input).si_quantity(amount)
        }
    }

    /// This is a simple parser that allows for words to have a period in them, as is common with
    /// unit names.
    pub fn unit_word<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
        alpha1
            .or(delimited(multispace0, tag("."), multispace0))
            .parse(input)
    }

    /// Parser for a food quantity. It is achieved by first matching on a numeric value and
    /// iteratively grabbing words until the resulting string matches an SI unit or it can grab no
    /// more. In the latter case, it returns the [`Quantity::Nominal`] variant.
    pub fn quantity<'a>(input: &'a str) -> IResult<&'a str, Quantity> {
        // the string buffer will build as we grab words. This will ultimately determine the unit
        // of the SI quantity or the string which is in the nominal variant.
        let mut string = String::new();

        // create a result with a first pass of a (number, word) parser.
        let number = delimited(multispace0, number, multispace0);
        let mut res = tuple((number, unit_word))(input);

        // if the first parse failed, this parser will as well; otherwise, append the first word
        // to the string
        match res {
            Ok((_, (_, word))) => string.push_str(&word.to_lowercase()),
            Err(e) => return Err(e),
        }

        // we construct a mutator function which will apply the word parser and append the consumed
        // word to the string buffer. It will return a boolean of the parser's success.
        let mutator =
            |res: &mut IResult<&'a str, (f32, &'a str)>, string: &mut String| match res.as_mut() {
                Ok((i, _)) => match delimited(multispace0, unit_word, multispace0)(i) {
                    Ok((i_new, word)) => {
                        if word == "." {
                            string.push_str(word);
                        } else {
                            string.push_str(" ");
                            string.push_str(&word.to_lowercase());
                        }
                        *i = i_new;
                        true
                    }
                    Err(_) => false,
                },
                Err(_) => false, // shouldn't happen in our loop
            };

        // create a loop for grabbing one word at a time
        loop {
            match res {
                // Check if we can make an SI quantity from the (f32, String) tuple. Otherwise,
                // continually request words with our mutator
                Ok((i, (val, _))) => {
                    // check the SI unit
                    match units::si_quantity(&string, val.clone()) {
                        Some(quantity) => return Ok((i, quantity)),
                        None => {}
                    }

                    // apply the mutator
                    let got_another_word = mutator(&mut res, &mut string);

                    // if the mutator did not pass, we say the quantity is nominal
                    if !got_another_word {
                        return res.map(|(i, (val, _))| (i, Quantity::Nominal(val, string)));
                    }
                }
                // this is if we immediately failed on the first parse
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use uom::si::{
        f32::{Mass, Volume},
        mass::kilogram,
        volume::{cubic_inch, cup, gallon, milliliter},
    };

    #[test]
    fn numbers() {
        assert_eq!(parse::number("1.123 blah"), Ok((" blah", 1.123)));
        assert_eq!(parse::number("1.123blah"), Ok(("blah", 1.123)));
        assert_eq!(parse::number("1/2."), Ok((".", 0.5)));
        assert_eq!(parse::number("1 1/2."), Ok((".", 1.5)));
    }

    #[test]
    fn quantity() {
        assert_eq!(
            parse::quantity("5.26 milliliters of rice (35g)"),
            Ok((
                " of rice (35g)",
                Quantity::Volume(Volume::new::<milliliter>(5.26))
            ))
        );
        assert_eq!(
            parse::quantity("5.26 cubic inches (35g)"),
            Ok(("(35g)", Quantity::Volume(Volume::new::<cubic_inch>(5.26))))
        );
        assert_eq!(
            parse::quantity("83.1512gal"),
            Ok(("", Quantity::Volume(Volume::new::<gallon>(83.1512))))
        );
        assert_eq!(
            parse::quantity("1/2 cup"),
            Ok(("", Quantity::Volume(Volume::new::<cup>(0.5))))
        );
        assert_eq!(
            parse::quantity("69.69kg"),
            Ok(("", Quantity::Mass(Mass::new::<kilogram>(69.69))))
        );
        assert_eq!(
            parse::quantity("1 package (23g Kernels)"),
            Ok((
                " (23g Kernels)",
                Quantity::Nominal(1.0, "package".to_string())
            ))
        )
    }
}
