fn main() {
    {
        // реализация декларативного макроса btreemap!
        use std::collections::BTreeMap ;

        /// реализация макроса btreemap! деклоративным способом
        macro_rules! btreemap {
            ($($key:expr => $value:expr),* $(,)?) => {
                {
                    let mut map = BTreeMap::new() ;
                    $(
                        map.insert($key, $value) ;
                    )*
                    map
                }
            };
        }

        let map = btreemap!(
            "one" => 1,
            "two" => 2,
            "three" => 3,
        ) ;

        println!("map: {:?}", map);
    }

    {
        use step_3_2::btreemap;

        let map = btreemap!(
            "one" => 1,
            "two" => 2,
            "three" => 3,
        ) ;

        println!("map: {:?}", map);        
    }
}
