use sqlx::postgres::PgPool;
use crate::internal::models::Order;
use log::error;

#[derive(Clone)]
pub struct PostgresClient {
    pool: PgPool,
}

impl PostgresClient {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPool::connect(database_url)
            .await
            .expect("Failed to connect to the database");
        Self { pool }
    }

    // Метод для сохранения заказа в базе данных
    pub async fn save_order(&self, order: &Order) -> Result<(), sqlx::Error> {
        let query = r#"
            INSERT INTO orders (order_uid, track_number, entry, locale, customer_id, date_created)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#;

        sqlx::query(query)
            .bind(&order.order_uid)
            .bind(&order.track_number)
            .bind(&order.entry)
            .bind(&order.locale)
            .bind(&order.customer_id)
            .bind(&order.date_created)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to save order: {:?}", e);
                e
            })?;
        Ok(())
    }

    // Метод для получения заказа по ID
    pub async fn get_order_by_id(&self, order_uid: &str) -> Result<Order, sqlx::Error> {
        let query = r#"
            SELECT order_uid, track_number, entry, locale, customer_id, date_created
            FROM orders WHERE order_uid = $1
            "#;

        let order = sqlx::query_as::<_, Order>(query)
            .bind(order_uid)
            .fetch_one(&self.pool)
            .await?;

        Ok(order)
    }
}
