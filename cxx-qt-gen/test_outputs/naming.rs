#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "property_name"]
        fn getPropertyName(self: &MyObjectQt) -> i32;
        #[rust_name = "set_property_name"]
        fn setPropertyName(self: Pin<&mut MyObjectQt>, value: i32);

        #[cxx_name = "unsafe_rust"]
        fn rust(self: &MyObjectQt) -> &RustObj;
        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafe_rust_mut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut RustObj>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type RustObj;

        #[cxx_name = "invokableName"]
        fn invokable_name(self: &RustObj);

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    pub type FFICppObj = super::my_object::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        pub fn invokable_name(&self) {
            println!("Bye from Rust!");
        }
    }

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn property_name(&self) -> i32 {
            self.cpp.property_name()
        }

        pub fn set_property_name(&mut self, value: i32) {
            self.cpp.as_mut().set_property_name(value);
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {
            self.set_property_name(data.property_name);
        }
    }

    #[derive(Default)]
    pub struct Data {
        property_name: i32,
    }

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(value: &CppObj<'a>) -> Self {
            Self {
                property_name: value.property_name().into(),
            }
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(value: &mut CppObj<'a>) -> Self {
            Self::from(&*value)
        }
    }

    pub fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
