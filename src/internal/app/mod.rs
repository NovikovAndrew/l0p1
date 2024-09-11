use axum::{extract::{Path, State}, http::StatusCode, routing::{get, post}, Json, Router};
use tokio::net::TcpListener;
use log::{info, error};
use std::{collections::HashMap, sync::{Arc, Mutex}, net::SocketAddr};
use crate::internal::models::Order;

#[derive(Clone)]
pub struct Server {
    order_cache: Arc<Mutex<HashMap<String, Order>>>,
}

impl Server {
    pub fn new() -> Arc<Self> {
        Arc::new(Server {
            order_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    // Метод для получения заказа по ID
    pub async fn get_order(State(state): State<Arc<Server>>, Path(order_id): Path<String>) -> Result<Json<Order>, StatusCode> {
        let cache = state.order_cache.lock().unwrap();
        match cache.get(&order_id) {
            Some(order) => {
                info!("Order {} retrieved successfully", order_id);
                Ok(Json(order.clone()))
            }
            None => {
                error!("Order {} not found", order_id);
                Err(StatusCode::NOT_FOUND)
            }
        }
    }

    // Метод для сохранения нового заказа в кэш
    pub async fn save_order(
        State(state): State<Arc<Server>>, Json(order): Json<Order>) -> (StatusCode, String) {
        let mut cache = state.order_cache.lock().unwrap();

        // Если заказ уже существует order, возвращаем ошибку
        if cache.contains_key(&order.order_uid) {
            return (
                StatusCode::CONFLICT,
                format!("Order with id {} already exists", order.order_uid),
            );
        }

        // Сохраняем новый заказ в кэше
        cache.insert(order.order_uid.clone(), order.clone());
        info!("Order {} saved successfully", order.order_uid);
        (StatusCode::CREATED, format!("Order {} saved", order.order_uid))
    }

    // Метод для запуска сервера
    pub async fn run(self: Arc<Self>, addr: &str) {
        let app = Router::new()
            .route("/orders", post(Server::save_order))
            .route("/orders/:id", get(Server::get_order))
            .with_state(self);

        let addr = addr.parse::<SocketAddr>().unwrap();
        let tcp_listener = TcpListener::bind(&addr).await.unwrap();

        info!("Server is running on {}", addr);

        axum::serve(tcp_listener, app).await.unwrap()
    }
}
