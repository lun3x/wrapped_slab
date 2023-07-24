pub use wrapped_slab_derive::WrappedSlab;

#[derive(Clone)]
pub enum Entry<T> {
    Vacant(usize),
    Occupied(T),
}

#[doc(hidden)]
pub use slab;
