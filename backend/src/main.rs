// #region My Modules
mod error;
mod routes;
mod prelude;
mod model;

use actix_web::web::{Data,get};
use actix_web::{App,HttpServer};
use prelude::*;
use crate::routes::*;
use model::db::init_db;

use env_logger;





#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    //let _db = init_db().await?;
    let app_data = Data::new(());

    HttpServer::new(move || {
        App::new().app_data(app_data.clone())
            .service(hello)
            .service(echo)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
