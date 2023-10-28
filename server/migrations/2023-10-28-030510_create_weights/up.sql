CREATE TABLE weights (
    weight_id BIGINT PRIMARY KEY DEFAULT next_id(),
    inserted_at BIGINT NOT NULL DEFAULT extract(epoch from current_timestamp),
    updated_at BIGINT NOT NULL DEFAULT extract(epoch from current_timestamp),
    measured_at BIGINT NOT NULL,
    weight_kg DOUBLE PRECISION NOT NULL
);

SELECT diesel_manage_updated_at('weights');
