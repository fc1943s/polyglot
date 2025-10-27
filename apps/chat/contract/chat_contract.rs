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
mod module_31a2fc18 {
    pub mod Chat_contract {
        use super::*;
        type DateTime = ();
        use fable_library_rust::List_::empty;
        use fable_library_rust::List_::ofArray;
        use fable_library_rust::List_::toArray;
        use fable_library_rust::Map_::find;
        use fable_library_rust::Map_::ofSeq;
        use fable_library_rust::Native_::Any;
        use fable_library_rust::Native_::Func0;
        use fable_library_rust::Native_::Func1;
        use fable_library_rust::Native_::LrcPtr;
        use fable_library_rust::Native_::MutCell;
        use fable_library_rust::Native_::OnceInit;
        use fable_library_rust::Native_::getNull;
        use fable_library_rust::Native_::on_startup;
        use fable_library_rust::Native_::referenceEquals;
        use fable_library_rust::Native_::unbox;
        use fable_library_rust::NativeArray_::new_array;
        use fable_library_rust::Option_::defaultValue;
        use fable_library_rust::Option_::map;
        use fable_library_rust::Seq_::ofList;
        use fable_library_rust::String_::append;
        use fable_library_rust::String_::endsWith3;
        use fable_library_rust::String_::getCharAt;
        use fable_library_rust::String_::length;
        use fable_library_rust::String_::printfn;
        use fable_library_rust::String_::sprintf;
        use fable_library_rust::String_::startsWith3;
        use fable_library_rust::String_::string;
        use fable_library_rust::String_::toLower;
        use fable_library_rust::String_::toString;
        use fable_library_rust::String_::trimEndChars;
        use fable_library_rust::String_::trimStartChars;
        use fable_library_rust::System::Collections::Generic::IEnumerable_1;
        pub trait IOsEnviron: core::fmt::Debug + core::fmt::Display {
            fn environ(&self) -> LrcPtr<dyn Any>;
        }
        impl<V: IOsEnviron + core::fmt::Debug + core::fmt::Display> IOsEnviron for LrcPtr<V> {
            #[inline]
            fn environ(&self) -> LrcPtr<dyn Any> {
                (**self).environ()
            }
        }
        pub mod TraceState {
            use super::*;
            pub fn trace_state() -> LrcPtr<
                MutCell<
                    Option<(
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    )>,
                >,
            > {
                static trace_state: OnceInit<
                    LrcPtr<
                        MutCell<
                            Option<(
                                LrcPtr<Chat_contract::Mut0>,
                                LrcPtr<Chat_contract::Mut1>,
                                LrcPtr<Chat_contract::Mut2>,
                                LrcPtr<Chat_contract::Mut3>,
                                LrcPtr<Chat_contract::Mut4>,
                                Option<i64>,
                            )>,
                        >,
                    >,
                > = OnceInit::new();
                trace_state
                    .get_or_init(|| {
                        LrcPtr::new(MutCell::new(
                            None::<(
                                LrcPtr<Chat_contract::Mut0>,
                                LrcPtr<Chat_contract::Mut1>,
                                LrcPtr<Chat_contract::Mut2>,
                                LrcPtr<Chat_contract::Mut3>,
                                LrcPtr<Chat_contract::Mut4>,
                                Option<i64>,
                            )>,
                        ))
                    })
                    .clone()
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US0 {
            US0_0,
            US0_1,
            US0_2,
            US0_3,
            US0_4,
        }
        impl core::fmt::Display for US0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut0 {
            pub l0: MutCell<i64>,
        }
        impl core::fmt::Display for Mut0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub struct Mut1 {
            pub l0: MutCell<Func1<string, ()>>,
        }
        impl core::fmt::Display for Mut1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut2 {
            pub l0: MutCell<bool>,
        }
        impl core::fmt::Display for Mut2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut3 {
            pub l0: MutCell<string>,
        }
        impl core::fmt::Display for Mut3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut4 {
            pub l0: MutCell<Chat_contract::US0>,
        }
        impl core::fmt::Display for Mut4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US1 {
            US1_0(Chat_contract::US0),
            US1_1,
        }
        impl core::fmt::Display for US1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US2 {
            US2_0(i64),
            US2_1,
        }
        impl core::fmt::Display for US2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US3 {
            US3_0,
            US3_1,
            US3_2,
        }
        impl core::fmt::Display for US3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US4 {
            US4_0(Chat_contract::US3),
            US4_1(Chat_contract::US3),
            US4_2(Chat_contract::US3),
            US4_3(Chat_contract::US3),
            US4_4(Chat_contract::US3),
            US4_5(Chat_contract::US3),
            US4_6(Chat_contract::US3),
        }
        impl core::fmt::Display for US4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US5 {
            US5_0(string),
            US5_1,
        }
        impl core::fmt::Display for US5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US6 {
            US6_0(
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ),
            US6_1,
        }
        impl core::fmt::Display for US6 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US7 {
            US7_0(std::string::String),
            US7_1,
        }
        impl core::fmt::Display for US7 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        pub fn closure1(unitVar: (), v0_1: char) -> bool {
            if v0_1.is_alphanumeric() {
                true
            } else {
                (v0_1) == '-'
            }
        }
        pub fn method3(v0_1: string) -> string {
            v0_1
        }
        pub fn method4() -> string {
            string("")
        }
        pub fn closure4(unitVar: (), v0_1: string) -> Chat_contract::US5 {
            Chat_contract::US5::US5_0(v0_1)
        }
        pub fn method5() -> Func1<string, Chat_contract::US5> {
            Func1::new(move |v: string| Chat_contract::closure4((), v))
        }
        pub fn method2(v0_1: string) -> string {
            panic!(
                "{}",
                sprintf!(
                    "env.get_environment_variable / target: {} / var: {}",
                    Chat_contract::US4::US4_4(Chat_contract::US3::US3_2),
                    v0_1
                ),
            )
        }
        pub fn method1() -> (Chat_contract::US1, Chat_contract::US2) {
            let v1: string = Chat_contract::method2(string("TRACE_LEVEL"));
            let v6: string = toLower(string("Critical"));
            let v13: string = toLower(string("Warning"));
            let v20: string = toLower(string("Info"));
            let v27: string = toLower(string("Debug"));
            let v34: string = toLower(string("Verbose"));
            let v41: Chat_contract::US1 = if string("Verbose") == (v1.clone()) {
                Chat_contract::US1::US1_0(Chat_contract::US0::US0_0)
            } else {
                Chat_contract::US1::US1_1
            };
            (
                match &v41 {
                    Chat_contract::US1::US1_0(v41_0_0) => Chat_contract::US1::US1_0(
                        match &v41 {
                            Chat_contract::US1::US1_0(x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                    ),
                    _ => {
                        let v48: Chat_contract::US1 = if string("Debug") == (v1.clone()) {
                            Chat_contract::US1::US1_0(Chat_contract::US0::US0_1)
                        } else {
                            Chat_contract::US1::US1_1
                        };
                        match &v48 {
                            Chat_contract::US1::US1_0(v48_0_0) => Chat_contract::US1::US1_0(
                                match &v48 {
                                    Chat_contract::US1::US1_0(x) => x.clone(),
                                    _ => unreachable!(),
                                }
                                .clone(),
                            ),
                            _ => {
                                let v55: Chat_contract::US1 = if string("Info") == (v1.clone()) {
                                    Chat_contract::US1::US1_0(Chat_contract::US0::US0_2)
                                } else {
                                    Chat_contract::US1::US1_1
                                };
                                match &v55 {
                                    Chat_contract::US1::US1_0(v55_0_0) => {
                                        Chat_contract::US1::US1_0(
                                            match &v55 {
                                                Chat_contract::US1::US1_0(x) => x.clone(),
                                                _ => unreachable!(),
                                            }
                                            .clone(),
                                        )
                                    }
                                    _ => {
                                        let v62: Chat_contract::US1 =
                                            if string("Warning") == (v1.clone()) {
                                                Chat_contract::US1::US1_0(Chat_contract::US0::US0_3)
                                            } else {
                                                Chat_contract::US1::US1_1
                                            };
                                        match &v62 {
                                            Chat_contract::US1::US1_0(v62_0_0) => {
                                                Chat_contract::US1::US1_0(
                                                    match &v62 {
                                                        Chat_contract::US1::US1_0(x) => x.clone(),
                                                        _ => unreachable!(),
                                                    }
                                                    .clone(),
                                                )
                                            }
                                            _ => {
                                                let v69: Chat_contract::US1 =
                                                    if string("Critical") == (v1.clone()) {
                                                        Chat_contract::US1::US1_0(
                                                            Chat_contract::US0::US0_4,
                                                        )
                                                    } else {
                                                        Chat_contract::US1::US1_1
                                                    };
                                                match &v69 {
                                                    Chat_contract::US1::US1_0(v69_0_0) => {
                                                        Chat_contract::US1::US1_0(
                                                            match &v69 {
                                                                Chat_contract::US1::US1_0(x) => {
                                                                    x.clone()
                                                                }
                                                                _ => unreachable!(),
                                                            }
                                                            .clone(),
                                                        )
                                                    }
                                                    _ => {
                                                        let v76: Chat_contract::US1 =
                                                            if (v34.clone()) == (v1.clone()) {
                                                                Chat_contract::US1::US1_0(
                                                                    Chat_contract::US0::US0_0,
                                                                )
                                                            } else {
                                                                Chat_contract::US1::US1_1
                                                            };
                                                        match &v76 {
                                                         Chat_contract::US1::US1_0(v76_0_0)
                                                         =>
                                                         Chat_contract::US1::US1_0(match &v76
                                                                                       {
                                                                                       Chat_contract::US1::US1_0(x)
                                                                                       =>
                                                                                       x.clone(),
                                                                                       _
                                                                                       =>
                                                                                       unreachable!(),
                                                                                   }.clone()),
                                                         _ => {
                                                             let v83:
                                                                     Chat_contract::US1 =
                                                                 if (v27.clone())
                                                                        ==
                                                                        (v1.clone())
                                                                    {
                                                                     Chat_contract::US1::US1_0(Chat_contract::US0::US0_1)
                                                                 } else {
                                                                     Chat_contract::US1::US1_1
                                                                 };
                                                             match &v83 {
                                                                 Chat_contract::US1::US1_0(v83_0_0)
                                                                 =>
                                                                 Chat_contract::US1::US1_0(match &v83
                                                                                               {
                                                                                               Chat_contract::US1::US1_0(x)
                                                                                               =>
                                                                                               x.clone(),
                                                                                               _
                                                                                               =>
                                                                                               unreachable!(),
                                                                                           }.clone()),
                                                                 _ => {
                                                                     let v90:
                                                                             Chat_contract::US1 =
                                                                         if (v20.clone())
                                                                                ==
                                                                                (v1.clone())
                                                                            {
                                                                             Chat_contract::US1::US1_0(Chat_contract::US0::US0_2)
                                                                         } else {
                                                                             Chat_contract::US1::US1_1
                                                                         };
                                                                     match &v90
                                                                         {
                                                                         Chat_contract::US1::US1_0(v90_0_0)
                                                                         =>
                                                                         Chat_contract::US1::US1_0(match &v90
                                                                                                       {
                                                                                                       Chat_contract::US1::US1_0(x)
                                                                                                       =>
                                                                                                       x.clone(),
                                                                                                       _
                                                                                                       =>
                                                                                                       unreachable!(),
                                                                                                   }.clone()),
                                                                         _ =>
                                                                         {
                                                                             let v97:
                                                                                     Chat_contract::US1 =
                                                                                 if (v13.clone())
                                                                                        ==
                                                                                        (v1.clone())
                                                                                    {
                                                                                     Chat_contract::US1::US1_0(Chat_contract::US0::US0_3)
                                                                                 } else {
                                                                                     Chat_contract::US1::US1_1
                                                                                 };
                                                                             match &v97
                                                                                 {
                                                                                 Chat_contract::US1::US1_0(v97_0_0)
                                                                                 =>
                                                                                 Chat_contract::US1::US1_0(match &v97
                                                                                                               {
                                                                                                               Chat_contract::US1::US1_0(x)
                                                                                                               =>
                                                                                                               x.clone(),
                                                                                                               _
                                                                                                               =>
                                                                                                               unreachable!(),
                                                                                                           }.clone()),
                                                                                 _
                                                                                 =>
                                                                                 {
                                                                                     let v104:
                                                                                             Chat_contract::US1 =
                                                                                         if (v6.clone())
                                                                                                ==
                                                                                                (v1.clone())
                                                                                            {
                                                                                             Chat_contract::US1::US1_0(Chat_contract::US0::US0_4)
                                                                                         } else {
                                                                                             Chat_contract::US1::US1_1
                                                                                         };
                                                                                     match &v104
                                                                                         {
                                                                                         Chat_contract::US1::US1_0(v104_0_0)
                                                                                         =>
                                                                                         Chat_contract::US1::US1_0(match &v104
                                                                                                                       {
                                                                                                                       Chat_contract::US1::US1_0(x)
                                                                                                                       =>
                                                                                                                       x.clone(),
                                                                                                                       _
                                                                                                                       =>
                                                                                                                       unreachable!(),
                                                                                                                   }.clone()),
                                                                                         _
                                                                                         =>
                                                                                         Chat_contract::US1::US1_1,
                                                                                     }
                                                                                 }
                                                                             }
                                                                         }
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                if (Chat_contract::method2(string("AUTOMATION"))) != string("True") {
                    Chat_contract::US2::US2_1
                } else {
                    let v147: DateTime = unbox::<DateTime>(fable_library_rust::Native_::getZero());
                    Chat_contract::US2::US2_0(unbox::<i64>(fable_library_rust::Native_::getZero()))
                },
            )
        }
        pub fn closure5(unitVar: (), v0_1: string) {
            ();
        }
        pub fn method0(
            v0_1: Chat_contract::US0,
        ) -> (
            LrcPtr<Chat_contract::Mut0>,
            LrcPtr<Chat_contract::Mut1>,
            LrcPtr<Chat_contract::Mut2>,
            LrcPtr<Chat_contract::Mut3>,
            LrcPtr<Chat_contract::Mut4>,
            Option<i64>,
        ) {
            let v94: string = string("option_env!(\"AUTOMATION\").unwrap_or(\"\")");
            let v95: &str = option_env!("AUTOMATION").unwrap_or("");
            let v106: std::string::String = String::from(v95);
            let _run_target_args__v3: (Chat_contract::US1, Chat_contract::US2) = (
                Chat_contract::US1::US1_1,
                if (fable_library_rust::String_::fromString(v106)) != string("True") {
                    Chat_contract::US2::US2_1
                } else {
                    Chat_contract::US2::US2_0(near_sdk::env::block_timestamp() as i64)
                },
            );
            let v185: Chat_contract::US2 = _run_target_args__v3.1.clone();
            let v184: Chat_contract::US1 = _run_target_args__v3.0.clone();
            (
                LrcPtr::new(Chat_contract::Mut0 {
                    l0: MutCell::new(1_i64),
                }),
                LrcPtr::new(Chat_contract::Mut1 {
                    l0: MutCell::new(Func1::new(move |v: string| Chat_contract::closure5((), v))),
                }),
                LrcPtr::new(Chat_contract::Mut2 {
                    l0: MutCell::new(true),
                }),
                LrcPtr::new(Chat_contract::Mut3 {
                    l0: MutCell::new(string("")),
                }),
                LrcPtr::new(Chat_contract::Mut4 {
                    l0: MutCell::new(match &v184 {
                        Chat_contract::US1::US1_0(v184_0_0) => match &v184 {
                            Chat_contract::US1::US1_0(x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                        _ => v0_1.clone(),
                    }),
                }),
                match &v185 {
                    Chat_contract::US2::US2_0(v185_0_0) => Some(match &v185 {
                        Chat_contract::US2::US2_0(x) => x.clone(),
                        _ => unreachable!(),
                    }),
                    _ => None::<i64>,
                },
            )
        }
        pub fn closure3(unitVar: (), unitVar_1: ()) {
            if Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .is_none()
            {
                let patternInput: (
                    LrcPtr<Chat_contract::Mut0>,
                    LrcPtr<Chat_contract::Mut1>,
                    LrcPtr<Chat_contract::Mut2>,
                    LrcPtr<Chat_contract::Mut3>,
                    LrcPtr<Chat_contract::Mut4>,
                    Option<i64>,
                ) = Chat_contract::method0(Chat_contract::US0::US0_0);
                Chat_contract::TraceState::trace_state().set(Some((
                    patternInput.0.clone(),
                    patternInput.1.clone(),
                    patternInput.2.clone(),
                    patternInput.3.clone(),
                    patternInput.4.clone(),
                    patternInput.5.clone(),
                )));
                ()
            };
        }
        pub fn closure6(unitVar: (), v0_1: i64) -> Chat_contract::US2 {
            Chat_contract::US2::US2_0(v0_1)
        }
        pub fn method7() -> Func1<i64, Chat_contract::US2> {
            Func1::new(move |v: i64| Chat_contract::closure6((), v))
        }
        pub fn method8() -> string {
            string("hh:mm:ss")
        }
        pub fn method9() -> string {
            string("HH:mm:ss")
        }
        pub fn method6(
            v0_1: LrcPtr<Chat_contract::Mut0>,
            v1: LrcPtr<Chat_contract::Mut1>,
            v2: LrcPtr<Chat_contract::Mut2>,
            v3: LrcPtr<Chat_contract::Mut3>,
            v4: LrcPtr<Chat_contract::Mut4>,
            v5: Option<i64>,
        ) -> string {
            let v638: u64 = near_sdk::env::block_timestamp();
            let v663: Chat_contract::US2 =
                defaultValue(Chat_contract::US2::US2_1, map(Chat_contract::method7(), v5));
            let v676: u64 = (match &v663 {
                Chat_contract::US2::US2_0(v663_0_0) => {
                    (v638)
                        - (match &v663 {
                            Chat_contract::US2::US2_0(x) => x.clone(),
                            _ => unreachable!(),
                        } as u64)
                }
                _ => v638,
            }) / 1000000000_u64;
            let v677: u64 = (v676) % 60_u64;
            let v679: u64 = ((v676) / 60_u64) % 60_u64;
            let v681: u64 = ((v676) / 3600_u64) % 24_u64;
            let v683: std::string::String = format!("{:02}:{:02}:{:02}", v681, v679, v677);
            fable_library_rust::String_::fromString(v683)
        }
        pub fn method12() -> string {
            string("")
        }
        pub fn closure7(v0_1: LrcPtr<Chat_contract::Mut3>, v1: string, unitVar: ()) {
            let v4: string = append((v0_1.l0.get().clone()), (v1));
            v0_1.l0.set(v4);
            ()
        }
        pub fn method11(v0_1: char) -> string {
            let v2: LrcPtr<Chat_contract::Mut3> = LrcPtr::new(Chat_contract::Mut3 {
                l0: MutCell::new(Chat_contract::method12()),
            });
            let v17: () = {
                Chat_contract::closure7(v2.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method10() -> string {
            let v83: &str = inline_colorization::color_bright_blue;
            let v94: std::string::String = String::from(v83);
            let v139: string = append(
                (fable_library_rust::String_::fromString(v94)),
                (Chat_contract::method11(getCharAt(toLower(string("Debug")), 0_i32))),
            );
            let v225: &str = inline_colorization::color_reset;
            let v236: std::string::String = String::from(v225);
            append((v139), (fable_library_rust::String_::fromString(v236)))
        }
        pub fn method14(
            v0_1: std::string::String,
            v1: u64,
            v2: std::string::String,
            v3: std::string::String,
        ) -> string {
            let v5: LrcPtr<Chat_contract::Mut3> = LrcPtr::new(Chat_contract::Mut3 {
                l0: MutCell::new(Chat_contract::method12()),
            });
            let v19: () = {
                Chat_contract::closure7(v5.clone(), string("{ "), ());
                ()
            };
            let v38: () = {
                Chat_contract::closure7(v5.clone(), string("alias"), ());
                ()
            };
            let v57: () = {
                Chat_contract::closure7(v5.clone(), string(" = "), ());
                ()
            };
            let v77: std::string::String = format!("{:#?}", v0_1);
            let v116: () = {
                Chat_contract::closure7(
                    v5.clone(),
                    fable_library_rust::String_::fromString(v77),
                    (),
                );
                ()
            };
            let v135: () = {
                Chat_contract::closure7(v5.clone(), string("; "), ());
                ()
            };
            let v154: () = {
                Chat_contract::closure7(v5.clone(), string("block_timestamp"), ());
                ()
            };
            let v171: () = {
                Chat_contract::closure7(v5.clone(), string(" = "), ());
                ()
            };
            let v191: () = {
                Chat_contract::closure7(v5.clone(), sprintf!("{}", v1), ());
                ()
            };
            let v208: () = {
                Chat_contract::closure7(v5.clone(), string("; "), ());
                ()
            };
            let v227: () = {
                Chat_contract::closure7(v5.clone(), string("signer_account_id"), ());
                ()
            };
            let v244: () = {
                Chat_contract::closure7(v5.clone(), string(" = "), ());
                ()
            };
            let v264: std::string::String = format!("{:#?}", v2);
            let v303: () = {
                Chat_contract::closure7(
                    v5.clone(),
                    fable_library_rust::String_::fromString(v264),
                    (),
                );
                ()
            };
            let v320: () = {
                Chat_contract::closure7(v5.clone(), string("; "), ());
                ()
            };
            let v339: () = {
                Chat_contract::closure7(v5.clone(), string("predecessor_account_id"), ());
                ()
            };
            let v356: () = {
                Chat_contract::closure7(v5.clone(), string(" = "), ());
                ()
            };
            let v376: std::string::String = format!("{:#?}", v3);
            let v415: () = {
                Chat_contract::closure7(
                    v5.clone(),
                    fable_library_rust::String_::fromString(v376),
                    (),
                );
                ()
            };
            let v434: () = {
                Chat_contract::closure7(v5.clone(), string(" }"), ());
                ()
            };
            v5.l0.get().clone()
        }
        pub fn method15(v0_1: string) -> string {
            trimEndChars(
                trimStartChars(v0_1, toArray(empty::<char>())),
                toArray(ofArray(new_array(&[' ', '/']))),
            )
        }
        pub fn method13(
            v0_1: LrcPtr<Chat_contract::Mut0>,
            v1: LrcPtr<Chat_contract::Mut1>,
            v2: LrcPtr<Chat_contract::Mut2>,
            v3: LrcPtr<Chat_contract::Mut3>,
            v4: LrcPtr<Chat_contract::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: std::string::String,
            v9: u64,
            v10: std::string::String,
            v11: std::string::String,
        ) -> string {
            let v12: string = Chat_contract::method14(v8, v9, v10, v11);
            Chat_contract::method15(append(
                (append(
                    (append(
                        (append(
                            (append(
                                (append((append((append((v6), string(" "))), (v7))), string(" #"))),
                                (toString(v0_1.l0.get().clone())),
                            )),
                            string(" "),
                        )),
                        string("chat_contract.claim_alias"),
                    )),
                    string(" / "),
                )),
                (v12),
            ))
        }
        pub fn closure8(v0_1: LrcPtr<Chat_contract::Mut0>, unitVar: ()) {
            let v2: i64 = (v0_1.l0.get().clone()) + 1_i64;
            v0_1.l0.set(v2);
            ()
        }
        pub fn closure10(v0_1: string, unitVar: ()) {
            printfn!("{0}", v0_1);
        }
        pub fn closure9(unitVar: (), v0_1: string) {
            let v4: () = {
                Chat_contract::closure10(v0_1, ());
                ()
            };
            ()
        }
        pub fn closure2(
            v0_1: std::string::String,
            v1: near_sdk::AccountId,
            v2: near_sdk::AccountId,
            v3: u64,
            unitVar: (),
        ) {
            fn v19() {
                Chat_contract::closure3((), ());
            }
            let v20: () = {
                v19();
                ()
            };
            let patternInput: (
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ) = Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v61: Chat_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v305: Chat_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v61,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Chat_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Chat_contract::US6::US6_1
                } else {
                    let v84: () = {
                        v19();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v112: Option<i64> = patternInput_1.5.clone();
                    let v111: LrcPtr<Chat_contract::Mut4> = patternInput_1.4.clone();
                    let v110: LrcPtr<Chat_contract::Mut3> = patternInput_1.3.clone();
                    let v109: LrcPtr<Chat_contract::Mut2> = patternInput_1.2.clone();
                    let v108: LrcPtr<Chat_contract::Mut1> = patternInput_1.1.clone();
                    let v107: LrcPtr<Chat_contract::Mut0> = patternInput_1.0.clone();
                    let v131: string = Chat_contract::method13(
                        v107.clone(),
                        v108.clone(),
                        v109.clone(),
                        v110.clone(),
                        v111.clone(),
                        v112.clone(),
                        Chat_contract::method6(v107, v108, v109, v110, v111, v112),
                        Chat_contract::method10(),
                        v0_1,
                        v3,
                        v1.to_string(),
                        v2.to_string(),
                    );
                    let v147: () = {
                        v19();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v173: LrcPtr<Chat_contract::Mut3> = patternInput_2.3.clone();
                    let v171: LrcPtr<Chat_contract::Mut1> = patternInput_2.1.clone();
                    let v170: LrcPtr<Chat_contract::Mut0> = patternInput_2.0.clone();
                    let v192: () = {
                        Chat_contract::closure8(v170.clone(), ());
                        ()
                    };
                    let v215: string = if (v173.l0.get().clone()) == string("") {
                        v131.clone()
                    } else {
                        if (v131.clone()) == string("") {
                            v173.l0.get().clone()
                        } else {
                            append(
                                (append((v173.l0.get().clone()), string("\n"))),
                                (v131.clone()),
                            )
                        }
                    };
                    let v226: &str = &*v215.clone();
                    let v253 = v226.chars();
                    let v255 = v253;
                    let v257: Vec<char> = v255.collect::<Vec<_>>();
                    let v259: Vec<Vec<char>> = v257
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v261: bool = true;
                    let _vec_map: Vec<_> = v259
                        .into_iter()
                        .map(|x| {
                            //;
                            let v263: Vec<char> = x;
                            let v265: std::string::String = String::from_iter(v263);
                            let v267: bool = true;
                            v265
                        })
                        .collect::<Vec<_>>();
                    let v269: Vec<std::string::String> = _vec_map;
                    if if (v131.clone()) != string("") {
                        (v269.clone().len() as i32) <= 1_i32
                    } else {
                        false
                    } {
                        v173.l0.set(v215);
                        ()
                    } else {
                        v173.l0.set(string(""));
                        {
                            let v294: bool = true;
                            v269.into_iter().for_each(|x| {
                                //;
                                let v296: std::string::String = x;
                                let v298: bool = true;
                                near_sdk::log!("{}", v296);
                                let v300: bool = true;
                                let v302: bool = true;
                            }); //;
                            ()
                        }
                    }
                    (v171.l0.get().clone())(v131);
                    Chat_contract::US6::US6_0(
                        v170,
                        v171,
                        patternInput_2.2.clone(),
                        v173,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn closure11(unitVar: (), v0_1: std::string::String) -> Chat_contract::US7 {
            Chat_contract::US7::US7_0(v0_1)
        }
        pub fn method16() -> Func1<std::string::String, Chat_contract::US7> {
            Func1::new(move |v: std::string::String| Chat_contract::closure11((), v))
        }
        pub fn method17() -> string {
            let v83: &str = inline_colorization::color_yellow;
            let v94: std::string::String = String::from(v83);
            let v139: string = append(
                (fable_library_rust::String_::fromString(v94)),
                (Chat_contract::method11(getCharAt(toLower(string("Warning")), 0_i32))),
            );
            let v225: &str = inline_colorization::color_reset;
            let v236: std::string::String = String::from(v225);
            append((v139), (fable_library_rust::String_::fromString(v236)))
        }
        pub fn method19(v0_1: string) -> string {
            let v2: LrcPtr<Chat_contract::Mut3> = LrcPtr::new(Chat_contract::Mut3 {
                l0: MutCell::new(Chat_contract::method12()),
            });
            let v16: () = {
                Chat_contract::closure7(v2.clone(), string("{ "), ());
                ()
            };
            let v35: () = {
                Chat_contract::closure7(v2.clone(), string("account_alias"), ());
                ()
            };
            let v54: () = {
                Chat_contract::closure7(v2.clone(), string(" = "), ());
                ()
            };
            let v71: () = {
                Chat_contract::closure7(v2.clone(), v0_1, ());
                ()
            };
            let v90: () = {
                Chat_contract::closure7(v2.clone(), string(" }"), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method18(
            v0_1: LrcPtr<Chat_contract::Mut0>,
            v1: LrcPtr<Chat_contract::Mut1>,
            v2: LrcPtr<Chat_contract::Mut2>,
            v3: LrcPtr<Chat_contract::Mut3>,
            v4: LrcPtr<Chat_contract::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
        ) -> string {
            let v9: string = Chat_contract::method19(v8);
            Chat_contract::method15(append(
                (append(
                    (append(
                        (append(
                            (append(
                                (append((append((append((v6), string(" "))), (v7))), string(" #"))),
                                (toString(v0_1.l0.get().clone())),
                            )),
                            string(" "),
                        )),
                        string("chat_contract.claim_alias / alias already claimed"),
                    )),
                    string(" / "),
                )),
                (v9),
            ))
        }
        pub fn closure12(v0_1: std::string::String, unitVar: ()) {
            fn v16() {
                Chat_contract::closure3((), ());
            }
            let v17: () = {
                v16();
                ()
            };
            let patternInput: (
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ) = Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v58: Chat_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v302: Chat_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    3_i32
                        >= (find(
                            v58,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Chat_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Chat_contract::US6::US6_1
                } else {
                    let v81: () = {
                        v16();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v109: Option<i64> = patternInput_1.5.clone();
                    let v108: LrcPtr<Chat_contract::Mut4> = patternInput_1.4.clone();
                    let v107: LrcPtr<Chat_contract::Mut3> = patternInput_1.3.clone();
                    let v106: LrcPtr<Chat_contract::Mut2> = patternInput_1.2.clone();
                    let v105: LrcPtr<Chat_contract::Mut1> = patternInput_1.1.clone();
                    let v104: LrcPtr<Chat_contract::Mut0> = patternInput_1.0.clone();
                    let v128: string = Chat_contract::method18(
                        v104.clone(),
                        v105.clone(),
                        v106.clone(),
                        v107.clone(),
                        v108.clone(),
                        v109.clone(),
                        Chat_contract::method6(v104, v105, v106, v107, v108, v109),
                        Chat_contract::method17(),
                        sprintf!("{:?}", v0_1),
                    );
                    let v144: () = {
                        v16();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v170: LrcPtr<Chat_contract::Mut3> = patternInput_2.3.clone();
                    let v168: LrcPtr<Chat_contract::Mut1> = patternInput_2.1.clone();
                    let v167: LrcPtr<Chat_contract::Mut0> = patternInput_2.0.clone();
                    let v189: () = {
                        Chat_contract::closure8(v167.clone(), ());
                        ()
                    };
                    let v212: string = if (v170.l0.get().clone()) == string("") {
                        v128.clone()
                    } else {
                        if (v128.clone()) == string("") {
                            v170.l0.get().clone()
                        } else {
                            append(
                                (append((v170.l0.get().clone()), string("\n"))),
                                (v128.clone()),
                            )
                        }
                    };
                    let v223: &str = &*v212.clone();
                    let v250 = v223.chars();
                    let v252 = v250;
                    let v254: Vec<char> = v252.collect::<Vec<_>>();
                    let v256: Vec<Vec<char>> = v254
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v258: bool = true;
                    let _vec_map: Vec<_> = v256
                        .into_iter()
                        .map(|x| {
                            //;
                            let v260: Vec<char> = x;
                            let v262: std::string::String = String::from_iter(v260);
                            let v264: bool = true;
                            v262
                        })
                        .collect::<Vec<_>>();
                    let v266: Vec<std::string::String> = _vec_map;
                    if if (v128.clone()) != string("") {
                        (v266.clone().len() as i32) <= 1_i32
                    } else {
                        false
                    } {
                        v170.l0.set(v212);
                        ()
                    } else {
                        v170.l0.set(string(""));
                        {
                            let v291: bool = true;
                            v266.into_iter().for_each(|x| {
                                //;
                                let v293: std::string::String = x;
                                let v295: bool = true;
                                near_sdk::log!("{}", v293);
                                let v297: bool = true;
                                let v299: bool = true;
                            }); //;
                            ()
                        }
                    }
                    (v168.l0.get().clone())(v128);
                    Chat_contract::US6::US6_0(
                        v167,
                        v168,
                        patternInput_2.2.clone(),
                        v170,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn method20(
            v0_1: LrcPtr<Chat_contract::Mut0>,
            v1: LrcPtr<Chat_contract::Mut1>,
            v2: LrcPtr<Chat_contract::Mut2>,
            v3: LrcPtr<Chat_contract::Mut3>,
            v4: LrcPtr<Chat_contract::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
        ) -> string {
            let v9: string = Chat_contract::method19(v8);
            Chat_contract::method15(append(
                (append(
                    (append(
                        (append(
                            (append(
                                (append((append((append((v6), string(" "))), (v7))), string(" #"))),
                                (toString(v0_1.l0.get().clone())),
                            )),
                            string(" "),
                        )),
                        string("chat_contract.claim_alias"),
                    )),
                    string(" / "),
                )),
                (v9),
            ))
        }
        pub fn closure13(v0_1: Option<std::string::String>, unitVar: ()) {
            fn v16() {
                Chat_contract::closure3((), ());
            }
            let v17: () = {
                v16();
                ()
            };
            let patternInput: (
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ) = Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v58: Chat_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v302: Chat_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v58,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Chat_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Chat_contract::US6::US6_1
                } else {
                    let v81: () = {
                        v16();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v109: Option<i64> = patternInput_1.5.clone();
                    let v108: LrcPtr<Chat_contract::Mut4> = patternInput_1.4.clone();
                    let v107: LrcPtr<Chat_contract::Mut3> = patternInput_1.3.clone();
                    let v106: LrcPtr<Chat_contract::Mut2> = patternInput_1.2.clone();
                    let v105: LrcPtr<Chat_contract::Mut1> = patternInput_1.1.clone();
                    let v104: LrcPtr<Chat_contract::Mut0> = patternInput_1.0.clone();
                    let v128: string = Chat_contract::method20(
                        v104.clone(),
                        v105.clone(),
                        v106.clone(),
                        v107.clone(),
                        v108.clone(),
                        v109.clone(),
                        Chat_contract::method6(v104, v105, v106, v107, v108, v109),
                        Chat_contract::method10(),
                        sprintf!("{:?}", v0_1),
                    );
                    let v144: () = {
                        v16();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v170: LrcPtr<Chat_contract::Mut3> = patternInput_2.3.clone();
                    let v168: LrcPtr<Chat_contract::Mut1> = patternInput_2.1.clone();
                    let v167: LrcPtr<Chat_contract::Mut0> = patternInput_2.0.clone();
                    let v189: () = {
                        Chat_contract::closure8(v167.clone(), ());
                        ()
                    };
                    let v212: string = if (v170.l0.get().clone()) == string("") {
                        v128.clone()
                    } else {
                        if (v128.clone()) == string("") {
                            v170.l0.get().clone()
                        } else {
                            append(
                                (append((v170.l0.get().clone()), string("\n"))),
                                (v128.clone()),
                            )
                        }
                    };
                    let v223: &str = &*v212.clone();
                    let v250 = v223.chars();
                    let v252 = v250;
                    let v254: Vec<char> = v252.collect::<Vec<_>>();
                    let v256: Vec<Vec<char>> = v254
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v258: bool = true;
                    let _vec_map: Vec<_> = v256
                        .into_iter()
                        .map(|x| {
                            //;
                            let v260: Vec<char> = x;
                            let v262: std::string::String = String::from_iter(v260);
                            let v264: bool = true;
                            v262
                        })
                        .collect::<Vec<_>>();
                    let v266: Vec<std::string::String> = _vec_map;
                    if if (v128.clone()) != string("") {
                        (v266.clone().len() as i32) <= 1_i32
                    } else {
                        false
                    } {
                        v170.l0.set(v212);
                        ()
                    } else {
                        v170.l0.set(string(""));
                        {
                            let v291: bool = true;
                            v266.into_iter().for_each(|x| {
                                //;
                                let v293: std::string::String = x;
                                let v295: bool = true;
                                near_sdk::log!("{}", v293);
                                let v297: bool = true;
                                let v299: bool = true;
                            }); //;
                            ()
                        }
                    }
                    (v168.l0.get().clone())(v128);
                    Chat_contract::US6::US6_0(
                        v167,
                        v168,
                        patternInput_2.2.clone(),
                        v170,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn closure14(unitVar: (), unitVar_1: ()) {
            ();
        }
        pub fn method21() -> Func0<()> {
            Func0::new(move || Chat_contract::closure14((), ()))
        }
        pub fn closure15(v0_1: Func0<()>, unitVar: ()) {
            fn v16() {
                Chat_contract::closure3((), ());
            }
            let v17: () = {
                v16();
                ()
            };
            let patternInput: (
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ) = Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v58: Chat_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v296: Chat_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v58,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Chat_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Chat_contract::US6::US6_1
                } else {
                    let v81: () = {
                        v16();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v122: string = Chat_contract::method6(
                        patternInput_1.0.clone(),
                        patternInput_1.1.clone(),
                        patternInput_1.2.clone(),
                        patternInput_1.3.clone(),
                        patternInput_1.4.clone(),
                        patternInput_1.5.clone(),
                    );
                    let v123: string = Chat_contract::method10();
                    let v139: () = {
                        v16();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v165: LrcPtr<Chat_contract::Mut3> = patternInput_2.3.clone();
                    let v163: LrcPtr<Chat_contract::Mut1> = patternInput_2.1.clone();
                    let v162: LrcPtr<Chat_contract::Mut0> = patternInput_2.0.clone();
                    let v184: () = {
                        Chat_contract::closure8(v162.clone(), ());
                        ()
                    };
                    let v208: string = if (v165.l0.get().clone()) == string("") {
                        string("")
                    } else {
                        v165.l0.get().clone()
                    };
                    let v219: &str = &*v208;
                    let v246 = v219.chars();
                    let v248 = v246;
                    let v250: Vec<char> = v248.collect::<Vec<_>>();
                    let v252: Vec<Vec<char>> = v250
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v254: bool = true;
                    let _vec_map: Vec<_> = v252
                        .into_iter()
                        .map(|x| {
                            //;
                            let v256: Vec<char> = x;
                            let v258: std::string::String = String::from_iter(v256);
                            let v260: bool = true;
                            v258
                        })
                        .collect::<Vec<_>>();
                    let v262: Vec<std::string::String> = _vec_map;
                    v165.l0.set(string(""));
                    {
                        let v285: bool = true;
                        v262.into_iter().for_each(|x| {
                            //;
                            let v287: std::string::String = x;
                            let v289: bool = true;
                            near_sdk::log!("{}", v287);
                            let v291: bool = true;
                            let v293: bool = true;
                        }); //;
                        ()
                    }
                    (v163.l0.get().clone())(string(""));
                    Chat_contract::US6::US6_0(
                        v162,
                        v163,
                        patternInput_2.2.clone(),
                        v165,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn method23(
            v0_1: near_sdk::AccountId,
            v1: Option<(std::string::String, u64, u32)>,
        ) -> string {
            let v3: LrcPtr<Chat_contract::Mut3> = LrcPtr::new(Chat_contract::Mut3 {
                l0: MutCell::new(Chat_contract::method12()),
            });
            let v17: () = {
                Chat_contract::closure7(v3.clone(), string("{ "), ());
                ()
            };
            let v36: () = {
                Chat_contract::closure7(v3.clone(), string("account_id"), ());
                ()
            };
            let v55: () = {
                Chat_contract::closure7(v3.clone(), string(" = "), ());
                ()
            };
            let v75: std::string::String = format!("{:#?}", v0_1);
            let v114: () = {
                Chat_contract::closure7(
                    v3.clone(),
                    fable_library_rust::String_::fromString(v75),
                    (),
                );
                ()
            };
            let v133: () = {
                Chat_contract::closure7(v3.clone(), string("; "), ());
                ()
            };
            let v152: () = {
                Chat_contract::closure7(v3.clone(), string("result"), ());
                ()
            };
            let v169: () = {
                Chat_contract::closure7(v3.clone(), string(" = "), ());
                ()
            };
            let v189: std::string::String = format!("{:#?}", v1);
            let v228: () = {
                Chat_contract::closure7(
                    v3.clone(),
                    fable_library_rust::String_::fromString(v189),
                    (),
                );
                ()
            };
            let v247: () = {
                Chat_contract::closure7(v3.clone(), string(" }"), ());
                ()
            };
            v3.l0.get().clone()
        }
        pub fn method22(
            v0_1: LrcPtr<Chat_contract::Mut0>,
            v1: LrcPtr<Chat_contract::Mut1>,
            v2: LrcPtr<Chat_contract::Mut2>,
            v3: LrcPtr<Chat_contract::Mut3>,
            v4: LrcPtr<Chat_contract::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: near_sdk::AccountId,
            v9: Option<(std::string::String, u64, u32)>,
        ) -> string {
            let v10: string = Chat_contract::method23(v8, v9);
            Chat_contract::method15(append(
                (append(
                    (append(
                        (append(
                            (append(
                                (append((append((append((v6), string(" "))), (v7))), string(" #"))),
                                (toString(v0_1.l0.get().clone())),
                            )),
                            string(" "),
                        )),
                        string("chat_contract.get_account_info"),
                    )),
                    string(" / "),
                )),
                (v10),
            ))
        }
        pub fn closure16(
            v0_1: near_sdk::AccountId,
            v1: Option<(std::string::String, u64, u32)>,
            unitVar: (),
        ) {
            fn v17() {
                Chat_contract::closure3((), ());
            }
            let v18: () = {
                v17();
                ()
            };
            let patternInput: (
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ) = Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v59: Chat_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v299: Chat_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v59,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Chat_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Chat_contract::US6::US6_1
                } else {
                    let v82: () = {
                        v17();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v110: Option<i64> = patternInput_1.5.clone();
                    let v109: LrcPtr<Chat_contract::Mut4> = patternInput_1.4.clone();
                    let v108: LrcPtr<Chat_contract::Mut3> = patternInput_1.3.clone();
                    let v107: LrcPtr<Chat_contract::Mut2> = patternInput_1.2.clone();
                    let v106: LrcPtr<Chat_contract::Mut1> = patternInput_1.1.clone();
                    let v105: LrcPtr<Chat_contract::Mut0> = patternInput_1.0.clone();
                    let v125: string = Chat_contract::method22(
                        v105.clone(),
                        v106.clone(),
                        v107.clone(),
                        v108.clone(),
                        v109.clone(),
                        v110.clone(),
                        Chat_contract::method6(v105, v106, v107, v108, v109, v110),
                        Chat_contract::method10(),
                        v0_1,
                        v1,
                    );
                    let v141: () = {
                        v17();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v167: LrcPtr<Chat_contract::Mut3> = patternInput_2.3.clone();
                    let v165: LrcPtr<Chat_contract::Mut1> = patternInput_2.1.clone();
                    let v164: LrcPtr<Chat_contract::Mut0> = patternInput_2.0.clone();
                    let v186: () = {
                        Chat_contract::closure8(v164.clone(), ());
                        ()
                    };
                    let v209: string = if (v167.l0.get().clone()) == string("") {
                        v125.clone()
                    } else {
                        if (v125.clone()) == string("") {
                            v167.l0.get().clone()
                        } else {
                            append(
                                (append((v167.l0.get().clone()), string("\n"))),
                                (v125.clone()),
                            )
                        }
                    };
                    let v220: &str = &*v209.clone();
                    let v247 = v220.chars();
                    let v249 = v247;
                    let v251: Vec<char> = v249.collect::<Vec<_>>();
                    let v253: Vec<Vec<char>> = v251
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v255: bool = true;
                    let _vec_map: Vec<_> = v253
                        .into_iter()
                        .map(|x| {
                            //;
                            let v257: Vec<char> = x;
                            let v259: std::string::String = String::from_iter(v257);
                            let v261: bool = true;
                            v259
                        })
                        .collect::<Vec<_>>();
                    let v263: Vec<std::string::String> = _vec_map;
                    if if (v125.clone()) != string("") {
                        (v263.clone().len() as i32) <= 1_i32
                    } else {
                        false
                    } {
                        v167.l0.set(v209);
                        ()
                    } else {
                        v167.l0.set(string(""));
                        {
                            let v288: bool = true;
                            v263.into_iter().for_each(|x| {
                                //;
                                let v290: std::string::String = x;
                                let v292: bool = true;
                                near_sdk::log!("{}", v290);
                                let v294: bool = true;
                                let v296: bool = true;
                            }); //;
                            ()
                        }
                    }
                    (v165.l0.get().clone())(v125);
                    Chat_contract::US6::US6_0(
                        v164,
                        v165,
                        patternInput_2.2.clone(),
                        v167,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn closure17(unitVar: (), unitVar_1: ()) {
            fn v15() {
                Chat_contract::closure3((), ());
            }
            let v16: () = {
                v15();
                ()
            };
            let patternInput: (
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ) = Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v57: Chat_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v295: Chat_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v57,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Chat_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Chat_contract::US6::US6_1
                } else {
                    let v80: () = {
                        v15();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v121: string = Chat_contract::method6(
                        patternInput_1.0.clone(),
                        patternInput_1.1.clone(),
                        patternInput_1.2.clone(),
                        patternInput_1.3.clone(),
                        patternInput_1.4.clone(),
                        patternInput_1.5.clone(),
                    );
                    let v122: string = Chat_contract::method10();
                    let v138: () = {
                        v15();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v164: LrcPtr<Chat_contract::Mut3> = patternInput_2.3.clone();
                    let v162: LrcPtr<Chat_contract::Mut1> = patternInput_2.1.clone();
                    let v161: LrcPtr<Chat_contract::Mut0> = patternInput_2.0.clone();
                    let v183: () = {
                        Chat_contract::closure8(v161.clone(), ());
                        ()
                    };
                    let v207: string = if (v164.l0.get().clone()) == string("") {
                        string("")
                    } else {
                        v164.l0.get().clone()
                    };
                    let v218: &str = &*v207;
                    let v245 = v218.chars();
                    let v247 = v245;
                    let v249: Vec<char> = v247.collect::<Vec<_>>();
                    let v251: Vec<Vec<char>> = v249
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v253: bool = true;
                    let _vec_map: Vec<_> = v251
                        .into_iter()
                        .map(|x| {
                            //;
                            let v255: Vec<char> = x;
                            let v257: std::string::String = String::from_iter(v255);
                            let v259: bool = true;
                            v257
                        })
                        .collect::<Vec<_>>();
                    let v261: Vec<std::string::String> = _vec_map;
                    v164.l0.set(string(""));
                    {
                        let v284: bool = true;
                        v261.into_iter().for_each(|x| {
                            //;
                            let v286: std::string::String = x;
                            let v288: bool = true;
                            near_sdk::log!("{}", v286);
                            let v290: bool = true;
                            let v292: bool = true;
                        }); //;
                        ()
                    }
                    (v162.l0.get().clone())(string(""));
                    Chat_contract::US6::US6_0(
                        v161,
                        v162,
                        patternInput_2.2.clone(),
                        v164,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn method25(v0_1: std::string::String) -> string {
            let v2: LrcPtr<Chat_contract::Mut3> = LrcPtr::new(Chat_contract::Mut3 {
                l0: MutCell::new(Chat_contract::method12()),
            });
            let v16: () = {
                Chat_contract::closure7(v2.clone(), string("{ "), ());
                ()
            };
            let v35: () = {
                Chat_contract::closure7(v2.clone(), string("alias"), ());
                ()
            };
            let v54: () = {
                Chat_contract::closure7(v2.clone(), string(" = "), ());
                ()
            };
            let v74: std::string::String = format!("{:#?}", v0_1);
            let v113: () = {
                Chat_contract::closure7(
                    v2.clone(),
                    fable_library_rust::String_::fromString(v74),
                    (),
                );
                ()
            };
            let v132: () = {
                Chat_contract::closure7(v2.clone(), string(" }"), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method24(
            v0_1: LrcPtr<Chat_contract::Mut0>,
            v1: LrcPtr<Chat_contract::Mut1>,
            v2: LrcPtr<Chat_contract::Mut2>,
            v3: LrcPtr<Chat_contract::Mut3>,
            v4: LrcPtr<Chat_contract::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: std::string::String,
        ) -> string {
            let v9: string = Chat_contract::method25(v8);
            Chat_contract::method15(append(
                (append(
                    (append(
                        (append(
                            (append(
                                (append((append((append((v6), string(" "))), (v7))), string(" #"))),
                                (toString(v0_1.l0.get().clone())),
                            )),
                            string(" "),
                        )),
                        string("chat_contract.get_alias_map"),
                    )),
                    string(" / "),
                )),
                (v9),
            ))
        }
        pub fn closure18(v0_1: std::string::String, unitVar: ()) {
            fn v16() {
                Chat_contract::closure3((), ());
            }
            let v17: () = {
                v16();
                ()
            };
            let patternInput: (
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ) = Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v58: Chat_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v298: Chat_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v58,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Chat_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Chat_contract::US6::US6_1
                } else {
                    let v81: () = {
                        v16();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v109: Option<i64> = patternInput_1.5.clone();
                    let v108: LrcPtr<Chat_contract::Mut4> = patternInput_1.4.clone();
                    let v107: LrcPtr<Chat_contract::Mut3> = patternInput_1.3.clone();
                    let v106: LrcPtr<Chat_contract::Mut2> = patternInput_1.2.clone();
                    let v105: LrcPtr<Chat_contract::Mut1> = patternInput_1.1.clone();
                    let v104: LrcPtr<Chat_contract::Mut0> = patternInput_1.0.clone();
                    let v124: string = Chat_contract::method24(
                        v104.clone(),
                        v105.clone(),
                        v106.clone(),
                        v107.clone(),
                        v108.clone(),
                        v109.clone(),
                        Chat_contract::method6(v104, v105, v106, v107, v108, v109),
                        Chat_contract::method10(),
                        v0_1,
                    );
                    let v140: () = {
                        v16();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v166: LrcPtr<Chat_contract::Mut3> = patternInput_2.3.clone();
                    let v164: LrcPtr<Chat_contract::Mut1> = patternInput_2.1.clone();
                    let v163: LrcPtr<Chat_contract::Mut0> = patternInput_2.0.clone();
                    let v185: () = {
                        Chat_contract::closure8(v163.clone(), ());
                        ()
                    };
                    let v208: string = if (v166.l0.get().clone()) == string("") {
                        v124.clone()
                    } else {
                        if (v124.clone()) == string("") {
                            v166.l0.get().clone()
                        } else {
                            append(
                                (append((v166.l0.get().clone()), string("\n"))),
                                (v124.clone()),
                            )
                        }
                    };
                    let v219: &str = &*v208.clone();
                    let v246 = v219.chars();
                    let v248 = v246;
                    let v250: Vec<char> = v248.collect::<Vec<_>>();
                    let v252: Vec<Vec<char>> = v250
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v254: bool = true;
                    let _vec_map: Vec<_> = v252
                        .into_iter()
                        .map(|x| {
                            //;
                            let v256: Vec<char> = x;
                            let v258: std::string::String = String::from_iter(v256);
                            let v260: bool = true;
                            v258
                        })
                        .collect::<Vec<_>>();
                    let v262: Vec<std::string::String> = _vec_map;
                    if if (v124.clone()) != string("") {
                        (v262.clone().len() as i32) <= 1_i32
                    } else {
                        false
                    } {
                        v166.l0.set(v208);
                        ()
                    } else {
                        v166.l0.set(string(""));
                        {
                            let v287: bool = true;
                            v262.into_iter().for_each(|x| {
                                //;
                                let v289: std::string::String = x;
                                let v291: bool = true;
                                near_sdk::log!("{}", v289);
                                let v293: bool = true;
                                let v295: bool = true;
                            }); //;
                            ()
                        }
                    }
                    (v164.l0.get().clone())(v124);
                    Chat_contract::US6::US6_0(
                        v163,
                        v164,
                        patternInput_2.2.clone(),
                        v166,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn method26() -> Func0<()> {
            Func0::new(move || Chat_contract::closure14((), ()))
        }
        pub fn closure19(v0_1: Func0<()>, unitVar: ()) {
            fn v16() {
                Chat_contract::closure3((), ());
            }
            let v17: () = {
                v16();
                ()
            };
            let patternInput: (
                LrcPtr<Chat_contract::Mut0>,
                LrcPtr<Chat_contract::Mut1>,
                LrcPtr<Chat_contract::Mut2>,
                LrcPtr<Chat_contract::Mut3>,
                LrcPtr<Chat_contract::Mut4>,
                Option<i64>,
            ) = Chat_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v58: Chat_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v296: Chat_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v58,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Chat_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Chat_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Chat_contract::US6::US6_1
                } else {
                    let v81: () = {
                        v16();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v122: string = Chat_contract::method6(
                        patternInput_1.0.clone(),
                        patternInput_1.1.clone(),
                        patternInput_1.2.clone(),
                        patternInput_1.3.clone(),
                        patternInput_1.4.clone(),
                        patternInput_1.5.clone(),
                    );
                    let v123: string = Chat_contract::method10();
                    let v139: () = {
                        v16();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Chat_contract::Mut0>,
                        LrcPtr<Chat_contract::Mut1>,
                        LrcPtr<Chat_contract::Mut2>,
                        LrcPtr<Chat_contract::Mut3>,
                        LrcPtr<Chat_contract::Mut4>,
                        Option<i64>,
                    ) = Chat_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v165: LrcPtr<Chat_contract::Mut3> = patternInput_2.3.clone();
                    let v163: LrcPtr<Chat_contract::Mut1> = patternInput_2.1.clone();
                    let v162: LrcPtr<Chat_contract::Mut0> = patternInput_2.0.clone();
                    let v184: () = {
                        Chat_contract::closure8(v162.clone(), ());
                        ()
                    };
                    let v208: string = if (v165.l0.get().clone()) == string("") {
                        string("")
                    } else {
                        v165.l0.get().clone()
                    };
                    let v219: &str = &*v208;
                    let v246 = v219.chars();
                    let v248 = v246;
                    let v250: Vec<char> = v248.collect::<Vec<_>>();
                    let v252: Vec<Vec<char>> = v250
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v254: bool = true;
                    let _vec_map: Vec<_> = v252
                        .into_iter()
                        .map(|x| {
                            //;
                            let v256: Vec<char> = x;
                            let v258: std::string::String = String::from_iter(v256);
                            let v260: bool = true;
                            v258
                        })
                        .collect::<Vec<_>>();
                    let v262: Vec<std::string::String> = _vec_map;
                    v165.l0.set(string(""));
                    {
                        let v285: bool = true;
                        v262.into_iter().for_each(|x| {
                            //;
                            let v287: std::string::String = x;
                            let v289: bool = true;
                            near_sdk::log!("{}", v287);
                            let v291: bool = true;
                            let v293: bool = true;
                        }); //;
                        ()
                    }
                    (v163.l0.get().clone())(string(""));
                    Chat_contract::US6::US6_0(
                        v162,
                        v163,
                        patternInput_2.2.clone(),
                        v165,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn closure0(unitVar: (), unitVar_1: ()) {} //;
        #[near_sdk::near_bindgen] //;
        #[derive(
            //;
            near_sdk::PanicOnDefault, //;
            borsh::BorshDeserialize,  //;
            borsh::BorshSerialize,    //;
        )] //;
        pub struct State(
            //;
            /*;
            {
                let v10:
                        */
            (
                u32,
                near_sdk::store::IterableSet<near_sdk::AccountId>,
                near_sdk::store::IterableSet<std::string::String>,
                near_sdk::store::LookupMap<near_sdk::AccountId, std::string::String>,
                near_sdk::store::LookupMap<
                    std::string::String,
                    std::collections::HashMap<near_sdk::AccountId, (u64, u32)>,
                >,
            ), /* =
                   fable_library_rust::Native_::getZero::<()>();
               */
        );
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            #[init] //;
            pub fn new() -> Self {
                // 1;
                {
                    let v19: bool = true; /*;
                    let v21: */
                    () /* = fable_library_rust::Native_::getZero();
                    let v25: bool = true; */;
                    let v27: string = string("b\"account_set\"");
                    let v28: &[u8] = b"account_set";
                    let v30: near_sdk::store::IterableSet<near_sdk::AccountId> =
                        near_sdk::store::IterableSet::new(v28);
                    let v32: string = string("b\"alias_set\"");
                    let v33: &[u8] = b"alias_set";
                    let v35: near_sdk::store::IterableSet<std::string::String> =
                        near_sdk::store::IterableSet::new(v33);
                    let v37: string = string("b\"account_map\"");
                    let v38: &[u8] = b"account_map";
                    let v40: near_sdk::store::LookupMap<near_sdk::AccountId, std::string::String> =
                        near_sdk::store::LookupMap::new(v38);
                    let v42: string = string("b\"alias_map\"");
                    let v43: &[u8] = b"alias_map";
                    {
                        let x: (
                            u32,
                            near_sdk::store::IterableSet<near_sdk::AccountId>,
                            near_sdk::store::IterableSet<std::string::String>,
                            near_sdk::store::LookupMap<near_sdk::AccountId, std::string::String>,
                            near_sdk::store::LookupMap<
                                std::string::String,
                                std::collections::HashMap<near_sdk::AccountId, (u64, u32)>,
                            >,
                        ) = (2_u32, v30, v35, v40, near_sdk::store::LookupMap::new(v43));
                        Self(x) // x
                    }
                } // 2.;
            } // 1.;
        } /* c;
        {
        let v50: bool =
        true; // ??? / i: 1uy / i': 1uy / acc: 0uy / n: 2uy;
        let v52: bool =
        true; */
 // ???? / i: 1uy / i': 2uy / acc: 0uy / n: 2uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            fn is_valid_alias(alias: String) -> bool {
                //;
                {
                    let v57: std::string::String = alias;
                    let v59: string = fable_library_rust::String_::fromString(v57);
                    let v60: i32 = length(v59.clone());
                    {
                        let x_1: bool = if (v60) > 0_i32 {
                            if (v60) < 64_i32 {
                                if (startsWith3(v59.clone(), string("-"), false)) == false {
                                    if (endsWith3(v59.clone(), string("-"), false)) == false {
                                        let v85: &str = &*v59;
                                        let v112 = v85.chars();
                                        let v114: bool = true; // 1;
                                        let mut v112 = v112;
                                        v112.all(|x| {
                                            Func1::new(move |v: char| {
                                                Chat_contract::closure1((), v)
                                            })(x)
                                        })
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        x_1
                    }
                }
            } //;
        } /* c;
        {
        let v126: bool =
        true; // ??? / i: 2uy / i': 1uy / acc: 2uy / n: 1uy;
        let v128: bool =
        true; */
 // ???? / i: 2uy / i': 2uy / acc: 2uy / n: 1uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            pub fn generate_cid(
                //;
                &self,            //;
                content: Vec<u8>, //;
            ) -> String {
                //;
                {
                    let v136: Vec<u8> = content;
                    fn encode_u64(value: u64) -> Vec<u8> {
                        //;
                        let mut buffer = unsigned_varint::encode::u64_buffer(); //;
                        unsigned_varint::encode::u64(value, &mut buffer).to_vec() //;
                    } //;
                    fn sha256_hash(content: &[u8]) -> Vec<u8> {
                        //;
                        let mut hasher: sha2::Sha256 = sha2::Digest::new(); //;
                        sha2::Digest::update(&mut hasher, content); //;
                        sha2::Digest::finalize(hasher).to_vec() //;
                    } //;
                    let version: u8 = 1; //;
                    let codec_raw: u64 = 0x55; //;
                    let codec_bytes = encode_u64(codec_raw); //;
                    let hash_result = sha256_hash(&v136); //;
                    let multihash = std::iter::once(0x12) //;
                        .chain(std::iter::once(32)) //;
                        .chain(hash_result.into_iter()) //;
                        .collect(); //;
                    let cid_bytes = [vec![version], codec_bytes, multihash].concat(); //;
                    let result = multibase::encode(multibase::Base::Base32Lower, &cid_bytes); //;
                    {
                        let x_2: std::string::String = result;
                        x_2
                    }
                }
            } //;
        } /* c;
        {
        let v161: bool =
        true; // ??? / i: 3uy / i': 1uy / acc: 3uy / n: 2uy;
        let v163: bool =
        true; */
 // ???? / i: 3uy / i': 2uy / acc: 3uy / n: 2uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            #[result_serializer(borsh)] //;
            pub fn generate_cid_borsh(
                //;
                &self,                                 //;
                #[serializer(borsh)] content: Vec<u8>, //;
            ) -> String {
                //;
                self.generate_cid(content) //;
            } //;
        } /* c;
        {
        let v175: bool =
        true; // ??? / i: 4uy / i': 1uy / acc: 5uy / n: 1uy;
        let v177: bool =
        true; */
 // ???? / i: 4uy / i': 2uy / acc: 5uy / n: 1uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            pub fn claim_alias(
                //;
                &mut self,     //;
                alias: String, //;
            ) {
                //;
                {
                    let v185: &mut (
                        u32,
                        near_sdk::store::IterableSet<near_sdk::AccountId>,
                        near_sdk::store::IterableSet<std::string::String>,
                        near_sdk::store::LookupMap<near_sdk::AccountId, std::string::String>,
                        near_sdk::store::LookupMap<
                            std::string::String,
                            std::collections::HashMap<near_sdk::AccountId, (u64, u32)>,
                        >,
                    ) = &mut self.0;
                    let v187: std::string::String = alias;
                    let v189: &mut near_sdk::store::IterableSet<near_sdk::AccountId> = &mut v185.1;
                    let v191: &mut near_sdk::store::IterableSet<std::string::String> = &mut v185.2;
                    let v193: &mut near_sdk::store::LookupMap<
                        near_sdk::AccountId,
                        std::string::String,
                    > = &mut v185.3;
                    let v195: &mut near_sdk::store::LookupMap<
                        std::string::String,
                        std::collections::HashMap<near_sdk::AccountId, (u64, u32)>,
                    > = &mut v185.4;
                    let v197: near_sdk::AccountId = near_sdk::env::signer_account_id();
                    let v199: near_sdk::AccountId = near_sdk::env::predecessor_account_id();
                    let v201: u64 = near_sdk::env::block_timestamp();
                    let v519: () = {
                        Chat_contract::closure2(v187.clone(), v197.clone(), v199, v201, ());
                        ()
                    };
                    let v839: string = fable_library_rust::String_::fromString(v187.clone());
                    let v840: i32 = length(v839.clone());
                    let v907: bool = if (if (v840) > 0_i32 {
                        if (v840) < 64_i32 {
                            if (startsWith3(v839.clone(), string("-"), false)) == false {
                                if (endsWith3(v839.clone(), string("-"), false)) == false {
                                    let v865: &str = &*v839;
                                    let v892 = v865.chars();
                                    let v894: bool = true; // 1;
                                    let mut v892 = v892;
                                    v892.all(|x| {
                                        Func1::new(move |v_1: char| {
                                            Chat_contract::closure1((), v_1)
                                        })(x)
                                    })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }) == false
                    {
                        let v906: bool = true;
                        near_sdk::env::panic_str(&*string(
                            "chat_contract.claim_alias / invalid alias",
                        )); //;
                        true
                    } else {
                        false
                    };
                    let v909: Option<&std::string::String> = v193.get(&v197.clone());
                    let v911: Option<std::string::String> = v909.cloned();
                    let v936: Chat_contract::US7 = defaultValue(
                        Chat_contract::US7::US7_1,
                        map(Chat_contract::method16(), v911.clone()),
                    );
                    if let Chat_contract::US7::US7_0(v936_0_0) = &v936 {
                        let v940: std::string::String = v936_0_0.clone();
                        if referenceEquals(&v940, &v187) {
                            let v1262: () = {
                                Chat_contract::closure12(v940, ());
                                ()
                            };
                            ()
                        } else {
                            let v1898: () = {
                                Chat_contract::closure13(v911.clone(), ());
                                ()
                            };
                            v195 //;
                                .get_mut(&v940) //;
                                .unwrap() //;
                                .remove(&v197.clone()); //;
                            v193.insert(v197.clone().clone(), v187.clone().clone()); //;
                            {
                                let v2223: bool = v189.insert(v197.clone());
                                let v2225: bool = v191.insert(v187.clone());
                                let new_alias_account_map = match v195.get(&v187.clone()) {
                                    //;
                                    None => {
                                        //;
                                        let mut new_map = std::collections::HashMap::new(); //;
                                        new_map.insert(v197.clone(), (v201, 0u32)); //;
                                        new_map //;
                                    } //;
                                    Some(accounts) => {
                                        //;
                                        let mut accounts_vec = accounts.iter().collect::<Vec<_>>(); //;
                                        accounts_vec.sort_unstable_by_key(|(_, (_, index))| index); //;
                                        let mut new_map = accounts_vec //;
                                                                    .iter() //;
                                                                    .enumerate() //;
                                                                    .map(|(i, (signer_account_id, (timestamp, _)))| { //;
                                                                      ((*signer_account_id).clone(), (*timestamp, i as u32)) //;
                                                                    }) //;
                                                                    .collect::<std::collections::HashMap<_, _>>(); //;
                                        new_map.insert(
                                            v197.clone(),
                                            (v201, accounts_vec.len() as u32),
                                        ); //;
                                        new_map //;
                                    } //;
                                }; //;
                                v195.insert(v187.clone(), new_alias_account_map); //;
                                ()
                            }
                        }
                    } else {
                        let v2564: () = {
                            Chat_contract::closure13(v911, ());
                            ()
                        };
                        if let Chat_contract::US7::US7_1 = &v936 {
                            ()
                        } else {
                            panic!("{}", string("Match failure"),)
                        }
                        v193.insert(v197.clone().clone(), v187.clone().clone()); //;
                        {
                            let v2885: bool = v189.insert(v197.clone());
                            let v2887: bool = v191.insert(v187.clone());
                            let new_alias_account_map = match v195.get(&v187.clone()) {
                                //;
                                None => {
                                    //;
                                    let mut new_map = std::collections::HashMap::new(); //;
                                    new_map.insert(v197.clone(), (v201, 0u32)); //;
                                    new_map //;
                                } //;
                                Some(accounts) => {
                                    //;
                                    let mut accounts_vec = accounts.iter().collect::<Vec<_>>(); //;
                                    accounts_vec.sort_unstable_by_key(|(_, (_, index))| index); //;
                                    let mut new_map = accounts_vec //;
                                                                .iter() //;
                                                                .enumerate() //;
                                                                .map(|(i, (signer_account_id, (timestamp, _)))| { //;
                                                                  ((*signer_account_id).clone(), (*timestamp, i as u32)) //;
                                                                }) //;
                                                                .collect::<std::collections::HashMap<_, _>>(); //;
                                    new_map.insert(v197, (v201, accounts_vec.len() as u32)); //;
                                    new_map //;
                                } //;
                            }; //;
                            v195.insert(v187, new_alias_account_map); //;
                            ()
                        }
                    }
                    {
                        let v3221: () = {
                            Chat_contract::closure15(Chat_contract::method21(), ());
                            ()
                        };
                    } //;
                } //;
            } //;
        } /* c;
        {
        let v3539: bool =
        true; // ??? / i: 5uy / i': 1uy / acc: 6uy / n: 3uy;
        let v3541: bool =
        true; */
 // ???? / i: 5uy / i': 2uy / acc: 6uy / n: 3uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            pub fn get_account_info(
                //;
                &self,                           //;
                account_id: near_sdk::AccountId, //;
            ) -> Option<(String, u64, u32)> {
                //;
                {
                    let v3549: &(
                        u32,
                        near_sdk::store::IterableSet<near_sdk::AccountId>,
                        near_sdk::store::IterableSet<std::string::String>,
                        near_sdk::store::LookupMap<near_sdk::AccountId, std::string::String>,
                        near_sdk::store::LookupMap<
                            std::string::String,
                            std::collections::HashMap<near_sdk::AccountId, (u64, u32)>,
                        >,
                    ) = &self.0;
                    let v3551: near_sdk::AccountId = account_id;
                    let v3553: &near_sdk::store::LookupMap<
                        near_sdk::AccountId,
                        std::string::String,
                    > = &v3549.3;
                    let v3555: &near_sdk::store::LookupMap<
                        std::string::String,
                        std::collections::HashMap<near_sdk::AccountId, (u64, u32)>,
                    > = &v3549.4;
                    let v3557: bool = true;
                    let result = v3553.get(&v3551.clone()).and_then(|alias| {
                        //;
                        let v3559: bool = true;
                        v3555.get(alias).map(|accounts| {
                            //;
                            let v3561: bool = true;
                            let result = (alias.clone(), *accounts.get(&v3551).unwrap()); //;
                            let v3563: bool = true;
                            (result.0, result.1.0, result.1.1)
                        })
                    }); //;
                    let v3565: Option<(std::string::String, u64, u32)> = result;
                    let v3879: () = {
                        Chat_contract::closure16(v3551, v3565.clone(), ());
                        ()
                    };
                    let v4505: () = {
                        Chat_contract::closure17((), ());
                        ()
                    };
                    v3565
                } // 4;
            } // 1;
        } /* c;
        {
        let v4821:
        bool =
        true; // ??? / i: 6uy / i': 1uy / acc: 9uy / n: 1uy;
        let v4823:
        bool =
        true; */
 // ???? / i: 6uy / i': 2uy / acc: 9uy / n: 1uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            pub fn get_alias_map(
                //;
                &self,         //;
                alias: String, //;
            ) -> Option<std::collections::HashMap<near_sdk::AccountId, (u64, u32)>> {
                //;
                {
                    let v4831: &near_sdk::store::LookupMap<
                        std::string::String,
                        std::collections::HashMap<near_sdk::AccountId, (u64, u32)>,
                    > = &self.0.4;
                    let v4833: std::string::String = alias;
                    let v5147: () = {
                        Chat_contract::closure18(v4833.clone(), ());
                        ()
                    };
                    let v5774: () = {
                        Chat_contract::closure19(Chat_contract::method26(), ());
                        ()
                    };
                    v4831.get(&v4833).cloned() //;
                } //;
            } //;
        } /* c;
        {
        let v6092:
        bool =
        true; // ??? / i: 7uy / i': 1uy / acc: 10uy / n: 2uy;
        let v6094:
        bool =
        true; */
 // ???? / i: 7uy / i': 2uy / acc: 10uy / n: 2uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            #[result_serializer(borsh)] //;
            pub fn get_alias_map_borsh(
                //;
                &self,                              //;
                #[serializer(borsh)] alias: String, //;
            ) -> Option<std::collections::HashMap<near_sdk::AccountId, (u64, u32)>> {
                //;
                self.get_alias_map(alias) //;
            } //;
        } /* c;
        {
        let v6106:
        bool =
        true; // ??? / i: 8uy / i': 1uy / acc: 12uy / n: 1uy;
        let v6108:
        bool =
        true; */
 // ???? / i: 8uy / i': 2uy / acc: 12uy / n: 1uy;
        fn _main() //;
        {
            {
                //;
                {
                    let v6112: bool = true;
                    {
                        (); // ?? / i': 1uy / n: 13uy;
                        let v6114: bool = true;
                        {
                            (); // ?? / i': 2uy / n: 13uy;
                            let v6116: bool = true;
                            {
                                (); // ?? / i': 3uy / n: 13uy;
                                let v6118: bool = true;
                                {
                                    (); // ?? / i': 4uy / n: 13uy;
                                    let v6120: bool = true;
                                    {
                                        (); // ?? / i': 5uy / n: 13uy;
                                        let v6122: bool = true;
                                        {
                                            (); // ?? / i': 6uy / n: 13uy;
                                            let v6124: bool = true;
                                            {
                                                (); // ?? / i': 7uy / n: 13uy;
                                                let v6126: bool = true;
                                                {
                                                    (); // ?? / i': 8uy / n: 13uy;
                                                    let v6128: bool = true;
                                                    {
                                                        (); // ?? / i': 9uy / n: 13uy;
                                                        let v6130: bool = true;
                                                        {
                                                            (); // ?? / i': 10uy / n: 13uy;
                                                            let v6132: bool = true;
                                                            {
                                                                (); // ?? / i': 11uy / n: 13uy;
                                                                let v6134: bool = true;
                                                                {
                                                                    (); // ?? / i': 12uy / n: 13uy;
                                                                    let v6136: bool = true;
                                                                    {
                                                                        (); // ?? / i': 13uy / n: 13uy;
                                                                        let v6138: bool = true;
                                                                        {
                                                                            {
                                                                                (); // ? / i': 14uy / n: 13uy;
                                                                                ()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        pub fn v0() -> Func0<()> {
            static v0: OnceInit<Func0<()>> = OnceInit::new();
            v0.get_or_init(|| Func0::new(move || Chat_contract::closure0((), ())))
                .clone()
        }
        on_startup!(());
    }
}
pub use module_31a2fc18::*;
#[path = "../../../deps/spiral/lib/spiral/async__contract.rs"]
mod module_2335f2f5;
pub use module_2335f2f5::*;
#[path = "../../../deps/spiral/lib/spiral/common_contract.rs"]
mod module_652e6d81;
pub use module_652e6d81::*;
#[path = "../../../deps/spiral/lib/spiral/crypto_contract.rs"]
mod module_dd5f95ef;
pub use module_dd5f95ef::*;
#[path = "../../../deps/spiral/lib/spiral/date_time_contract.rs"]
mod module_ca5e6cb2;
pub use module_ca5e6cb2::*;
#[path = "../../../deps/spiral/lib/spiral/file_system_contract.rs"]
mod module_5ab1faf0;
pub use module_5ab1faf0::*;
#[path = "../../../deps/spiral/lib/spiral/lib_contract.rs"]
mod module_b386774b;
pub use module_b386774b::*;
#[path = "../../../deps/spiral/lib/spiral/networking_contract.rs"]
mod module_ce497f72;
pub use module_ce497f72::*;
#[path = "../../../deps/spiral/lib/spiral/platform_contract.rs"]
mod module_9a61edd3;
pub use module_9a61edd3::*;
#[path = "../../../deps/spiral/lib/spiral/runtime_contract.rs"]
mod module_502d7e30;
pub use module_502d7e30::*;
#[path = "../../../deps/spiral/lib/spiral/sm_contract.rs"]
mod module_34f67952;
pub use module_34f67952::*;
#[path = "../../../deps/spiral/lib/spiral/threading_contract.rs"]
mod module_11c0c5c2;
pub use module_11c0c5c2::*;
#[path = "../../../deps/spiral/lib/spiral/trace_contract.rs"]
mod module_28ecba0d;
pub use module_28ecba0d::*;
#[path = "../../../lib/fsharp/Common_contract.rs"]
mod module_ad43931;
pub use module_ad43931::*;
pub mod Polyglot {
    pub use crate::module_ad43931::Polyglot::*;
}
