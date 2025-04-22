#[cxx::bridge(namespace = "handlegraph")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-handlegraph/libhandlegraph/src/include/handlegraph/types.hpp");
        include!("cxx-handlegraph/libhandlegraph/src/include/handlegraph/util.hpp");
        pub type handle_t;

        pub fn as_handle(value: &u64) -> &handle_t;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_handle_t() {
        let value: u64 = 42;
        let handle: &ffi::handle_t = ffi::as_handle(&value);
        // assert_eq!(handle, &value);
    }
}