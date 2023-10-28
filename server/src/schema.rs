// @generated automatically by Diesel CLI.

diesel::table! {
    weights (weight_id) {
        weight_id -> Int8,
        inserted_at -> Int8,
        updated_at -> Int8,
        measured_at -> Int8,
        weight_kg -> Float8,
    }
}
