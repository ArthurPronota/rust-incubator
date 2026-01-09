use std::env ;
use std::borrow::Cow ;

const DEFAULT_PATH: &str = "/etc/app/app.conf" ;

fn main() {

    let mut args = env::args()
                        .skip(1)
                        ;
    let mut config_path: Option<Cow<'_, str>> = None ;

    while let Some(arg) = args.next() {
        if arg == "--conf" {
            if let Some(val) = args.next() {
                if ! val.is_empty() {
                    config_path = Some(Cow::Owned(val)) ;
                }
                break;
            }
        }
    }

    if config_path.is_none() {
        if let Ok(val) = env::var("APP_CONF") {
            if ! val.is_empty() {
                config_path = Some(Cow::Owned(val)) ;
            }
        }
    }

    if config_path.is_none() {
        config_path = Some(Cow::Borrowed(DEFAULT_PATH)) ;
    }

    println!(
        "Path: {}",
        config_path
            .as_deref() 
            .unwrap()
    ) ;

}
