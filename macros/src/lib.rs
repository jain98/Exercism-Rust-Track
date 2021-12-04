#[macro_export]
macro_rules! hashmap {
    ( $($k: expr => $v: expr),+ $(,)?) => {
        {
            let mut temp = ::std::collections::HashMap::new();
            $(
                temp.insert($k, $v);
            )*
            temp
        }
    };
    () => (::std::collections::HashMap::new());
}
