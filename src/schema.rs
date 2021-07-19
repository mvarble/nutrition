table! {
    foods (id) {
        id -> Int4,
        name -> Varchar,
        mass -> Float4,
        nutrition -> Array<Float4>,
        g2l_density -> Nullable<Float4>,
        img -> Nullable<Varchar>,
        brand -> Nullable<Varchar>,
        upc -> Nullable<Varchar>,
    }
}

table! {
    meals (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        time -> Timestamptz,
        servings -> Float4,
        servings_consumed -> Float4,
        food_ids -> Array<Int4>,
        food_masses -> Array<Float4>,
    }
}

table! {
    servings (id) {
        id -> Int4,
        food_id -> Int4,
        name -> Varchar,
        mass -> Float4,
    }
}

joinable!(servings -> foods (food_id));

allow_tables_to_appear_in_same_query!(foods, meals, servings,);
