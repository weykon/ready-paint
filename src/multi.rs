use std::{any::TypeId, marker::PhantomData};

use crate::scene::{HashTypeId2Data, Ready};
pub trait InRefOrMut {
    type AccessMode;
    type Output;
}
/// # Safety
/// 1. T1 and T2 .. T4 must be different types from each.
/// 2. T1 and T2 .. T4 must be wrap in Ref<T> Or Mut<T>, T must be impl `Ready`.
/// example:
/// ```
/// refs_muts::<(Ref<Data1>, Mut<Data2>)>(&mut data);
/// ```
pub fn refs_muts<T: TurpleAccess>(data: &mut HashTypeId2Data) -> T::Output<'_> {
    T::accesss(data)
}
pub trait AccessMode {}
pub struct Read;
pub struct Write;
impl AccessMode for Read {}
impl AccessMode for Write {}
pub trait RefOrMut {
    type Target: 'static;
    type Mode: AccessMode;
    type Output<'a>: 'a
    where
        Self::Target: 'a;
    fn process<'a>(data: &'a mut HashTypeId2Data) -> Self::Output<'a>;
}
pub struct Ref<T>(PhantomData<T>);
impl<T: 'static + Ready> RefOrMut for Ref<T> {
    type Target = T;
    type Mode = Read;
    type Output<'a> = &'a T;

    fn process<'a>(data: &'a mut HashTypeId2Data) -> Self::Output<'a> {
        data.get(&TypeId::of::<T>())
            .unwrap()
            .downcast_ref()
            .unwrap()
    }
}

pub struct Mut<T>(PhantomData<T>);
impl<T: 'static + Ready> RefOrMut for Mut<T> {
    type Target = T;
    type Mode = Write;
    type Output<'a> = &'a mut T;

    fn process<'a>(data: &'a mut HashTypeId2Data) -> Self::Output<'a> {
        data.get_mut(&TypeId::of::<T>())
            .unwrap()
            .downcast_mut()
            .unwrap()
    }
}
pub trait TurpleAccess {
    type Output<'a>;
    fn accesss<'a>(data: &'a mut HashTypeId2Data) -> Self::Output<'a>;
}
impl<T1, T2> TurpleAccess for (T1, T2)
where
    T1: RefOrMut + 'static,
    T2: RefOrMut + 'static,
{
    type Output<'a> = (T1::Output<'a>, T2::Output<'a>);

    fn accesss<'a>(data: &'a mut HashTypeId2Data) -> Self::Output<'a> {
        assert_ne!(TypeId::of::<T1>(), TypeId::of::<T2>());
        unsafe {
            // 使用原始指针来绕过借用检查
            let data_ptr = data as *mut HashTypeId2Data;
            (T1::process(&mut *data_ptr), T2::process(&mut *data_ptr))
        }
    }
}

impl<T1, T2, T3> TurpleAccess for (T1, T2, T3)
where
    T1: RefOrMut + 'static,
    T2: RefOrMut + 'static,
    T3: RefOrMut + 'static,
{
    type Output<'a> = (T1::Output<'a>, T2::Output<'a>, T3::Output<'a>);

    fn accesss<'a>(data: &'a mut HashTypeId2Data) -> Self::Output<'a> {
        assert_ne!(TypeId::of::<T1>(), TypeId::of::<T2>());
        assert_ne!(TypeId::of::<T1>(), TypeId::of::<T3>());
        assert_ne!(TypeId::of::<T2>(), TypeId::of::<T3>());
        unsafe {
            let data_ptr = data as *mut HashTypeId2Data;
            (
                T1::process(&mut *data_ptr),
                T2::process(&mut *data_ptr),
                T3::process(&mut *data_ptr),
            )
        }
    }
}

impl<T1, T2, T3, T4> TurpleAccess for (T1, T2, T3, T4)
where
    T1: RefOrMut + 'static,
    T2: RefOrMut + 'static,
    T3: RefOrMut + 'static,
    T4: RefOrMut + 'static,
{
    type Output<'a> = (
        T1::Output<'a>,
        T2::Output<'a>,
        T3::Output<'a>,
        T4::Output<'a>,
    );

    fn accesss<'a>(data: &'a mut HashTypeId2Data) -> Self::Output<'a> {
        assert_ne!(TypeId::of::<T1>(), TypeId::of::<T2>());
        assert_ne!(TypeId::of::<T1>(), TypeId::of::<T3>());
        assert_ne!(TypeId::of::<T1>(), TypeId::of::<T4>());
        assert_ne!(TypeId::of::<T2>(), TypeId::of::<T3>());
        assert_ne!(TypeId::of::<T2>(), TypeId::of::<T4>());
        assert_ne!(TypeId::of::<T3>(), TypeId::of::<T4>());
        unsafe {
            let data_ptr = data as *mut HashTypeId2Data;
            (
                T1::process(&mut *data_ptr),
                T2::process(&mut *data_ptr),
                T3::process(&mut *data_ptr),
                T4::process(&mut *data_ptr),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use crate::{
        gfx::Gfx,
        multi::{refs_muts, Mut, Ref},
        scene::{HashTypeId2Data, Ready},
    };
    #[derive(Debug)]
    struct Data1 {
        val: i32,
    }
    #[derive(Debug)]
    struct Data2 {
        val: i32,
    }
    impl Ready for Data1 {
        fn ready(&mut self, _: &mut HashTypeId2Data, _: &Gfx) {}
    }
    impl Ready for Data2 {
        fn ready(&mut self, _: &mut HashTypeId2Data, _: &Gfx) {}
    }
    #[test]
    fn test() {
        use std::any::Any;
        use std::collections::HashMap;
        let mut data: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
        data.insert(TypeId::of::<Data1>(), Box::new(Data1 { val: 42 }));
        data.insert(TypeId::of::<Data2>(), Box::new(Data2 { val: 3 }));

        let (r1, r2) = refs_muts::<(Ref<Data1>, Mut<Data2>)>(&mut data);
        r2.val = 5;
        assert_eq!(r1.val, 42);
        assert_eq!(r2.val, 5);
    }
}
