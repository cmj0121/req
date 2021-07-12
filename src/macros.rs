// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
#[macro_export]
macro_rules! value {
    () => {
        Value::default()
    };

    ($re: expr) => {
        Value::from($re)
    };

    (@array [$( $expr: expr ,)*]) => {
        let mut vec: Vec<Value> = vec!{};
        $(
            vec.push(value!($expr));
        )*
        Value::Array(vec)
    };

    (@object {$($key: literal => $value: expr),*}) => {
        let mut map: HashMap<String, Value> = HashMap::new();
        $(
            map.insert($key, value!($value));
        )*
        Value::Object(map)
    }
}

// vim: set ts=4 sw=4 expandtab:
