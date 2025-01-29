// use std::env; //unable us to read the environment variable
// extern crate dotenv;
// // allow us to read the .env file 
// use dotenv::dotenv; //allow us to load the environment variables



// use futures::stream::TryStreamExt; //kind of type in asynchronous request of R<T, E>

// use mongodb::{
//     bson::{doc, extjson::de::Error, oid::ObjectId, DateTime},
//     results::{DeleteResult, InsertOneResult, UpdateResult},
//     Client //entry position for connecting to mongodb
//     , Collection,
// }; //modules and types for interacting with mongodb


// use crate::models::todo_model::Todo;

use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{self, doc, extjson::de::Error, oid::ObjectId}, 
    results::{  DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use futures::stream::TryStreamExt; //add this
use crate::models::todo_model::Todo;

pub struct MongoRepo{
    col : Collection<Todo>,
}

impl MongoRepo{

    pub async fn init() -> Self{
        dotenv().ok();
        let uri = match env::var("MONGOURI"){
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rust_todo");
        let col : Collection<Todo> = db.collection("Todo");
        MongoRepo {col}
    }

    pub async fn create_todo(&self, new_todo: Todo) -> Result<InsertOneResult, Error>{

        let new_doc = Todo{
            id : None, 
            title : new_todo.title,
            tag : new_todo.tag,
            description : new_todo.description,
            state : new_todo.state,
            position : new_todo.position,
            created_at : new_todo.created_at,
            date : new_todo.date
        };

        let todo = self
            .col
            .insert_one(new_doc)
            .await
            .ok()
            .expect("Error creating todo");

        Ok(todo)
    }

    pub async fn get_all_todos(&self) -> Result<Vec<Todo>, mongodb::error::Error> {
        let filter = doc! {};
        let mut cursor = self.col.find(filter).await?; // No second argument

        let mut todos = Vec::new();

        while let Some(todo) = cursor.try_next().await? {
        todos.push(todo);
        } 
        Ok(todos)
    }

    pub async fn get_todo(&self, id : &String) -> Result<Todo, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id" : obj_id};
        let todo_detail = self
            .col
            .find_one(filter)
            .await
            .ok()
            .expect("Error getting user's details");
        Ok(todo_detail.unwrap())
    }


    pub async fn update_todo(&self, id : &String, new_todo : Todo)-> Result<UpdateResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let update_doc = doc! {
            "$set": {
                "position": new_todo.position,
                "description": new_todo.description,
                "tag": new_todo.tag.to_string(),  // assuming you have a method for serialization of Tag
                "state": new_todo.state.to_string(),  // same for State enum
                "date": new_todo.date,
                "created_at": new_todo.created_at,
            }
        };
        let updated_doc = self
            .col.update_one(filter,update_doc)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }


    pub async fn delete_todo(&self, id : &String)-> Result<DeleteResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id" : obj_id};
        let todo_detail = self
            .col
            .delete_one(filter)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(todo_detail)
    }
}






