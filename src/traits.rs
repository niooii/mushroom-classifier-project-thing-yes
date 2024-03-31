pub trait CsvParsable {
    fn from_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_csv_parsable {
    ($type:ty, $size:expr) => {
        impl CsvParsable for $type {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; $size];
                arr.copy_from_slice(&bytes[..$size]);
                <$type>::from_ne_bytes(arr)
            }
        }
    };
}

impl_csv_parsable!(i8, 1);
impl_csv_parsable!(i16, 2);
impl_csv_parsable!(i32, 4);
impl_csv_parsable!(i64, 8);
impl_csv_parsable!(u8, 1);
impl_csv_parsable!(u16, 2);
impl_csv_parsable!(u32, 4);
impl_csv_parsable!(u64, 8);
impl_csv_parsable!(f32, 4);
impl_csv_parsable!(f64, 8);

impl CsvParsable for char {
    fn from_bytes(bytes: &[u8]) -> Self {
        let char_str = std::str::from_utf8(bytes).expect("Invalid UTF-8 sequence");
        char_str.chars().next().expect("Empty string")
    }
}

