use diesel::table;

table! {
    weights (weight_id) {
        weight_id -> Int8,
        weight_kg -> Float8,
        measured_at -> Int8,
        created_at -> Int8,
        updated_at -> Int8,
    }
}
