use crate::registry::resource_name::ResourceName;

pub mod resource_name;

pub trait Registry<T, U = ResourceName> {}
