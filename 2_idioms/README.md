Шаг 2: Идиомы
==============

__Estimated time__: 2 days

Эти шаги описывают распространенные идиомы, необходимые для написания хорошо продуманного и идиоматического кода на [Rust].

> ❗️Перед завершением этого шага необходимо выполнить все его подшаги.

After doing them you should be able to answer the following questions:
После выполнения этих заданий вы сможете ответить на следующие вопросы:

- [`Почему меня должны волновать типы и способ выражения информации с помощью типов? Как типы помогают повысить гарантии корректности программы?`](#почему-меня-должны-волновать-типы-и-способ-выражения-информации-с-помощью-типов-как-типы-помогают-повысить-гарантии-корректности-программы)

- What is essential for writing well-designed and ergonomic APIs in [Rust] and why?
- Why `mem::replace` exists and what purpose does it solve? When and why is it really helpful?
- How input type polymorphism is usually organized in [Rust] APIs? What cost does it have?
- Which ways and tools do exist for future-proofing source code in [Rust]?

<hr>

<h3>Почему меня должны волновать типы и способ выражения информации с помощью типов? Как типы помогают повысить гарантии корректности программы?</h3>

В Rust типы — это не просто разметка памяти или способ подсказать компилятору размер переменной. Это инструмент проектирования и верификации, который позволяет переложить проверку бизнес-логики с человека на машину.

Вот почему типы в Rust напрямую влияют на корректность вашей программы:

<h4>1. Превращение логических ошибок в ошибки компиляции</h4>

Самое мощное преимущество — возможность сделать невалидные состояния непредставимыми.

- Пример: Вместо того чтобы проверять `if user.is_activated` в каждой функции, вы создаете два типа: `IncompleteUser` и `ActivatedUser`. Функция отправки уведомления принимает только `ActivatedUser`.
- Результат: Вы физически не сможете скомпилировать код, который пытается отправить письмо неактивированному пользователю. Ошибка обнаруживается не в 3 часа ночи на сервере, а сразу в IDE.

<h4>2. Гарантии безопасности памяти без GC (Garbage Collection)</h4>

Система типов Rust (через механизмы владения и времен жизни — `lifetimes`) гарантирует отсутствие целого класса критических багов:

- Dangling pointers (висячие указатели).
- Double free (двойное освобождение памяти).
- Data races (состояния гонки в многопоточности).
    Компилятор использует типы как «доказательство» того, что доступ к памяти безопасен. Если доказательство не сходится, программа не соберется.


<hr>

## Task

Design and implement a `VendingMachine` type, which behaves like a [vending machine][1]:
- `Product` should have a price and a name;
- `VendingMachine` should have a limited capacity of `Product`s;
- `VendingMachine` should be able to give change;
- `VendingMachine` should reject purchase if it cannot give change;
- `Coin` nominal values could only be `1`, `2`, `5`, `10`, `20` and `50`.

Make its usage API as convenient as you're capable to.




[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Vending_machine
