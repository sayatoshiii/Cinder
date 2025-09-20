// Reference: https://stackoverflow.com/a/58119924
#[allow(dead_code)]
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
