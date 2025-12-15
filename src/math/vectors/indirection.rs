//! Field overload in Rust is impossible. Unless you do pointer indirection!

use std::{
    ops::{Deref, DerefMut},
    ptr::from_ref,
};

use super::Vector;

#[repr(C)]
pub struct VX<T> {
    pub x: T,
}

impl<T> Deref for Vector<T, 1> {
    type Target = VX<T>;

    fn deref(&self) -> &Self::Target {
        // Array is length 1 and of the same type. These are basically the same thing.
        unsafe { &*(from_ref(self) as *const Self::Target) }
    }
}

impl<T> DerefMut for Vector<T, 1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Array is length 1 and of the same type. These are basically the same thing.
        unsafe { &mut *(from_ref(self) as *mut Self::Target) }
    }
}

#[repr(C)]
pub struct VXY<T> {
    pub x: T,
    pub y: T,
}

impl<T> Deref for Vector<T, 2> {
    type Target = VXY<T>;

    fn deref(&self) -> &Self::Target {
        // Array is length 2 and of the same type. These are basically the same thing.
        unsafe { &*(from_ref(self) as *const Self::Target) }
    }
}

impl<T> DerefMut for Vector<T, 2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Array is length 2 and of the same type. These are basically the same thing.
        unsafe { &mut *(from_ref(self) as *mut Self::Target) }
    }
}

#[repr(C)]
pub struct VXYZ<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Deref for Vector<T, 3> {
    type Target = VXYZ<T>;

    fn deref(&self) -> &Self::Target {
        // Array is length 3 and of the same type. These are basically the same thing.
        unsafe { &*(from_ref(self) as *const Self::Target) }
    }
}

impl<T> DerefMut for Vector<T, 3> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Array is length 3 and of the same type. These are basically the same thing.
        unsafe { &mut *(from_ref(self) as *mut Self::Target) }
    }
}

#[repr(C)]
pub struct VXYZW<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Deref for Vector<T, 4> {
    type Target = VXYZW<T>;

    fn deref(&self) -> &Self::Target {
        // Array is length 4 and of the same type. These are basically the same thing.
        unsafe { &*(from_ref(self) as *const Self::Target) }
    }
}

impl<T> DerefMut for Vector<T, 4> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Array is length 4 and of the same type. These are basically the same thing.
        unsafe { &mut *(from_ref(self) as *mut Self::Target) }
    }
}
