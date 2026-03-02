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

use mozjpeg::{
        Compress,
        ColorSpace
    };

// Является ли path_to_img url
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
         ) ->Result
                <
                ()
                > {

    let start = Instant::now();

    println!("1) path_to_img: {}", path_to_img) ;

    let img_bytes = if path_to_img_is_url(path_to_img) { // Это url

        println!("2)") ;

        // Создание асинхронного HTTP клиента
        let http_client = match reqwest::Client::builder() 
                                .connect_timeout(std::time::Duration::from_secs(conf_now.time_out as u64))
                                .timeout(std::time::Duration::from_secs(conf_now.time_out as u64))
                                .build() {
            Ok(v) => v,
            Err(err) => return Err(err.into())
        } ;

        println!("3)") ;

        let resp = http_client
                    .get(path_to_img)
                    .send()
                    .await? ;

        println!("4)") ;

        if ! resp.status().is_success() {
            return Err(anyhow::anyhow!("Error loading URL: {}, code: {}", path_to_img, resp.status()));
        }

        println!("5)") ;

        resp
            .bytes()
            .await?
            .to_vec()
    } else {  // Это файл
        println!("6)") ;
        match tokio::fs::read(path_to_img).await {
            Ok(v) => v,
            Err(err) => {
                return Err(anyhow::anyhow!("{} from: {}", err, path_to_img))
            }
        }
    } ;

    println!("7)") ;

    let img = match
            image::load_from_memory(&img_bytes) {
        Ok(v) => v,
        Err(err) => {
            return Err(anyhow::anyhow!("{} from: {}", err, path_to_img));
        }
    } ;

    let img_format = match image::guess_format(&img_bytes) {
        Ok(imf) if imf == ImageFormat::Jpeg || imf == ImageFormat::Png=> imf,
        Ok(imf_other) => {
            return Err(anyhow::anyhow!("Unsupported image format: {:?} from {}", imf_other, path_to_img)) ;
        },
        Err(err) => {
            return Err(anyhow::anyhow!("{} from: {}", err,path_to_img));
        }
    } ;

    match img_format {
        ImageFormat::Jpeg => {
            let rgb_img = img.to_rgb8() ;
            let pixels = rgb_img.as_raw() ;
            let mut comp = Compress::new(ColorSpace::JCS_RGB) ;
            comp.set_quality(conf_now.img_quality as f32) ;
            let mut comp_started = match comp.start_compress(Vec::new()) {
                Ok(v) => v,
                Err(err) => {
                    return Err(anyhow::anyhow!("{} from: {}", err, path_to_img)) ;
                }
            } ;
            
            if let Err(err) = comp_started.write_scanlines(pixels) {
                return Err(anyhow::anyhow!("{} from: {}", err, path_to_img)) ;
            }

            let compressed_data = match comp_started.finish() {
                Ok(v) => v,
                Err(err) => {
                    return Err(anyhow::anyhow!("{} from: {}", err, path_to_img));
                }
            } ;

        },
        ImageFormat::Png => {

        },
        _ => {
            return Err(anyhow::anyhow!("Unsupported image format: {:?} from {}", img_format, path_to_img)) ;            
        },
    }

    /*
let raw_bytes = resp.bytes().await?;

// 2. Сжали через mozjpeg
let processed_jpeg = compress_image_mozjpeg(&raw_bytes, conf_now.img_quality)?;

// 3. Сохранили результат
let mut out_file = tokio::fs::File::create(conf_now.img_output_dir.join("result.jpg")).await?;
tokio::io::copy(&mut &processed_jpeg[..], &mut out_file).await?;    
     */
    println!("8)") ;

    Ok(())
}