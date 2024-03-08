


use crate::prelude::*;
use crate::model::models::*;

use sqlx::sqlite::SqlitePool;


pub async fn init_db() -> Result<SqlitePool>{
    let db = SqlitePool::connect("sqlite:todos.db").await?;
    
    Ok(db)
}

pub async fn create_todo(db : &SqlitePool,todo : Todo) -> Result<i64>{
    let mut conn = db.acquire().await?;
    let Todo {description,completed,title} = todo;
    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
                r#"
                INSERT INTO todos ( title, description,completed )
                VALUES ( ?1,?2,?3 )
                "#,
        title,description,completed
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub async fn get_todo(db : &SqlitePool,_todo_id : i64) -> Result<Option<TodoRecord>>{
    let mut conn = db.acquire().await?;
    let res = sqlx::query_as!(TodoRecord,r#"
        SELECT * FROM todos
    "#)
    .fetch_one(&mut *conn)
    .await;

    if res.is_err(){
        return Ok(None);
    }
    else {
        return Ok(Some(res.unwrap()));
    }

}

pub async fn update_todo(db : &SqlitePool,todo_id : i64, patch : UpdateTodo) -> Result<()>{
    let todo = get_todo(db, todo_id).await;
    let mut conn = db.acquire().await?;
    let UpdateTodo{completed,description,title} = patch;
    if let Ok(todo) = todo{
        if let Some(todo) = todo{
            let _res = sqlx::query(r#"
                UPDATE todos SET title = ?1, description = ?2, completed = ?3 WHERE id = ?4"#)
                .bind(title.unwrap_or(todo.title))
                .bind(description.unwrap_or(todo.description))
                .bind( completed.unwrap_or(todo.completed))
                .bind(todo_id)
            .execute(&mut *conn).await?;
        }
    }

    Ok(())
}

pub async fn delete_todo(db : &SqlitePool,todo_id : i64) -> Result<()>{
    let mut conn = db.acquire().await?;
    sqlx::query!(r#"
        DELETE FROM todos WHERE id = ?1
    "#, todo_id)
    .execute(&mut *conn)
    .await?;
    Ok(())
}

pub async fn list_todos(db : &SqlitePool) -> Result<Option<Vec<TodoRecord>>>{

    let mut conn = db.acquire().await?;
    let v : Vec<TodoRecord> = sqlx::query_as!(TodoRecord,r#"
        SELECT * FROM todos
    "#)
    .fetch_all(&mut *conn)
    .await?;
    if v.len() == 0{
        return Ok(None);
    }
    else {
        return Ok(Some(v));
    }

}


mod test{
    
    
    
    
    #[tokio::test]
    async fn test_db_init() -> Result<()>{
        let db = init_db().await?;
        
        Ok(())
    }

}