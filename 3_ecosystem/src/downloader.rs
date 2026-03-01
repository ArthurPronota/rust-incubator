use crate::conf_load::ConfigLoad ;

use std::time::Instant;

use anyhow::{
        Error,
        Result
    } ;

use image::{
        self,
        ImageFormat,
        ImageOutputFormat
    };


// является ли path_to_img url
fn path_to_img_is_url(path_to_img: &str) ->bool {
    if path_to_img.starts_with("http://") || path_to_img.starts_with("http://") {
        true
    } else {
        false
    }
}

/// Загрузка изображения
async fn download_img(
            path_to_img: &str,
            conf_now: &ConfigLoad
         ) ->Result<()> {

    let start = Instant::now();

    // Это url
    let img_bytes = if path_to_img_is_url(path_to_img) {

        // Создание асинхронного HTTP клиента
        let http_client = match reqwest::Client::builder() 
                                .connect_timeout(std::time::Duration::from_secs(conf_now.time_out as u64))
                                .timeout(std::time::Duration::from_secs(conf_now.time_out as u64))
                                .build() {
            Ok(v) => v,
            Err(err) => return Err(err.into())
        } ;

        let resp = http_client
                    .get(path_to_img)
                    .send()
                    .await? ;

        if ! resp.status().is_success() {
            return Err(anyhow::anyhow!("Error loading URL: {}, code: {}", path_to_img, resp.status()));
        }

        resp
            .bytes()
            .await?
            .to_vec()
    } else {  // Это файл
        tokio::fs::read(&path_to_img).await?
    } ;

    let img = image::load_from_memory(&img_bytes)?;

    Ok(())
}