-- food
ALTER TABLE food ALTER COLUMN data_type SET NOT NULL;
UPDATE food SET description=NULLIF(description, '');

-- branded_food
ALTER TABLE branded_food ALTER COLUMN market_country SET NOT NULL;
UPDATE branded_food SET 
  brand_owner=NULLIF(brand_owner, ''),
  brand_name=NULLIF(brand_name, ''),
  subbrand_name=NULLIF(subbrand_name, ''),
  gtin_upc=NULLIF(gtin_upc, ''),
  ingredients=NULLIF(ingredients, ''),
  not_a_significant_source_of=NULLIF(not_a_significant_source_of, ''),
  serving_size_unit=NULLIF(serving_size_unit, ''),
  household_serving_fulltext=NULLIF(household_serving_fulltext, ''),
  branded_food_category=NULLIF(branded_food_category, '');

-- food_nutrient
UPDATE food_nutrient SET footnote=NULLIF(footnote, '');

-- food_nutrient_derivation
ALTER TABLE food_nutrient_derivation 
  ALTER COLUMN code SET NOT NULL,
  ALTER COLUMN description SET NOT NULL;

-- nutrient
ALTER TABLE nutrient 
  ALTER COLUMN name SET NOT NULL,
  ALTER COLUMN unit_name SET NOT NULL;

-- food_nutrient_source
ALTER TABLE food_nutrient_source ALTER COLUMN description SET NOT NULL;
