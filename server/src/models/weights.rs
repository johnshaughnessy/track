use chrono::NaiveDateTime;
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::update;
use diesel::Insertable;
use log::info;
use serde_derive::{Deserialize, Serialize};

use crate::schema::weights;

use super::ID;

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = crate::schema::weights)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(weight_id))]
pub struct Weight {
    pub weight_id: ID,
    #[serde(skip_serializing)]
    pub inserted_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
    pub measured_at: NaiveDateTime,
    pub weight_kg: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::weights)]
pub struct CreateWeightPayload {
    pub measured_at: NaiveDateTime,
    pub weight_kg: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::weights)]
pub struct UpdateWeightPayload {
    pub measured_at: NaiveDateTime,
    pub weight_kg: f64,
}

pub fn create_weight(
    conn: &mut PgConnection,
    create_weight_payload: &CreateWeightPayload,
) -> Result<Weight, diesel::result::Error> {
    let inserted_weight = diesel::insert_into(weights::table)
        .values(create_weight_payload)
        .returning(weights::all_columns)
        .get_result(conn)?;

    info!("create_weight results: \n{:?}", inserted_weight);
    Ok(inserted_weight)
}

pub fn read_weights(conn: &mut PgConnection) -> Result<Vec<Weight>, diesel::result::Error> {
    use crate::schema::weights::dsl::*;

    let results: Result<Vec<Weight>, diesel::result::Error> = weights.limit(100).load(conn);

    info!("read_weights results: \n{:?}", results);
    results
}

pub fn update_weight(
    conn: &mut PgConnection,
    target_weight_id: ID,
    update_weight_payload: &UpdateWeightPayload,
) -> Result<Weight, diesel::result::Error> {
    use crate::schema::weights::dsl::*;

    let target = weights.filter(weight_id.eq(target_weight_id));
    update(target).set(update_weight_payload).execute(conn)?;
    // TODO: How do I return the updated weight without re-querying?
    let updated_weight = weights
        .filter(weight_id.eq(target_weight_id))
        .get_result(conn)?;

    info!("update_weight results: \n{:?}", updated_weight);
    Ok(updated_weight)
}

pub fn delete_weight(
    conn: &mut PgConnection,
    target_weight_id: ID,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::weights::dsl::*;

    let num_deleted =
        diesel::delete(weights.filter(weight_id.eq(target_weight_id))).execute(conn)?;

    info!("delete_weight results: \n{:?}", num_deleted);

    if num_deleted != 1 {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    Ok(num_deleted)
}

#[cfg(test)]
mod tests {

    use diesel::r2d2::{self, ConnectionManager, PooledConnection};

    use super::*;

    fn setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_test_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        pool.get().unwrap()
    }

    fn create_weight_for_test(conn: &mut PgConnection) -> Weight {
        let new_weight = CreateWeightPayload {
            measured_at: NaiveDateTime::from_timestamp_opt(1635535802, 0)
                .expect("Failed to create NaiveDateTime from timestamp"),
            weight_kg: 70.5,
        };
        create_weight(conn, &new_weight).unwrap()
    }

    #[test]
    fn test_create_weight() {
        setup();
        let mut conn = get_test_connection();

        let new_weight = CreateWeightPayload {
            measured_at: NaiveDateTime::from_timestamp_opt(1635535802, 0)
                .expect("Failed to create NaiveDateTime from timestamp"),
            weight_kg: 70.5,
        };

        let result = create_weight(&mut conn, &new_weight);
        assert!(result.is_ok());
        let weight = result.unwrap();
        assert_eq!(new_weight.measured_at, weight.measured_at);
        assert_eq!(new_weight.weight_kg, weight.weight_kg);
    }

    #[test]
    fn test_read_weights() {
        setup();
        let mut conn = get_test_connection();
        create_weight_for_test(&mut conn); // Ensure there is at least one weight in the database
        let result = read_weights(&mut conn);
        assert!(result.is_ok());
        let weights = result.unwrap();
        assert!(!weights.is_empty());
    }

    #[test]
    fn test_update_weight() {
        setup();
        let mut conn = get_test_connection();
        let target_weight_id = create_weight_for_test(&mut conn).weight_id;
        info!("{:?}", create_weight_for_test(&mut conn));
        let update_payload = UpdateWeightPayload {
            measured_at: NaiveDateTime::from_timestamp_opt(1635535802, 0)
                .expect("Failed to create NaiveDateTime from timestamp"),
            weight_kg: 71.0,
        };
        let result = update_weight(&mut conn, target_weight_id, &update_payload);
        assert!(result.is_ok());
        let updated_weight = result.unwrap();
        assert_eq!(update_payload.measured_at, updated_weight.measured_at);
        assert_eq!(update_payload.weight_kg, updated_weight.weight_kg);
    }

    #[test]
    fn test_delete_weight() {
        setup();
        let mut conn = get_test_connection();
        let target_weight_id = create_weight_for_test(&mut conn).weight_id;
        let result = delete_weight(&mut conn, target_weight_id);
        assert!(result.is_ok());
        assert_eq!(1, result.unwrap());
    }
}
