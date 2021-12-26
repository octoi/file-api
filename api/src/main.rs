use bytes::BufMut;
use futures::TryStreamExt;
use std::convert::Infallible;
use uuid::Uuid;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

#[tokio::main]
async fn main() {}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject();
    })?;

    for part in parts {
        if part.name() == "file" {
            let content_type = part.content_type();
            let file_extension;

            match content_type {
                Some(file_type) => {
                    let file_type_split: Vec<&str> = file_type.clone().split("/").collect();

                    if file_type_split.len() == 2 {
                        file_extension = file_type.split("/").collect()[1];
                    } else {
                        eprintln!("invalid file type found: {}", file_type);
                        return Err(warp::reject::reject());
                    }
                }
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = part
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                }).await
                .map_err(|e| {
                    eprintln!("Failed to read file: {}", e);
                    warp::reject::reject()
                })?;

            let file_name = format!("./files/{}.{}", Uuid::new_v4().to_string(), file_extension);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprintln!("Error writing file : {}", e);
                warp::reject::reject()
            })?;

            println!("created file: {}", file_name);
        }
    }

    Ok("success")
}

