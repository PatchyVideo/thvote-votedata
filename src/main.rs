
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use context::AppContext;
use mongodb::{Client, options::ClientOptions};

pub mod comm;
pub mod models;
pub mod context;
pub mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let client_options = ClientOptions::parse(comm::MONGO_ADDRESS).await.expect("Failed to parse MongoDB parameters");
	let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");

	let db = client.database("thvote_users");

    let ctx = context::AppContext {
        db: db.clone(),
    };
    HttpServer::new(move || {
        App::new().app_data(ctx.clone())
            .route("/v1/characters", web::get().to(handlers::get_all_characters))
            .route("/v1/works", web::get().to(handlers::get_all_works))
            .route("/v1/musics", web::get().to(handlers::get_all_musics))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
