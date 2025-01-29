

use crate::{models::todo_model::Todo, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put, web::{Data, Json, Path}, HttpResponse
};
use mongodb::bson::{oid::ObjectId,  DateTime};



#[get("/todos")]
pub async fn get_all_todos(db : Data<MongoRepo>) -> HttpResponse {
    match db.get_all_todos().await {
        Ok(todos) => HttpResponse::Ok().json(todos),  // If successful, return the todos in a JSON response
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()), // On error, return 500
    }
}

#[post("/todo")]
pub async fn create_todo(db : Data<MongoRepo>, new_todo : Json<Todo>) -> HttpResponse{
    println!("i am triggerd");
    let data = Todo{
        id : None,
        title : new_todo.title.to_owned(),
        description : new_todo.description.to_owned(),
        tag : new_todo.tag.clone(),
        position : new_todo.position,
        state : new_todo.state.clone(),
        created_at : Some(DateTime::now()),
        date : Some(DateTime::now())
    };
    let todo_details = db.create_todo(data).await;
    match todo_details{
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[get("/todo/{id}")]
pub async  fn get_todo(db:Data<MongoRepo>, path : Path<String>)-> HttpResponse{
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid id")
    }

    let todo_details = db.get_todo(&id).await;
    match  todo_details{
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[delete("/todo/{id}")]
pub async  fn delete_todo(db : Data<MongoRepo>, path : Path<String>) -> HttpResponse{
    let id = path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("invlaid id");
    }
    let result = db.delete_todo(&id).await;
    match result{
        Ok(res) =>{
            if res.deleted_count == 1{
                return HttpResponse::Ok().json("Todo deleted successfully");
            }else{
                return HttpResponse::NotFound().json("User iwth specidired id not found");
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}


#[put("/todo/{id}")]
pub async fn update_todo(db : Data<MongoRepo>, path : Path<String>,
new_todo : Json<Todo>)-> HttpResponse{
    let id = path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("invalid id");
    }
    let data = Todo{
        id : Some(ObjectId::parse_str(&id).unwrap()),
        title : new_todo.title.to_owned(),
        description : new_todo.description.to_owned(),
        tag : new_todo.tag.clone(),
        state : new_todo.state.clone(),
        created_at : new_todo.created_at,
        date : Some(DateTime::now()),
        position : new_todo.position
        
    };
    let update_result = db.update_todo(&id, data).await;
    match update_result{
        Ok(update) =>{
            if update.matched_count == 1{
                let updated_todo_info = db.get_todo(&id).await;
                return match updated_todo_info{
                    Ok(todo) => HttpResponse::Ok().json(todo),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string())
                };
            }else{
                return HttpResponse::NotFound().body("No user found with the give id ");
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }

}





// pub async fn create_todo(db: Data<MongoRepo>, todo: String) -> HttpResponse {
//     println!("I am triggered");

//     // Convert the incoming String to a Todo object
//     let convert_todo: Result<Todo, serde_json::Error> = serde_json::from_str(&todo);
//     println!("converted todo : {:?}",convert_todo);

//     let new_todo = match convert_todo {
//         Ok(new_todo) => {
//             // Create a new Todo object
//             Todo {
//                 id: None,
//                 title: new_todo.title.to_owned(),
//                 description: new_todo.description.to_owned(),
//                 tag: new_todo.tag.clone(),
//                 position: new_todo.position,
//                 state: new_todo.state.clone(),
//                 created_at: Some(DateTime::now()), // Use Utc::now() for the timestamp
//                 date: Some(DateTime::now()),
//             }
//         }
//         Err(_) => {
//             // If the JSON is invalid, return a BadRequest response
//             return HttpResponse::BadRequest().body("Invalid JSON format");
//         }
//     };

//     // Create the Todo in the database
//     let todo_details = db.create_todo(new_todo).await;
//     match todo_details {
//         Ok(todo) => HttpResponse::Ok().json(todo),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }