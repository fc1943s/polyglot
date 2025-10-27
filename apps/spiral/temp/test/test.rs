#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unreachable_code)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
mod module_b448d9d1 {
    pub mod Test {
        use super::*;
        use fable_library_rust::Native_::Func0;
        use fable_library_rust::Native_::OnceInit;
        use fable_library_rust::Native_::on_startup;
        use fable_library_rust::String_::printfn;
        use fable_library_rust::String_::string;
        pub fn closure1(unitVar: (), unitVar_1: ()) {
            printfn!("{0}", string("test"));
        }
        pub fn closure0(unitVar: (), unitVar_1: ()) -> i32 {
            let v5: () = {
                Test::closure1((), ());
                ()
            };
            0_i32
        }
        pub fn v0() -> Func0<i32> {
            static v0: OnceInit<Func0<i32>> = OnceInit::new();
            v0.get_or_init(|| Func0::new(move || Test::closure0((), ())))
                .clone()
        }
        on_startup!(());
    }
}
pub use module_b448d9d1::*;
