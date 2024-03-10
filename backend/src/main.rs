// #region My Modules
mod error;
mod routes;
mod prelude;
mod model;

use actix_web::web::{Data,get};
use actix_web::{App,HttpServer};
use prelude::*;
use crate::routes::*;
use model::db::Database;

use env_logger::{self, init};
use actix_cors::Cors;


#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    //let _db = init_db().await?;
    let app_data = Data::new(Database::new().await?);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new().app_data(app_data.clone())
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(list_todos)
            .service(get_todo_route)
            .service(delete_todo)
            .service(create_todo)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
