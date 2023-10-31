CREATE TABLE weights (
    weight_id BIGINT PRIMARY KEY DEFAULT next_id(),
    inserted_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    measured_at TIMESTAMP NOT NULL,
    weight_kg DOUBLE PRECISION NOT NULL
);

SELECT diesel_manage_updated_at('weights');
