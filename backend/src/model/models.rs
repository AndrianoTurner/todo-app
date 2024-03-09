use serde::{Serialize,Deserialize};
use sqlx::prelude::FromRow;

#[derive(Serialize,Deserialize,Debug,FromRow)]
pub struct Todo{
    pub title : String,
    pub description : String,
    pub completed : bool
}
#[derive(Serialize,Deserialize,Debug,FromRow)]
pub struct TodoRecord{
    pub id : i64,
    pub title : String,
    pub description : String,
    pub completed : bool
}
#[derive(Serialize,Deserialize,Debug,FromRow)]
pub struct UpdateTodo{
    pub title : Option<String>,
    pub description : Option<String>,
    pub completed : Option<bool>
}

