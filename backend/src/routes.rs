use actix_web::{delete, get, post, web::{Data, Json, Path}, HttpResponse, Responder};
use log::info;
use sqlx::{Pool, Sqlite};
use crate::model::{db::Database, models::Todo};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/todos")]
async fn list_todos(db : Data<Database>) -> impl Responder{
    let todos = db.list_todos().await;
    match todos {
        Ok(val) =>{
            if let Some(v) = val{
                return HttpResponse::Ok().json(v);
            }
            else{
                return HttpResponse::Ok().json("None");
            }
        },
        Err(_) => return HttpResponse::InternalServerError().finish()
    }
}

#[post("/create-todo")]
async fn create_todo(db : Data<Database>,todo : Json<Todo>) -> impl Responder{
    let todo = todo.0;
    let res = db.create_todo(todo).await;
    match res{
        Ok(id) => HttpResponse::Created().json(id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/todo/{id}")]
async fn delete_todo(db : Data<Database>,id : Path<(i64)>) -> impl Responder{
    let res = db.delete_todo(*id).await;
    match res{
        Ok(_) => HttpResponse::Ok().json("Deleted!"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[get("/todo/{id}")]
async fn get_todo_route(db : Data<Database>,path : Path<(i64)>) -> impl Responder{
    let (id) = path.into_inner();
    info!("Request id : {:?}",id);
    let res = db.get_todo(id).await;
    match res{
        Ok(record) => {
            match record {
                Some(r) =>  HttpResponse::Ok().json(r),
                None => HttpResponse::NotFound().json("Not found!"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}