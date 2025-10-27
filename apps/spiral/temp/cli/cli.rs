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
pub mod Cli {
    use super::*;
    use fable_library_rust::Interfaces_::System::Collections::Generic::IEnumerable_1;
    use fable_library_rust::NativeArray_::count;
    use fable_library_rust::NativeArray_::new_array;
    use fable_library_rust::NativeArray_::Array;
    use fable_library_rust::Native_::on_startup;
    use fable_library_rust::Native_::Any;
    use fable_library_rust::Native_::Func0;
    use fable_library_rust::Native_::Func1;
    use fable_library_rust::Native_::LrcPtr;
    use fable_library_rust::Native_::MutCell;
    use fable_library_rust::Range_::rangeNumeric;
    use fable_library_rust::Seq_::toArray;
    use fable_library_rust::String_::append;
    use fable_library_rust::String_::length;
    use fable_library_rust::String_::sprintf;
    use fable_library_rust::String_::string;
    use fable_library_rust::String_::substring2;
    use fable_library_rust::String_::toString;
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
    pub enum US0 {
        US0_0,
        US0_1,
        US0_2,
        US0_3,
        US0_4,
    }
    impl core::fmt::Display for Cli::US0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug)]
    pub enum UH0 {
        UH0_0(std::string::String, LrcPtr<Cli::UH0>),
        UH0_1,
    }
    impl core::fmt::Display for Cli::UH0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug)]
    pub struct Mut0 {
        pub l0: MutCell<LrcPtr<Cli::UH0>>,
    }
    impl core::fmt::Display for Cli::Mut0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
    pub enum UH1 {
        UH1_0(string, LrcPtr<Cli::UH1>),
        UH1_1,
    }
    impl core::fmt::Display for Cli::UH1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
    pub enum US1 {
        US1_0,
        US1_1(string),
    }
    impl core::fmt::Display for Cli::US1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
    pub struct Mut1 {
        pub l0: MutCell<i32>,
        pub l1: MutCell<Cli::US1>,
    }
    impl core::fmt::Display for Cli::Mut1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    pub fn method0(v0_1: Option<std::string::String>) -> Option<std::string::String> {
        v0_1
    }
    pub fn method1(v0_1: std::string::String) -> std::string::String {
        v0_1
    }
    pub fn closure1(unitVar: (), v0_1: std::string::String) -> string {
        let v1: std::string::String = Cli::method1(v0_1);
        fable_library_rust::String_::fromString(v1)
    }
    pub fn method2() -> (string, string, string, bool) {
        let v1 = &string("");
        (string(""), string(""), string(""), true)
    }
    pub fn method3() -> string {
        string("rs-path")
    }
    pub fn method4(
        v0_1: Option<&'static std::string::String>,
    ) -> Option<&'static std::string::String> {
        v0_1
    }
    pub fn method5(v0_1: &'static std::string::String) -> &'static std::string::String {
        v0_1
    }
    pub fn method6(v0_1: string) -> string {
        v0_1
    }
    pub fn method7() -> string {
        string("")
    }
    pub fn closure2(unitVar: (), v0_1: &'static std::string::String) -> string {
        let v1: &'static std::string::String = Cli::method5(v0_1);
        let v4: std::string::String = Cli::method1(*v1);
        let v6: string = fable_library_rust::String_::fromString(v4);
        let v9: string = Cli::method6(string("--rs-path="));
        let v11: &str = fable_library_rust::String_::LrcStr::as_str(&v9);
        let v13: std::string::String = String::from(core::ops::Deref::deref(&v11));
        let v14: string = Cli::method7();
        let v16: &str = fable_library_rust::String_::LrcStr::as_str(&v14);
        let v18: std::string::String = String::from(core::ops::Deref::deref(&v16));
        let v19: string = Cli::method6(v6);
        let v21: &str = fable_library_rust::String_::LrcStr::as_str(&v19);
        let v23: std::string::String = String::from(core::ops::Deref::deref(&v21));
        let v26: std::string::String = Cli::method1(v23.replace(&v13, &v18));
        fable_library_rust::String_::fromString(v26)
    }
    pub fn method8(v0_1: std::string::String) -> std::string::String {
        v0_1
    }
    pub fn method9(v0_1: std::path::PathBuf) -> std::path::PathBuf {
        v0_1
    }
    pub fn method10(v0_1: string) -> std::string::String {
        String::from(core::ops::Deref::deref(&v0_1)).clone()
    }
    pub fn method11(v0_1: std::path::PathBuf) -> std::string::String {
        format!("{:?}", &v0_1).clone()
    }
    pub fn method13(v0_1: i32, v1: string) -> string {
        let v0_1: MutCell<i32> = MutCell::new(v0_1);
        let v1: MutCell<string> = MutCell::new(v1.clone());
        '_method13: loop {
            break '_method13 (if length(v1.get()) < v0_1.get() {
                let v0_1_temp: i32 = v0_1.get();
                let v1_temp: string = sprintf!("0{}", &v1.get());
                v0_1.set(v0_1_temp);
                v1.set(v1_temp);
                continue '_method13;
            } else {
                v1.get()
            });
        }
    }
    pub fn method14(v0_1: u8) -> std::string::String {
        format!("{:?}", &v0_1).clone()
    }
    pub fn method15(v0_1: std::string::String) -> std::string::String {
        (&v0_1).clone()
    }
    pub fn method16(v0_1: Array<std::string::String>) -> std::string::String {
        let v2: &Array<std::string::String> = &v0_1;
        let v5 = core::ops::Deref::deref(v2);
        format!("{:?}", v5).clone()
    }
    pub fn method17(v0_1: string) -> string {
        v0_1
    }
    pub fn method18() -> string {
        string("$1=$2")
    }
    pub fn method19() -> string {
        string("[$1]")
    }
    pub fn method20() -> string {
        string("\"")
    }
    pub fn method21() -> string {
        string(" ")
    }
    pub fn method12(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> =
            new_array(&[Cli::method10(string("> post_fsx_rs_build ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("666666"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("666666"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("666666"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method22(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> =
            new_array(&[Cli::method10(string("> read_lines ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("881749"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("881749"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("881749"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method24(v0_1: i32) -> std::string::String {
        format!("{:?}", &v0_1).clone()
    }
    pub fn method25(v0_1: i64, v1: Array<std::string::String>) {
        ();
    }
    pub fn closure3(v0_1: i64, v1: LrcPtr<Cli::Mut0>, v2: Vec<u8>) -> Result<bool, std::io::Error> {
        let v4: std::string::String = Cli::method10(string("line len"));
        let v6 = v2.len();
        Cli::method25(
            v0_1,
            new_array(&[v4, Cli::method24(v6.try_into().unwrap())]),
        );
        {
            let v12: Vec<u8> = v2.to_owned();
            let v14: std::string::String = std::string::String::from_utf8(v12).unwrap();
            let v15: LrcPtr<Cli::UH0> = v1.l0.get().clone();
            v1.l0
                .set(LrcPtr::new(Cli::UH0::UH0_0(v14.clone(), v15.clone())));
            Ok(true)
        }
    }
    pub fn method23(
        v0_1: i64,
        v1: LrcPtr<Cli::Mut0>,
    ) -> Func1<Vec<u8>, Result<bool, std::io::Error>> {
        Func1::new({
            let v0_1 = v0_1.clone();
            let v1 = v1.clone();
            move |v: Vec<u8>| Cli::closure3(v0_1, v1.clone(), v.clone())
        })
    }
    pub fn method26(v0_1: LrcPtr<Cli::UH0>, v1: LrcPtr<Cli::UH0>) -> LrcPtr<Cli::UH0> {
        let v0_1: MutCell<LrcPtr<Cli::UH0>> = MutCell::new(v0_1.clone());
        let v1: MutCell<LrcPtr<Cli::UH0>> = MutCell::new(v1.clone());
        '_method26: loop {
            break '_method26 (match v0_1.get().as_ref() {
                Cli::UH0::UH0_1 => v1.get(),
                Cli::UH0::UH0_0(v0_1_0_0, v0_1_0_1) => {
                    let v0_1_temp: LrcPtr<Cli::UH0> = v0_1_0_1.clone();
                    let v1_temp: LrcPtr<Cli::UH0> =
                        LrcPtr::new(Cli::UH0::UH0_0(v0_1_0_0.clone(), v1.get()));
                    v0_1.set(v0_1_temp);
                    v1.set(v1_temp);
                    continue '_method26;
                }
            });
        }
    }
    pub fn method27(v0_1: LrcPtr<Cli::UH0>, v1: LrcPtr<Cli::UH1>) -> LrcPtr<Cli::UH1> {
        match v0_1.as_ref() {
            Cli::UH0::UH0_1 => v1.clone(),
            Cli::UH0::UH0_0(v0_1_0_0, v0_1_0_1) => {
                let v4: LrcPtr<Cli::UH1> = Cli::method27(v0_1_0_1.clone(), v1);
                let v5: std::string::String = Cli::method1(v0_1_0_0.clone());
                LrcPtr::new(Cli::UH1::UH1_0(
                    fable_library_rust::String_::fromString(v5),
                    v4,
                ))
            }
        }
    }
    pub fn method28(v0_1: LrcPtr<Cli::UH1>, v1: i32) -> i32 {
        let v0_1: MutCell<LrcPtr<Cli::UH1>> = MutCell::new(v0_1.clone());
        let v1: MutCell<i32> = MutCell::new(v1);
        '_method28: loop {
            break '_method28 (match v0_1.get().as_ref() {
                Cli::UH1::UH1_1 => v1.get(),
                Cli::UH1::UH1_0(v0_1_0_0, v0_1_0_1) => {
                    let v0_1_temp: LrcPtr<Cli::UH1> = v0_1_0_1.clone();
                    let v1_temp: i32 = v1.get() + 1_i32;
                    v0_1.set(v0_1_temp);
                    v1.set(v1_temp);
                    continue '_method28;
                }
            });
        }
    }
    pub fn method30() -> string {
        string("/node_modules/")
    }
    pub fn method31() -> string {
        string(".cs")
    }
    pub fn method29(
        v0_1: i64,
        v1: LrcPtr<Cli::UH1>,
        v2: i32,
        v3: LrcPtr<Cli::UH1>,
    ) -> (i32, LrcPtr<Cli::UH1>) {
        let v0_1: MutCell<i64> = MutCell::new(v0_1);
        let v1: MutCell<LrcPtr<Cli::UH1>> = MutCell::new(v1.clone());
        let v2: MutCell<i32> = MutCell::new(v2);
        let v3: MutCell<LrcPtr<Cli::UH1>> = MutCell::new(v3.clone());
        '_method29: loop {
            break '_method29 (match v1.get().as_ref() {
                Cli::UH1::UH1_1 => (v2.get(), v3.get()),
                Cli::UH1::UH1_0(v1_0_0, v1_0_1) => {
                    let v4: string = v1_0_0.clone();
                    let v6: string = Cli::method30();
                    let v8: &str = fable_library_rust::String_::LrcStr::as_str(&v6);
                    let v10: std::string::String = String::from(core::ops::Deref::deref(&v8));
                    let patternInput: (i32, LrcPtr<Cli::UH1>) = if if v4.contains(&v10) {
                        let v13: string = Cli::method31();
                        let v15: &str = fable_library_rust::String_::LrcStr::as_str(&v13);
                        let v17: std::string::String = String::from(core::ops::Deref::deref(&v15));
                        v4.contains(&v17)
                    } else {
                        false
                    } {
                        Cli::method12(
                            v0_1.get(),
                            new_array(&[
                                Cli::method10(string("found line. removing.")),
                                Cli::method10(string(" line")),
                                Cli::method10(v4.clone()),
                            ]),
                        );
                        (1_i32, v3.get())
                    } else {
                        if 0_i32 == v2.get() {
                            (0_i32, LrcPtr::new(Cli::UH1::UH1_0(v4, v3.get())))
                        } else {
                            if 2_i32 == v2.get() {
                                (0_i32, v3.get())
                            } else {
                                (v2.get() + 1_i32, v3.get())
                            }
                        }
                    };
                    {
                        let v0_1_temp: i64 = v0_1.get();
                        let v1_temp: LrcPtr<Cli::UH1> = v1_0_1.clone();
                        let v2_temp: i32 = patternInput.0.clone();
                        let v3_temp: LrcPtr<Cli::UH1> = patternInput.1.clone();
                        v0_1.set(v0_1_temp);
                        v1.set(v1_temp);
                        v2.set(v2_temp);
                        v3.set(v3_temp);
                        continue '_method29;
                    }
                }
            });
        }
    }
    pub fn method32(v0_1: LrcPtr<Cli::UH1>, v1: LrcPtr<Cli::UH1>) -> LrcPtr<Cli::UH1> {
        let v0_1: MutCell<LrcPtr<Cli::UH1>> = MutCell::new(v0_1.clone());
        let v1: MutCell<LrcPtr<Cli::UH1>> = MutCell::new(v1.clone());
        '_method32: loop {
            break '_method32 (match v0_1.get().as_ref() {
                Cli::UH1::UH1_1 => v1.get(),
                Cli::UH1::UH1_0(v0_1_0_0, v0_1_0_1) => {
                    let v0_1_temp: LrcPtr<Cli::UH1> = v0_1_0_1.clone();
                    let v1_temp: LrcPtr<Cli::UH1> =
                        LrcPtr::new(Cli::UH1::UH1_0(v0_1_0_0.clone(), v1.get()));
                    v0_1.set(v0_1_temp);
                    v1.set(v1_temp);
                    continue '_method32;
                }
            });
        }
    }
    pub fn method33(v0_1: LrcPtr<Cli::UH1>, v1: string) -> string {
        let v0_1: MutCell<LrcPtr<Cli::UH1>> = MutCell::new(v0_1.clone());
        let v1: MutCell<string> = MutCell::new(v1.clone());
        '_method33: loop {
            break '_method33 (match v0_1.get().as_ref() {
                Cli::UH1::UH1_1 => v1.get(),
                Cli::UH1::UH1_0(v0_1_0_0, v0_1_0_1) => {
                    let v0_1_temp: LrcPtr<Cli::UH1> = v0_1_0_1.clone();
                    let v1_temp: string = append(v1.get(), v0_1_0_0.clone());
                    v0_1.set(v0_1_temp);
                    v1.set(v1_temp);
                    continue '_method33;
                }
            });
        }
    }
    pub fn method34() -> string {
        string("spi-path")
    }
    pub fn closure4(unitVar: (), v0_1: &'static std::string::String) -> string {
        let v1: &'static std::string::String = Cli::method5(v0_1);
        let v4: std::string::String = Cli::method1(*v1);
        let v6: string = fable_library_rust::String_::fromString(v4);
        let v9: string = Cli::method6(string("--spi-path="));
        let v11: &str = fable_library_rust::String_::LrcStr::as_str(&v9);
        let v13: std::string::String = String::from(core::ops::Deref::deref(&v11));
        let v14: string = Cli::method7();
        let v16: &str = fable_library_rust::String_::LrcStr::as_str(&v14);
        let v18: std::string::String = String::from(core::ops::Deref::deref(&v16));
        let v19: string = Cli::method6(v6);
        let v21: &str = fable_library_rust::String_::LrcStr::as_str(&v19);
        let v23: std::string::String = String::from(core::ops::Deref::deref(&v21));
        let v26: std::string::String = Cli::method1(v23.replace(&v13, &v18));
        fable_library_rust::String_::fromString(v26)
    }
    pub fn method35() -> string {
        string("fsx-path")
    }
    pub fn method36(v0_1: &'static std::string::String) -> &'static std::string::String {
        v0_1
    }
    pub fn closure5(unitVar: (), v0_1: &'static std::string::String) -> std::path::PathBuf {
        let v1: &'static std::string::String = Cli::method36(v0_1);
        std::path::PathBuf::from(v1)
    }
    pub fn method37(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> =
            new_array(&[Cli::method10(string("> spi_to_fsx ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("de884c"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("de884c"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("de884c"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method38(v0_1: std::path::PathBuf) -> std::path::PathBuf {
        v0_1
    }
    pub fn method39(v0_1: &std::path::Path) -> &std::path::Path {
        v0_1
    }
    pub fn method40() -> string {
        string("package.spiproj")
    }
    pub fn method41(v0_1: &std::path::Path) -> &std::path::Path {
        v0_1
    }
    pub fn closure6(unitVar: (), v0_1: &std::path::Path) -> std::path::PathBuf {
        let v1: string = Cli::method40();
        let v3: &str = v1.as_str();
        let v4: &std::path::Path = Cli::method41(v0_1);
        v4.join(v3)
    }
    pub fn method42(v0_1: std::path::PathBuf) -> std::path::PathBuf {
        v0_1
    }
    pub fn closure7(unitVar: (), v0_1: std::path::PathBuf) -> bool {
        let v1: std::path::PathBuf = Cli::method42(v0_1);
        v1.exists()
    }
    pub fn method43() -> string {
        string(".spir")
    }
    pub fn method44() -> string {
        string(".spi")
    }
    pub fn method45() -> string {
        string(".fsx")
    }
    pub fn method46() -> string {
        string("")
    }
    pub fn method47(v0_1: u16) -> std::string::String {
        format!("{:?}", &v0_1).clone()
    }
    pub fn method48(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> = new_array(&[Cli::method10(string("> ping ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("e36a62"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("e36a62"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("e36a62"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method49(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> =
            new_array(&[Cli::method10(string("> zmq_request ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("fcfb1b"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("fcfb1b"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("fcfb1b"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method50(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> = new_array(&[Cli::method10(string("> spi_open ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("e36a62"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("e36a62"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("e36a62"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method51(v0_1: &str) -> &str {
        v0_1
    }
    pub fn method52(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> =
            new_array(&[Cli::method10(string("> spiproj_open ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("e36a62"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("e36a62"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("e36a62"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method53(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> =
            new_array(&[Cli::method10(string("> spi_build_file ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("98d5ef"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("98d5ef"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("98d5ef"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method54() -> string {
        string("Fsharp")
    }
    pub fn method55(v0_1: std::time::SystemTime) -> std::string::String {
        format!("{:?}", &v0_1).clone()
    }
    pub fn method56(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v28: Array<std::string::String> =
            new_array(&[Cli::method10(string("> wait_file_change ()"))]);
        let v30: &Array<std::string::String> = &v28;
        let v33 = core::ops::Deref::deref(v30);
        let v35: std::string::String = format!("{:?}", v33).clone();
        let v38: string = toString(format!("{}", &v35));
        let v40: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v41: &str = r#""([^"]+)", "([^"]+)""#;
        let v43: regex::RegexBuilder = regex::RegexBuilder::new(&v41);
        let v45: &regex::Regex = &v43.build().unwrap();
        let v46: string = Cli::method17(v38);
        let v48: &str = v46.as_str();
        let v49: string = Cli::method18();
        let v51: &str = v49.as_str();
        let v54: std::string::String = Cli::method1(v45.replace_all(&v48, v51).to_string());
        let v56: string = fable_library_rust::String_::fromString(v54);
        let v58: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v59: &str = r#"\["(.*?)"\]"#;
        let v60: regex::RegexBuilder = regex::RegexBuilder::new(&v59);
        let v61: &regex::Regex = &v60.build().unwrap();
        let v62: string = Cli::method17(v56);
        let v64: &str = v62.as_str();
        let v65: string = Cli::method19();
        let v67: &str = v65.as_str();
        let v69: std::string::String = Cli::method1(v61.replace_all(&v64, v67).to_string());
        let v71: string = fable_library_rust::String_::fromString(v69);
        let v73: string = string("r#\"\\\\\"\"#");
        let v74: &str = r#"\\""#;
        let v75: regex::RegexBuilder = regex::RegexBuilder::new(&v74);
        let v76: &regex::Regex = &v75.build().unwrap();
        let v77: string = Cli::method17(v71);
        let v79: &str = v77.as_str();
        let v80: string = Cli::method20();
        let v82: &str = v80.as_str();
        let v84: std::string::String = Cli::method1(v76.replace_all(&v79, v82).to_string());
        let v86: string = fable_library_rust::String_::fromString(v84);
        let v88: string = string("r#\", \"#");
        let v89: &str = r#", "#;
        let v90: regex::RegexBuilder = regex::RegexBuilder::new(&v89);
        let v91: &regex::Regex = &v90.build().unwrap();
        let v92: string = Cli::method17(v86);
        let v94: &str = v92.as_str();
        let v95: string = Cli::method21();
        let v97: &str = v95.as_str();
        let v99: std::string::String = Cli::method1(v91.replace_all(&v94, v97).to_string());
        let v102: std::string::String = Cli::method10(fable_library_rust::String_::fromString(v99));
        let v104: &Array<std::string::String> = &v1;
        let v107 = core::ops::Deref::deref(v104);
        let v109: std::string::String = format!("{:?}", v107).clone();
        let v111: string = toString(format!("{}", &v109));
        let v112: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v113: &str = r#""([^"]+)", "([^"]+)""#;
        let v114: regex::RegexBuilder = regex::RegexBuilder::new(&v113);
        let v115: &regex::Regex = &v114.build().unwrap();
        let v116: string = Cli::method17(v111);
        let v118: &str = v116.as_str();
        let v119: string = Cli::method18();
        let v121: &str = v119.as_str();
        let v123: std::string::String = Cli::method1(v115.replace_all(&v118, v121).to_string());
        let v125: string = fable_library_rust::String_::fromString(v123);
        let v126: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v127: &str = r#"\["(.*?)"\]"#;
        let v128: regex::RegexBuilder = regex::RegexBuilder::new(&v127);
        let v129: &regex::Regex = &v128.build().unwrap();
        let v130: string = Cli::method17(v125);
        let v132: &str = v130.as_str();
        let v133: string = Cli::method19();
        let v135: &str = v133.as_str();
        let v137: std::string::String = Cli::method1(v129.replace_all(&v132, v135).to_string());
        let v139: string = fable_library_rust::String_::fromString(v137);
        let v140: string = string("r#\"\\\\\"\"#");
        let v141: &str = r#"\\""#;
        let v142: regex::RegexBuilder = regex::RegexBuilder::new(&v141);
        let v143: &regex::Regex = &v142.build().unwrap();
        let v144: string = Cli::method17(v139);
        let v146: &str = v144.as_str();
        let v147: string = Cli::method20();
        let v149: &str = v147.as_str();
        let v151: std::string::String = Cli::method1(v143.replace_all(&v146, v149).to_string());
        let v153: string = fable_library_rust::String_::fromString(v151);
        let v154: string = string("r#\", \"#");
        let v155: &str = r#", "#;
        let v156: regex::RegexBuilder = regex::RegexBuilder::new(&v155);
        let v157: &regex::Regex = &v156.build().unwrap();
        let v158: string = Cli::method17(v153);
        let v160: &str = v158.as_str();
        let v161: string = Cli::method21();
        let v163: &str = v161.as_str();
        let v165: std::string::String = Cli::method1(v157.replace_all(&v160, v163).to_string());
        let v168: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v165));
        let v170: string = toString(format!("{}", &v25));
        let v171: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v172: &str = r#""([^"]+)", "([^"]+)""#;
        let v173: regex::RegexBuilder = regex::RegexBuilder::new(&v172);
        let v174: &regex::Regex = &v173.build().unwrap();
        let v175: string = Cli::method17(v170);
        let v177: &str = v175.as_str();
        let v178: string = Cli::method18();
        let v180: &str = v178.as_str();
        let v182: std::string::String = Cli::method1(v174.replace_all(&v177, v180).to_string());
        let v184: string = fable_library_rust::String_::fromString(v182);
        let v185: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v186: &str = r#"\["(.*?)"\]"#;
        let v187: regex::RegexBuilder = regex::RegexBuilder::new(&v186);
        let v188: &regex::Regex = &v187.build().unwrap();
        let v189: string = Cli::method17(v184);
        let v191: &str = v189.as_str();
        let v192: string = Cli::method19();
        let v194: &str = v192.as_str();
        let v196: std::string::String = Cli::method1(v188.replace_all(&v191, v194).to_string());
        let v198: string = fable_library_rust::String_::fromString(v196);
        let v199: string = string("r#\"\\\\\"\"#");
        let v200: &str = r#"\\""#;
        let v201: regex::RegexBuilder = regex::RegexBuilder::new(&v200);
        let v202: &regex::Regex = &v201.build().unwrap();
        let v203: string = Cli::method17(v198);
        let v205: &str = v203.as_str();
        let v206: string = Cli::method20();
        let v208: &str = v206.as_str();
        let v210: std::string::String = Cli::method1(v202.replace_all(&v205, v208).to_string());
        let v212: string = fable_library_rust::String_::fromString(v210);
        let v213: string = string("r#\", \"#");
        let v214: &str = r#", "#;
        let v215: regex::RegexBuilder = regex::RegexBuilder::new(&v214);
        let v216: &regex::Regex = &v215.build().unwrap();
        let v217: string = Cli::method17(v212);
        let v219: &str = v217.as_str();
        let v220: string = Cli::method21();
        let v222: &str = v220.as_str();
        let v224: std::string::String = Cli::method1(v216.replace_all(&v219, v222).to_string());
        let v227: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v224));
        let v229: string = substring2(string("387957"), 0_i32, 2_i32);
        let v231: u8 = u8::from_str_radix(&v229, 16).unwrap();
        let v232: string = substring2(string("387957"), 2_i32, 4_i32);
        let v233: u8 = u8::from_str_radix(&v232, 16).unwrap();
        let v234: string = substring2(string("387957"), 4_i32, 6_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v237: colored::ColoredString =
            colored::Colorize::truecolor(&*v168.to_string(), v231, v233, v235);
        let v238: std::string::String = format!("{}", &v237);
        let v240: string = substring2(string("385779"), 0_i32, 2_i32);
        let v241: u8 = u8::from_str_radix(&v240, 16).unwrap();
        let v242: string = substring2(string("385779"), 2_i32, 4_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 4_i32, 6_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: colored::ColoredString =
            colored::Colorize::truecolor(&*v227.to_string(), v241, v243, v245);
        let v248: string = sprintf!("{} {} {}", &v102, &v238, format!("{}", &v246));
        println!("{0}", v248,);
        ()
    }
    pub fn method57(v0_1: i32, v1: LrcPtr<Cli::Mut1>) -> bool {
        v1.l0.get() < v0_1
    }
    pub fn method58() -> u64 {
        60_u64
    }
    pub fn method60(v0_1: &str) -> std::string::String {
        format!("{:?}", &v0_1).clone()
    }
    pub fn method59(v0_1: i64, v1: Array<std::string::String>) {
        let v3: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let v9: string = Cli::method13(6_i32, toString((&v3).timestamp() - v0_1));
        let v11: u8 = rand::random::<u8>();
        let v25: std::string::String = Cli::method16(new_array(&[
            Cli::method10(string("timestamp")),
            Cli::method10(v9),
            Cli::method10(string("run_id")),
            Cli::method14(v11),
            Cli::method10(string("log_level")),
            Cli::method15(format!("{:?}", &Cli::US0::US0_0)),
        ]));
        let v27: string = string("r#\"> app ()\"#");
        let v30: Array<std::string::String> = new_array(&[Cli::method60(r#"> app ()"#)]);
        let v32: &Array<std::string::String> = &v30;
        let v35 = core::ops::Deref::deref(v32);
        let v37: std::string::String = format!("{:?}", v35).clone();
        let v40: string = toString(format!("{}", &v37));
        let v42: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v43: &str = r#""([^"]+)", "([^"]+)""#;
        let v45: regex::RegexBuilder = regex::RegexBuilder::new(&v43);
        let v47: &regex::Regex = &v45.build().unwrap();
        let v48: string = Cli::method17(v40);
        let v50: &str = v48.as_str();
        let v51: string = Cli::method18();
        let v53: &str = v51.as_str();
        let v56: std::string::String = Cli::method1(v47.replace_all(&v50, v53).to_string());
        let v58: string = fable_library_rust::String_::fromString(v56);
        let v60: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v61: &str = r#"\["(.*?)"\]"#;
        let v62: regex::RegexBuilder = regex::RegexBuilder::new(&v61);
        let v63: &regex::Regex = &v62.build().unwrap();
        let v64: string = Cli::method17(v58);
        let v66: &str = v64.as_str();
        let v67: string = Cli::method19();
        let v69: &str = v67.as_str();
        let v71: std::string::String = Cli::method1(v63.replace_all(&v66, v69).to_string());
        let v73: string = fable_library_rust::String_::fromString(v71);
        let v75: string = string("r#\"\\\\\"\"#");
        let v76: &str = r#"\\""#;
        let v77: regex::RegexBuilder = regex::RegexBuilder::new(&v76);
        let v78: &regex::Regex = &v77.build().unwrap();
        let v79: string = Cli::method17(v73);
        let v81: &str = v79.as_str();
        let v82: string = Cli::method20();
        let v84: &str = v82.as_str();
        let v86: std::string::String = Cli::method1(v78.replace_all(&v81, v84).to_string());
        let v88: string = fable_library_rust::String_::fromString(v86);
        let v90: string = string("r#\", \"#");
        let v91: &str = r#", "#;
        let v92: regex::RegexBuilder = regex::RegexBuilder::new(&v91);
        let v93: &regex::Regex = &v92.build().unwrap();
        let v94: string = Cli::method17(v88);
        let v96: &str = v94.as_str();
        let v97: string = Cli::method21();
        let v99: &str = v97.as_str();
        let v101: std::string::String = Cli::method1(v93.replace_all(&v96, v99).to_string());
        let v104: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v101));
        let v106: &Array<std::string::String> = &v1;
        let v109 = core::ops::Deref::deref(v106);
        let v111: std::string::String = format!("{:?}", v109).clone();
        let v113: string = toString(format!("{}", &v111));
        let v114: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v115: &str = r#""([^"]+)", "([^"]+)""#;
        let v116: regex::RegexBuilder = regex::RegexBuilder::new(&v115);
        let v117: &regex::Regex = &v116.build().unwrap();
        let v118: string = Cli::method17(v113);
        let v120: &str = v118.as_str();
        let v121: string = Cli::method18();
        let v123: &str = v121.as_str();
        let v125: std::string::String = Cli::method1(v117.replace_all(&v120, v123).to_string());
        let v127: string = fable_library_rust::String_::fromString(v125);
        let v128: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v129: &str = r#"\["(.*?)"\]"#;
        let v130: regex::RegexBuilder = regex::RegexBuilder::new(&v129);
        let v131: &regex::Regex = &v130.build().unwrap();
        let v132: string = Cli::method17(v127);
        let v134: &str = v132.as_str();
        let v135: string = Cli::method19();
        let v137: &str = v135.as_str();
        let v139: std::string::String = Cli::method1(v131.replace_all(&v134, v137).to_string());
        let v141: string = fable_library_rust::String_::fromString(v139);
        let v142: string = string("r#\"\\\\\"\"#");
        let v143: &str = r#"\\""#;
        let v144: regex::RegexBuilder = regex::RegexBuilder::new(&v143);
        let v145: &regex::Regex = &v144.build().unwrap();
        let v146: string = Cli::method17(v141);
        let v148: &str = v146.as_str();
        let v149: string = Cli::method20();
        let v151: &str = v149.as_str();
        let v153: std::string::String = Cli::method1(v145.replace_all(&v148, v151).to_string());
        let v155: string = fable_library_rust::String_::fromString(v153);
        let v156: string = string("r#\", \"#");
        let v157: &str = r#", "#;
        let v158: regex::RegexBuilder = regex::RegexBuilder::new(&v157);
        let v159: &regex::Regex = &v158.build().unwrap();
        let v160: string = Cli::method17(v155);
        let v162: &str = v160.as_str();
        let v163: string = Cli::method21();
        let v165: &str = v163.as_str();
        let v167: std::string::String = Cli::method1(v159.replace_all(&v162, v165).to_string());
        let v170: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v167));
        let v172: string = toString(format!("{}", &v25));
        let v173: string = string("r#\"\"([^\"]+)\", \"([^\"]+)\"\"#");
        let v174: &str = r#""([^"]+)", "([^"]+)""#;
        let v175: regex::RegexBuilder = regex::RegexBuilder::new(&v174);
        let v176: &regex::Regex = &v175.build().unwrap();
        let v177: string = Cli::method17(v172);
        let v179: &str = v177.as_str();
        let v180: string = Cli::method18();
        let v182: &str = v180.as_str();
        let v184: std::string::String = Cli::method1(v176.replace_all(&v179, v182).to_string());
        let v186: string = fable_library_rust::String_::fromString(v184);
        let v187: string = string("r#\"\\[\"(.*?)\"\\]\"#");
        let v188: &str = r#"\["(.*?)"\]"#;
        let v189: regex::RegexBuilder = regex::RegexBuilder::new(&v188);
        let v190: &regex::Regex = &v189.build().unwrap();
        let v191: string = Cli::method17(v186);
        let v193: &str = v191.as_str();
        let v194: string = Cli::method19();
        let v196: &str = v194.as_str();
        let v198: std::string::String = Cli::method1(v190.replace_all(&v193, v196).to_string());
        let v200: string = fable_library_rust::String_::fromString(v198);
        let v201: string = string("r#\"\\\\\"\"#");
        let v202: &str = r#"\\""#;
        let v203: regex::RegexBuilder = regex::RegexBuilder::new(&v202);
        let v204: &regex::Regex = &v203.build().unwrap();
        let v205: string = Cli::method17(v200);
        let v207: &str = v205.as_str();
        let v208: string = Cli::method20();
        let v210: &str = v208.as_str();
        let v212: std::string::String = Cli::method1(v204.replace_all(&v207, v210).to_string());
        let v214: string = fable_library_rust::String_::fromString(v212);
        let v215: string = string("r#\", \"#");
        let v216: &str = r#", "#;
        let v217: regex::RegexBuilder = regex::RegexBuilder::new(&v216);
        let v218: &regex::Regex = &v217.build().unwrap();
        let v219: string = Cli::method17(v214);
        let v221: &str = v219.as_str();
        let v222: string = Cli::method21();
        let v224: &str = v222.as_str();
        let v226: std::string::String = Cli::method1(v218.replace_all(&v221, v224).to_string());
        let v229: std::string::String =
            Cli::method10(fable_library_rust::String_::fromString(v226));
        let v231: string = substring2(string("387958"), 0_i32, 2_i32);
        let v233: u8 = u8::from_str_radix(&v231, 16).unwrap();
        let v234: string = substring2(string("387958"), 2_i32, 4_i32);
        let v235: u8 = u8::from_str_radix(&v234, 16).unwrap();
        let v236: string = substring2(string("387958"), 4_i32, 6_i32);
        let v237: u8 = u8::from_str_radix(&v236, 16).unwrap();
        let v239: colored::ColoredString =
            colored::Colorize::truecolor(&*v170.to_string(), v233, v235, v237);
        let v240: std::string::String = format!("{}", &v239);
        let v242: string = substring2(string("385779"), 0_i32, 2_i32);
        let v243: u8 = u8::from_str_radix(&v242, 16).unwrap();
        let v244: string = substring2(string("385779"), 2_i32, 4_i32);
        let v245: u8 = u8::from_str_radix(&v244, 16).unwrap();
        let v246: string = substring2(string("385779"), 4_i32, 6_i32);
        let v247: u8 = u8::from_str_radix(&v246, 16).unwrap();
        let v248: colored::ColoredString =
            colored::Colorize::truecolor(&*v229.to_string(), v243, v245, v247);
        let v250: string = sprintf!("{} {} {}", &v104, &v240, format!("{}", &v248));
        println!("{0}", v250,);
        ()
    }
    pub fn closure0(unitVar: (), unitVar_1: ()) -> i32 {
        let v1: string = string("r#\"cli2\"#");
        let v2: &str = r#"cli2"#;
        let v6: Option<std::string::String> =
            Cli::method0(Some(String::from(core::ops::Deref::deref(&v2))));
        let v9: Option<string> = v6.map(&*Func1::new(move |v: std::string::String| {
            Cli::closure1((), v.clone())
        }));
        let v11: std::string::String = format!("{:?}", &v9);
        println!("{0}", &v11,);
        {
            let patternInput: (string, string, string, bool) = Cli::method2();
            let v17: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
            let v19: i64 = (&v17).timestamp();
            let v21: string = string("r#\"app1\"#");
            let v22: &str = r#"app1"#;
            let v33: string =
                string("clap::Command::new(v22).version(env!(\"CARGO_PKG_VERSION\")).propagate_version(true).subcommand_required(false).about(\"command about\").arg_required_else_help(true)");
            let v34: clap::builder::Command = clap::Command::new(v22)
                .version(env!("CARGO_PKG_VERSION"))
                .propagate_version(true)
                .subcommand_required(false)
                .about("command about")
                .arg_required_else_help(true);
            let v36: string = string("r#\"PostFsxRsBuild\"#");
            let v37: &str = r#"PostFsxRsBuild"#;
            let v43: string =
                string("clap::Command::new(v37).version(env!(\"CARGO_PKG_VERSION\")).propagate_version(true).subcommand_required(false).about(\"command about\").arg_required_else_help(true)");
            let v44: clap::builder::Command = clap::Command::new(v37)
                .version(env!("CARGO_PKG_VERSION"))
                .propagate_version(true)
                .subcommand_required(false)
                .about("command about")
                .arg_required_else_help(true);
            let v46: string = string("r#\"rs-path\"#");
            let v47: &str = r#"rs-path"#;
            let v49: clap::builder::Arg = clap::Arg::new(v47);
            let v51: clap::builder::Command = v44.arg(v49);
            let v53: clap::builder::Command = v34.subcommand(v51);
            let v55: string = string("r#\"SpiFsxBuild\"#");
            let v56: &str = r#"SpiFsxBuild"#;
            let v62: string =
                string("clap::Command::new(v56).version(env!(\"CARGO_PKG_VERSION\")).propagate_version(true).subcommand_required(false).about(\"command about\").arg_required_else_help(true)");
            let v63: clap::builder::Command = clap::Command::new(v56)
                .version(env!("CARGO_PKG_VERSION"))
                .propagate_version(true)
                .subcommand_required(false)
                .about("command about")
                .arg_required_else_help(true);
            let v65: string = string("r#\"spi-path\"#");
            let v66: &str = r#"spi-path"#;
            let v68: clap::builder::Arg = clap::Arg::new(v66);
            let v70: clap::builder::Command = v63.arg(v68);
            let v72: string = string("r#\"fsx-path\"#");
            let v73: &str = r#"fsx-path"#;
            let v75: clap::builder::Arg = clap::Arg::new(v73);
            let v77: clap::builder::Command = v70.arg(v75);
            let v79: clap::builder::Command = v53.subcommand(v77);
            let v81: clap::parser::ArgMatches = v79.get_matches();
            let v83: std::sync::Arc<clap::parser::ArgMatches> = std::sync::Arc::new(v81);
            let v85: Option<(&str, std::sync::Arc<clap::parser::ArgMatches>)> = v83
                .subcommand()
                .map(|(name, matches)| (name, std::sync::Arc::new(matches.clone())));
            let patternInput_1: (&str, std::sync::Arc<clap::parser::ArgMatches>) = v85.unwrap();
            let v89 = &(patternInput_1.0.clone(), patternInput_1.1.clone());
            let v91: &str = v89.0;
            let v93: std::sync::Arc<clap::parser::ArgMatches> = std::sync::Arc::clone(&v89.1);
            let patternInput_2: (&str, &clap::parser::ArgMatches) =
                panic!("{}", 1_i32.get_Message(),);
            let v97: string = fable_library_rust::String_::string(v94);
            if string("PostFsxRsBuild") == v97.clone() {
                let v99: string = Cli::method3();
                let v101: &str = v99.as_str();
                let v104: Option<&'static std::string::String> =
                    Cli::method4(v95.get_one::<String>(v101));
                let v107: Option<string> =
                    v104.map(&*Func1::new(move |v_1: &'static std::string::String| {
                        Cli::closure2((), v_1.clone())
                    }));
                let v109: string = string("get_clap_one: rs-path");
                let v112: string = Cli::method6(v107.unwrap_or(v109));
                let v114: &str = fable_library_rust::String_::LrcStr::as_str(&v112);
                let v117: std::string::String =
                    Cli::method8(String::from(core::ops::Deref::deref(&v114)));
                let v120: std::path::PathBuf = Cli::method9(std::path::PathBuf::from(v117));
                let v122: Result<std::path::PathBuf, std::io::Error> = v120.canonicalize();
                let v124: std::path::PathBuf = v122.unwrap();
                let patternInput_3: (string, string, string, bool) = Cli::method2();
                let v129: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                let v130: i64 = (&v129).timestamp();
                Cli::method12(
                    v130,
                    new_array(&[
                        Cli::method10(string("rs_path")),
                        Cli::method11(v124.clone()),
                    ]),
                );
                {
                    let patternInput_4: (string, string, string, bool) = Cli::method2();
                    let v139: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                    let v140: i64 = (&v139).timestamp();
                    let v141: std::path::PathBuf = Cli::method9(v124);
                    let v143: Result<std::path::PathBuf, std::io::Error> = v141.canonicalize();
                    let v145: std::path::PathBuf = v143.unwrap();
                    let v147: std::string::String = Cli::method10(string("path_clean"));
                    let v150: std::string::String = Cli::method1(format!("{}", v145.display()));
                    Cli::method22(
                        v140,
                        new_array(&[
                            v147,
                            Cli::method10(fable_library_rust::String_::fromString(v150)),
                        ]),
                    );
                    {
                        let v156: Result<std::fs::File, std::io::Error> = std::fs::File::open(v145);
                        let v158: std::fs::File = v156.unwrap();
                        let v160: std::cell::RefCell<linereader::LineReader<std::fs::File>> =
                            std::cell::RefCell::new(linereader::LineReader::new(v158));
                        Cli::method22(
                            v140,
                            new_array(&[
                                Cli::method10(string("reader")),
                                Cli::method15(format!("{:?}", &v160)),
                            ]),
                        );
                        {
                            let v167: LrcPtr<Cli::Mut0> = LrcPtr::new(Cli::Mut0 {
                                l0: MutCell::new(LrcPtr::new(Cli::UH0::UH0_1)),
                            });
                            let v168 = Cli::method23(v140, v167.clone());
                            let v170: Result<(), std::io::Error> =
                                v160.borrow_mut().for_each(move |x| v168(x.to_vec()));
                            v170.unwrap();
                            {
                                let v176: LrcPtr<Cli::UH1> = Cli::method27(
                                    Cli::method26(
                                        v167.l0.get().clone(),
                                        LrcPtr::new(Cli::UH0::UH0_1),
                                    ),
                                    LrcPtr::new(Cli::UH1::UH1_1),
                                );
                                Cli::method22(
                                    v140,
                                    new_array(&[
                                        Cli::method10(string("lines len")),
                                        Cli::method24(Cli::method28(v176.clone(), 0_i32)),
                                    ]),
                                );
                                {
                                    let v190: string = Cli::method33(
                                        Cli::method32(
                                            (Cli::method29(
                                                v130,
                                                v176,
                                                0_i32,
                                                LrcPtr::new(Cli::UH1::UH1_1),
                                            ))
                                            .1
                                            .clone(),
                                            LrcPtr::new(Cli::UH1::UH1_1),
                                        ),
                                        string(""),
                                    );
                                    let v192: std::string::String =
                                        Cli::method10(string("text_len"));
                                    let v194 = v190.len();
                                    Cli::method12(
                                        v130,
                                        new_array(&[v192, Cli::method24(v194.try_into().unwrap())]),
                                    );
                                    {
                                        let v199: string = Cli::method17(v190);
                                        let v201: &str = v199.as_str();
                                        let v203: Result<(), std::io::Error> =
                                            std::fs::write(v124, v201);
                                        v203.unwrap();
                                        Cli::method12(
                                            v130,
                                            new_array(&[Cli::method10(string(
                                                "> fs_write rs_path_clean text; ok",
                                            ))]),
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if string("SpiFsxBuild") == v97 {
                    let v209: string = Cli::method34();
                    let v211: &str = v209.as_str();
                    let v214: Option<&'static std::string::String> =
                        Cli::method4(v95.get_one::<String>(v211));
                    let v217: Option<string> =
                        v214.map(&*Func1::new(move |v_2: &'static std::string::String| {
                            Cli::closure4((), v_2.clone())
                        }));
                    let v219: string = string("get_clap_one: spi-path");
                    let v222: string = Cli::method6(v217.unwrap_or(v219));
                    let v224: &str = fable_library_rust::String_::LrcStr::as_str(&v222);
                    let v227: std::string::String =
                        Cli::method8(String::from(core::ops::Deref::deref(&v224)));
                    let v230: std::path::PathBuf = Cli::method9(std::path::PathBuf::from(v227));
                    let v232: Result<std::path::PathBuf, std::io::Error> = v230.canonicalize();
                    let v234: std::path::PathBuf = v232.unwrap();
                    let v235: string = Cli::method35();
                    let v237: &str = v235.as_str();
                    let v240: Option<&'static std::string::String> =
                        Cli::method4(v95.get_one::<String>(v237));
                    let v243: Option<std::path::PathBuf> =
                        v240.map(&*Func1::new(move |v_3: &'static std::string::String| {
                            Cli::closure5((), v_3.clone())
                        }));
                    let patternInput_6: (string, string, string, bool) = Cli::method2();
                    let v248: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                    let v249: i64 = (&v248).timestamp();
                    let v251: std::string::String = Cli::method10(string("pwd"));
                    let v253: Result<std::path::PathBuf, std::io::Error> = std::env::current_dir();
                    let v256: std::path::PathBuf = Cli::method9(v253.unwrap());
                    let v258: Result<std::path::PathBuf, std::io::Error> = v256.canonicalize();
                    let v260: std::path::PathBuf = v258.unwrap();
                    let v263: std::string::String = Cli::method1(format!("{}", v260.display()));
                    let v266: std::string::String =
                        Cli::method10(fable_library_rust::String_::fromString(v263));
                    let v268: std::string::String = Cli::method10(string("spi_path"));
                    let v271: std::string::String = Cli::method1(format!("{}", v234.display()));
                    Cli::method37(
                        v249,
                        new_array(&[
                            v251,
                            v266,
                            v268,
                            Cli::method10(fable_library_rust::String_::fromString(v271)),
                            Cli::method10(string("new_fsx_path")),
                            Cli::method15(format!("{:?}", &v243)),
                        ]),
                    );
                    {
                        let v280: std::path::PathBuf = Cli::method38(v234);
                        let v282: Option<&std::path::Path> = v280.parent();
                        let v285: &std::path::Path = Cli::method39(v282.unwrap());
                        let v287: std::path::PathBuf = std::path::PathBuf::from(v285);
                        let v289: std::string::String = Cli::method10(string("src_path"));
                        let v292: std::string::String = Cli::method1(format!("{}", v287.display()));
                        Cli::method37(
                            v249,
                            new_array(&[
                                v289,
                                Cli::method10(fable_library_rust::String_::fromString(v292)),
                            ]),
                        );
                        {
                            let v298: std::path::Ancestors = v287.ancestors();
                            let v299: usize = 10_i32 as usize;
                            let v301: _ = v298.take(v299);
                            let v304: _ =
                                v301.into_iter()
                                    .map(Func1::new(move |v_4: &std::path::Path| {
                                        Cli::closure6((), v_4.clone())
                                    }));
                            let v307: Option<std::path::PathBuf> =
                                v304.into_iter().find(Func1::new(
                                    move |v_5: std::path::PathBuf| Cli::closure7((), v_5.clone()),
                                ));
                            let v309: std::path::PathBuf = v307.unwrap();
                            let v311: std::string::String = Cli::method10(string("spiproj_path"));
                            let v314: std::string::String =
                                Cli::method1(format!("{}", v309.display()));
                            Cli::method37(
                                v249,
                                new_array(&[
                                    v311,
                                    Cli::method10(fable_library_rust::String_::fromString(v314)),
                                ]),
                            );
                            {
                                let v320: Option<&str> = v234.to_str();
                                let v322: &str = v320.unwrap();
                                let v324: string = fable_library_rust::String_::string(v322);
                                let v325: string = Cli::method43();
                                let v327: &str = fable_library_rust::String_::LrcStr::as_str(&v325);
                                let v329: std::string::String =
                                    String::from(core::ops::Deref::deref(&v327));
                                let v330: string = Cli::method44();
                                let v332: &str = fable_library_rust::String_::LrcStr::as_str(&v330);
                                let v334: std::string::String =
                                    String::from(core::ops::Deref::deref(&v332));
                                let v335: string = Cli::method6(v324);
                                let v337: &str = fable_library_rust::String_::LrcStr::as_str(&v335);
                                let v339: std::string::String =
                                    String::from(core::ops::Deref::deref(&v337));
                                let v342: std::string::String =
                                    Cli::method1(v339.replace(&v329, &v334));
                                let v344: string = fable_library_rust::String_::fromString(v342);
                                let v345: string = Cli::method44();
                                let v347: &str = fable_library_rust::String_::LrcStr::as_str(&v345);
                                let v349: std::string::String =
                                    String::from(core::ops::Deref::deref(&v347));
                                let v350: string = Cli::method45();
                                let v352: &str = fable_library_rust::String_::LrcStr::as_str(&v350);
                                let v354: std::string::String =
                                    String::from(core::ops::Deref::deref(&v352));
                                let v355: string = Cli::method6(v344);
                                let v357: &str = fable_library_rust::String_::LrcStr::as_str(&v355);
                                let v359: std::string::String =
                                    String::from(core::ops::Deref::deref(&v357));
                                let v362: std::string::String =
                                    Cli::method1(v359.replace(&v349, &v354));
                                let v365: string =
                                    Cli::method6(fable_library_rust::String_::fromString(v362));
                                let v367: &str = fable_library_rust::String_::LrcStr::as_str(&v365);
                                let v370: std::string::String =
                                    Cli::method8(String::from(core::ops::Deref::deref(&v367)));
                                let v373: std::path::PathBuf =
                                    Cli::method9(std::path::PathBuf::from(v370));
                                let v375: Result<std::path::PathBuf, std::io::Error> =
                                    v373.canonicalize();
                                let v377: std::path::PathBuf = v375.unwrap();
                                let v379: std::string::String = Cli::method10(string("fsx_path"));
                                let v382: std::string::String =
                                    Cli::method1(format!("{}", v377.display()));
                                Cli::method37(
                                    v249,
                                    new_array(&[
                                        v379,
                                        Cli::method10(fable_library_rust::String_::fromString(
                                            v382,
                                        )),
                                    ]),
                                );
                                {
                                    let v387: string = Cli::method46();
                                    let v389: &str = v387.as_str();
                                    let v391: Result<(), std::io::Error> =
                                        std::fs::write(v377, v389);
                                    v391.unwrap();
                                    Cli::method37(
                                        v249,
                                        new_array(&[Cli::method10(string(
                                            "> fs_write fsx_path \"\"; ok",
                                        ))]),
                                    );
                                    {
                                        let v397: u16 = 13805.try_into().unwrap();
                                        Cli::method37(
                                            v249,
                                            new_array(&[
                                                Cli::method10(string("port")),
                                                Cli::method47(v397.clone()),
                                            ]),
                                        );
                                        {
                                            let v403: string =
                                                string("r#\"../The-Spiral-Language/The Spiral Language 2/bin/Release/net7.0/Spiral.dll\"#");
                                            let v404: &str = r#"../The-Spiral-Language/The Spiral Language 2/bin/Release/net7.0/Spiral.dll"#;
                                            let v406: string = string("r#\"dotnet\"#");
                                            let v407: &str = r#"dotnet"#;
                                            let v409 = std::process::Command::new(v407);
                                            let mut v409 = v409;
                                            {
                                                let v412 = v409.arg(v404);
                                                let v416: string = Cli::method6(append(
                                                    string("port="),
                                                    toString(v397.clone()),
                                                ));
                                                let v418: &str =
                                                    fable_library_rust::String_::LrcStr::as_str(
                                                        &v416,
                                                    );
                                                let v420: std::string::String =
                                                    String::from(core::ops::Deref::deref(&v418));
                                                let v422 = v412.arg(v420);
                                                let v424 =
                                                    v422.stdout(std::process::Stdio::inherit());
                                                let v426 =
                                                    v424.stderr(std::process::Stdio::inherit());
                                                let v428: Result<
                                                    std::process::Child,
                                                    std::io::Error,
                                                > = v426.spawn();
                                                let v430: std::process::Child = v428.unwrap();
                                                Cli::method37(
                                                    v249,
                                                    new_array(&[
                                                        Cli::method10(string("process")),
                                                        Cli::method15(format!("{:?}", &v430)),
                                                    ]),
                                                );
                                                {
                                                    let v437: std::net::SocketAddr =
                                                        std::net::SocketAddr::from((
                                                            [0, 0, 0, 0],
                                                            v397,
                                                        ));
                                                    let v460: string =
                                                        string("async move {     loop {         let result = std::net::TcpStream::connect(&v437);         println!(\"connect_timeout: {:?}\", result);         if result.is_ok() {             tokio::time::sleep(std::time::Duration::from_secs(1)).await;             break;         } else {             tokio::time::sleep(std::time::Duration::from_millis(60)).await;         }     } }");
                                                    let v462 =
                                                        tokio::runtime::Builder::new_current_thread(
                                                        );
                                                    let mut v462 = v462;
                                                    {
                                                        let v465 = v462.enable_all();
                                                        let v467 = v465.build();
                                                        let v469 = v467.unwrap();
                                                        let v473: string = append(
                                                            append(string("v469.block_on("), v460),
                                                            string(")"),
                                                        );
                                                        v469.block_on(async move {     loop {         let result = std::net::TcpStream::connect(&v437);         println!("connect_timeout: {:?}", result);         if result.is_ok() {             tokio::time::sleep(std::time::Duration::from_secs(1)).await;             break;         } else {             tokio::time::sleep(std::time::Duration::from_millis(60)).await;         }     } });
                                                        {
                                                            let patternInput_7: (
                                                                string,
                                                                string,
                                                                string,
                                                                bool,
                                                            ) = Cli::method2();
                                                            let v478: chrono::DateTime<
                                                                chrono::Utc,
                                                            > = chrono::Utc::now();
                                                            let v479: i64 = (&v478).timestamp();
                                                            let v488: string =
                                                                string("serde_json::json!({\"Ping\": true}).to_string()");
                                                            let v489: std::string::String =
                                                                serde_json::json!({"Ping": true})
                                                                    .to_string();
                                                            let v491: std::string::String =
                                                                Cli::method10(string("json_len"));
                                                            let v493 = v489.len();
                                                            Cli::method48(
                                                                v479,
                                                                new_array(&[
                                                                    v491,
                                                                    Cli::method24(
                                                                        v493.try_into().unwrap(),
                                                                    ),
                                                                ]),
                                                            );
                                                            {
                                                                let patternInput_8: (
                                                                    string,
                                                                    string,
                                                                    string,
                                                                    bool,
                                                                ) = Cli::method2();
                                                                let v502: chrono::DateTime<
                                                                    chrono::Utc,
                                                                > = chrono::Utc::now();
                                                                let v503: i64 = (&v502).timestamp();
                                                                let v505: std::string::String =
                                                                    Cli::method10(string("host"));
                                                                let v507: std::string::String =
                                                                    Cli::method10(string(
                                                                        "127.0.0.1",
                                                                    ));
                                                                let v508: std::string::String =
                                                                    Cli::method10(string("port"));
                                                                let v509: std::string::String =
                                                                    Cli::method47(v397.clone());
                                                                let v511: std::string::String =
                                                                    Cli::method10(string(
                                                                        "msg_len",
                                                                    ));
                                                                let v513 = v489.len();
                                                                Cli::method49(
                                                                    v503,
                                                                    new_array(&[
                                                                        v505,
                                                                        v507,
                                                                        v508,
                                                                        v509,
                                                                        v511,
                                                                        Cli::method24(
                                                                            v513.try_into()
                                                                                .unwrap(),
                                                                        ),
                                                                    ]),
                                                                );
                                                                {
                                                                    let v519 = zmq::Context::new();
                                                                    let v521 = v519
                                                                        .socket(zmq::REQ)
                                                                        .unwrap();
                                                                    let v522: string = sprintf!(
                                                                        "tcp://{}:{}",
                                                                        &string("127.0.0.1"),
                                                                        &v397
                                                                    );
                                                                    v521.connect(&v522.clone())
                                                                        .unwrap();
                                                                    v521.send(v489.as_str(), 0)
                                                                        .unwrap();
                                                                    {
                                                                        let v526:
                                                                                std::string::String =
                                                                            v521.recv_string(0).unwrap().unwrap();
                                                                        v521.disconnect(&v522)
                                                                            .unwrap();
                                                                        {
                                                                            let v529:
                                                                                    std::string::String =
                                                                                Cli::method10(string("result_len"));
                                                                            let v531 = v526.len();
                                                                            Cli::method49(v503,
                                                                                          new_array(&[v529,
                                                                                                      Cli::method24(v531.try_into().unwrap())]));
                                                                            Cli::method37(v249,
                                                                                          new_array(&[Cli::method10(string("> ping; ok"))]));
                                                                            {
                                                                                let v540:
                                                                                        Result<std::string::String,
                                                                                               std::io::Error> =
                                                                                    std::fs::read_to_string(v234);
                                                                                let v542:
                                                                                        std::string::String =
                                                                                    (&v540).as_ref().unwrap().to_string();
                                                                                let patternInput_9:
                                                                                        (string,
                                                                                         string,
                                                                                         string,
                                                                                         bool) =
                                                                                    Cli::method2();
                                                                                let v547:
                                                                                        chrono::DateTime<chrono::Utc> =
                                                                                    chrono::Utc::now();
                                                                                let v548: i64 =
                                                                                    (&v547)
                                                                                        .timestamp(
                                                                                        );
                                                                                let v549:
                                                                                        std::string::String =
                                                                                    Cli::method10(string("spi_path"));
                                                                                let v552:
                                                                                        std::string::String =
                                                                                    Cli::method1(format!("{}", v234.display()));
                                                                                let v555:
                                                                                        std::string::String =
                                                                                    Cli::method10(fable_library_rust::String_::fromString(v552));
                                                                                let v557:
                                                                                        std::string::String =
                                                                                    Cli::method10(string("spi_text_len"));
                                                                                let v559 =
                                                                                    v542.len();
                                                                                Cli::method50(v548,
                                                                                              new_array(&[v549,
                                                                                                          v555,
                                                                                                          v557,
                                                                                                          Cli::method24(v559.try_into().unwrap())]));
                                                                                {
                                                                                    let v573:
                                                                                            string =
                                                                                        string("serde_json::json!({\"FileOpen\": {\"spiText\": v542,\"uri\": \"file://\".to_owned() + v234.to_str().unwrap()}}).to_string()");
                                                                                    let v574:
                                                                                            std::string::String =
                                                                                        serde_json::json!({"FileOpen": {"spiText": v542,"uri": "file://".to_owned() + v234.to_str().unwrap()}}).to_string();
                                                                                    let v575:
                                                                                            std::string::String =
                                                                                        Cli::method10(string("json_len"));
                                                                                    let v577 =
                                                                                        v574.len();
                                                                                    Cli::method50(v548,
                                                                                                  new_array(&[v575,
                                                                                                              Cli::method24(v577.try_into().unwrap())]));
                                                                                    {
                                                                                        let patternInput_10:
                                                                                                (string,
                                                                                                 string,
                                                                                                 string,
                                                                                                 bool) =
                                                                                            Cli::method2();
                                                                                        let v586:
                                                                                                chrono::DateTime<chrono::Utc> =
                                                                                            chrono::Utc::now();
                                                                                        let v587:
                                                                                                i64 =
                                                                                            (&v586).timestamp();
                                                                                        let v588:
                                                                                                std::string::String =
                                                                                            Cli::method10(string("host"));
                                                                                        let v589:
                                                                                                std::string::String =
                                                                                            Cli::method10(string("127.0.0.1"));
                                                                                        let v590:
                                                                                                std::string::String =
                                                                                            Cli::method10(string("port"));
                                                                                        let v591:
                                                                                                std::string::String =
                                                                                            Cli::method47(v397.clone());
                                                                                        let v592:
                                                                                                std::string::String =
                                                                                            Cli::method10(string("msg_len"));
                                                                                        let v594 =
                                                                                            v574.len();
                                                                                        Cli::method49(v587,
                                                                                                      new_array(&[v588,
                                                                                                                  v589,
                                                                                                                  v590,
                                                                                                                  v591,
                                                                                                                  v592,
                                                                                                                  Cli::method24(v594.try_into().unwrap())]));
                                                                                        {
                                                                                            let v599 =
                                                                                                zmq::Context::new();
                                                                                            let v600 =
                                                                                                v599.socket(zmq::REQ).unwrap();
                                                                                            let v601:
                                                                                                    string =
                                                                                                sprintf!("tcp://{}:{}",
                                                                                                         &string("127.0.0.1"),
                                                                                                         &v397);
                                                                                            v600.connect(&v601.clone()).unwrap();
                                                                                            v600.send(v574.as_str(), 0).unwrap();
                                                                                            {
                                                                                                let v603:
                                                                                                        std::string::String =
                                                                                                    v600.recv_string(0).unwrap().unwrap();
                                                                                                v600.disconnect(&v601).unwrap();
                                                                                                {
                                                                                                    let v604:
                                                                                                            std::string::String =
                                                                                                        Cli::method10(string("result_len"));
                                                                                                    let v606 =
                                                                                                        v603.len();
                                                                                                    Cli::method49(v587,
                                                                                                                  new_array(&[v604,
                                                                                                                              Cli::method24(v606.try_into().unwrap())]));
                                                                                                    Cli::method37(v249,
                                                                                                                  new_array(&[Cli::method10(string("> spi_open; ok"))]));
                                                                                                    {
                                                                                                        let patternInput_11:
                                                                                                                (string,
                                                                                                                 string,
                                                                                                                 string,
                                                                                                                 bool) =
                                                                                                            Cli::method2();
                                                                                                        let v618:
                                                                                                                chrono::DateTime<chrono::Utc> =
                                                                                                            chrono::Utc::now();
                                                                                                        let v619:
                                                                                                                i64 =
                                                                                                            (&v618).timestamp();
                                                                                                        let v624:
                                                                                                                string =
                                                                                                            string("serde_json::json!({\"Ping\": true}).to_string()");
                                                                                                        let v625:
                                                                                                                std::string::String =
                                                                                                            serde_json::json!({"Ping": true}).to_string();
                                                                                                        let v626:
                                                                                                                std::string::String =
                                                                                                            Cli::method10(string("json_len"));
                                                                                                        let v628 =
                                                                                                            v625.len();
                                                                                                        Cli::method48(v619,
                                                                                                                      new_array(&[v626,
                                                                                                                                  Cli::method24(v628.try_into().unwrap())]));
                                                                                                        {
                                                                                                            let patternInput_12:
                                                                                                                    (string,
                                                                                                                     string,
                                                                                                                     string,
                                                                                                                     bool) =
                                                                                                                Cli::method2();
                                                                                                            let v637:
                                                                                                                    chrono::DateTime<chrono::Utc> =
                                                                                                                chrono::Utc::now();
                                                                                                            let v638:
                                                                                                                    i64 =
                                                                                                                (&v637).timestamp();
                                                                                                            let v639:
                                                                                                                    std::string::String =
                                                                                                                Cli::method10(string("host"));
                                                                                                            let v640:
                                                                                                                    std::string::String =
                                                                                                                Cli::method10(string("127.0.0.1"));
                                                                                                            let v641:
                                                                                                                    std::string::String =
                                                                                                                Cli::method10(string("port"));
                                                                                                            let v642:
                                                                                                                    std::string::String =
                                                                                                                Cli::method47(v397.clone());
                                                                                                            let v643:
                                                                                                                    std::string::String =
                                                                                                                Cli::method10(string("msg_len"));
                                                                                                            let v645 =
                                                                                                                v625.len();
                                                                                                            Cli::method49(v638,
                                                                                                                          new_array(&[v639,
                                                                                                                                      v640,
                                                                                                                                      v641,
                                                                                                                                      v642,
                                                                                                                                      v643,
                                                                                                                                      Cli::method24(v645.try_into().unwrap())]));
                                                                                                            {
                                                                                                                let v650 =
                                                                                                                    zmq::Context::new();
                                                                                                                let v651 =
                                                                                                                    v650.socket(zmq::REQ).unwrap();
                                                                                                                let v652:
                                                                                                                        string =
                                                                                                                    sprintf!("tcp://{}:{}",
                                                                                                                             &string("127.0.0.1"),
                                                                                                                             &v397);
                                                                                                                v651.connect(&v652.clone()).unwrap();
                                                                                                                v651.send(v625.as_str(), 0).unwrap();
                                                                                                                {
                                                                                                                    let v654:
                                                                                                                            std::string::String =
                                                                                                                        v651.recv_string(0).unwrap().unwrap();
                                                                                                                    v651.disconnect(&v652).unwrap();
                                                                                                                    {
                                                                                                                        let v655:
                                                                                                                                std::string::String =
                                                                                                                            Cli::method10(string("result_len"));
                                                                                                                        let v657 =
                                                                                                                            v654.len();
                                                                                                                        Cli::method49(v638,
                                                                                                                                      new_array(&[v655,
                                                                                                                                                  Cli::method24(v657.try_into().unwrap())]));
                                                                                                                        Cli::method37(v249,
                                                                                                                                      new_array(&[Cli::method10(string("> ping; ok"))]));
                                                                                                                        {
                                                                                                                            let v665:
                                                                                                                                    string =
                                                                                                                                string("r#\"../The-Spiral-Language/VS Code Plugin/core/package.spiproj\"#");
                                                                                                                            let v667:
                                                                                                                                    &str =
                                                                                                                                Cli::method51(r#"../The-Spiral-Language/VS Code Plugin/core/package.spiproj"#);
                                                                                                                            let v670:
                                                                                                                                    std::path::PathBuf =
                                                                                                                                Cli::method9(std::path::PathBuf::from(v667));
                                                                                                                            let v672:
                                                                                                                                    Result<std::path::PathBuf,
                                                                                                                                           std::io::Error> =
                                                                                                                                v670.canonicalize();
                                                                                                                            let v674:
                                                                                                                                    std::path::PathBuf =
                                                                                                                                v672.unwrap();
                                                                                                                            let v676:
                                                                                                                                    Result<std::string::String,
                                                                                                                                           std::io::Error> =
                                                                                                                                std::fs::read_to_string(v674);
                                                                                                                            let v677:
                                                                                                                                    std::string::String =
                                                                                                                                (&v676).as_ref().unwrap().to_string();
                                                                                                                            let patternInput_13:
                                                                                                                                    (string,
                                                                                                                                     string,
                                                                                                                                     string,
                                                                                                                                     bool) =
                                                                                                                                Cli::method2();
                                                                                                                            let v682:
                                                                                                                                    chrono::DateTime<chrono::Utc> =
                                                                                                                                chrono::Utc::now();
                                                                                                                            let v683:
                                                                                                                                    i64 =
                                                                                                                                (&v682).timestamp();
                                                                                                                            let v684:
                                                                                                                                    std::string::String =
                                                                                                                                Cli::method10(string("spiproj_path"));
                                                                                                                            let v687:
                                                                                                                                    std::string::String =
                                                                                                                                Cli::method1(format!("{}", v674.display()));
                                                                                                                            let v690:
                                                                                                                                    std::string::String =
                                                                                                                                Cli::method10(fable_library_rust::String_::fromString(v687));
                                                                                                                            let v692:
                                                                                                                                    std::string::String =
                                                                                                                                Cli::method10(string("spiproj_text_len"));
                                                                                                                            let v694 =
                                                                                                                                v677.len();
                                                                                                                            Cli::method52(v683,
                                                                                                                                          new_array(&[v684,
                                                                                                                                                      v690,
                                                                                                                                                      v692,
                                                                                                                                                      Cli::method24(v694.try_into().unwrap())]));
                                                                                                                            {
                                                                                                                                let v708:
                                                                                                                                        string =
                                                                                                                                    string("serde_json::json!({\"ProjectFileOpen\": {\"spiprojText\": v677,\"uri\": \"file://\".to_owned() + v674.to_str().unwrap()}}).to_string()");
                                                                                                                                let v709:
                                                                                                                                        std::string::String =
                                                                                                                                    serde_json::json!({"ProjectFileOpen": {"spiprojText": v677,"uri": "file://".to_owned() + v674.to_str().unwrap()}}).to_string();
                                                                                                                                let v710:
                                                                                                                                        std::string::String =
                                                                                                                                    Cli::method10(string("json_len"));
                                                                                                                                let v712 =
                                                                                                                                    v709.len();
                                                                                                                                Cli::method52(v683,
                                                                                                                                              new_array(&[v710,
                                                                                                                                                          Cli::method24(v712.try_into().unwrap())]));
                                                                                                                                {
                                                                                                                                    let patternInput_14:
                                                                                                                                            (string,
                                                                                                                                             string,
                                                                                                                                             string,
                                                                                                                                             bool) =
                                                                                                                                        Cli::method2();
                                                                                                                                    let v721:
                                                                                                                                            chrono::DateTime<chrono::Utc> =
                                                                                                                                        chrono::Utc::now();
                                                                                                                                    let v722:
                                                                                                                                            i64 =
                                                                                                                                        (&v721).timestamp();
                                                                                                                                    let v723:
                                                                                                                                            std::string::String =
                                                                                                                                        Cli::method10(string("host"));
                                                                                                                                    let v724:
                                                                                                                                            std::string::String =
                                                                                                                                        Cli::method10(string("127.0.0.1"));
                                                                                                                                    let v725:
                                                                                                                                            std::string::String =
                                                                                                                                        Cli::method10(string("port"));
                                                                                                                                    let v726:
                                                                                                                                            std::string::String =
                                                                                                                                        Cli::method47(v397.clone());
                                                                                                                                    let v727:
                                                                                                                                            std::string::String =
                                                                                                                                        Cli::method10(string("msg_len"));
                                                                                                                                    let v729 =
                                                                                                                                        v709.len();
                                                                                                                                    Cli::method49(v722,
                                                                                                                                                  new_array(&[v723,
                                                                                                                                                              v724,
                                                                                                                                                              v725,
                                                                                                                                                              v726,
                                                                                                                                                              v727,
                                                                                                                                                              Cli::method24(v729.try_into().unwrap())]));
                                                                                                                                    {
                                                                                                                                        let v734 =
                                                                                                                                            zmq::Context::new();
                                                                                                                                        let v735 =
                                                                                                                                            v734.socket(zmq::REQ).unwrap();
                                                                                                                                        let v736:
                                                                                                                                                string =
                                                                                                                                            sprintf!("tcp://{}:{}",
                                                                                                                                                     &string("127.0.0.1"),
                                                                                                                                                     &v397);
                                                                                                                                        v735.connect(&v736.clone()).unwrap();
                                                                                                                                        v735.send(v709.as_str(), 0).unwrap();
                                                                                                                                        {
                                                                                                                                            let v738:
                                                                                                                                                    std::string::String =
                                                                                                                                                v735.recv_string(0).unwrap().unwrap();
                                                                                                                                            v735.disconnect(&v736).unwrap();
                                                                                                                                            {
                                                                                                                                                let v739:
                                                                                                                                                        std::string::String =
                                                                                                                                                    Cli::method10(string("result_len"));
                                                                                                                                                let v741 =
                                                                                                                                                    v738.len();
                                                                                                                                                Cli::method49(v722,
                                                                                                                                                              new_array(&[v739,
                                                                                                                                                                          Cli::method24(v741.try_into().unwrap())]));
                                                                                                                                                Cli::method37(v249,
                                                                                                                                                              new_array(&[Cli::method10(string("> spiproj_open base_spiproj_path; ok"))]));
                                                                                                                                                {
                                                                                                                                                    let v750:
                                                                                                                                                            Result<std::string::String,
                                                                                                                                                                   std::io::Error> =
                                                                                                                                                        std::fs::read_to_string(v309);
                                                                                                                                                    let v751:
                                                                                                                                                            std::string::String =
                                                                                                                                                        (&v750).as_ref().unwrap().to_string();
                                                                                                                                                    let patternInput_15:
                                                                                                                                                            (string,
                                                                                                                                                             string,
                                                                                                                                                             string,
                                                                                                                                                             bool) =
                                                                                                                                                        Cli::method2();
                                                                                                                                                    let v756:
                                                                                                                                                            chrono::DateTime<chrono::Utc> =
                                                                                                                                                        chrono::Utc::now();
                                                                                                                                                    let v757:
                                                                                                                                                            i64 =
                                                                                                                                                        (&v756).timestamp();
                                                                                                                                                    let v758:
                                                                                                                                                            std::string::String =
                                                                                                                                                        Cli::method10(string("spiproj_path"));
                                                                                                                                                    let v761:
                                                                                                                                                            std::string::String =
                                                                                                                                                        Cli::method1(format!("{}", v309.display()));
                                                                                                                                                    let v764:
                                                                                                                                                            std::string::String =
                                                                                                                                                        Cli::method10(fable_library_rust::String_::fromString(v761));
                                                                                                                                                    let v765:
                                                                                                                                                            std::string::String =
                                                                                                                                                        Cli::method10(string("spiproj_text_len"));
                                                                                                                                                    let v767 =
                                                                                                                                                        v751.len();
                                                                                                                                                    Cli::method52(v757,
                                                                                                                                                                  new_array(&[v758,
                                                                                                                                                                              v764,
                                                                                                                                                                              v765,
                                                                                                                                                                              Cli::method24(v767.try_into().unwrap())]));
                                                                                                                                                    {
                                                                                                                                                        let v780:
                                                                                                                                                                string =
                                                                                                                                                            string("serde_json::json!({\"ProjectFileOpen\": {\"spiprojText\": v751,\"uri\": \"file://\".to_owned() + v309.to_str().unwrap()}}).to_string()");
                                                                                                                                                        let v781:
                                                                                                                                                                std::string::String =
                                                                                                                                                            serde_json::json!({"ProjectFileOpen": {"spiprojText": v751,"uri": "file://".to_owned() + v309.to_str().unwrap()}}).to_string();
                                                                                                                                                        let v782:
                                                                                                                                                                std::string::String =
                                                                                                                                                            Cli::method10(string("json_len"));
                                                                                                                                                        let v784 =
                                                                                                                                                            v781.len();
                                                                                                                                                        Cli::method52(v757,
                                                                                                                                                                      new_array(&[v782,
                                                                                                                                                                                  Cli::method24(v784.try_into().unwrap())]));
                                                                                                                                                        {
                                                                                                                                                            let patternInput_16:
                                                                                                                                                                    (string,
                                                                                                                                                                     string,
                                                                                                                                                                     string,
                                                                                                                                                                     bool) =
                                                                                                                                                                Cli::method2();
                                                                                                                                                            let v793:
                                                                                                                                                                    chrono::DateTime<chrono::Utc> =
                                                                                                                                                                chrono::Utc::now();
                                                                                                                                                            let v794:
                                                                                                                                                                    i64 =
                                                                                                                                                                (&v793).timestamp();
                                                                                                                                                            let v795:
                                                                                                                                                                    std::string::String =
                                                                                                                                                                Cli::method10(string("host"));
                                                                                                                                                            let v796:
                                                                                                                                                                    std::string::String =
                                                                                                                                                                Cli::method10(string("127.0.0.1"));
                                                                                                                                                            let v797:
                                                                                                                                                                    std::string::String =
                                                                                                                                                                Cli::method10(string("port"));
                                                                                                                                                            let v798:
                                                                                                                                                                    std::string::String =
                                                                                                                                                                Cli::method47(v397.clone());
                                                                                                                                                            let v799:
                                                                                                                                                                    std::string::String =
                                                                                                                                                                Cli::method10(string("msg_len"));
                                                                                                                                                            let v801 =
                                                                                                                                                                v781.len();
                                                                                                                                                            Cli::method49(v794,
                                                                                                                                                                          new_array(&[v795,
                                                                                                                                                                                      v796,
                                                                                                                                                                                      v797,
                                                                                                                                                                                      v798,
                                                                                                                                                                                      v799,
                                                                                                                                                                                      Cli::method24(v801.try_into().unwrap())]));
                                                                                                                                                            {
                                                                                                                                                                let v806 =
                                                                                                                                                                    zmq::Context::new();
                                                                                                                                                                let v807 =
                                                                                                                                                                    v806.socket(zmq::REQ).unwrap();
                                                                                                                                                                let v808:
                                                                                                                                                                        string =
                                                                                                                                                                    sprintf!("tcp://{}:{}",
                                                                                                                                                                             &string("127.0.0.1"),
                                                                                                                                                                             &v397);
                                                                                                                                                                v807.connect(&v808.clone()).unwrap();
                                                                                                                                                                v807.send(v781.as_str(), 0).unwrap();
                                                                                                                                                                {
                                                                                                                                                                    let v810:
                                                                                                                                                                            std::string::String =
                                                                                                                                                                        v807.recv_string(0).unwrap().unwrap();
                                                                                                                                                                    v807.disconnect(&v808).unwrap();
                                                                                                                                                                    {
                                                                                                                                                                        let v811:
                                                                                                                                                                                std::string::String =
                                                                                                                                                                            Cli::method10(string("result_len"));
                                                                                                                                                                        let v813 =
                                                                                                                                                                            v810.len();
                                                                                                                                                                        Cli::method49(v794,
                                                                                                                                                                                      new_array(&[v811,
                                                                                                                                                                                                  Cli::method24(v813.try_into().unwrap())]));
                                                                                                                                                                        Cli::method37(v249,
                                                                                                                                                                                      new_array(&[Cli::method10(string("> spiproj_open spiproj_path; ok"))]));
                                                                                                                                                                        {
                                                                                                                                                                            let patternInput_17:
                                                                                                                                                                                    (string,
                                                                                                                                                                                     string,
                                                                                                                                                                                     string,
                                                                                                                                                                                     bool) =
                                                                                                                                                                                Cli::method2();
                                                                                                                                                                            let v825:
                                                                                                                                                                                    chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                chrono::Utc::now();
                                                                                                                                                                            let v826:
                                                                                                                                                                                    i64 =
                                                                                                                                                                                (&v825).timestamp();
                                                                                                                                                                            let v831:
                                                                                                                                                                                    string =
                                                                                                                                                                                string("serde_json::json!({\"Ping\": true}).to_string()");
                                                                                                                                                                            let v832:
                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                serde_json::json!({"Ping": true}).to_string();
                                                                                                                                                                            let v833:
                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                Cli::method10(string("json_len"));
                                                                                                                                                                            let v835 =
                                                                                                                                                                                v832.len();
                                                                                                                                                                            Cli::method48(v826,
                                                                                                                                                                                          new_array(&[v833,
                                                                                                                                                                                                      Cli::method24(v835.try_into().unwrap())]));
                                                                                                                                                                            {
                                                                                                                                                                                let patternInput_18:
                                                                                                                                                                                        (string,
                                                                                                                                                                                         string,
                                                                                                                                                                                         string,
                                                                                                                                                                                         bool) =
                                                                                                                                                                                    Cli::method2();
                                                                                                                                                                                let v844:
                                                                                                                                                                                        chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                    chrono::Utc::now();
                                                                                                                                                                                let v845:
                                                                                                                                                                                        i64 =
                                                                                                                                                                                    (&v844).timestamp();
                                                                                                                                                                                let v846:
                                                                                                                                                                                        std::string::String =
                                                                                                                                                                                    Cli::method10(string("host"));
                                                                                                                                                                                let v847:
                                                                                                                                                                                        std::string::String =
                                                                                                                                                                                    Cli::method10(string("127.0.0.1"));
                                                                                                                                                                                let v848:
                                                                                                                                                                                        std::string::String =
                                                                                                                                                                                    Cli::method10(string("port"));
                                                                                                                                                                                let v849:
                                                                                                                                                                                        std::string::String =
                                                                                                                                                                                    Cli::method47(v397.clone());
                                                                                                                                                                                let v850:
                                                                                                                                                                                        std::string::String =
                                                                                                                                                                                    Cli::method10(string("msg_len"));
                                                                                                                                                                                let v852 =
                                                                                                                                                                                    v832.len();
                                                                                                                                                                                Cli::method49(v845,
                                                                                                                                                                                              new_array(&[v846,
                                                                                                                                                                                                          v847,
                                                                                                                                                                                                          v848,
                                                                                                                                                                                                          v849,
                                                                                                                                                                                                          v850,
                                                                                                                                                                                                          Cli::method24(v852.try_into().unwrap())]));
                                                                                                                                                                                {
                                                                                                                                                                                    let v857 =
                                                                                                                                                                                        zmq::Context::new();
                                                                                                                                                                                    let v858 =
                                                                                                                                                                                        v857.socket(zmq::REQ).unwrap();
                                                                                                                                                                                    let v859:
                                                                                                                                                                                            string =
                                                                                                                                                                                        sprintf!("tcp://{}:{}",
                                                                                                                                                                                                 &string("127.0.0.1"),
                                                                                                                                                                                                 &v397);
                                                                                                                                                                                    v858.connect(&v859.clone()).unwrap();
                                                                                                                                                                                    v858.send(v832.as_str(), 0).unwrap();
                                                                                                                                                                                    {
                                                                                                                                                                                        let v861:
                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                            v858.recv_string(0).unwrap().unwrap();
                                                                                                                                                                                        v858.disconnect(&v859).unwrap();
                                                                                                                                                                                        {
                                                                                                                                                                                            let v862:
                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                Cli::method10(string("result_len"));
                                                                                                                                                                                            let v864 =
                                                                                                                                                                                                v861.len();
                                                                                                                                                                                            Cli::method49(v845,
                                                                                                                                                                                                          new_array(&[v862,
                                                                                                                                                                                                                      Cli::method24(v864.try_into().unwrap())]));
                                                                                                                                                                                            Cli::method37(v249,
                                                                                                                                                                                                          new_array(&[Cli::method10(string("> ping; ok"))]));
                                                                                                                                                                                            {
                                                                                                                                                                                                let patternInput_19:
                                                                                                                                                                                                        (string,
                                                                                                                                                                                                         string,
                                                                                                                                                                                                         string,
                                                                                                                                                                                                         bool) =
                                                                                                                                                                                                    Cli::method2();
                                                                                                                                                                                                let v875:
                                                                                                                                                                                                        chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                                    chrono::Utc::now();
                                                                                                                                                                                                let v876:
                                                                                                                                                                                                        i64 =
                                                                                                                                                                                                    (&v875).timestamp();
                                                                                                                                                                                                let v877:
                                                                                                                                                                                                        std::string::String =
                                                                                                                                                                                                    Cli::method10(string("spi_path"));
                                                                                                                                                                                                let v880:
                                                                                                                                                                                                        std::string::String =
                                                                                                                                                                                                    Cli::method1(format!("{}", v234.display()));
                                                                                                                                                                                                Cli::method53(v876,
                                                                                                                                                                                                              new_array(&[v877,
                                                                                                                                                                                                                          Cli::method10(fable_library_rust::String_::fromString(v880)),
                                                                                                                                                                                                                          Cli::method10(string("backend")),
                                                                                                                                                                                                                          Cli::method10(string("Fsharp"))]));
                                                                                                                                                                                                {
                                                                                                                                                                                                    let v889:
                                                                                                                                                                                                            string =
                                                                                                                                                                                                        Cli::method54();
                                                                                                                                                                                                    let v891:
                                                                                                                                                                                                            &str =
                                                                                                                                                                                                        fable_library_rust::String_::LrcStr::as_str(&v889);
                                                                                                                                                                                                    let v893:
                                                                                                                                                                                                            std::string::String =
                                                                                                                                                                                                        String::from(core::ops::Deref::deref(&v891));
                                                                                                                                                                                                    let v903:
                                                                                                                                                                                                            string =
                                                                                                                                                                                                        string("serde_json::json!({\"BuildFile\": {\"uri\": \"file://\".to_owned() + v234.to_str().unwrap(),\"backend\": v893}}).to_string()");
                                                                                                                                                                                                    let v904:
                                                                                                                                                                                                            std::string::String =
                                                                                                                                                                                                        serde_json::json!({"BuildFile": {"uri": "file://".to_owned() + v234.to_str().unwrap(),"backend": v893}}).to_string();
                                                                                                                                                                                                    let v905:
                                                                                                                                                                                                            std::string::String =
                                                                                                                                                                                                        Cli::method10(string("json_len"));
                                                                                                                                                                                                    let v907 =
                                                                                                                                                                                                        v904.len();
                                                                                                                                                                                                    Cli::method53(v876,
                                                                                                                                                                                                                  new_array(&[v905,
                                                                                                                                                                                                                              Cli::method24(v907.try_into().unwrap())]));
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let patternInput_20:
                                                                                                                                                                                                                (string,
                                                                                                                                                                                                                 string,
                                                                                                                                                                                                                 string,
                                                                                                                                                                                                                 bool) =
                                                                                                                                                                                                            Cli::method2();
                                                                                                                                                                                                        let v916:
                                                                                                                                                                                                                chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                                            chrono::Utc::now();
                                                                                                                                                                                                        let v917:
                                                                                                                                                                                                                i64 =
                                                                                                                                                                                                            (&v916).timestamp();
                                                                                                                                                                                                        let v918:
                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                            Cli::method10(string("host"));
                                                                                                                                                                                                        let v919:
                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                            Cli::method10(string("127.0.0.1"));
                                                                                                                                                                                                        let v920:
                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                            Cli::method10(string("port"));
                                                                                                                                                                                                        let v921:
                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                            Cli::method47(v397.clone());
                                                                                                                                                                                                        let v922:
                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                            Cli::method10(string("msg_len"));
                                                                                                                                                                                                        let v924 =
                                                                                                                                                                                                            v904.len();
                                                                                                                                                                                                        Cli::method49(v917,
                                                                                                                                                                                                                      new_array(&[v918,
                                                                                                                                                                                                                                  v919,
                                                                                                                                                                                                                                  v920,
                                                                                                                                                                                                                                  v921,
                                                                                                                                                                                                                                  v922,
                                                                                                                                                                                                                                  Cli::method24(v924.try_into().unwrap())]));
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let v929 =
                                                                                                                                                                                                                zmq::Context::new();
                                                                                                                                                                                                            let v930 =
                                                                                                                                                                                                                v929.socket(zmq::REQ).unwrap();
                                                                                                                                                                                                            let v931:
                                                                                                                                                                                                                    string =
                                                                                                                                                                                                                sprintf!("tcp://{}:{}",
                                                                                                                                                                                                                         &string("127.0.0.1"),
                                                                                                                                                                                                                         &v397);
                                                                                                                                                                                                            v930.connect(&v931.clone()).unwrap();
                                                                                                                                                                                                            v930.send(v904.as_str(), 0).unwrap();
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let v933:
                                                                                                                                                                                                                        std::string::String =
                                                                                                                                                                                                                    v930.recv_string(0).unwrap().unwrap();
                                                                                                                                                                                                                v930.disconnect(&v931).unwrap();
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let v934:
                                                                                                                                                                                                                            std::string::String =
                                                                                                                                                                                                                        Cli::method10(string("result_len"));
                                                                                                                                                                                                                    let v936 =
                                                                                                                                                                                                                        v933.len();
                                                                                                                                                                                                                    Cli::method49(v917,
                                                                                                                                                                                                                                  new_array(&[v934,
                                                                                                                                                                                                                                              Cli::method24(v936.try_into().unwrap())]));
                                                                                                                                                                                                                    Cli::method37(v249,
                                                                                                                                                                                                                                  new_array(&[Cli::method10(string("> spi_build_file; ok"))]));
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let patternInput_21:
                                                                                                                                                                                                                                (string,
                                                                                                                                                                                                                                 string,
                                                                                                                                                                                                                                 string,
                                                                                                                                                                                                                                 bool) =
                                                                                                                                                                                                                            Cli::method2();
                                                                                                                                                                                                                        let v948:
                                                                                                                                                                                                                                chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                                                            chrono::Utc::now();
                                                                                                                                                                                                                        let v949:
                                                                                                                                                                                                                                i64 =
                                                                                                                                                                                                                            (&v948).timestamp();
                                                                                                                                                                                                                        let v954:
                                                                                                                                                                                                                                string =
                                                                                                                                                                                                                            string("serde_json::json!({\"Ping\": true}).to_string()");
                                                                                                                                                                                                                        let v955:
                                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                                            serde_json::json!({"Ping": true}).to_string();
                                                                                                                                                                                                                        let v956:
                                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                                            Cli::method10(string("json_len"));
                                                                                                                                                                                                                        let v958 =
                                                                                                                                                                                                                            v955.len();
                                                                                                                                                                                                                        Cli::method48(v949,
                                                                                                                                                                                                                                      new_array(&[v956,
                                                                                                                                                                                                                                                  Cli::method24(v958.try_into().unwrap())]));
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let patternInput_22:
                                                                                                                                                                                                                                    (string,
                                                                                                                                                                                                                                     string,
                                                                                                                                                                                                                                     string,
                                                                                                                                                                                                                                     bool) =
                                                                                                                                                                                                                                Cli::method2();
                                                                                                                                                                                                                            let v967:
                                                                                                                                                                                                                                    chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                                                                chrono::Utc::now();
                                                                                                                                                                                                                            let v968:
                                                                                                                                                                                                                                    i64 =
                                                                                                                                                                                                                                (&v967).timestamp();
                                                                                                                                                                                                                            let v969:
                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                Cli::method10(string("host"));
                                                                                                                                                                                                                            let v970:
                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                Cli::method10(string("127.0.0.1"));
                                                                                                                                                                                                                            let v971:
                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                Cli::method10(string("port"));
                                                                                                                                                                                                                            let v972:
                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                Cli::method47(v397.clone());
                                                                                                                                                                                                                            let v973:
                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                Cli::method10(string("msg_len"));
                                                                                                                                                                                                                            let v975 =
                                                                                                                                                                                                                                v955.len();
                                                                                                                                                                                                                            Cli::method49(v968,
                                                                                                                                                                                                                                          new_array(&[v969,
                                                                                                                                                                                                                                                      v970,
                                                                                                                                                                                                                                                      v971,
                                                                                                                                                                                                                                                      v972,
                                                                                                                                                                                                                                                      v973,
                                                                                                                                                                                                                                                      Cli::method24(v975.try_into().unwrap())]));
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let v980 =
                                                                                                                                                                                                                                    zmq::Context::new();
                                                                                                                                                                                                                                let v981 =
                                                                                                                                                                                                                                    v980.socket(zmq::REQ).unwrap();
                                                                                                                                                                                                                                let v982:
                                                                                                                                                                                                                                        string =
                                                                                                                                                                                                                                    sprintf!("tcp://{}:{}",
                                                                                                                                                                                                                                             &string("127.0.0.1"),
                                                                                                                                                                                                                                             &v397);
                                                                                                                                                                                                                                v981.connect(&v982.clone()).unwrap();
                                                                                                                                                                                                                                v981.send(v955.as_str(), 0).unwrap();
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let v984:
                                                                                                                                                                                                                                            std::string::String =
                                                                                                                                                                                                                                        v981.recv_string(0).unwrap().unwrap();
                                                                                                                                                                                                                                    v981.disconnect(&v982).unwrap();
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let v985:
                                                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                                                            Cli::method10(string("result_len"));
                                                                                                                                                                                                                                        let v987 =
                                                                                                                                                                                                                                            v984.len();
                                                                                                                                                                                                                                        Cli::method49(v968,
                                                                                                                                                                                                                                                      new_array(&[v985,
                                                                                                                                                                                                                                                                  Cli::method24(v987.try_into().unwrap())]));
                                                                                                                                                                                                                                        Cli::method37(v249,
                                                                                                                                                                                                                                                      new_array(&[Cli::method10(string("> ping; ok"))]));
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            let patternInput_23:
                                                                                                                                                                                                                                                    (string,
                                                                                                                                                                                                                                                     string,
                                                                                                                                                                                                                                                     string,
                                                                                                                                                                                                                                                     bool) =
                                                                                                                                                                                                                                                Cli::method2();
                                                                                                                                                                                                                                            let v998:
                                                                                                                                                                                                                                                    chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                                                                                chrono::Utc::now();
                                                                                                                                                                                                                                            let v999:
                                                                                                                                                                                                                                                    i64 =
                                                                                                                                                                                                                                                (&v998).timestamp();
                                                                                                                                                                                                                                            let v1001:
                                                                                                                                                                                                                                                    std::time::SystemTime =
                                                                                                                                                                                                                                                std::fs::metadata(v377).unwrap().modified().unwrap();
                                                                                                                                                                                                                                            let v1002:
                                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                                Cli::method10(string("fsx_path"));
                                                                                                                                                                                                                                            let v1005:
                                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                                Cli::method1(format!("{}", v377.display()));
                                                                                                                                                                                                                                            Cli::method56(v999,
                                                                                                                                                                                                                                                          new_array(&[v1002,
                                                                                                                                                                                                                                                                      Cli::method10(fable_library_rust::String_::fromString(v1005)),
                                                                                                                                                                                                                                                                      Cli::method10(string("start")),
                                                                                                                                                                                                                                                                      Cli::method55(v1001)]));
                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                let v1013:
                                                                                                                                                                                                                                                        Array<i32> =
                                                                                                                                                                                                                                                    toArray(rangeNumeric(0_i32,
                                                                                                                                                                                                                                                                         1_i32,
                                                                                                                                                                                                                                                                         20_i32).clone());
                                                                                                                                                                                                                                                let v1014:
                                                                                                                                                                                                                                                        i32 =
                                                                                                                                                                                                                                                    count(v1013.clone());
                                                                                                                                                                                                                                                let v1016:
                                                                                                                                                                                                                                                        LrcPtr<Cli::Mut1> =
                                                                                                                                                                                                                                                    LrcPtr::new(Cli::Mut1{l0:
                                                                                                                                                                                                                                                                              MutCell::new(0_i32),
                                                                                                                                                                                                                                                                          l1:
                                                                                                                                                                                                                                                                              MutCell::new(Cli::US1::US1_0),});
                                                                                                                                                                                                                                                while Cli::method57(v1014,
                                                                                                                                                                                                                                                                    v1016.clone())
                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                    let v1018:
                                                                                                                                                                                                                                                            i32 =
                                                                                                                                                                                                                                                        v1016.l0.get();
                                                                                                                                                                                                                                                    let v1019:
                                                                                                                                                                                                                                                            Cli::US1 =
                                                                                                                                                                                                                                                        v1016.l1.get().clone();
                                                                                                                                                                                                                                                    let v1020:
                                                                                                                                                                                                                                                            i32 =
                                                                                                                                                                                                                                                        v1013[v1018].clone();
                                                                                                                                                                                                                                                    let v1046:
                                                                                                                                                                                                                                                            Cli::US1 =
                                                                                                                                                                                                                                                        match &v1019
                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                            Cli::US1::US1_1(v1019_1_0)
                                                                                                                                                                                                                                                            =>
                                                                                                                                                                                                                                                            Cli::US1::US1_1(v1019_1_0.clone()),
                                                                                                                                                                                                                                                            _
                                                                                                                                                                                                                                                            =>
                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                let v1024:
                                                                                                                                                                                                                                                                        std::time::SystemTime =
                                                                                                                                                                                                                                                                    std::fs::metadata(v377).unwrap().modified().unwrap();
                                                                                                                                                                                                                                                                Cli::method56(v999,
                                                                                                                                                                                                                                                                              new_array(&[Cli::method10(string("i")),
                                                                                                                                                                                                                                                                                          Cli::method24(v1020),
                                                                                                                                                                                                                                                                                          Cli::method10(string("last_modified")),
                                                                                                                                                                                                                                                                                          Cli::method55(v1024)]));
                                                                                                                                                                                                                                                                if v1024 > v1001
                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                    let v1035:
                                                                                                                                                                                                                                                                            Result<std::string::String,
                                                                                                                                                                                                                                                                                   std::io::Error> =
                                                                                                                                                                                                                                                                        std::fs::read_to_string(v377);
                                                                                                                                                                                                                                                                    let v1037:
                                                                                                                                                                                                                                                                            std::string::String =
                                                                                                                                                                                                                                                                        Cli::method1((&v1035).as_ref().unwrap().to_string());
                                                                                                                                                                                                                                                                    Cli::US1::US1_1(fable_library_rust::String_::fromString(v1037))
                                                                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                                                                    let v1041:
                                                                                                                                                                                                                                                                            u64 =
                                                                                                                                                                                                                                                                        Cli::method58();
                                                                                                                                                                                                                                                                    std::thread::sleep(std::time::Duration::from_millis(v1041));
                                                                                                                                                                                                                                                                    Cli::US1::US1_0
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                        };
                                                                                                                                                                                                                                                    let v1047:
                                                                                                                                                                                                                                                            i32 =
                                                                                                                                                                                                                                                        v1018
                                                                                                                                                                                                                                                            +
                                                                                                                                                                                                                                                            1_i32;
                                                                                                                                                                                                                                                    v1016.l0.set(v1047);
                                                                                                                                                                                                                                                    v1016.l1.set(v1046.clone());
                                                                                                                                                                                                                                                    ()
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                    let v1048:
                                                                                                                                                                                                                                                            Cli::US1 =
                                                                                                                                                                                                                                                        v1016.l1.get().clone();
                                                                                                                                                                                                                                                    let v1052:
                                                                                                                                                                                                                                                            string =
                                                                                                                                                                                                                                                        match &v1048
                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                            Cli::US1::US1_1(v1048_1_0)
                                                                                                                                                                                                                                                            =>
                                                                                                                                                                                                                                                            v1048_1_0.clone(),
                                                                                                                                                                                                                                                            _
                                                                                                                                                                                                                                                            =>
                                                                                                                                                                                                                                                            string(""),
                                                                                                                                                                                                                                                        };
                                                                                                                                                                                                                                                    let v1054:
                                                                                                                                                                                                                                                            std::string::String =
                                                                                                                                                                                                                                                        Cli::method10(string("new_fsx_len"));
                                                                                                                                                                                                                                                    let v1056 =
                                                                                                                                                                                                                                                        v1052.len();
                                                                                                                                                                                                                                                    Cli::method37(v249,
                                                                                                                                                                                                                                                                  new_array(&[v1054,
                                                                                                                                                                                                                                                                              Cli::method24(v1056.try_into().unwrap())]));
                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                        let patternInput_24:
                                                                                                                                                                                                                                                                (string,
                                                                                                                                                                                                                                                                 string,
                                                                                                                                                                                                                                                                 string,
                                                                                                                                                                                                                                                                 bool) =
                                                                                                                                                                                                                                                            Cli::method2();
                                                                                                                                                                                                                                                        let v1065:
                                                                                                                                                                                                                                                                chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                                                                                            chrono::Utc::now();
                                                                                                                                                                                                                                                        let v1066:
                                                                                                                                                                                                                                                                i64 =
                                                                                                                                                                                                                                                            (&v1065).timestamp();
                                                                                                                                                                                                                                                        let v1071:
                                                                                                                                                                                                                                                                string =
                                                                                                                                                                                                                                                            string("serde_json::json!({\"Ping\": true}).to_string()");
                                                                                                                                                                                                                                                        let v1072:
                                                                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                                                                            serde_json::json!({"Ping": true}).to_string();
                                                                                                                                                                                                                                                        let v1073:
                                                                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                                                                            Cli::method10(string("json_len"));
                                                                                                                                                                                                                                                        let v1075 =
                                                                                                                                                                                                                                                            v1072.len();
                                                                                                                                                                                                                                                        Cli::method48(v1066,
                                                                                                                                                                                                                                                                      new_array(&[v1073,
                                                                                                                                                                                                                                                                                  Cli::method24(v1075.try_into().unwrap())]));
                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                            let patternInput_25:
                                                                                                                                                                                                                                                                    (string,
                                                                                                                                                                                                                                                                     string,
                                                                                                                                                                                                                                                                     string,
                                                                                                                                                                                                                                                                     bool) =
                                                                                                                                                                                                                                                                Cli::method2();
                                                                                                                                                                                                                                                            let v1084:
                                                                                                                                                                                                                                                                    chrono::DateTime<chrono::Utc> =
                                                                                                                                                                                                                                                                chrono::Utc::now();
                                                                                                                                                                                                                                                            let v1085:
                                                                                                                                                                                                                                                                    i64 =
                                                                                                                                                                                                                                                                (&v1084).timestamp();
                                                                                                                                                                                                                                                            let v1086:
                                                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                                                Cli::method10(string("host"));
                                                                                                                                                                                                                                                            let v1087:
                                                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                                                Cli::method10(string("127.0.0.1"));
                                                                                                                                                                                                                                                            let v1088:
                                                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                                                Cli::method10(string("port"));
                                                                                                                                                                                                                                                            let v1089:
                                                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                                                Cli::method47(v397.clone());
                                                                                                                                                                                                                                                            let v1090:
                                                                                                                                                                                                                                                                    std::string::String =
                                                                                                                                                                                                                                                                Cli::method10(string("msg_len"));
                                                                                                                                                                                                                                                            let v1092 =
                                                                                                                                                                                                                                                                v1072.len();
                                                                                                                                                                                                                                                            Cli::method49(v1085,
                                                                                                                                                                                                                                                                          new_array(&[v1086,
                                                                                                                                                                                                                                                                                      v1087,
                                                                                                                                                                                                                                                                                      v1088,
                                                                                                                                                                                                                                                                                      v1089,
                                                                                                                                                                                                                                                                                      v1090,
                                                                                                                                                                                                                                                                                      Cli::method24(v1092.try_into().unwrap())]));
                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                let v1097 =
                                                                                                                                                                                                                                                                    zmq::Context::new();
                                                                                                                                                                                                                                                                let v1098 =
                                                                                                                                                                                                                                                                    v1097.socket(zmq::REQ).unwrap();
                                                                                                                                                                                                                                                                let v1099:
                                                                                                                                                                                                                                                                        string =
                                                                                                                                                                                                                                                                    sprintf!("tcp://{}:{}",
                                                                                                                                                                                                                                                                             &string("127.0.0.1"),
                                                                                                                                                                                                                                                                             &v397);
                                                                                                                                                                                                                                                                v1098.connect(&v1099.clone()).unwrap();
                                                                                                                                                                                                                                                                v1098.send(v1072.as_str(), 0).unwrap();
                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                    let v1101:
                                                                                                                                                                                                                                                                            std::string::String =
                                                                                                                                                                                                                                                                        v1098.recv_string(0).unwrap().unwrap();
                                                                                                                                                                                                                                                                    v1098.disconnect(&v1099).unwrap();
                                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                                        let v1102:
                                                                                                                                                                                                                                                                                std::string::String =
                                                                                                                                                                                                                                                                            Cli::method10(string("result_len"));
                                                                                                                                                                                                                                                                        let v1104 =
                                                                                                                                                                                                                                                                            v1101.len();
                                                                                                                                                                                                                                                                        Cli::method49(v1085,
                                                                                                                                                                                                                                                                                      new_array(&[v1102,
                                                                                                                                                                                                                                                                                                  Cli::method24(v1104.try_into().unwrap())]));
                                                                                                                                                                                                                                                                        Cli::method37(v249,
                                                                                                                                                                                                                                                                                      new_array(&[Cli::method10(string("> ping; ok"))]));
                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                            let v1112 =
                                                                                                                                                                                                                                                                                v430.wait_with_output().unwrap();
                                                                                                                                                                                                                                                                            Cli::method37(v249,
                                                                                                                                                                                                                                                                                          new_array(&[Cli::method10(string("output")),
                                                                                                                                                                                                                                                                                                      Cli::method15(format!("{:?}", v1112))]));
                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                let v1119:
                                                                                                                                                                                                                                                                                        std::path::PathBuf =
                                                                                                                                                                                                                                                                                    v243.unwrap_or(v377);
                                                                                                                                                                                                                                                                                let v1120:
                                                                                                                                                                                                                                                                                        string =
                                                                                                                                                                                                                                                                                    Cli::method17(v1052.clone());
                                                                                                                                                                                                                                                                                let v1122:
                                                                                                                                                                                                                                                                                        &str =
                                                                                                                                                                                                                                                                                    v1120.as_str();
                                                                                                                                                                                                                                                                                let v1124:
                                                                                                                                                                                                                                                                                        Result<(),
                                                                                                                                                                                                                                                                                               std::io::Error> =
                                                                                                                                                                                                                                                                                    std::fs::write(v1119, v1122);
                                                                                                                                                                                                                                                                                v1124.unwrap();
                                                                                                                                                                                                                                                                                Cli::method37(v249,
                                                                                                                                                                                                                                                                                              new_array(&[Cli::method10(string("> fs_write new_fsx_path; ok"))]));
                                                                                                                                                                                                                                                                                Cli::method59(v19,
                                                                                                                                                                                                                                                                                              new_array(&[Cli::method10(string("fsx.len")),
                                                                                                                                                                                                                                                                                                          Cli::method24(length(v1052))]))
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
            Cli::method59(
                v19,
                new_array(&[
                    Cli::method10(string("cli end")),
                    Cli::method10(string("???")),
                ]),
            );
            0_i32
        }
    }
    pub fn v0() -> Func0<i32> {
        static v0: MutCell<Option<Func0<i32>>> = MutCell::new(None);
        v0.get_or_init(|| Func0::new(move || Cli::closure0((), ())))
    }
    on_startup!((),);
}
#[path = "../rust/Types.rs"]
mod module_b7e639af;
pub use module_b7e639af::*;
