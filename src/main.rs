use salvo::{catcher::Catcher, prelude::*};
use salvo_admin_sqlite_api::{
    api::home, api::sys, catcher::handle_invalid_http_code, config::APPLICATION_CONFIG,
    db::sqlite::init_database, log::init_log, cache::init_cache,
};

// you can see the log when you visit the web url 'http://127.0.0.1:5800/'
#[handler]
async fn hello() -> &'static str {
    tracing::info!("hello");
    tracing::info!("test");
    "Hello World"
}

#[tokio::main]
async fn main() {
    // init log
    init_log();

    // init databse
    init_database().await;

    // init cache
    init_cache().await;

    // init router
    let router = Router::new()
        .push(
            Router::with_path("api")
                .push(sys::Routes::new())
                .push(home::Routes::new()),
        )
        .push(Router::new().handle(hello));

    // init service
    let service = Service::new(router).catcher(Catcher::default().hoop(handle_invalid_http_code));

    // init acceptor
    let acceptor = TcpListener::new(APPLICATION_CONFIG.server.host.as_str())
        .bind()
        .await;

    tracing::info!("server ready to start");

    // start server
    Server::new(acceptor).serve(service).await;
}
