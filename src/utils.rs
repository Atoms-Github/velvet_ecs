use std::any::TypeId;

pub type TypeIdNum = u64;


struct CrackerTypeId {
    t: TypeIdNum,
}

pub fn crack_type_id(type_id: TypeId) -> TypeIdNum{
    let emp_exposed: CrackerTypeId = unsafe {
        std::mem::transmute(type_id)
    };
    return emp_exposed.t;
}
pub fn gett<T : 'static>() -> TypeIdNum{
    crack_type_id(TypeId::of::<T>())
}

#[macro_export]
macro_rules! unwrap {
        ($enum:path, $expr:expr) => {{
            if let $enum(item) = $expr {
                item
            } else {
                panic!("Wrong match type!!")
            }
        }};
    }
pub use unwrap;

pub unsafe fn struct_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

pub unsafe fn struct_as_u8_slice_mut<T: Sized>(p: &mut T) -> &mut [u8] {
    ::std::slice::from_raw_parts_mut(
        (p as *mut T) as *mut u8,
        ::std::mem::size_of::<T>(),
    )
}

pub unsafe fn u8_slice_to_ref<T>(bytes: &[u8]) -> &T {
    let bytes_ptr = bytes.as_ptr();
    let test : *const T = unsafe{ std::mem::transmute(bytes_ptr) };
    let value = unsafe {test.as_ref()}.unwrap();
    return value;
}

pub unsafe fn u8_slice_to_ref_mut<T>(bytes: &mut [u8]) -> &mut T {
    let bytes_ptr = bytes.as_ptr();
    let test : *mut T = unsafe{ std::mem::transmute(bytes_ptr) };
    let value : &mut T = unsafe {test.as_mut()}.unwrap();
    return value;
}

pub unsafe fn unsafe_const_cheat<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}
