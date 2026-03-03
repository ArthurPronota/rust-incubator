use crate::conf_load::ConfigLoad ;

use std::{process, str::Bytes, time::{Duration, Instant}};

use anyhow::{
        //Error,
        Result,
    } ;

use futures::StreamExt;
use image::{
        self,
        ImageFormat,
        //ImageOutputFormat
    };

use mozjpeg::{
        Compress,
        ColorSpace
    };

use oxipng::{
    self,
    optimize_from_memory, 
    Options
};

use sha2::{
        Sha256, 
        Digest
    };

use log::info ;

/*
use img_parts::{
        ImageICC, jpeg::{Jpeg, JpegSegment}
} ;
 */

use web_image_meta::{
            jpeg,
            png,
        } ;

/// Является ли path_to_img url
fn path_to_img_is_url(path_to_img: &str) ->bool {
    if path_to_img.starts_with("https://") || path_to_img.starts_with("http://") {
        true
    } else {
        false
    }
}

/// Загрузка изображения
pub async fn download_img(
            path_to_img: &str,
            conf_now: &ConfigLoad
         ) ->Result<()> {

    let start = Instant::now();

    let mut img_bytes = if path_to_img_is_url(path_to_img) { // Это url

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

        if conf_now.rate_limit == 0 { // нет ограничений по скорости загрузки
            resp
                .bytes()
                .await?
                .to_vec()
        } else {
            /*
            use tokio::io::AsyncReadExt;
            use throttled_reader::ThrottledReader;
            use futures_util::TryStreamExt;
            use futures::TryStreamExt;

            let bytes_stream = 
                    resp
                        .bytes_stream()
                        .map_err(|err|
                            std::io::Error::new(std::io::ErrorKind::Other, err)
                            //anyhow::anyhow!("{}", err)
                        ) ;
            let reader = tokio_util::io::StreamReader::new(bytes_stream);
            
            let mut throttled_reader = ThrottledReader::new(reader) ;

            let mut buffer = Vec::new();

            throttled_reader.read(buf)

            throttled_reader.read_to_end(&mut buffer).await? ;

            buffer
            */

            let mut stream = resp.bytes_stream() ;
            let mut downloaded = 0 ;
            let start = Instant::now() ;

            let mut all_data = Vec::new();

            while let Some(chunk) = stream.next().await {
                let chunk = 
                        chunk
                            .map_err(|err|
                                anyhow::anyhow!("{} from: {}", err, path_to_img)
                            ) ? ;
                downloaded += chunk.len() ;

                all_data.extend_from_slice(&chunk);

                let elapsed = start.elapsed().as_secs_f64();
                let expected_bytes = (elapsed * (conf_now.rate_limit * 1024) as f64) as usize ;
                if downloaded > expected_bytes {
                    tokio::time::sleep(
                        Duration::from_secs_f64(
                            (downloaded - expected_bytes) as f64 / (conf_now.rate_limit * 1024) as f64
                        )
                    )
                    .await ;
                }
            }

            all_data
        }
    } else {  // Это файл
        match tokio::fs::read(path_to_img).await {
            Ok(v) => v,
            Err(err) => {
                return Err(anyhow::anyhow!("{} from: {}", err, path_to_img))
            }
        }
    } ;

    let img_format = match image::guess_format(&img_bytes) {
        Ok(imf) if imf == ImageFormat::Jpeg || imf == ImageFormat::Png=> imf,
        Ok(imf_other) => {
            return Err(anyhow::anyhow!("Unsupported image format: {:?} from {}", imf_other, path_to_img)) ;
        },
        Err(err) => {
            return Err(anyhow::anyhow!("{} from: {}", err, path_to_img));
        }
    } ;

    // удаление метаданных из файла изображения
    img_bytes = match img_format {
        ImageFormat::Jpeg => {
            jpeg::clean_metadata(&img_bytes)
                .map_err(|err|
                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                )?
        },
        ImageFormat::Png => {
            png::clean_chunks(&img_bytes)
                .map_err(|err|
                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                )?
        },
        _ => {
            return Err(anyhow::anyhow!("Unsupported image format: {:?} from {}", img_format, path_to_img)) ;            
        },
    } ;

    // изменение качества изображения
    if conf_now.img_quality != 100 {
        // let img_bytes_out 
        img_bytes = match img_format {
            ImageFormat::Jpeg => {
                let img = 
                            image::load_from_memory(&img_bytes)
                                .map_err(|err|
                                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                                )? ;

                let rgb_img = img.to_rgb8() ;
                let (width, height) = rgb_img.dimensions() ;
                let pixels = rgb_img.as_raw() ;

                let mut comp = Compress::new(ColorSpace::JCS_RGB) ;
                comp.set_size(width as usize, height as usize);
                comp.set_quality(conf_now.img_quality as f32) ;
            
                let mut comp_started = 
                            comp
                                .start_compress(Vec::new())
                                .map_err(|err|
                                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                                )? ;
            
                comp_started
                    .write_scanlines(pixels)
                    .map_err(|err|
                        anyhow::anyhow!("{} from: {}", err, path_to_img)
                    )? ;

                comp_started
                    .finish()
                    .map_err(|err| 
                        anyhow::anyhow!("{} from: {}", err, path_to_img)
                    )?
            },
            ImageFormat::Png => {
                let mut options_img = Options::from_preset((conf_now.img_quality as u64 * 7 / 100) as u8) ;

                options_img.interlace = None ;

                optimize_from_memory(&img_bytes, &options_img)
                    .map_err(|err|
                        anyhow::anyhow!("{} from: {}", err, path_to_img)  
                    )?
            },
            _ => {
                return Err(anyhow::anyhow!("Unsupported image format: {:?} from {}", img_format, path_to_img)) ;            
            },
        } ;
    }

    // создатём имя выходного файла
    let mut hasher = Sha256::new() ;

    hasher.update(path_to_img);

    let result_hasher = hasher.finalize() ;

    let out_file_path = conf_now
                                    .img_output_dir
                                    .join(
                                        format!("{}.{}",
                                            hex::encode(result_hasher),
                                            img_format.extensions_str()[0].to_lowercase()
                                        )
                                    ) ;

    let mut out_file = 
                tokio::fs::File::create(&out_file_path)
                    .await
                    .map_err(|err|
                        anyhow::anyhow!("{}, cannot create file: {:?}", err, out_file_path)
                    )?;

    tokio::io::copy(
                //&mut &img_bytes_out[..], 
                &mut &img_bytes[..],
                &mut out_file
            )
            .await
            .map_err(|err|
                anyhow::anyhow!("{}, cannot copy data to file: {:?}", err, out_file_path)
            )?;

    let elapsed = start.elapsed();
    info!("Processed {} in {:?}", path_to_img, elapsed);

    Ok(())
}