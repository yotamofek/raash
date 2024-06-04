use std::{marker::PhantomData, ptr::NonNull};

pub struct Field<'a, T>(NonNull<T>, PhantomData<&'a mut ()>);

impl<'a, T> Field<'a, T> {
    pub(super) unsafe fn new(ptr: NonNull<T>) -> Self {
        Self(ptr, PhantomData)
    }

    pub fn get(&self) -> T {
        unsafe { self.0.read() }
    }

    pub fn set(&mut self, value: T) {
        unsafe { self.0.write(value) }
    }
}

#[macro_export]
macro_rules! impl_fields {
    {
        struct $struct:ident$(<$lt:lifetime>)? {
            $(
                $(#[doc = $lit:literal])*
                $vis:vis $field:ident: $ty:ty
            ),+,
        }
    } => {
        impl $struct$(<$lt>)? {
            $(
                $(#[doc = $lit])*
                #[inline]
                $vis fn $field(&self) -> $crate::field::Field<$ty> {
                    unsafe {
                        let ptr = NonNull::new_unchecked(::std::ptr::addr_of_mut!((*self.0.as_ptr()).$field));
                        $crate::field::Field::new(ptr)
                    }
                }
            )+
        }
    };
}
