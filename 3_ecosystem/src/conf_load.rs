use serde::{
        //Serialize,
        Deserialize
    } ;

use std::{
    fs, path::{
        Path, PathBuf
    }
} ;

use validator::Validate ;

use anyhow::Result ;

/// Конфигурация загрузки
#[derive(
    //Serialize,
    Deserialize,
    Validate,
    Debug
 )
]
pub struct ConfigLoad {
    /// Количество одновременно обрабатываемых картинок от 1 до 10_000
    #[validate(range(min = 1, max = 10_000))]
    img_concurrency:    u16,
    
    /// Выходной директорий для хранения оброботанных картинок.
    img_output_dir:     PathBuf,

    /// Качество обрабатываемых картинок от 1 до 100
    #[validate(range(min = 1, max = 100))]
    img_quality:        u8,

    /// Ограничение скорости загрузки изображений в KiB
    /// 
    /// 0 - отсутствие ограничений
    rate_limit:     u32,

    /// Timeout для загрузки одного изображения в секундах от 1 до 255.
    #[validate(range(min = 1, max = 255))]
    time_out:       u8,
}

// Создание значения по умолчанию для ConfigLoad
impl Default for ConfigLoad {
    fn default() -> Self {
        Self { 
            img_concurrency: 10, 
            img_output_dir: "img_output".into(), 
            img_quality: 30, 
            rate_limit: 0, 
            time_out: 30,
        }
    }
}

/// Загрузка конфигурации
pub fn load_config(args: &crate::args::Args) ->Result<ConfigLoad>{
    // Создание конфигурации по умолчанию
    let mut conf_now = ConfigLoad::default() ;

    // Проверка полученной конфигурации
    conf_now.validate().map_err(|err|
        anyhow::anyhow!("Config error: {}", err)
    )? ;

    // Проверка наличия конфигурационного файла из аргументов CL
    if let Some(conf_file) = &args.config_file_path {
        if conf_file.exists() {
            // Получение контента из конфигуоационнго файла
            let top_cont = std::fs::read_to_string(conf_file)? ;
            // Загрузка Новой конфигуоации
            conf_now = toml::from_str::<ConfigLoad>(&top_cont)? ;
            // Проверка полученной конфигурации
            conf_now.validate().map_err(|err|
                anyhow::anyhow!("Config error: {}", err)
            )? ;            
        }
    }

    // Установить img_concurrency из CI
    if let Some(v) = args.img_concurrency {
        conf_now.img_concurrency = v ;
    }

    // Установка выходной директория для хранения оброботанных картинок.
    if let Some(v) = &args.img_output_dir {
        conf_now.img_output_dir = v.to_path_buf() ;
    }

    // Установка качества обрабатываемых картинок
    if let Some(v) = args.img_quality {
        conf_now.img_quality = v ;
    }

    // Установка ограничения скорости загрузки изображений в KiB
    if let Some(v) = args.rate_limit {
        conf_now.rate_limit = v ;
    }

    // Установка Timeout для загрузки одного изображения в секундах
    if let Some(v) = args.time_out {
        conf_now.time_out = v ;
    }

    // Создание директория для хранения обработанных картинок
    if ! Path::new(&conf_now.img_output_dir).is_dir() {
        fs::create_dir_all(&conf_now.img_output_dir)? ;
    }

    Ok(conf_now)
}