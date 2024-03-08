// #region My Modules
mod error;
mod routes;
mod prelude;
mod model;

use prelude::*;

use model::db::init_db;

use env_logger;





#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let _db = init_db().await?;
    // HttpServer::new(move || {
    //     App::new().app_data(app_data.clone())
    //         .service(hello)
    //         .service(echo)
    //         .route("/hey", web::get().to(manual_hello))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await?;

    Ok(())
}
