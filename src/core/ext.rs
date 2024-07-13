use actix_multipart::Multipart;
use actix_web::{web, Error as ActixError};
use std::io::Write;
use futures_util::TryStreamExt as _;
use uuid::Uuid;

pub trait MultipartExt {

    async fn save_file(&mut self) -> Result<(), ActixError>;
}

impl MultipartExt for Multipart {
    async fn save_file(&mut self) -> Result<(), ActixError> {
        while let Some(mut field) = self.try_next().await? {
            let content_disposition = field.content_disposition().unwrap();

            let filename = content_disposition
                .get_filename()
                .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

            let filepath = format!("./tmp/{filename}");

            // File::create is blocking operation, use thread pool
            let mut f = web::block(|| std::fs::File::create(filepath)).await??;

            while let Some(chunk) = field.try_next().await? {
                // filesystem operations are blocking, we have to use thread pool
                f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
            }
        }

        Ok(())
    }
}
