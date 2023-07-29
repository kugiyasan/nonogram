/// https://stackoverflow.com/a/73752148
#[macro_export]
macro_rules! vecvec {
    ([$($elem:expr),*]; $n:expr) => {{
        let mut vec = Vec::new();
        vec.resize_with($n, || vec![$($elem),*]);
        vec
    }};
    ($([$($x:expr),*]),* $(,)?) => {
        vec![$(vec![$($x),*]),*]
    };
}
