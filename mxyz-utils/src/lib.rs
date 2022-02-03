
pub mod dom;
pub mod file_io;


pub fn type_of<T>(_: &T) -> &'static str{
    std::any::type_name::<T>()
}

