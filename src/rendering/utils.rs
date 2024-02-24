
#[allow(dead_code)]
pub fn print_type<T>(_x: T) {
    println!("Type of 'x': {}", std::any::type_name::<T>());
}