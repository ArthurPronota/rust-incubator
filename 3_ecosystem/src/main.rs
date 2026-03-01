mod args ;
mod conf_load;
mod downloader ;

use std::time::Instant ;
use log::{
        //debug,
        error, 
        info, 
        //warn,
        // error
};

use futures::stream::{
        self,
        StreamExt,
} ;

use tokio::task ;

use std::process ;

/// Вывод в лог: Total processing time ...
fn tot_proc_time(st_time: &Instant) {
    let elapsed = st_time.elapsed();
    info!("Total processing time: {:?}", elapsed);
}

fn main() /*->anyhow::Result<()>*/{
    let start_time = Instant::now();

    // Инициализирует глобальный логгер с помощью env logger.
    // Поддержка переменной окружения RUST_LOG
    env_logger::init();

    // Получить аргументы командной строки
    let args = args::get_args() ;
    println!("args: {:?}", args) ;

    // сформировать текущую конфигурацию
    let conf_now = match conf_load::load_config(&args) {
        Ok(v) => v,
        Err(err) => {
            error!("{}", err) ;
            //return Err(err);
            process::exit(1) ;
        }
    } ;
    println!("conf_now: {:#?}", conf_now) ;


    // Софрмировать список изображений для загрузки
    let list_images = match args::get_list_all_images(&args) {
        Ok(v) => v,
        Err(err) => {
            error!("{}", err) ;
            //return Err(err);
            process::exit(1) ;
        }
    } ;
    println!("list_images: {}\n{:#?}", list_images.len(), list_images) ;

    if list_images.is_empty() {
        error!("No images to process") ;
        //return Ok(());
        process::exit(1) ;
    }

    /*
    // Создание асинхронного HTTP клиента
    let client = match reqwest::Client::builder() 
                            .connect_timeout(std::time::Duration::from_secs(conf_now.time_out as u64))
                            .timeout(std::time::Duration::from_secs(conf_now.time_out as u64))
                            .build() {
        Ok(v) => v,
        Err(err) => {
            error!("{}", err) ;
            //return Err(err.into());
            process::exit(1) ;
        }
    } ;
     */

    /*
    async fn download_pages(
     */

    let res_load = 
            stream::iter(&list_images)
                .map(|u_f_in| {
                    //let client = client.clone() ;
                    let u_f = u_f_in.to_string() ;

                    task::spawn(async move {

                    })
                 }
                ) ;



    tot_proc_time(&start_time) ;

    //Ok(())
}
/*
Запуск:
cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --config-file tmp_contents\config.toml
Или так:
more tmp_contents\from_stdin.txt|cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --config-file tmp_contents\config.toml --stdin
Или так:
cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --stdin --config-file tmp_contents\config.toml < tmp_contents\from_stdin.txt

// Структура проекта
image-optimizer/
├── Cargo.toml
├── .env.example
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── processor.rs
│   ├── downloader.rs
│   └── error.rs
├── tests/
│   ├── integration_test.rs
│   └── test_images/
├── examples/
│   └── basic_usage.rs
└── README.md

// ------------------------------------------------------
// 1. Cargo.toml
[package]
name = "image-optimizer"
version = "0.1.0"
edition = "2021"
description = "CLI tool for image optimization (JPEG/PNG metadata removal and compression)"
authors = ["Your Name <email@example.com>"]

[dependencies]
# CLI and configuration
clap = { version = "4.0", features = ["derive", "env"] }
config = "0.13"
dotenv = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Async runtime
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"

# Image processing
image = "0.24"
mozjpeg = "0.2"  # Better JPEG compression
png = "0.17"

# HTTP client
reqwest = { version = "0.11", features = ["json", "stream"] }

# Logging
env_logger = "0.10"
log = "0.4"
humantime = "2.1"
humantime-serde = "1.1"

# Progress bar
indicatif = "0.17"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# URL parsing
url = "2.3"

# File operations
tempfile = "3.3"

[dev-dependencies]
tempfile = "3.3"
assert_cmd = "2.0"
predicates = "3.0"
wiremock = "0.5"
rand = "0.8"

// ------------------------------------------------------
// 2. Основной файл (src/main.rs)
mod cli;
mod config;
mod processor;
mod downloader;
mod error;

use clap::Parser;
use log::{info, error, debug, warn};
use std::time::Instant;
use tokio::time::timeout;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Инициализация логирования с поддержкой RUST_LOG
    env_logger::init();
    
    let start_time = Instant::now();
    
    // Парсинг аргументов CLI
    let cli = cli::Cli::parse();
    
    // Загрузка конфигурации с возрастающим приоритетом
    let config = config::load_config(&cli)?;
    info!("Configuration loaded: {:?}", config);
    
    // Получение списка изображений для обработки
    let images = cli::get_image_list(&cli).await?;
    
    if images.is_empty() {
        warn!("No images to process");
        return Ok(());
    }
    
    info!("Found {} images to process", images.len());
    
    // Создание выходной директории
    tokio::fs::create_dir_all(&config.output_dir).await?;
    
    // Настройка ограничений
    let semaphore = Arc::new(tokio::sync::Semaphore::new(config.concurrency));
    let rate_limiter = if config.rate_limit > 0 {
        Some(Arc::new(governor::RateLimiter::direct(
            governor::Quota::per_second(std::num::NonZeroU32::new(config.rate_limit).unwrap())
        )))
    } else {
        None
    };
    
    // Создание HTTP клиента
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    // Обработка изображений
    let processor = processor::ImageProcessor::new(config.clone(), client);
    
    let tasks: Vec<_> = images.into_iter().map(|image| {
        let processor = processor.clone();
        let semaphore = semaphore.clone();
        let rate_limiter = rate_limiter.clone();
        
        tokio::spawn(async move {
            // Применяем семафор для ограничения конкурентности
            let _permit = semaphore.acquire().await.unwrap();
            
            // Применяем rate limiting если нужно
            if let Some(limiter) = rate_limiter {
                limiter.until_ready().await;
            }
            
            // Обработка изображения с таймаутом
            timeout(
                std::time::Duration::from_secs(config.timeout_seconds),
                processor.process_image(image)
            ).await
        })
    }).collect();
    
    // Сбор результатов
    let pb = indicatif::ProgressBar::new(tasks.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    let mut results = Vec::new();
    for task in tasks {
        match task.await {
            Ok(Ok(Ok(result))) => {
                pb.inc(1);
                info!("✅ {}", result);
                results.push(result);
            }
            Ok(Ok(Err(e))) => {
                pb.inc(1);
                error!("❌ {}", e);
            }
            Ok(Err(e)) => {
                error!("❌ Task timeout: {}", e);
            }
            Err(e) => {
                error!("❌ Task join error: {}", e);
            }
        }
    }
    
    pb.finish_with_message("Processing complete");
    
    let elapsed = start_time.elapsed();
    info!("Total processing time: {:?}", elapsed);
    info!("Successfully processed: {}/{}", results.len(), tasks.len());
    
    Ok(())
}

// ------------------------------------------------------
// 3. Модуль CLI (src/cli.rs)

use clap::Parser;
use std::path::{Path, PathBuf};
use tokio::io::{AsyncBufReadExt, BufReader};
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input images (files or URLs)
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    pub images: Option<Vec<String>>,
    
    /// File containing list of images (one per line)
    #[clap(short, long)]
    pub file: Option<PathBuf>,
    
    /// Output directory for processed images
    #[clap(short, long, env = "IMG_OUTPUT_DIR")]
    pub output: Option<PathBuf>,
    
    /// JPEG quality (1-100)
    #[clap(short, long, env = "IMG_QUALITY", default_value = "85")]
    pub quality: u8,
    
    /// Number of concurrent operations
    #[clap(short, long, env = "IMG_CONCURRENCY", default_value = "4")]
    pub concurrency: usize,
    
    /// Rate limit for downloads (requests per second)
    #[clap(long, env = "IMG_RATE_LIMIT", default_value = "10")]
    pub rate_limit: u32,
    
    /// Timeout in seconds for each operation
    #[clap(long, env = "IMG_TIMEOUT", default_value = "30")]
    pub timeout: u64,
    
    /// Configuration file path
    #[clap(short, long, env = "IMG_CONFIG")]
    pub config: Option<PathBuf>,
    
    /// Read images from STDIN
    #[clap(long)]
    pub stdin: bool,
}

/// Получение списка изображений из различных источников
pub async fn get_image_list(cli: &Cli) -> Result<Vec<String>> {
    let mut images = Vec::new();
    
    // Из аргументов CLI
    if let Some(cli_images) = &cli.images {
        images.extend(cli_images.clone());
    }
    
    // Из файла
    if let Some(file_path) = &cli.file {
        let content = tokio::fs::read_to_string(file_path)
            .await
            .context("Failed to read input file")?;
        
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                images.push(trimmed.to_string());
            }
        }
    }
    
    // Из STDIN
    if cli.stdin {
        let stdin = tokio::io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();
        
        while let Some(line) = lines.next_line().await? {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                images.push(trimmed.to_string());
            }
        }
    }
    
    // Удаление дубликатов
    images.sort();
    images.dedup();
    
    Ok(images)
}

/// Проверка, является ли строка URL
pub fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Получение имени файла из URL или пути
pub fn get_filename(input: &str) -> String {
    if is_url(input) {
        if let Ok(url) = url::Url::parse(input) {
            if let Some(segments) = url.path_segments() {
                if let Some(last) = segments.last() {
                    if !last.is_empty() {
                        return last.to_string();
                    }
                }
            }
        }
        // Генерация имени из URL если не удалось извлечь
        format!("image_{}.jpg", input.replace(|c: char| !c.is_alphanumeric(), "_"))
    } else {
        Path::new(input)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
}

// ------------------------------------------------------
// 4. Модуль конфигурации (src/config.rs)

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub output_dir: PathBuf,
    pub quality: u8,
    pub concurrency: usize,
    pub rate_limit: u32,
    pub timeout_seconds: u64,
    pub strip_metadata: bool,
    pub resize_max_width: Option<u32>,
    pub resize_max_height: Option<u32>,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Auto,
    Jpeg,
    Png,
    WebP,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./optimized"),
            quality: 85,
            concurrency: 4,
            rate_limit: 10,
            timeout_seconds: 30,
            strip_metadata: true,
            resize_max_width: None,
            resize_max_height: None,
            output_format: OutputFormat::Auto,
        }
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Auto
    }
}

/// Загрузка конфигурации с возрастающим приоритетом:
/// 1. Файл конфигурации (низший приоритет)
/// 2. Переменные окружения
/// 3. Аргументы CLI (высший приоритет)
pub fn load_config(cli: &crate::cli::Cli) -> Result<Config> {
    let mut config = Config::default();
    
    // 1. Загрузка из файла если указан
    if let Some(config_path) = &cli.config {
        if config_path.exists() {
            config = load_from_file(config_path)?;
        }
    }
    
    // 2. Переменные окружения уже применены через clap
    //    (благодаря env в атрибутах)
    
    // 3. Применение CLI аргументов (высший приоритет)
    if let Some(output) = &cli.output {
        config.output_dir = output.clone();
    }
    
    config.quality = cli.quality.clamp(1, 100);
    config.concurrency = cli.concurrency.max(1);
    config.rate_limit = cli.rate_limit;
    config.timeout_seconds = cli.timeout;
    
    Ok(config)
}

fn load_from_file(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read config file")?;
    
    match path.extension().and_then(|e| e.to_str()) {
        Some("toml") => toml::from_str(&content)
            .context("Failed to parse TOML config"),
        Some("json") => serde_json::from_str(&content)
            .context("Failed to parse JSON config"),
        Some("yaml") | Some("yml") => serde_yaml::from_str(&content)
            .context("Failed to parse YAML config"),
        _ => Err(anyhow::anyhow!("Unsupported config file format")),
    }
}

// ------------------------------------------------------
// 5. Модуль обработки изображений (src/processor.rs)

use crate::cli;
use crate::config::Config;
use crate::downloader::Downloader;
use crate::error::ProcessingError;
use image::{ImageFormat, ImageOutputFormat};
use log::{debug, info};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Clone)]
pub struct ImageProcessor {
    config: Config,
    client: reqwest::Client,
    downloader: Downloader,
}

impl ImageProcessor {
    pub fn new(config: Config, client: reqwest::Client) -> Self {
        let downloader = Downloader::new(client.clone());
        Self { config, client, downloader }
    }
    
    pub async fn process_image(&self, input: String) -> Result<ProcessedImage, ProcessingError> {
        let start = Instant::now();
        
        // Определяем источник и загружаем если нужно
        let image_data = if cli::is_url(&input) {
            debug!("Downloading from URL: {}", input);
            self.downloader.download(&input).await?
        } else {
            debug!("Reading local file: {}", input);
            tokio::fs::read(&input).await
                .map_err(|e| ProcessingError::IoError(e))?
        };
        
        // Загрузка изображения
        let img = image::load_from_memory(&image_data)
            .map_err(|e| ProcessingError::ImageError(e))?;
        
        // Определяем формат
        let format = self.determine_format(&input, &image_data)?;
        
        // Оптимизация
        let optimized = self.optimize_image(&img, format)?;
        
        // Сохранение
        let output_path = self.get_output_path(&input, format);
        tokio::fs::write(&output_path, &optimized).await
            .map_err(|e| ProcessingError::IoError(e))?;
        
        // Вычисляем статистику
        let original_size = image_data.len();
        let optimized_size = optimized.len();
        let savings = if original_size > 0 {
            ((original_size - optimized_size) as f64 / original_size as f64 * 100.0) as u8
        } else {
            0
        };
        
        let elapsed = start.elapsed();
        info!("Processed {} in {:?}", input, elapsed);
        
        Ok(ProcessedImage {
            input,
            output_path,
            original_size,
            optimized_size,
            savings,
            processing_time: elapsed,
        })
    }
    
    fn determine_format(&self, input: &str, data: &[u8]) -> Result<ImageFormat, ProcessingError> {
        match self.config.output_format {
            crate::config::OutputFormat::Auto => {
                // Определяем по входному файлу или содержимому
                if cli::is_url(input) {
                    // По расширению в URL
                    if input.to_lowercase().ends_with(".png") {
                        Ok(ImageFormat::Png)
                    } else {
                        Ok(ImageFormat::Jpeg)
                    }
                } else {
                    // По содержимому файла
                    image::guess_format(data)
                        .map_err(|_| ProcessingError::UnknownFormat)
                }
            }
            crate::config::OutputFormat::Jpeg => Ok(ImageFormat::Jpeg),
            crate::config::OutputFormat::Png => Ok(ImageFormat::Png),
            crate::config::OutputFormat::WebP => Ok(ImageFormat::WebP),
        }
    }
    
    fn optimize_image(&self, img: &image::DynamicImage, format: ImageFormat) -> Result<Vec<u8>, ProcessingError> {
        let mut result = Vec::new();
        
        // Ресайз если нужно
        let img = if let (Some(w), Some(h)) = (self.config.resize_max_width, self.config.resize_max_height) {
            img.resize(w, h, image::imageops::FilterType::Lanczos3)
        } else {
            img.clone()
        };
        
        match format {
            ImageFormat::Jpeg => {
                let mut encoder = mozjpeg::Encoder::new(&mut result, mozjpeg::ColorSpace::JCS_YCbCr);
                encoder.set_quality(self.config.quality as f32 / 100.0);
                // Strip metadata
                if self.config.strip_metadata {
                    encoder.set_markers(mozjpeg::Marker::NONE);
                }
                
                let rgb_img = img.to_rgb8();
                encoder.set_size(rgb_img.width() as usize, rgb_img.height() as usize);
                let mut comp = encoder.start_compress().unwrap();
                comp.write_scanlines(&rgb_img).unwrap();
                comp.finish().unwrap();
            }
            ImageFormat::Png => {
                let mut encoder = png::Encoder::new(&mut result, img.width(), img.height());
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);
                
                if self.config.strip_metadata {
                    encoder.set_trns(vec![]); // Удаление tRNS
                }
                
                let rgba_img = img.to_rgba8();
                let mut writer = encoder.write_header().unwrap();
                writer.write_image_data(&rgba_img).unwrap();
            }
            _ => {
                // Для других форматов используем стандартный энкодер
                img.write_to(&mut result, format)
                    .map_err(|e| ProcessingError::ImageError(e))?;
            }
        }
        
        Ok(result)
    }
    
    fn get_output_path(&self, input: &str, format: ImageFormat) -> PathBuf {
        let base_name = cli::get_filename(input);
        let extension = match format {
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Png => "png",
            ImageFormat::WebP => "webp",
            _ => "jpg",
        };
        
        let file_name = if base_name.contains('.') {
            // Замена расширения
            let stem = Path::new(&base_name)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            format!("{}.{}", stem, extension)
        } else {
            format!("{}.{}", base_name, extension)
        };
        
        self.config.output_dir.join(file_name)
    }
}

#[derive(Debug)]
pub struct ProcessedImage {
    pub input: String,
    pub output_path: PathBuf,
    pub original_size: usize,
    pub optimized_size: usize,
    pub savings: u8,
    pub processing_time: std::time::Duration,
}

impl std::fmt::Display for ProcessedImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} ({} bytes → {} bytes, {}% savings, {:?})",
            self.input,
            self.output_path.display(),
            self.original_size,
            self.optimized_size,
            self.savings,
            self.processing_time
        )
    }
}

// ------------------------------------------------------
// 6. Модуль загрузки (src/downloader.rs)

use crate::error::ProcessingError;
use bytes::Bytes;
use futures::stream::StreamExt;
use log::debug;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Downloader {
    client: reqwest::Client,
    stats: Arc<Mutex<DownloadStats>>,
}

#[derive(Default)]
struct DownloadStats {
    total_bytes: usize,
    total_requests: usize,
    failed_requests: usize,
}

impl Downloader {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
            stats: Arc::new(Mutex::new(DownloadStats::default())),
        }
    }
    
    pub async fn download(&self, url: &str) -> Result<Vec<u8>, ProcessingError> {
        debug!("Downloading: {}", url);
        
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| ProcessingError::HttpError(e))?;
        
        if !response.status().is_success() {
            return Err(ProcessingError::HttpStatus(response.status().as_u16()));
        }
        
        // Получение размера если доступен
        let content_length = response
            .content_length()
            .unwrap_or(0) as usize;
        
        // Стриминговая загрузка с прогрессом
        let mut stream = response.bytes_stream();
        let mut buffer = Vec::with_capacity(content_length);
        let mut downloaded = 0;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| ProcessingError::HttpError(e))?;
            buffer.extend_from_slice(&chunk);
            downloaded += chunk.len();
            
            // Обновление статистики
            let mut stats = self.stats.lock().await;
            stats.total_bytes += chunk.len();
        }
        
        // Обновление статистики запросов
        let mut stats = self.stats.lock().await;
        stats.total_requests += 1;
        
        debug!("Downloaded {} bytes from {}", downloaded, url);
        
        Ok(buffer)
    }
    
    pub async fn get_stats(&self) -> (usize, usize, usize) {
        let stats = self.stats.lock().await;
        (stats.total_bytes, stats.total_requests, stats.failed_requests)
    }
}

/// Параллельная загрузка нескольких файлов
pub async fn download_many(
    downloader: &Downloader,
    urls: &[String],
    concurrency: usize,
) -> Vec<Result<Vec<u8>, ProcessingError>> {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
    let mut handles = Vec::new();
    
    for url in urls {
        let downloader = downloader.clone();
        let url = url.clone();
        let semaphore = semaphore.clone();
        
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            downloader.download(&url).await
        });
        
        handles.push(handle);
    }
    
    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => results.push(Err(ProcessingError::TaskError(e.to_string()))),
        }
    }
    
    results
}

// ------------------------------------------------------
// 7. Модуль ошибок (src/error.rs)

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("HTTP status error: {0}")]
    HttpStatus(u16),
    
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[error("Unknown image format")]
    UnknownFormat,
    
    #[error("Task error: {0}")]
    TaskError(String),
    
    #[error("Timeout error")]
    Timeout,
    
    #[error("Rate limit exceeded")]
    RateLimit,
}

impl From<tokio::time::error::Elapsed> for ProcessingError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        ProcessingError::Timeout
    }
}

// ------------------------------------------------------
// 8. Интеграционные тесты (tests/integration_test.rs)

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};
use std::fs;

#[tokio::test]
async fn test_cli_help() {
    let mut cmd = Command::cargo_bin("image-optimizer").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("Options"));
}

#[tokio::test]
async fn test_process_local_image() {
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");
    
    // Создание тестового изображения
    let test_image = create_test_jpeg();
    let image_path = temp_dir.path().join("test.jpg");
    fs::write(&image_path, &test_image).unwrap();
    
    let mut cmd = Command::cargo_bin("image-optimizer").unwrap();
    cmd.arg("--images")
        .arg(image_path.to_str().unwrap())
        .arg("--output")
        .arg(output_dir.to_str().unwrap())
        .arg("--quality")
        .arg("80")
        .assert()
        .success();
    
    // Проверка, что выходной файл создан
    assert!(output_dir.exists());
    assert!(output_dir.join("test.jpg").exists());
}

#[tokio::test]
async fn test_download_from_url() {
    let mock_server = MockServer::start().await;
    
    // Создание тестового изображения
    let test_image = create_test_jpeg();
    
    // Настройка мок-сервера
    Mock::given(method("GET"))
        .and(path("/image.jpg"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(test_image))
        .mount(&mock_server)
        .await;
    
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");
    
    let url = format!("{}/image.jpg", mock_server.uri());
    
    let mut cmd = Command::cargo_bin("image-optimizer").unwrap();
    cmd.arg("--images")
        .arg(&url)
        .arg("--output")
        .arg(output_dir.to_str().unwrap())
        .arg("--quality")
        .arg("80")
        .assert()
        .success();
    
    // Проверка, что файл загружен и обработан
    assert!(output_dir.exists());
    assert!(output_dir.join("image.jpg").exists());
}

#[tokio::test]
async fn test_config_file() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    let output_dir = temp_dir.path().join("custom_output");
    
    // Создание конфигурации
    let config_content = format!(
        r#"
        output_dir = "{}"
        quality = 90
        concurrency = 8
        strip_metadata = true
        "#,
        output_dir.to_str().unwrap()
    );
    
    fs::write(&config_path, config_content).unwrap();
    
    let test_image = create_test_jpeg();
    let image_path = temp_dir.path().join("test.jpg");
    fs::write(&image_path, &test_image).unwrap();
    
    let mut cmd = Command::cargo_bin("image-optimizer").unwrap();
    cmd.arg("--images")
        .arg(image_path.to_str().unwrap())
        .arg("--config")
        .arg(config_path.to_str().unwrap())
        .assert()
        .success();
    
    // Должен использовать output_dir из конфига
    assert!(output_dir.join("test.jpg").exists());
}

#[tokio::test]
async fn test_stdin_input() {
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");
    
    let test_image = create_test_jpeg();
    let image_path = temp_dir.path().join("test.jpg");
    fs::write(&image_path, &test_image).unwrap();
    
    let mut cmd = Command::cargo_bin("image-optimizer").unwrap();
    cmd.arg("--stdin")
        .arg("--output")
        .arg(output_dir.to_str().unwrap())
        .write_stdin(image_path.to_str().unwrap())
        .assert()
        .success();
}

fn create_test_jpeg() -> Vec<u8> {
    // Создание простого тестового JPEG
    let mut img = image::ImageBuffer::new(100, 100);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([
            (x * 255 / 100) as u8,
            (y * 255 / 100) as u8,
            128,
        ]);
    }
    
    let mut bytes = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Jpeg(85))
        .unwrap();
    
    bytes
}

// ------------------------------------------------------
// 9. Пример конфигурации (.env.example)

# Image Optimizer Configuration

# Output directory for processed images
IMG_OUTPUT_DIR=./optimized

# JPEG quality (1-100)
IMG_QUALITY=85

# Number of concurrent operations
IMG_CONCURRENCY=4

# Rate limit for downloads (requests per second)
IMG_RATE_LIMIT=10

# Timeout in seconds for each operation
IMG_TIMEOUT=30

# Logging level (debug, info, warn, error)
RUST_LOG=info

# Optional: Resize dimensions (uncomment to enable)
# IMG_MAX_WIDTH=1920
# IMG_MAX_HEIGHT=1080

// ------------------------------------------------------
// 10. Пример использования (examples/basic_usage.rs)

use image_optimizer::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Пример использования библиотеки напрямую
    let config = config::Config {
        output_dir: PathBuf::from("./my_output"),
        quality: 90,
        concurrency: 4,
        rate_limit: 5,
        timeout_seconds: 30,
        strip_metadata: true,
        resize_max_width: Some(1920),
        resize_max_height: Some(1080),
        output_format: config::OutputFormat::Auto,
    };
    
    let client = reqwest::Client::new();
    let processor = processor::ImageProcessor::new(config, client);
    
    let images = vec![
        "test1.jpg".to_string(),
        "https://example.com/image.jpg".to_string(),
    ];
    
    for image in images {
        match processor.process_image(image).await {
            Ok(result) => println!("✅ {}", result),
            Err(e) => eprintln!("❌ {}", e),
        }
    }
    
    Ok(())
}

// ------------------------------------------------------
// 11. README.md

# Image Optimizer CLI

CLI tool for removing metadata from JPEG/PNG images and reducing their size - a simplified alternative to tinyjpg.com.

## Features

- ✅ Remove metadata from JPEG and PNG images
- ✅ Optimize image size while maintaining quality
- ✅ Support for local files and remote URLs
- ✅ Configurable concurrency for batch processing
- ✅ Configurable output directory
- ✅ Adjustable JPEG quality (1-100)
- ✅ Configuration with increasing priority (file → env → CLI)
- ✅ Detailed logging with RUST_LOG support
- ✅ Processing time tracking
- ✅ Rate limiting for downloads
- ✅ Comprehensive test suite

## Installation

```bash
cargo install image-optimizer
```

Or build from source:
```bash
git clone https://github.com/yourusername/image-optimizer
cd image-optimizer
cargo build --release
./target/release/image-optimizer --help
```

// ------------------------------------------------------
// Basic usage

# Process local files
image-optimizer --images image1.jpg image2.png --output ./optimized

# Process URLs
image-optimizer --images https://example.com/image.jpg --output ./optimized

# Read from file
image-optimizer --file images.txt --output ./optimized

# Read from STDIN
cat images.txt | image-optimizer --stdin --output ./optimized

// -----------------------
// Advanced options

# Set quality and concurrency
image-optimizer --images *.jpg --output ./optimized --quality 80 --concurrency 8

# With rate limiting
image-optimizer --images urls.txt --output ./optimized --rate-limit 5

# Use configuration file
image-optimizer --config config.toml --images image.jpg

// ------------------------------------------------------
// Configuration

Create a config.toml file:

output_dir = "./optimized"
quality = 85
concurrency = 4
rate_limit = 10
timeout_seconds = 30
strip_metadata = true
resize_max_width = 1920
resize_max_height = 1080
output_format = "Auto"

// ------------------------------------------------------
// Logging

Control log levels with RUST_LOG:

RUST_LOG=debug image-optimizer --images image.jpg
RUST_LOG=image_optimizer=info,reqwest=warn image-optimizer --images image.jpg

// ------------------------------------------------------
// Testing

# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

// ------------------------------------------------------
// License

    MIT


## Заключение

Этот инструмент предоставляет:

1. **Гибкий ввод** - файлы, URL, STDIN, файл со списком
2. **Конфигурация с приоритетами** - файл → env → CLI
3. **Детальное логирование** с RUST_LOG
4. **Время выполнения** каждой операции
5. **Ограничение конкурентности** и rate limiting
6. **Модульные и интеграционные тесты**
7. **Удаление метаданных** и оптимизация размера

Для запуска тестов используйте `cargo test`. Для сборки релиза `cargo build --release`.
    
*/



/*
use rand::{self, prelude::*};

fn main() {
    let mut rng = rand::thread_rng();

    // 1. Случайное число (целое или с плавающей точкой)
    let n: u32 = rng.gen_range(0..100);
    println!("n: {}", n) ;

    let x: f64 = rng.r#gen(); // от 0.0 до 1.0
    println!("x: {}", x) ;

    // 2. Случайный выбор из списка
    let choices = ["Rust", "Go", "C++"];
    if let Some(&lang) = choices.choose(&mut rng) {
        println!("Выбран язык: {}", lang);
    }

    // 3. Перемешивание вектора
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("{:?}", nums) ;
}

// ------------------------------------

Что можно настроить через RUST_LOG
Синтаксис RUST_LOG довольно гибкий и позволяет точно управлять фильтрацией.

Глобальный уровень: Просто укажите уровень, например, RUST_LOG=info. Тогда будут выводиться все сообщения уровня info и выше (warn, error) из всех модулей проекта .

bash
RUST_LOG=info ./my_app
Поуровнево для модулей: Можно задать разные уровни для разных модулей (крейтов). Формат такой: path::to::module=level. Уровень по умолчанию указывается первым .

bash
RUST_LOG="warn,my_crate::module1=info,my_crate::module2=debug" ./my_app
Эта команда установит:

По умолчанию уровень warn для всех.

Для my_crate::module1 — уровень info.

Для my_crate::module2 — уровень debug.

Отключение логирования: Используйте псевдо-уровень off, чтобы полностью отключить вывод для всего приложения или конкретного модуля .

bash
seset RUST_LOG=debug
cargo run
Доступные уровни логирования (от наиболее до наименее подробного): error, warn, info, debug, trace . Регистр букв в названиях уровней не имеет значения .    
 */