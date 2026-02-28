use clap::Parser ;
use std::{
        fs, 
        io::{
            self, 
            BufRead
        },
        time::Instant,
        path::PathBuf
    } ;
use log::{
        //debug,
        info,
        error
    };
use anyhow::{Result} ;
//use num_cpus ;

// Аргументы CLI
#[derive(
    // Генерирует реализацию парсера.
    Parser,
    Debug
)]
pub struct Args {
    /// List of images as URLs or files: https://o.i/img1.jpg c:\img2.png
    #[clap(
        // Задает сокращенную версию аргумента.
        //short = 'i',
        short,
        // Задает длинную версию аргумента.
        //long = "images",
        long,
        // Заполнитель для значения аргумента в справочном сообщении/примере использования.
        value_name = "Img",
        // Разрешить группировку нескольких значений с помощью разделителя.
        value_delimiter = ' ',
        // Указывает количество аргументов, обрабатываемых за одно вхождение.
        num_args = 1..,
     )
    ]
    images: Option<Vec<String>>,

    /// Path to the file containing the list of images.
    #[clap(
        short = 'f',
        long = "images-file",
        value_name = "FILE",
     )
    ]
    imgs_file:  Option<PathBuf>,

    /// Number of images processed concurrency (1..10000)
    #[clap(
        short = 'c',
        long = "img-concurrency",
        value_name = "NUMBER",
        env = "IMG_CONCURRENCY",
        //default_value_t = (num_cpus::get() as u16).into(),
        value_parser = clap::value_parser!(u16).range(1..=10000),
     )
    ]
    img_concurrency:    Option<u16>,

    /// Output directory for storing processed images.
    #[clap(
        short = 'o',
        long = "output-dir",
        value_name = "DIR",
        env = "IMG_OUTPUT_DIR",
     )
    ]
    img_output_dir:     Option<PathBuf>,

    /// Output quality of processed images (1..100)
    #[clap(
        short = 'q',
        long = "quality-img",
        value_name = "NUMBER",
        env = "IMG_QUALITY",
        value_parser = clap::value_parser!(u8).range(1..=100),
        //default_value_t = 60,
     )
    ]
    img_quality:        Option<u8>,

    /// Rate limit for image downloads (KiB)
    #[clap(
        short = 'r',
        long = "rlim",
        value_name = "NUMBER",
        env = "IMG_RATE_LIMIT",
        value_parser = clap::value_parser!(u32).range(1..),
    )]
    rate_limit:     Option<u32>,

    /// Timeout for loading one image (1..255)
    #[clap(
        short = 't',
        long = "timeout",
        value_name = "NUMBER",
        env = "IMG_TIMEOUT",
        value_parser = clap::value_parser!(u8).range(1..255),
     )
    ]
    time_out:       Option<u8>,

    /// Read a list of images from STDIN
    // Если нет параметра то false, если естьто true
    #[clap(
        long = "stdin",
     )
    ]
    img_stdin:      bool,

}

/// Разбор параметров CLI
pub fn get_args() ->Args {
    Args::parse()
}

/// Получить список всех изображений изо всех доступных источниклв:
/// 1. командная строка 
/// 2. файл с изображениями
/// 3. STDIN
/// 
/// Если получаем данные из STDIN напрямую (без перенаправления из файла)
/// в конце ввода нажать на Ctrl+D
pub fn get_list_all_images(cl_arg: &Args) ->Result<Vec<String>>{

    let start_time = Instant::now();

    // результирующий список изображений
    let mut list_images = vec![];
    
    // Получение изображений из перечня командной строки
    if let Some(v) = &cl_arg.images {
        list_images.extend(
        v
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().to_string())
            .collect::<Vec<_>>()
        ) ;
    }

    // Получение изображений из файла с изображениями
    if let Some(f_img) = &cl_arg.imgs_file {
        // Файл существует
        if f_img.exists() {
            list_images.extend(
                fs::read_to_string(f_img)?
                    .lines()
                    .filter(|x| !x.trim().is_empty())
                    .map(|x| x.trim().to_string())
                    .collect::<Vec<_>>()
            ) ;
        } 
        // файла не существует
        else {
            error!("The image file: {:?} does not exist.", f_img) ;
            //return Err(anyhow!("The image file: {:?} does not exist.", f_img)) ;
        }
    }

    // Получить список изображений из STDIN
    if cl_arg.img_stdin {
        list_images.extend(            
            io::stdin()
            .lock()
            .lines()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().to_string())
            .collect::<Vec<_>>()
        ) ;
    }

    list_images.sort();
    
    list_images.dedup();

    let elapsed = start_time.elapsed();
    info!("Total processing get_list_all_images(): {:?}", elapsed);

    Ok(list_images)
}