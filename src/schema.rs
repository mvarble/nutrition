table! {
    acquisition_samples (id) {
        fdc_id_of_sample_food -> Int4,
        fdc_id_of_acquisition_food -> Int4,
        id -> Int4,
    }
}

table! {
    agricultural_samples (id) {
        fdc_id -> Int4,
        acquisition_date -> Date,
        market_class -> Nullable<Varchar>,
        treatment -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        id -> Int4,
    }
}

table! {
    branded_food (fdc_id) {
        fdc_id -> Int4,
        brand_owner -> Nullable<Varchar>,
        brand_name -> Nullable<Varchar>,
        subbrand_name -> Nullable<Varchar>,
        gtin_upc -> Nullable<Varchar>,
        ingredients -> Nullable<Varchar>,
        not_a_significant_source_of -> Nullable<Varchar>,
        serving_size -> Nullable<Float8>,
        serving_size_unit -> Nullable<Varchar>,
        household_serving_fulltext -> Nullable<Varchar>,
        branded_food_category -> Nullable<Varchar>,
        data_source -> Nullable<Varchar>,
        modified_date -> Nullable<Date>,
        available_date -> Nullable<Date>,
        market_country -> Nullable<Varchar>,
        discontinued_date -> Nullable<Date>,
    }
}

table! {
    fndds_derivation (derivation_code) {
        derivation_code -> Varchar,
        derivation_description -> Nullable<Varchar>,
    }
}

table! {
    fndds_ingredient_nutrient_value (id) {
        ingredient_code -> Int4,
        sr_description -> Nullable<Varchar>,
        nutrient_code -> Int4,
        nutrient_value -> Float8,
        nutrient_value_source -> Nullable<Varchar>,
        derivation_code -> Nullable<Varchar>,
        sr_addmod_year -> Nullable<Int4>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        id -> Int4,
    }
}

table! {
    food (fdc_id) {
        fdc_id -> Int4,
        data_type -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        food_category_id -> Nullable<Int4>,
        publication_date -> Date,
    }
}

table! {
    food_attribute (id) {
        id -> Int4,
        fdc_id -> Int4,
        seq_num -> Nullable<Int4>,
        food_attribute_type_id -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        value -> Nullable<Varchar>,
    }
}

table! {
    food_attribute_type (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
    }
}

table! {
    food_calorie_conversion_factor (food_nutrient_conversion_factor_id) {
        food_nutrient_conversion_factor_id -> Int4,
        protein_value -> Nullable<Float8>,
        fat_value -> Nullable<Float8>,
        carbohydrate_value -> Nullable<Float8>,
    }
}

table! {
    food_category (id) {
        id -> Int4,
        code -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
    }
}

table! {
    food_component (id) {
        id -> Int4,
        fdc_id -> Int4,
        name -> Nullable<Varchar>,
        pct_weight -> Nullable<Float8>,
        is_refuse -> Bool,
        gram_weight -> Float8,
        data_points -> Int4,
        min_year_acquired -> Nullable<Int4>,
    }
}

table! {
    food_nutrient (id) {
        id -> Int4,
        fdc_id -> Int4,
        nutrient_id -> Int4,
        amount -> Float8,
        data_points -> Nullable<Int4>,
        derivation_id -> Nullable<Int4>,
        min -> Nullable<Float8>,
        max -> Nullable<Float8>,
        median -> Nullable<Float8>,
        footnote -> Nullable<Varchar>,
        min_year_acquired -> Nullable<Int4>,
        nutrient_id_nid -> Nullable<Int4>,
        nutrient_id_nnbr -> Nullable<Float8>,
    }
}

table! {
    food_nutrient_conversion_factor (id) {
        id -> Int4,
        fdc_id -> Int4,
    }
}

table! {
    food_nutrient_derivation (id) {
        id -> Int4,
        code -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        source_id -> Int4,
    }
}

table! {
    food_nutrient_source (id) {
        id -> Int4,
        code -> Int4,
        description -> Nullable<Varchar>,
    }
}

table! {
    food_portion (id) {
        id -> Int4,
        fdc_id -> Int4,
        seq_num -> Nullable<Int4>,
        amount -> Float8,
        measure_unit_id -> Int4,
        portion_description -> Nullable<Varchar>,
        modifier -> Nullable<Varchar>,
        gram_weight -> Float8,
        data_points -> Nullable<Int4>,
        footnote -> Nullable<Varchar>,
        min_year_acquired -> Nullable<Int4>,
    }
}

table! {
    food_protein_conversion_factor (food_nutrient_conversion_factor_id) {
        food_nutrient_conversion_factor_id -> Int4,
        value -> Float8,
    }
}

table! {
    food_update_log_entry (id) {
        id -> Int4,
        description -> Nullable<Varchar>,
        last_updated -> Date,
    }
}

table! {
    foundation_food (fdc_id) {
        fdc_id -> Int4,
        ndb_number -> Int4,
        footnote -> Nullable<Varchar>,
    }
}

table! {
    input_food (id) {
        id -> Int4,
        fdc_id -> Int4,
        fdc_of_input_food -> Int4,
        seq_num -> Nullable<Int4>,
        amount -> Nullable<Float8>,
    }
}

table! {
    lab_method (id) {
        id -> Int4,
        description -> Nullable<Varchar>,
        technique -> Nullable<Varchar>,
    }
}

table! {
    lab_method_code (id) {
        id -> Int4,
        lab_method_id -> Nullable<Int4>,
        code -> Nullable<Varchar>,
    }
}

table! {
    lab_method_nutrient (id) {
        id -> Int4,
        lab_method_id -> Int4,
        nutrient_id -> Int4,
    }
}

table! {
    market_acquisition (fdc_id) {
        fdc_id -> Int4,
        brand_description -> Nullable<Varchar>,
        expiration_date -> Nullable<Date>,
        label_weight -> Nullable<Float8>,
        location -> Nullable<Varchar>,
        acquisition_date -> Nullable<Date>,
        sales_type -> Nullable<Varchar>,
        sample_lot_nbr -> Nullable<Varchar>,
        sell_by_date -> Nullable<Date>,
        store_city -> Nullable<Varchar>,
        store_name -> Nullable<Varchar>,
        store_state -> Nullable<Varchar>,
        upc_code -> Nullable<Varchar>,
    }
}

table! {
    measure_unit (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    nutrient (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        unit_name -> Nullable<Varchar>,
        nutrient_nbr -> Nullable<Float8>,
        rank -> Nullable<Varchar>,
    }
}

table! {
    nutrient_incoming_name (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        nutrient_id -> Int4,
    }
}

table! {
    retention_factor (id) {
        id -> Int4,
        code -> Int4,
        food_group_id -> Int4,
        description -> Nullable<Varchar>,
    }
}

table! {
    sample_food (fdc_id) {
        fdc_id -> Int4,
    }
}

table! {
    sr_legacy_food (fdc_id) {
        fdc_id -> Int4,
        ndb_number -> Int4,
    }
}

table! {
    sub_sample_food (fdc_id) {
        fdc_id -> Int4,
        fdc_id_of_sample_food -> Int4,
    }
}

table! {
    sub_sample_result (id) {
        food_nutrient_id -> Int4,
        adjusted_amount -> Nullable<Float8>,
        lab_method_id -> Int4,
        nutrient_name -> Nullable<Varchar>,
        id -> Int4,
    }
}

table! {
    survey_fndds_food (food_code) {
        fdc_id -> Int4,
        food_code -> Int4,
        wweia_category_code -> Int4,
        start_date -> Date,
        end_date -> Date,
    }
}

table! {
    wweia_food_category (id) {
        id -> Int4,
        description -> Nullable<Varchar>,
    }
}

joinable!(agricultural_samples -> food (fdc_id));
joinable!(branded_food -> food (fdc_id));
joinable!(fndds_ingredient_nutrient_value -> fndds_derivation (derivation_code));
joinable!(food -> wweia_food_category (food_category_id));
joinable!(food_attribute -> food (fdc_id));
joinable!(food_attribute -> food_attribute_type (food_attribute_type_id));
joinable!(food_component -> food (fdc_id));
joinable!(food_nutrient -> food (fdc_id));
joinable!(food_nutrient -> food_nutrient_derivation (derivation_id));
joinable!(food_nutrient_conversion_factor -> food (fdc_id));
joinable!(food_nutrient_derivation -> food_nutrient_source (source_id));
joinable!(food_portion -> food (fdc_id));
joinable!(food_portion -> measure_unit (measure_unit_id));
joinable!(foundation_food -> food (fdc_id));
joinable!(lab_method_code -> lab_method (lab_method_id));
joinable!(lab_method_nutrient -> lab_method (lab_method_id));
joinable!(lab_method_nutrient -> nutrient (nutrient_id));
joinable!(nutrient_incoming_name -> nutrient (nutrient_id));
joinable!(sample_food -> food (fdc_id));
joinable!(sr_legacy_food -> food (fdc_id));
joinable!(sub_sample_result -> food_nutrient (food_nutrient_id));
joinable!(sub_sample_result -> lab_method (lab_method_id));
joinable!(survey_fndds_food -> food (fdc_id));
joinable!(survey_fndds_food -> wweia_food_category (wweia_category_code));

allow_tables_to_appear_in_same_query!(
    acquisition_samples,
    agricultural_samples,
    branded_food,
    fndds_derivation,
    fndds_ingredient_nutrient_value,
    food,
    food_attribute,
    food_attribute_type,
    food_calorie_conversion_factor,
    food_category,
    food_component,
    food_nutrient,
    food_nutrient_conversion_factor,
    food_nutrient_derivation,
    food_nutrient_source,
    food_portion,
    food_protein_conversion_factor,
    food_update_log_entry,
    foundation_food,
    input_food,
    lab_method,
    lab_method_code,
    lab_method_nutrient,
    market_acquisition,
    measure_unit,
    nutrient,
    nutrient_incoming_name,
    retention_factor,
    sample_food,
    sr_legacy_food,
    sub_sample_food,
    sub_sample_result,
    survey_fndds_food,
    wweia_food_category,
);
