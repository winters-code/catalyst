
// Print the type of a variable; useful for coding
#[allow(dead_code)]
pub fn print_type<T>(_x: T) {
    println!("Type of 'x': {}", std::any::type_name::<T>());
}