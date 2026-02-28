use clap::Parser ;
use std::path::PathBuf ;
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
