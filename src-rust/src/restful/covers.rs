use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
use log::{info, warn};
use warp::{filters::multipart::FormData, reject::Rejection, reply::Reply};

/// Handles uploading a rom cover to the cache.
pub async fn upload_cover(rom_id: String, cover_cache_dir: String, form: FormData) -> Result<impl Reply, Rejection> {
  let mut parts = form.into_stream();

  while let Some(Ok(p)) = parts.next().await {
    if p.name() == "file" {
      let content_type = p.content_type();
      let file_ending;

      match content_type {
        Some(file_type) => match file_type {
          "image/png" => {
            file_ending = "png";
          }
          v => {
            warn!("invalid file type found: {}", v);
            return Err(warp::reject::reject());
          }
        },
        None => {
          warn!("file type could not be determined");
          return Err(warp::reject::reject());
        }
      }

      let value = p
        .stream()
        .try_fold(Vec::new(), |mut vec, data| {
          vec.put(data);
          async move { Ok(vec) }
        })
        .await
        .map_err(|e| {
          warn!("error reading file: {}", e);
          warp::reject::reject()
        })?;

      let file_name = format!("{}/{}.{}", cover_cache_dir, rom_id, file_ending);

      tokio::fs::write(&file_name, value).await.map_err(|e| {
        warn!("error writing file: {}", e);
        warp::reject::reject()
      })?;

      info!("created file: {}", file_name);
    }
  }

  return Ok("success");
}

/// Handles deleting a rom cover from the cache.
pub async fn delete_cover(rom_id: String, cover_cache_dir: String) -> Result<impl Reply, Rejection> {
  let file_name = format!("{}/{}.png", cover_cache_dir, rom_id);

  tokio::fs::remove_file(&file_name).await.map_err(|e| {
    warn!("error deleting file: {}", e);
    warp::reject::reject()
  })?;

  return Ok("success");
}