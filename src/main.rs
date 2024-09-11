mod internal {
    pub mod app;
    pub mod models;
}

use internal::app::Server;

#[tokio::main]
async fn main() {
    // Инициализируем логгер
    env_logger::init();

    // Создаем сервер
    let server = Server::new();

    // Запускаем сервер
    server.run("0.0.0.0:9090").await;
}
