// @generated automatically by Diesel CLI.

diesel::table! {
    weights (weight_id) {
        weight_id -> Int8,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
        measured_at -> Timestamp,
        weight_kg -> Float8,
    }
}
