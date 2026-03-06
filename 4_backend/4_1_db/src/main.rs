use anyhow::Result ;


fn main() ->Result<()> {
    // Загрузка в переменные окружения сожержимого файла .env
    dotenv::dotenv()
        // Преобразует Result<PathBuf, Error> в Option<PathBuf>.
        // В случае ошибки (например отсутствия файла .env) None
        .ok() ;


    Ok(())
}

