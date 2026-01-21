use std::{
    borrow::{Borrow, BorrowMut},
    num::NonZeroU64, // Это позволяет оптимизировать размещение данных в памяти.
    /* 20260121 Не нужный код
    sync::{Arc, Mutex},
    */
};

/// Прогнозируемое состояние, сформированное на основе ряда событий.
pub trait Aggregate: Default {
    /// Статическая строка, представляющая тип агрегата.
    ///
    /// Примечание: это значение должно быть постоянным и никогда не должно меняться.
    fn aggregate_type() -> &'static str;

    /// Обрабатывает событие, применяя его последствия к агрегатору.
    fn apply<E>(&mut self, event: E)
    where
        E: AggregateEvent<Self>,
    {
        event.apply_to(self);
    }
}

/// Идентификатор агрегата.
pub trait AggregateId<A>
where
    A: Aggregate,
{
    /// Получает строковый идентификатор агрегата.
    fn as_str(&self) -> &str;
}

/// Событие, которое произошло.
pub trait Event {
    /// Статическое описание события.
    fn event_type(&self) -> &'static str;
}

/// Событие, которое может быть применено к агрегату.
pub trait AggregateEvent<A: Aggregate>: Event {
    /// Обрабатывает событие, применяя его последствия к агрегатору.
    fn apply_to(self, aggregate: &mut A);
}

/// Представляет собой порядковый номер события, начиная с 1.
#[derive(
    Clone,
    Copy,
    /* 20260121 Не нужный код
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord
     */
  )
]
pub struct EventNumber(NonZeroU64);

impl EventNumber {
    /// Минимальное значение [EventNumber].
    #[allow(unsafe_code)]
    pub const MIN_VALUE: EventNumber =  // создание const
        // Единица абсолютно не равна нулю, и это необходимо для того, 
        // чтобы это можно было использовать в контексте констант.
        EventNumber(
            unsafe {
                NonZeroU64::new_unchecked(1) // Создает ненулевое значение, не проверяя, является ли оно ненулевым.
            }
        );

    /// Увеличивает номер события до следующего значения.
    #[inline]
    pub fn incr(&mut self) {
        self.0 = NonZeroU64::new(   // Создает ненулевое значение, если заданное значение не равно нулю.
                    self.0.get() // Возвращает содержащееся значение как примитивный тип.
                    +
                    1
                )
                .unwrap()
                ;
    }
}

/// Верся агрегата
#[derive(
    Clone,
    Copy,
    /* 20260121 Не нужный код
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord
    */
  )
]
pub enum Version {
    /// Версия агрегата, к которому еще не были применены никакие события.
    Initial,
    /// The version of the last event applied to the aggregate.
    /// Версия последнего события, примененная к агрегату.
    Number(EventNumber),
}

// реализация Default для Version
impl Default for Version {
    #[inline]
    fn default() -> Self {
        Version::Initial
    }
}


impl Version {
    /// Создает новую версию из заданного числа.
    ///
    /// Число `0` интерпретируется как `Version::Initial`, а любое другое
    /// число интерпретируется как применен последний номер события.
    #[inline]
    pub fn new(number: u64) -> Self {
        NonZeroU64::new(number) // Option<NonZeroU64>
            .map(EventNumber)    // Option<EventNumber(NonZeroU64)>
            .map(Version::Number)    // Option<Version::Number(EventNumber(NonZeroU64))>
            .unwrap_or(Version::Initial)    // Version::Number(EventNumber(NonZeroU64)) или Version::Initial
    }

    /// Увеличивает номер версии до следующей по порядку.
    #[inline]
    pub fn incr(&mut self) {
        match *self {
            Version::Initial => *self = Version::Number(EventNumber::MIN_VALUE),
            Version::Number(ref mut en) => en.incr(),
        }
    }
}

/// Агрегат, загруженный из источника, который отслеживает версию своего 
/// последнего снимка и текущую версию агрегата.
#[derive(
    Clone,
    Copy,
    /* 20260121 Не нужный код
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq
    */
  )
]
pub struct HydratedAggregate<A>
/* 20260121 Не нужный код
where
    A: Aggregate,
 */
{
    version: Version,
    snapshot_version: Option<Version>,
    state: A,
}

impl<A> HydratedAggregate<A>
where
    A: Aggregate,
{
    /// Текущая версия агрегата.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Версия снимка, из которого был загружен агрегат.
    pub fn snapshot_version(&self) -> Option<Version> {
        self.snapshot_version
    }

    /// Обновляет версию снимка. Обычно используется для указания того, что был сделан снимок.
    pub fn set_snapshot_version(&mut self, new_snapshot_version: Version) {
        self.snapshot_version = Some(new_snapshot_version);
    }

    /// Актуальный агрегат.
    pub fn state(&self) -> &A {
        &self.state
    }

    /// Применяет последовательность событий к внутреннему агрегату.
    pub fn apply_events<E: AggregateEvent<A>, I: IntoIterator<Item = E>>(&mut self, events: I) {
        for event in events {
            self.apply(event);
        }
    }

    /// Применяет одно событие к агрегату, отслеживая новую версию агрегата.
    pub fn apply<E: AggregateEvent<A>>(&mut self, event: E) {
        self.state.apply(event);
        self.version.incr();
    }
}

impl<A> AsRef<A> for HydratedAggregate<A>
where
    A: Aggregate,
{
    fn as_ref(&self) -> &A {
        &self.state
    }
}

impl<A> Borrow<A> for HydratedAggregate<A>
where
    A: Aggregate,
{
    fn borrow(&self) -> &A {
        &self.state
    }
}

/// Идентифицированный, конкретный пример гидратированного агрегата.
#[derive(
    Clone,
    /* 20260121 Не нужный код
    Copy,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq
    */
  )
]
pub struct Entity<I, A>
/* 20260121 Не нужный код
where
    A: Aggregate,
    I: AggregateId<A>,
 */
{
    id: I,
    aggregate: HydratedAggregate<A>,
}

impl<I, A> Entity<I, A>
where
    A: Aggregate,
    I: AggregateId<A>,
{
    /// Создает новый объект на основе идентификатора и связанного с ним гидратированного агрегата.
    pub fn new(id: I, aggregate: HydratedAggregate<A>) -> Self {
        Entity { id, aggregate }
    }

    /// Идентификатор объекта.
    pub fn id(&self) -> &I {
        &self.id
    }

    /// Неизменяемая ссылка на лежащий в основе агрегат.
    pub fn aggregate(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }

    /// Изменяемая ссылка на базовый агрегат.
    pub fn aggregate_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}

impl<I, A> From<Entity<I, A>> for HydratedAggregate<A>
where
    A: Aggregate,
    I: AggregateId<A>,
{
    fn from(entity: Entity<I, A>) -> Self {
        entity.aggregate
    }
}

impl<I, A> AsRef<HydratedAggregate<A>> for Entity<I, A>
where
    A: Aggregate,
    I: AggregateId<A>,
{
    fn as_ref(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }
}

impl<I, A> AsMut<HydratedAggregate<A>> for Entity<I, A>
where
    A: Aggregate,
    I: AggregateId<A>,
{
    fn as_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}

impl<I, A> Borrow<HydratedAggregate<A>> for Entity<I, A>
where
    A: Aggregate,
    I: AggregateId<A>,
{
    fn borrow(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }
}

impl<I, A> Borrow<A> for Entity<I, A>
where
    A: Aggregate,
    I: AggregateId<A>,
{
    fn borrow(&self) -> &A {
        self.aggregate.borrow()
    }
}

impl<I, A> BorrowMut<HydratedAggregate<A>> for Entity<I, A>
where
    A: Aggregate,
    I: AggregateId<A>,
{
    fn borrow_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}

fn main() {
    println!("The code has been refactored.");
}