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
mod module_d4b04e8b {
    pub mod Proxy {
        use super::*;
        use fable_library_rust::NativeArray_::new_array;
        use fable_library_rust::NativeArray_::Array;
        use fable_library_rust::Native_::box_;
        use fable_library_rust::Native_::on_startup;
        use fable_library_rust::Native_::Any;
        use fable_library_rust::Native_::Func0;
        use fable_library_rust::Native_::LrcPtr;
        use fable_library_rust::Native_::MutCell;
        use fable_library_rust::Native_::OnceInit;
        use fable_library_rust::String_::append;
        use fable_library_rust::String_::length;
        use fable_library_rust::String_::printfn;
        use fable_library_rust::String_::sprintf;
        use fable_library_rust::String_::string;
        use fable_library_rust::String_::substring2;
        use fable_library_rust::String_::toString;
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq)]
        pub enum US0 {
            US0_0,
            US0_1,
            US0_2,
            US0_3,
            US0_4,
        }
        impl Proxy::US0 {
            pub fn get_IsUS0_0(this_: &MutCell<Proxy::US0>, unitArg: ()) -> bool {
                if unreachable!() {
                    true
                } else {
                    false
                }
            }
            pub fn get_IsUS0_1(this_: &MutCell<Proxy::US0>, unitArg: ()) -> bool {
                if unreachable!() {
                    true
                } else {
                    false
                }
            }
            pub fn get_IsUS0_2(this_: &MutCell<Proxy::US0>, unitArg: ()) -> bool {
                if unreachable!() {
                    true
                } else {
                    false
                }
            }
            pub fn get_IsUS0_3(this_: &MutCell<Proxy::US0>, unitArg: ()) -> bool {
                if unreachable!() {
                    true
                } else {
                    false
                }
            }
            pub fn get_IsUS0_4(this_: &MutCell<Proxy::US0>, unitArg: ()) -> bool {
                if unreachable!() {
                    true
                } else {
                    false
                }
            }
        }
        impl core::fmt::Display for Proxy::US0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        pub fn method0() -> (string, string, string, bool) {
            let v1 = box_(string(""));
            (string(""), string(""), string(""), true)
        }
        pub fn method1(v0_1: string) -> std::string::String {
            String::from(core::ops::Deref::deref(v0_1))
        }
        pub fn method2(
            v0_1: rocket::http::uri::Segments<'_, rocket::http::uri::fmt::Path>,
        ) -> std::string::String {
            format!("{:?}", v0_1)
        }
        pub fn method4(v0_1: i32, v1: string) -> string {
            let v0_1: MutCell<i32> = MutCell::new(v0_1);
            let v1: MutCell<string> = MutCell::new(v1.clone());
            '_method4: loop {
                break '_method4 (if length(v1.get().clone()) < v0_1.get().clone() {
                    let v0_1_temp: i32 = v0_1.get().clone();
                    let v1_temp: string = sprintf!("0{}", v1.get().clone());
                    v0_1.set(v0_1_temp);
                    v1.set(v1_temp);
                    continue '_method4;
                } else {
                    v1.get().clone()
                });
            }
        }
        pub fn method5(v0_1: u8) -> std::string::String {
            format!("{:?}", v0_1)
        }
        pub fn method6(v0_1: std::string::String) -> std::string::String {
            v0_1
        }
        pub fn method7(v0_1: Array<std::string::String>) -> std::string::String {
            let v2: &Array<std::string::String> = v0_1;
            let v5 = core::ops::Deref::deref(v2);
            format!("{:?}", v5)
        }
        pub fn method8(v0_1: string) -> string {
            v0_1
        }
        pub fn method9() -> string {
            string("$1=$2")
        }
        pub fn method10(v0_1: std::string::String) -> std::string::String {
            v0_1
        }
        pub fn method11() -> string {
            string("[$1]")
        }
        pub fn method12() -> string {
            string("\"")
        }
        pub fn method13() -> string {
            string(" ")
        }
        pub fn method3(v0_1: i64, v1: Array<std::string::String>) {
            let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
            let v9: string = Proxy::method4(6_i32, toString((v3).timestamp() - v0_1));
            let v11: u8 = rand::random::<u8>();
            let v25: std::string::String = Proxy::method7(new_array(&[
                Proxy::method1(string("timestamp")),
                Proxy::method1(v9),
                Proxy::method1(string("run_id")),
                Proxy::method5(v11),
                Proxy::method1(string("log_level")),
                Proxy::method6(format!("{:?}", Proxy::US0::US0_1)),
            ]));
            let v28: Array<std::string::String> =
                new_array(&[Proxy::method1(string("> parseUrls ()"))]);
            let v30: &Array<std::string::String> = v28;
            let v33 = core::ops::Deref::deref(v30);
            let v35: std::string::String = format!("{:?}", v33);
            let v38: string = toString(format!("{}", v35));
            let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
            let v41: &str = r#""([^"]+)", "([^"]+)""#;
            let v43: regex::RegexBuilder = regex::RegexBuilder::new(v41);
            let v45: &regex::Regex = v43.build().unwrap();
            let v46: string = Proxy::method8(v38);
            let v48: &str = v46.as_str();
            let v49: string = Proxy::method9();
            let v51: &str = v49.as_str();
            let v54: std::string::String = Proxy::method10(v45.replace_all(&v48, v51).to_string());
            let v56: string = fable_library_rust::String_::fromString(v54);
            let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
            let v59: &str = r#"\["(.*?)"\]"#;
            let v60: regex::RegexBuilder = regex::RegexBuilder::new(v59);
            let v61: &regex::Regex = v60.build().unwrap();
            let v62: string = Proxy::method8(v56);
            let v64: &str = v62.as_str();
            let v65: string = Proxy::method11();
            let v67: &str = v65.as_str();
            let v69: std::string::String = Proxy::method10(v61.replace_all(&v64, v67).to_string());
            let v71: string = fable_library_rust::String_::fromString(v69);
            let v73: string = string("r#\"\\\\\"\"#");
            let v74: &str = r#"\\""#;
            let v75: regex::RegexBuilder = regex::RegexBuilder::new(v74);
            let v76: &regex::Regex = v75.build().unwrap();
            let v77: string = Proxy::method8(v71);
            let v79: &str = v77.as_str();
            let v80: string = Proxy::method12();
            let v82: &str = v80.as_str();
            let v84: std::string::String = Proxy::method10(v76.replace_all(&v79, v82).to_string());
            let v86: string = fable_library_rust::String_::fromString(v84);
            let v88: string = string("r#\", \"#");
            let v89: &str = r#", "#;
            let v90: regex::RegexBuilder = regex::RegexBuilder::new(v89);
            let v91: &regex::Regex = v90.build().unwrap();
            let v92: string = Proxy::method8(v86);
            let v94: &str = v92.as_str();
            let v95: string = Proxy::method13();
            let v97: &str = v95.as_str();
            let v99: std::string::String = Proxy::method10(v91.replace_all(&v94, v97).to_string());
            let v102: std::string::String =
                Proxy::method1(fable_library_rust::String_::fromString(v99));
            let v104: &Array<std::string::String> = v1;
            let v107 = core::ops::Deref::deref(v104);
            let v109: std::string::String = format!("{:?}", v107);
            let v111: string = toString(format!("{}", v109));
            let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
            let v113: &str = r#""([^"]+)", "([^"]+)""#;
            let v114: regex::RegexBuilder = regex::RegexBuilder::new(v113);
            let v115: &regex::Regex = v114.build().unwrap();
            let v116: string = Proxy::method8(v111);
            let v118: &str = v116.as_str();
            let v119: string = Proxy::method9();
            let v121: &str = v119.as_str();
            let v123: std::string::String =
                Proxy::method10(v115.replace_all(&v118, v121).to_string());
            let v125: string = fable_library_rust::String_::fromString(v123);
            let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
            let v127: &str = r#"\["(.*?)"\]"#;
            let v128: regex::RegexBuilder = regex::RegexBuilder::new(v127);
            let v129: &regex::Regex = v128.build().unwrap();
            let v130: string = Proxy::method8(v125);
            let v132: &str = v130.as_str();
            let v133: string = Proxy::method11();
            let v135: &str = v133.as_str();
            let v137: std::string::String =
                Proxy::method10(v129.replace_all(&v132, v135).to_string());
            let v139: string = fable_library_rust::String_::fromString(v137);
            let v140: string = string("r#\"\\\\\"\"#");
            let v141: &str = r#"\\""#;
            let v142: regex::RegexBuilder = regex::RegexBuilder::new(v141);
            let v143: &regex::Regex = v142.build().unwrap();
            let v144: string = Proxy::method8(v139);
            let v146: &str = v144.as_str();
            let v147: string = Proxy::method12();
            let v149: &str = v147.as_str();
            let v151: std::string::String =
                Proxy::method10(v143.replace_all(&v146, v149).to_string());
            let v153: string = fable_library_rust::String_::fromString(v151);
            let v154: string = string("r#\", \"#");
            let v155: &str = r#", "#;
            let v156: regex::RegexBuilder = regex::RegexBuilder::new(v155);
            let v157: &regex::Regex = v156.build().unwrap();
            let v158: string = Proxy::method8(v153);
            let v160: &str = v158.as_str();
            let v161: string = Proxy::method13();
            let v163: &str = v161.as_str();
            let v165: std::string::String =
                Proxy::method10(v157.replace_all(&v160, v163).to_string());
            let v168: std::string::String =
                Proxy::method1(fable_library_rust::String_::fromString(v165));
            let v170: string = toString(format!("{}", v25));
            let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
            let v172: &str = r#""([^"]+)", "([^"]+)""#;
            let v173: regex::RegexBuilder = regex::RegexBuilder::new(v172);
            let v174: &regex::Regex = v173.build().unwrap();
            let v175: string = Proxy::method8(v170);
            let v177: &str = v175.as_str();
            let v178: string = Proxy::method9();
            let v180: &str = v178.as_str();
            let v182: std::string::String =
                Proxy::method10(v174.replace_all(&v177, v180).to_string());
            let v184: string = fable_library_rust::String_::fromString(v182);
            let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
            let v186: &str = r#"\["(.*?)"\]"#;
            let v187: regex::RegexBuilder = regex::RegexBuilder::new(v186);
            let v188: &regex::Regex = v187.build().unwrap();
            let v189: string = Proxy::method8(v184);
            let v191: &str = v189.as_str();
            let v192: string = Proxy::method11();
            let v194: &str = v192.as_str();
            let v196: std::string::String =
                Proxy::method10(v188.replace_all(&v191, v194).to_string());
            let v198: string = fable_library_rust::String_::fromString(v196);
            let v199: string = string("r#\"\\\\\"\"#");
            let v200: &str = r#"\\""#;
            let v201: regex::RegexBuilder = regex::RegexBuilder::new(v200);
            let v202: &regex::Regex = v201.build().unwrap();
            let v203: string = Proxy::method8(v198);
            let v205: &str = v203.as_str();
            let v206: string = Proxy::method12();
            let v208: &str = v206.as_str();
            let v210: std::string::String =
                Proxy::method10(v202.replace_all(&v205, v208).to_string());
            let v212: string = fable_library_rust::String_::fromString(v210);
            let v213: string = string("r#\", \"#");
            let v214: &str = r#", "#;
            let v215: regex::RegexBuilder = regex::RegexBuilder::new(v214);
            let v216: &regex::Regex = v215.build().unwrap();
            let v217: string = Proxy::method8(v212);
            let v219: &str = v217.as_str();
            let v220: string = Proxy::method13();
            let v222: &str = v220.as_str();
            let v224: std::string::String =
                Proxy::method10(v216.replace_all(&v219, v222).to_string());
            let v227: std::string::String =
                Proxy::method1(fable_library_rust::String_::fromString(v224));
            let v229: string = substring2(string("006666"), 0_i32, 2_i32);
            let v231: u8 = u8::from_str_radix(v229, 16).unwrap();
            let v232: string = substring2(string("006666"), 2_i32, 4_i32);
            let v233: u8 = u8::from_str_radix(v232, 16).unwrap();
            let v234: string = substring2(string("006666"), 4_i32, 6_i32);
            let v235: u8 = u8::from_str_radix(v234, 16).unwrap();
            let v237: colored::ColoredString =
                colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
            let v238: std::string::String = format!("{}", v237);
            let v240: string = substring2(string("385779"), 0_i32, 2_i32);
            let v241: u8 = u8::from_str_radix(v240, 16).unwrap();
            let v242: string = substring2(string("385779"), 2_i32, 4_i32);
            let v243: u8 = u8::from_str_radix(v242, 16).unwrap();
            let v244: string = substring2(string("385779"), 4_i32, 6_i32);
            let v245: u8 = u8::from_str_radix(v244, 16).unwrap();
            let v246: colored::ColoredString =
                colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
            let v248: string = sprintf!("{} {} {}", v102, v238, format!("{}", v246));
            printfn!("{0}", v248);
            ()
        }
        pub fn closure1(
            unitVar: (),
            v0_1: rocket::http::uri::Segments<'_, rocket::http::uri::fmt::Path>,
        ) {
            let patternInput: (string, string, string, bool) = Proxy::method0();
            let v6: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
            Proxy::method3(
                (v6).timestamp(),
                new_array(&[Proxy::method1(string("uri")), Proxy::method2(v0_1)]),
            )
        }
        pub fn method14(v0_1: i64, v1: Array<std::string::String>) {
            let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
            let v9: string = Proxy::method4(6_i32, toString((v3).timestamp() - v0_1));
            let v11: u8 = rand::random::<u8>();
            let v25: std::string::String = Proxy::method7(new_array(&[
                Proxy::method1(string("timestamp")),
                Proxy::method1(v9),
                Proxy::method1(string("run_id")),
                Proxy::method5(v11),
                Proxy::method1(string("log_level")),
                Proxy::method6(format!("{:?}", Proxy::US0::US0_1)),
            ]));
            let v28: Array<std::string::String> = new_array(&[Proxy::method1(string("> init ()"))]);
            let v30: &Array<std::string::String> = v28;
            let v33 = core::ops::Deref::deref(v30);
            let v35: std::string::String = format!("{:?}", v33);
            let v38: string = toString(format!("{}", v35));
            let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
            let v41: &str = r#""([^"]+)", "([^"]+)""#;
            let v43: regex::RegexBuilder = regex::RegexBuilder::new(v41);
            let v45: &regex::Regex = v43.build().unwrap();
            let v46: string = Proxy::method8(v38);
            let v48: &str = v46.as_str();
            let v49: string = Proxy::method9();
            let v51: &str = v49.as_str();
            let v54: std::string::String = Proxy::method10(v45.replace_all(&v48, v51).to_string());
            let v56: string = fable_library_rust::String_::fromString(v54);
            let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
            let v59: &str = r#"\["(.*?)"\]"#;
            let v60: regex::RegexBuilder = regex::RegexBuilder::new(v59);
            let v61: &regex::Regex = v60.build().unwrap();
            let v62: string = Proxy::method8(v56);
            let v64: &str = v62.as_str();
            let v65: string = Proxy::method11();
            let v67: &str = v65.as_str();
            let v69: std::string::String = Proxy::method10(v61.replace_all(&v64, v67).to_string());
            let v71: string = fable_library_rust::String_::fromString(v69);
            let v73: string = string("r#\"\\\\\"\"#");
            let v74: &str = r#"\\""#;
            let v75: regex::RegexBuilder = regex::RegexBuilder::new(v74);
            let v76: &regex::Regex = v75.build().unwrap();
            let v77: string = Proxy::method8(v71);
            let v79: &str = v77.as_str();
            let v80: string = Proxy::method12();
            let v82: &str = v80.as_str();
            let v84: std::string::String = Proxy::method10(v76.replace_all(&v79, v82).to_string());
            let v86: string = fable_library_rust::String_::fromString(v84);
            let v88: string = string("r#\", \"#");
            let v89: &str = r#", "#;
            let v90: regex::RegexBuilder = regex::RegexBuilder::new(v89);
            let v91: &regex::Regex = v90.build().unwrap();
            let v92: string = Proxy::method8(v86);
            let v94: &str = v92.as_str();
            let v95: string = Proxy::method13();
            let v97: &str = v95.as_str();
            let v99: std::string::String = Proxy::method10(v91.replace_all(&v94, v97).to_string());
            let v102: std::string::String =
                Proxy::method1(fable_library_rust::String_::fromString(v99));
            let v104: &Array<std::string::String> = v1;
            let v107 = core::ops::Deref::deref(v104);
            let v109: std::string::String = format!("{:?}", v107);
            let v111: string = toString(format!("{}", v109));
            let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
            let v113: &str = r#""([^"]+)", "([^"]+)""#;
            let v114: regex::RegexBuilder = regex::RegexBuilder::new(v113);
            let v115: &regex::Regex = v114.build().unwrap();
            let v116: string = Proxy::method8(v111);
            let v118: &str = v116.as_str();
            let v119: string = Proxy::method9();
            let v121: &str = v119.as_str();
            let v123: std::string::String =
                Proxy::method10(v115.replace_all(&v118, v121).to_string());
            let v125: string = fable_library_rust::String_::fromString(v123);
            let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
            let v127: &str = r#"\["(.*?)"\]"#;
            let v128: regex::RegexBuilder = regex::RegexBuilder::new(v127);
            let v129: &regex::Regex = v128.build().unwrap();
            let v130: string = Proxy::method8(v125);
            let v132: &str = v130.as_str();
            let v133: string = Proxy::method11();
            let v135: &str = v133.as_str();
            let v137: std::string::String =
                Proxy::method10(v129.replace_all(&v132, v135).to_string());
            let v139: string = fable_library_rust::String_::fromString(v137);
            let v140: string = string("r#\"\\\\\"\"#");
            let v141: &str = r#"\\""#;
            let v142: regex::RegexBuilder = regex::RegexBuilder::new(v141);
            let v143: &regex::Regex = v142.build().unwrap();
            let v144: string = Proxy::method8(v139);
            let v146: &str = v144.as_str();
            let v147: string = Proxy::method12();
            let v149: &str = v147.as_str();
            let v151: std::string::String =
                Proxy::method10(v143.replace_all(&v146, v149).to_string());
            let v153: string = fable_library_rust::String_::fromString(v151);
            let v154: string = string("r#\", \"#");
            let v155: &str = r#", "#;
            let v156: regex::RegexBuilder = regex::RegexBuilder::new(v155);
            let v157: &regex::Regex = v156.build().unwrap();
            let v158: string = Proxy::method8(v153);
            let v160: &str = v158.as_str();
            let v161: string = Proxy::method13();
            let v163: &str = v161.as_str();
            let v165: std::string::String =
                Proxy::method10(v157.replace_all(&v160, v163).to_string());
            let v168: std::string::String =
                Proxy::method1(fable_library_rust::String_::fromString(v165));
            let v170: string = toString(format!("{}", v25));
            let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
            let v172: &str = r#""([^"]+)", "([^"]+)""#;
            let v173: regex::RegexBuilder = regex::RegexBuilder::new(v172);
            let v174: &regex::Regex = v173.build().unwrap();
            let v175: string = Proxy::method8(v170);
            let v177: &str = v175.as_str();
            let v178: string = Proxy::method9();
            let v180: &str = v178.as_str();
            let v182: std::string::String =
                Proxy::method10(v174.replace_all(&v177, v180).to_string());
            let v184: string = fable_library_rust::String_::fromString(v182);
            let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
            let v186: &str = r#"\["(.*?)"\]"#;
            let v187: regex::RegexBuilder = regex::RegexBuilder::new(v186);
            let v188: &regex::Regex = v187.build().unwrap();
            let v189: string = Proxy::method8(v184);
            let v191: &str = v189.as_str();
            let v192: string = Proxy::method11();
            let v194: &str = v192.as_str();
            let v196: std::string::String =
                Proxy::method10(v188.replace_all(&v191, v194).to_string());
            let v198: string = fable_library_rust::String_::fromString(v196);
            let v199: string = string("r#\"\\\\\"\"#");
            let v200: &str = r#"\\""#;
            let v201: regex::RegexBuilder = regex::RegexBuilder::new(v200);
            let v202: &regex::Regex = v201.build().unwrap();
            let v203: string = Proxy::method8(v198);
            let v205: &str = v203.as_str();
            let v206: string = Proxy::method12();
            let v208: &str = v206.as_str();
            let v210: std::string::String =
                Proxy::method10(v202.replace_all(&v205, v208).to_string());
            let v212: string = fable_library_rust::String_::fromString(v210);
            let v213: string = string("r#\", \"#");
            let v214: &str = r#", "#;
            let v215: regex::RegexBuilder = regex::RegexBuilder::new(v214);
            let v216: &regex::Regex = v215.build().unwrap();
            let v217: string = Proxy::method8(v212);
            let v219: &str = v217.as_str();
            let v220: string = Proxy::method13();
            let v222: &str = v220.as_str();
            let v224: std::string::String =
                Proxy::method10(v216.replace_all(&v219, v222).to_string());
            let v227: std::string::String =
                Proxy::method1(fable_library_rust::String_::fromString(v224));
            let v229: string = substring2(string("666666"), 0_i32, 2_i32);
            let v231: u8 = u8::from_str_radix(v229, 16).unwrap();
            let v232: string = substring2(string("666666"), 2_i32, 4_i32);
            let v233: u8 = u8::from_str_radix(v232, 16).unwrap();
            let v234: string = substring2(string("666666"), 4_i32, 6_i32);
            let v235: u8 = u8::from_str_radix(v234, 16).unwrap();
            let v237: colored::ColoredString =
                colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
            let v238: std::string::String = format!("{}", v237);
            let v240: string = substring2(string("385779"), 0_i32, 2_i32);
            let v241: u8 = u8::from_str_radix(v240, 16).unwrap();
            let v242: string = substring2(string("385779"), 2_i32, 4_i32);
            let v243: u8 = u8::from_str_radix(v242, 16).unwrap();
            let v244: string = substring2(string("385779"), 4_i32, 6_i32);
            let v245: u8 = u8::from_str_radix(v244, 16).unwrap();
            let v246: colored::ColoredString =
                colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
            let v248: string = sprintf!("{} {} {}", v102, v238, format!("{}", v246));
            printfn!("{0}", v248);
            ()
        }
        pub fn closure0(unitVar: (), unitVar_1: ()) -> i32 {
            let patternInput: (string, string, string, bool) = Proxy::method0();
            let v5: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
            let v7: i64 = (v5).timestamp();
            let v28: string =
                string("   let (host_protocol, host_host) = host_url.split_once(\"://\").unwrap_or((\"https\", \"www.github.com\"));");
            let v48: string =
                string("   println!(\"uri={uri}; new_url={new_url}; host={host}; path={path}; suffix={suffix}; host_url={host_url}; url={url}; host_host={host_host}; host_protocol={host_protocol}; protocol={protocol}\");");
            let v81: string =
                append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(append(string("#[rocket::get(\"/<uri..>\")] async fn proxy(   uri: rocket::http::uri::Segments<\'_, rocket::http::uri::fmt::Path>,   client: &rocket::State<reqwest::Client> ) -> Result<rocket::response::content::RawHtml<String>, rocket::http::Status> {   v17(uri.clone());   let uri = uri.collect::<Vec<_>>();   let uri = uri.join(\"/\");   let uri = uri.replace(\":/\", \"://\");   let (host_url, url) = uri.split_once(\'|\').unwrap_or((\"\", &uri));"),
                                                                                                                                                                                                             v28),
                                                                                                                                                                                                      string("   let (protocol, suffix) = url.split_once(\"://\").unwrap_or((host_protocol, url));")),
                                                                                                                                                                                               string("   let (host, path) = match (url.split_once(\"://\"), suffix.split_once(\"/\")) {")),
                                                                                                                                                                                        string("     (Some(_), None) if suffix != \"\" => (suffix, \"\"),")),
                                                                                                                                                                                 string("     _ => (host_host, suffix),")),
                                                                                                                                                                          string("   };")),
                                                                                                                                                                   string("   let new_url = match path {")),
                                                                                                                                                            string("     \"\" => format!(\"{protocol}://{host}\"),")),
                                                                                                                                                     string("     _ => format!(\"{protocol}://{host}/{path}\"),")),
                                                                                                                                              string("   };")),
                                                                                                                                       v48),
                                                                                                                                string("   let res = client")),
                                                                                                                         string("     .get(new_url)")),
                                                                                                                  string("     .send()")),
                                                                                                           string("     .await")),
                                                                                                    string("     .map_err(|_| rocket::http::Status::NotFound);")),
                                                                                             string("   match res {")),
                                                                                      string("       Ok(res) => {")),
                                                                               string("           let content = res.text();")),
                                                                        string("           let content = content.await;")),
                                                                 string("           let content = content.map_err(|_| rocket::http::Status::InternalServerError);")),
                                                          string("           let content = content.unwrap();")),
                                                   string("           Ok(rocket::response::content::RawHtml(content))")),
                                            string("       }")),
                                     string("       Err(status) => Err(status),")),
                              string("   }")), string("} ()"));
            #[rocket::get("/<uri..>")]
            async fn proxy(
                uri: rocket::http::uri::Segments<'_, rocket::http::uri::fmt::Path>,
                client: &rocket::State<reqwest::Client>,
            ) -> Result<rocket::response::content::RawHtml<String>, rocket::http::Status>
            {
                v17(uri.clone());
                let uri = uri.collect::<Vec<_>>();
                let uri = uri.join("/");
                let uri = uri.replace(":/", "://");
                let (host_url, url) = uri.split_once('|').unwrap_or(("", &uri));
                let (host_protocol, host_host) = host_url
                    .split_once("://")
                    .unwrap_or(("https", "www.github.com"));
                let (protocol, suffix) = url.split_once("://").unwrap_or((host_protocol, url));
                let (host, path) = match (url.split_once("://"), suffix.split_once("/")) {
                    (Some(_), None) if suffix != "" => (suffix, ""),
                    _ => (host_host, suffix),
                };
                let new_url = match path {
                    "" => format!("{protocol}://{host}"),
                    _ => format!("{protocol}://{host}/{path}"),
                };
                println!("uri={uri}; new_url={new_url}; host={host}; path={path}; suffix={suffix}; host_url={host_url}; url={url}; host_host={host_host}; host_protocol={host_protocol}; protocol={protocol}");
                let res = client
                    .get(new_url)
                    .send()
                    .await
                    .map_err(|_| rocket::http::Status::NotFound);
                match res {
                    Ok(res) => {
                        let content = res.text();
                        let content = content.await;
                        let content =
                            content.map_err(|_| rocket::http::Status::InternalServerError);
                        let content = content.unwrap();
                        Ok(rocket::response::content::RawHtml(content))
                    }
                    Err(status) => Err(status),
                }
            }
            ();
            {
                let v98: string =
                    string("#[rocket::get(\"/file/<file..>\")] async fn file(   file: std::path::PathBuf,   client: &rocket::State<reqwest::Client> ) -> std::option::Option<rocket::fs::NamedFile> {   rocket::fs::NamedFile::open(std::path::Path::new(\"./\").join(file))     .await     .ok()} ()");
                #[rocket::get("/file/<file..>")]
                async fn file(
                    file: std::path::PathBuf,
                    client: &rocket::State<reqwest::Client>,
                ) -> std::option::Option<rocket::fs::NamedFile> {
                    rocket::fs::NamedFile::open(std::path::Path::new("./").join(file))
                        .await
                        .ok()
                }
                ();
                {
                    let v109: string =
                        string("#[rocket::launch] fn rocket() -> _ {    rocket::build()      .manage(reqwest::Client::new())      .mount(\"/\", rocket::routes![proxy, file])} ()");
                    #[rocket::launch]
                    fn rocket() -> _ {
                        rocket::build()
                            .manage(reqwest::Client::new())
                            .mount("/", rocket::routes![proxy, file])
                    }
                    ();
                    Proxy::method14(
                        v7,
                        new_array(&[
                            Proxy::method1(string("app end")),
                            Proxy::method1(string("???")),
                        ]),
                    );
                    0_i32
                }
            }
        }
        pub fn v0() -> Func0<i32> {
            static v0: OnceInit<Func0<i32>> = OnceInit::new();
            v0.get_or_insert_with(|| Func0::new(move || Proxy::closure0((), ())))
                .clone()
        }
        on_startup!((),);
    }
}
pub use module_d4b04e8b::*;
#[path = "../rust/Types.rs"]
mod module_26514e41;
pub use module_26514e41::*;
fn main() {}
fn main() {}
