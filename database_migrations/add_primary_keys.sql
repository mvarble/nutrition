/**
 * This file contains edits that we must make to the database from 
 * `fooddata-central-postgres` in order to have primary keys for diesel to 
 * work.
 */
ALTER TABLE acquisition_samples 
  ADD COLUMN id SERIAL, 
  ADD CONSTRAINT acquisition_samples_pkey PRIMARY KEY (id);

ALTER TABLE agricultural_samples 
  ADD COLUMN id SERIAL, 
  ADD CONSTRAINT agricultural_samples_pkey PRIMARY KEY (id);

ALTER TABLE branded_food 
  ADD CONSTRAINT branded_food_pkey PRIMARY KEY (fdc_id);

ALTER TABLE foundation_food 
  ADD CONSTRAINT foundation_food_pkey PRIMARY KEY (fdc_id);

ALTER TABLE market_acquisition 
  ADD CONSTRAINT market_acquisition_pkey PRIMARY KEY (fdc_id);

ALTER TABLE sample_food 
  ADD CONSTRAINT sample_food_pkey PRIMARY KEY (fdc_id);

ALTER TABLE sr_legacy_food 
  ADD CONSTRAINT sr_legacy_food_pkey PRIMARY KEY (fdc_id);

ALTER TABLE sub_sample_food 
  ADD CONSTRAINT sub_sample_food_pkey PRIMARY KEY (fdc_id);

ALTER TABLE sub_sample_result 
  ADD COLUMN id SERIAL, 
  ADD CONSTRAINT sub_sample_result_pkey PRIMARY KEY (id);
