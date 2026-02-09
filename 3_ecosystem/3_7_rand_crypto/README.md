Шаг 3.7: Случайность и криптография
=====================================

__Estimated time__: 1 day




## Случайность

Для генерации случайных значений в экосистеме [Rust] существует крейт [`rand`], предоставляющий __унифицированный интерфейс__ и множество __реализаций генератора случайных значений__ с различными гарантиями качества и производительности__.


[The Rust Rand Book] не только объясняет, как использовать примитивы крейта [`rand`], но и является хорошим введением в [основы проблемы генерации случайных значений][1] и [как она решается в современном мире][2]. Прочитайте её, чтобы понять, какие примитивы следует использовать в разных ситуациях:
- когда целью является результативность;
- когда целью является криптографическая безопасность и высокое качество статистических данных;
- Что подходит для общего пользования.

Один из наиболее распространенных случаев, когда необходимо генерировать случайные значения, — это генерация универсальных уникальных идентификаторов (таких как [UUID]). К счастью, в [Rust] уже есть крейт [`uuid`], который реализует [все версии спецификации UUID][3].


Дополнительная информация:
- [Aleksey Kladov: On Random Numbers][16]
- [Orhun Parmaksız: Zero-dependency random number generation in Rust][17]


## Шифрование и подписание

Хотя на данный момент в [Rust] нет криптографической библиотеки, в его экосистеме есть множество хорошо реализованных (и все еще развивающихся) библиотек для различных целей.


### [`ring`]

Библиотека [`ring`] реализует основной набор криптографических операций, предоставляемых через простой в использовании (и трудно для неправильно используемый) API. Она начиналась как подмножество известной библиотеки [BoringSSL] (строка «ring» является подстрокой «Bo_ring_SSL»), поэтому наследует часть её кода и регулярно объединяет изменения из неё.


[`ring`] ориентирован на криптографию общего назначения. Если вам нужны только базовые криптографические примитивы — это то, что вам нужно. Используйте его, когда вам необходимо создать:
- цифровая подпись;
- просто зашифровать незашифрованные данные;
- вывод ключа;
- и так далее...

Если вам требуются более сложные реализации (например, проверка сертификатов WebPKI [X.509] или криптографические протоколы, такие как [TLS], [SSH]), рассмотрите возможность использования других библиотек (которые часто построены на основе [`ring`]).


### [dalek]

В то время как [`ring`] ориентирован на предоставление универсальных криптографических примитивов, крейты [dalek] предоставляют лишь несколько, но ориентированы на реализацию лучших теоретических примитивов.

Если вы собираетесь создавать что-то, использующее только высокоуровневые криптографические примитивы (например, [Curve25519] для подписи и проверки), вам стоит попробовать [dalek].


### [AWS] Libcrypto

[`aws-lc-rs`] — это криптографическая библиотека, совместимая с [`ring`], использующая криптографические операции, предоставляемые [AWS-LC].


Мотивация [предоставленная авторами][18] вполне понятна сама по себе:
> Разработчикам на [Rust] все чаще требуется развертывать приложения, соответствующие криптографическим требованиям правительств США и Канады. Мы оценили, как обеспечить криптографию, соответствующую стандарту [FIPS], на идиоматичном и производительном языке [Rust], построенном на основе нашего решения AWS-LC. Мы обнаружили, что популярная библиотека [`ring`] удовлетворяет большую часть криптографических потребностей сообщества [Rust], но не отвечает потребностям разработчиков, предъявляющих требования [FIPS]. Наша цель — предложить замену `ring`, которая обеспечит поддержку [FIPS] и будет совместима с API `ring`. Разработчики на [Rust] с заданными криптографическими требованиями смогут беспрепятственно интегрировать `aws-lc-rs` в свои приложения и развертывать их в регионах [AWS].

Дополнительная информация:
- [Sean McGrai: Introducing AWS Libcrypto for Rust, an Open Source Cryptographic Library for Rust][19]




## Хэширование


### Сырые хеш-функции

Базовый набор исходных [криптографических хеш-функций][11] представлен в коллекции крейтов [RustCrypto/hashes].


__НЕ используйте их для хеширования паролей!__ Вместо этого рассмотрите возможность использования какого-либо алгоритма хеширования паролей ([Argon2], [bcrypt], [scrypt] или [PBKDF2]).

### Хэширование паролей

Существует аналогичная коллекция библиотек [RustCrypto/password-hashing] для хеширования паролей.

Однако в нем отсутствует реализация алгоритмов [Argon2] и [bcrypt], поэтому их [следует найти][12] и выбрать по своему усмотрению. Что касается [Argon2], то, похоже, наиболее зрелым на данный момент является крейт [`rust-argon2`].



## Константное время сравнения

Для [сравнения за постоянное время][13] в [Rust] рассмотрите возможность использования крейта [`subtle`] из [dalek].


## TLS / SSL

Для использования [TLS] в экосистеме [Rust] в настоящее время существуют два распространенных решения:

### [`native-tls`]

Крейт [`native-tls`] представляет собой абстракцию над платформенно-специфичными реализациями [TLS]. Он использует [SChannel] в Windows (через крейт [`schannel`]), Secure Transport в OSX (через крейт [`security-framework`]), [OpenSSL] на всех остальных платформах (через крейт [`openssl`]) и предоставляет унифицированный интерфейс для использования этих библиотек.

Хотя это решение требует наличия внешних библиотек, не относящихся к Rust, оно является стабильным и основано на готовых к использованию реализациях TLS.

### [`rustls`]

Крейт [`rustls`] — это реализация [TLS] на чистом [Rust]. Он построен на основе крейтов [`ring`] и [`webpki`].


Несмотря на то, что это довольно многофункциональное решение, оно [не имеет хорошей поддержки старой и устаревшей криптографии][14] и пока не имеет стабильной версии. Рассмотрите возможность его использования, если устаревшие технологии для вас не имеют значения.



## Больше материалов для чтения

- [Sylvain Kerkour: Overview of the Rust cryptography ecosystem][15] (Tue, Aug 24, 2021)
- [Sahil Mahapatra: Axum Backend Series: Implement JWT Access Token][20]




## Task

Implement the following functions:
1. `generate_password()`: generates random password of given length and symbols set;
2. `select_rand_val()`: retrieves random element from a given slice;
3. `new_access_token()`: generates unique cryptographically secure random value in `a-zA-Z0-9` symbols set and has exactly `64` symbols.
4. `get_file_hash()`: returns SHA-3 hash of a file specified by its path.
5. `hash_password()`: returns [Argon2] password hash for a given password.




## Questions


После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- [В чём заключается главный компромисс при генерации случайных чисел? Как это применяется на практике?](https://github.com/ArthurPronota/rust-incubator/tree/main/3_ecosystem/3_7_rand_crypto#в-чём-заключается-главный-компромисс-при-генерации-случайных-чисел-как-это-применяется-на-практике)

- What is symmetric cryptography? What is asymmetric cryptography? Which benefits does each one have? 
- What is signing in asymmetric cryptography? What is encryption in asymmetric cryptography? How do they work given the same private and public keys?
- What is hash function? What is password hashing? Why is it not enough to use just a raw hash function for password hashing?
- What is constant-time comparison? When and why it should be used?
- Which are options of using [TLS] in [Rust]? Which advantages and disadvantages does each one have?

<hr>

### В чём заключается главный компромисс при генерации случайных чисел? Как это применяется на практике?

В Rust главный компромисс при генерации случайных чисел (RNG) заключается в выборе между криптографической стойкостью и скоростью выполнения. Этот баланс выражается формулой: __Безопасность__ <-> __Предсказуемость__ <-> __Производительность__. 

1. #### Суть компромисса

Криптографическая стойкость (CSPRNG)

- Пример: StdRng или прямые вызовы ОС (getrandom).
- Плюс: Числа невозможно предсказать. Даже если злоумышленник узнает миллион предыдущих чисел, он не вычислит следующее.
- Минус: Это медленно. Требует обращения к энтропии операционной системы (системные вызовы) или сложных математических преобразований (ChaCha20).

Скорость (Non-Crypto PRNG)

- Пример: SmallRng, Pcg64, Xoshiro2048.
- Плюс: Экстремально быстро (в десятки раз быстрее CSPRNG). Работает полностью в пользовательском пространстве, используя простые битовые сдвиги и умножения.
- Минус: Предсказуемость. Зная небольшую последовательность чисел, можно восстановить внутреннее состояние генератора («зерно» / seed) и предсказать все будущие значения.

2. #### Как это применяется на практике?

В экосистеме Rust (через крейт [rand](https://docs.rs/rand/latest/rand/)) выбор автоматизирован под конкретные сценарии:

А. По умолчанию: Безопасность прежде всего

Метод `thread_rng()` возвращает генератор, который является:

- Криптографически стойким (сейчас это обычно ChaCha12).
- Периодически обновляемым (автоматически подмешивает новую энтропию из ОС).
    - Применение: Генерация паролей, токенов сессий, ключей шифрования.

Б. Игры и симуляции: Скорость

Если вам нужно генерировать миллионы частиц в секунду или шум Перлина, `thread_rng()` будет «бутылочным горлышком».

- Решение: Используйте SmallRng.
- Применение: Процедурная генерация миров, частицы, Монте-Карло симуляции.

В. Тестирование: Воспроизводимость

Иногда вам нужно, чтобы «случайность» была одинаковой при каждом запуске (например, для отладки бага).

- Решение: Использование фиксированного зерна (SeedableRng).
- Применение: Регрессионные тесты, Property-based testing (как мы обсуждали в proptest).

3. #### Сводная таблица выбора

|Тип генератора|Крейт/Метод|Стойкость|Скорость|Использование|
|--------------|-----------|---------|--------|-------------|
|ThreadRng|rand::thread_rng|Да|Средняя|Общие задачи, Web, Security|
|SmallRng|rand::rngs::SmallRng|Нет|Очень высокая|Игры, математика|
|StdRng|rand::rngs::StdRng|Да|Высокая|Предсказуемо лучший CSPRNG для платформы|

#### Практический совет 2026:

Никогда не используйте SmallRng для чего-то, что связано с безопасностью (даже для создания ID заказов, если они должны быть секретными). В Rust  популярна атака «предсказания состояния», когда по публичным ID восстанавливают генератор и крадут данные.


<hr>

[`aws-lc-rs`]: https://docs.rs/aws-lc-rs
[`native-tls`]: https://docs.rs/native-tls
[`openssl`]: https://docs.rs/openssl
[`rand`]: https://docs.rs/rand
[`ring`]: https://docs.rs/ring
[`rust-argon2`]: https://docs.rs/rust-argon2
[`rustls`]: https://docs.rs/rustls
[`schannel`]: https://docs.rs/schannel
[`security-framework`]: https://docs.rs/security-framework
[`subtle`]: https://docs.rs/subtle
[`uuid`]: https://docs.rs/uuid
[`webpki`]: https://docs.rs/webpki
[Argon2]: https://en.wikipedia.org/wiki/Argon2
[AWS]: https://aws.amazon.com
[AWS-LC]: https://github.com/awslabs/aws-lc
[bcrypt]: https://en.wikipedia.org/wiki/Bcrypt
[BoringSSL]: https://github.com/google/boringssl
[Curve25519]: https://en.wikipedia.org/wiki/Curve25519
[dalek]: https://dalek.rs
[FIPS]: https://en.wikipedia.org/wiki/Federal_Information_Processing_Standards
[OpenSSL]: https://en.wikipedia.org/wiki/OpenSSL
[PBKDF2]: https://en.wikipedia.org/wiki/PBKDF2
[Rust]: https://www.rust-lang.org
[RustCrypto/hashes]: https://github.com/RustCrypto/hashes
[RustCrypto/password-hashing]: https://github.com/RustCrypto/password-hashing
[SChannel]: https://en.wikipedia.org/wiki/Security_Support_Provider_Interface
[scrypt]: https://en.wikipedia.org/wiki/Scrypt
[SSH]: https://en.wikipedia.org/wiki/Secure_Shell
[The Rust Rand Book]: https://rust-random.github.io/book
[TLS]: https://en.wikipedia.org/wiki/Transport_Layer_Security
[UUID]: https://en.wikipedia.org/wiki/Universally_unique_identifier
[X.509]: https://en.wikipedia.org/wiki/X.509

[1]: https://rust-random.github.io/book/guide-data.html
[2]: https://rust-random.github.io/book/guide-gen.html
[3]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Versions
[11]: https://en.wikipedia.org/wiki/Cryptographic_hash_function
[12]: https://crates.io/search?q=argon2
[13]: https://codahale.com/a-lesson-in-timing-attacks
[14]: https://docs.rs/rustls/#non-features
[15]: https://kerkour.com/blog/rust-cryptography-ecosystem
[16]: https://matklad.github.io/2023/01/04/on-random-numbers.html 
[17]: https://blog.orhun.dev/zero-deps-random-in-rust
[18]: https://github.com/awslabs/aws-lc-rs#motivation
[19]: https://aws.amazon.com/blogs/opensource/introducing-aws-libcrypto-for-rust-an-open-source-cryptographic-library-for-rust
[20]: https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-jwt-access-token
