Шаг 3.9: Аргументы командной строки, переменные среды и конфигурации
===================================================================

__Estimated time__: 1 day




## CLI

[Rust] предоставляет простой итератор [`std::env::Arg`] для доступа к аргументам командной строки, передаваемым программе.


Однако в большинстве случаев для этого требуется более продвинутый инструмент, который предоставляет флаги `--help` и `--version` «из коробки», а также удобный способ настройки и описания пользовательских параметров/флагов для создания собственного [CLI (интерфейса командной строки)][CLI]. Для таких случаев существует хорошо известный крейт [`clap`] в экосистеме [Rust].

В нем есть [`derive` Cargo feature][6] (ранее — крейт [`structopt`]), позволяющая определять [CLI] _декларативным и чистым способом_.

Чтобы лучше понять и освоить инструменты командной строки [CLI] в экосистеме [Rust], ознакомьтесь со следующими материалами:
- [Rust Book: 12.1. Accepting Command Line Arguments][1]
- [Official `std::env::Arg` docs][`std::env::Arg`]
- [Official `clap` crate docs][`clap`]
- [Pavlo Myroniuk: Rust Clap recipes][9]




## Environment variables


[Rust] предоставляет общие примитивы в [`std::env`] для работы с [переменными окружения][2] в виде строк.


Однако в большинстве случаев вам нужно работать с типизированными данными, а не с необработанными строками. Аналогично [`clap`] для CLI, в экосистеме [Rust] существует крейт [`envy`], который использует [`serde`] в качестве фасада и позволяет считывать данные из [переменных окружения][2] _декларативным и чистым способом_ (благодаря поддержке [атрибутов serde][4]).


Стоит отметить, что крейт [`clap`] также [способен анализировать переменные окружения][7], что очень удобно, когда речь идет о поддержке [CLI] с помощью [переменных окружения][2].

Наконец, следует упомянуть крейт [`dotenv`]. Он устанавливает [переменные окружения][2] на основе содержимого [`.env` файла][8], что является широко распространенным соглашением для упрощения настройки окружения и позволяет избежать ручного объявления всех необходимых переменных окружения каждый раз при запуске какой-либо программы. Этот крейт особенно _полезен в разработке_ (рассмотрите также [`rs-env`] и [`direnv`] для лучшего опыта разработки).


Чтобы лучше понять и ознакомиться с инструментами [переменные среды][2] в экосистеме [Rust], прочтите следующее:
- [Rust Book: 12.5. Working with Environment Variables][3]
- [Official `std::env` docs][`std::env`]
- [Official `envy` crate docs][`envy`]
- [Official `dotenv` crate docs][`dotenv`]




## Configuration

Для работы с конфигурациями в экосистеме [Rust] существует хорошо известный крейт [`config`], который упрощает создание и использование иерархически типизированных структур конфигурации в соответствии с принципами [12-factor] архитектуры.


> Параметр `Config` позволяет задать набор параметров по умолчанию, а затем расширить их путем объединения конфигурационных данных из различных источников:
> - Переменные окружающей среды
> - Строковые литералы в общеизвестных форматах
> - Ещё один экземпляр `Config`

> - Files: TOML, JSON, YAML, INI, RON, JSON5 and custom ones defined with `Format` trait
> - Manual, programmatic override (via a `.set` method on the `Config` instance)
>
> Additionally, `Config` supports:
> - Live watching and re-reading of configuration files
> - Deep access into the merged configuration via a path syntax
> - Deserialization via `serde` of the configuration or any subset defined via a path

To better understand and be familiar with [`config`] crate's design, concepts, usage and features, read through:
- [Official `config` crate docs][`config`]
- [`config` crate examples][5]




## Task

Write a simple program which prints out its actual configuration. Configuration should be implemented as a typed hierarchical structure, which is able to parse from a specified file and/or environment variables. 

The following priority should be applied (in ascending order) when merging:
1. Default values declared directly in [Rust] sources;
2. Values read from TOML file;
3. Values set by environment variables with `CONF_` prefix.

[CLI] of the program should look like:
```
$ cargo run -- --help
step_3_9 0.1.0
Prints its configuration to STDOUT.

USAGE:
    step_3_9 [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Enables debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --conf <conf>         Path to configuration file [env: CONF_FILE=]  [default: config.toml]
```




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What are the benefits of having strongly-type configuration?
- Why environment variables are useful for configuring an application? What is the main use-case for them?
- How is [`config`] crate really useful? Why should we it and cannot just deserialize a file into structs via [`serde`]?




[`clap`]: https://docs.rs/clap
[`config`]: https://docs.rs/config
[`direnv`]: https://direnv.net
[`dotenv`]: https://docs.rs/dotenv
[`envy`]: https://docs.rs/envy
[`rs-env`]: https://github.com/sysid/rs-env
[`serde`]: https://docs.rs/serde
[`std::env`]: https://doc.rust-lang.org/std/env/index.html
[`std::env::Arg`]: https://doc.rust-lang.org/std/env/struct.Args.html
[`structopt`]: https://docs.rs/structopt
[12-factor]: https://12factor.net/config
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
[2]: https://en.wikipedia.org/wiki/Environment_variable
[3]: https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html
[4]: https://serde.rs/attributes.html#field-attributes
[5]: https://github.com/mehcode/config-rs/tree/master/examples
[6]: https://docs.rs/clap/latest/clap#example
[7]: https://docs.rs/clap/latest/clap/parser/enum.ValueSource.html#variant.EnvVariable
[8]: https://github.com/bkeepers/dotenv#usage
[9]: https://tbt.qkation.com/posts/clap-recipes
