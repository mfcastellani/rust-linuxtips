use axum::extract::Path;
use crate::app::models::todo::Todo;

pub async fn get_todo_path_handler(Path(id): Path<String>) -> axum::Json<serde_json::Value> {
    let request_url = format!("https://jsonplaceholder.typicode.com/todos/{id}");
    let empty_json = serde_json::Value::Object(serde_json::Map::new());

    let ret = match reqwest::get(&request_url).await {
        Ok(todo) => {
            match todo.json::<Todo>().await {
                Ok(todo) => {
                    tracing::info!("batata {:?}", todo);
                    // ia fazer mais coisas
                    serde_json::to_value(todo).unwrap_or_else(|_| {
                        empty_json
                    })
                },
                Err(_) => empty_json,
            }
        },
        Err(_) => empty_json,
    };

    axum::Json(ret)
}
