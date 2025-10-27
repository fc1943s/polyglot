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
use fable_library_rust::NativeArray_::array_from;
use fable_library_rust::String_::fromString;
mod module_b7a9935b {
    pub mod Math {
        use super::*;
        use fable_library_rust::Native_::Func0;
        use fable_library_rust::Native_::Func1;
        use fable_library_rust::Native_::LrcPtr;
        use fable_library_rust::Native_::MutCell;
        use fable_library_rust::Native_::OnceInit;
        use fable_library_rust::Native_::on_startup;
        use fable_library_rust::NativeArray_::Array;
        use fable_library_rust::NativeArray_::get_Count;
        use fable_library_rust::NativeArray_::new_array;
        use fable_library_rust::NativeArray_::new_init;
        use fable_library_rust::Option_::defaultValue;
        use fable_library_rust::Option_::map;
        use fable_library_rust::String_::append;
        use fable_library_rust::String_::concat;
        use fable_library_rust::String_::printfn;
        use fable_library_rust::String_::sprintf;
        use fable_library_rust::String_::string;
        on_startup!();
        use pyo3::prelude::PyAnyMethods;
        //,);
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut0 {
            pub l0: MutCell<i32>,
        }
        impl core::fmt::Display for Mut0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut1 {
            pub l0: MutCell<i32>,
            pub l1: MutCell<string>,
            pub l2: MutCell<string>,
        }
        impl core::fmt::Display for Mut1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub struct Mut2 {
            pub l0: MutCell<i32>,
            pub l1: MutCell<num_complex::Complex<f64>>,
        }
        impl core::fmt::Display for Mut2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US0 {
            US0_0(num_complex::Complex<f64>),
            US0_1,
        }
        impl core::fmt::Display for US0 {
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
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub enum UH0 {
            UH0_0,
            UH0_1(f64, LrcPtr<Math::UH0>),
        }
        impl core::fmt::Display for UH0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum UH1 {
            UH1_0,
            UH1_1(num_complex::Complex<f64>, LrcPtr<Math::UH1>),
        }
        impl core::fmt::Display for UH1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        pub fn method2(v0_1: i32, v1_1: LrcPtr<Math::Mut0>) -> bool {
            (v1_1.l0.get().clone()) < (v0_1)
        }
        pub fn method3(v0_1: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
            v0_1
        }
        pub fn method6(v0_1: i32, v1_1: LrcPtr<Math::Mut1>) -> bool {
            (v1_1.l0.get().clone()) < (v0_1)
        }
        pub fn method5(v0_1: Array<string>) -> string {
            let v1_1: i32 = get_Count(v0_1.clone());
            let v3: LrcPtr<Math::Mut1> = LrcPtr::new(Math::Mut1 {
                l0: MutCell::new(0_i32),
                l1: MutCell::new(string("")),
                l2: MutCell::new(string("")),
            });
            while Math::method6(v1_1, v3.clone()) {
                let v5: i32 = v3.l0.get().clone();
                let v8: i32 = ((v5.wrapping_neg()) + (v1_1)) - 1_i32;
                let matchValue: string = v3.l1.get().clone();
                let matchValue_1: string = v3.l2.get().clone();
                let v17: string =
                    append((append((v0_1[v8].clone()), (matchValue_1))), (matchValue));
                let v20: i32 = (v5) + 1_i32;
                v3.l0.set(v20);
                v3.l1.set(v17);
                v3.l2.set(string("\n"));
                ()
            }
            {
                let matchValue_2: string = v3.l1.get().clone();
                let matchValue_3: string = v3.l2.get().clone();
                matchValue_2
            }
        }
        pub fn method7(v0_1: pyo3::Python) -> pyo3::Python {
            v0_1
        }
        pub fn method8() -> string {
            string("fn")
        }
        pub fn method9(
            v0_1: pyo3::Bound<pyo3::types::PyModule>,
        ) -> pyo3::Bound<pyo3::types::PyModule> {
            v0_1
        }
        pub fn method10(
            v0_: bool,
            v0__1: LrcPtr<(f64, f64)>,
        ) -> LrcPtr<(bool, LrcPtr<(f64, f64)>)> {
            LrcPtr::new((v0_, v0__1))
        }
        pub fn method11(v0_1: pyo3::Bound<pyo3::PyAny>) -> pyo3::Bound<pyo3::PyAny> {
            v0_1
        }
        pub fn method12(v0_1: pyo3::Bound<pyo3::PyAny>) -> pyo3::Bound<pyo3::PyAny> {
            v0_1
        }
        pub fn method4(
            v0_1: pyo3::Python,
            v1_1: string,
            v2: num_complex::Complex<f64>,
        ) -> Result<num_complex::Complex<f64>, std::string::String> {
            let v13: string = string(
                "            args = { k: v for k, v in frame.f_locals.items() if frame.f_code.co_name != \'make_mpc\' and k not in [\'ctx\'] and not callable(v) }",
            );
            let v14: string = string(
                "            args_str = \', \'.join([ f\"{k}={re.sub(memory_address_pattern, \' at 0x<?>\', repr(v))}\" for k, v in args.items() ])",
            );
            let v36: string = Math::method5(new_array(&[
                string("import sys"),
                string("import traceback"),
                string("import re"),
                string("count = 0"),
                string("memory_address_pattern = re.compile(r\' at 0x[0-9a-fA-F]+\')"),
                string("def trace_calls(frame, event, arg):"),
                string("    global count"),
                string("    count += 1"),
                string("    if count < 200:"),
                string("        try:"),
                v13,
                v14,
                concat(new_array(&[
                    string("            print(f\"{event}("),
                    string("zeta_"),
                    string(
                        ") / f_code.co_name: {frame.f_code.co_name} / f_locals: {args_str} / f_lineno: {frame.f_lineno} / f_code.co_filename: {frame.f_code.co_filename.split(\'site-packages\')[-1]} / f_back.f_lineno: { \'\' if frame.f_back is None else frame.f_back.f_lineno } / f_back.f_code.co_filename: { \'\' if frame.f_back is None else frame.f_back.f_code.co_filename.split(\'site-packages\')[-1] } / arg: {re.sub(memory_address_pattern, \' at 0x<?>\', repr(arg))}\", flush=True)",
                    ),
                ])),
                string("        except ValueError as e:"),
                concat(new_array(&[
                    string("            print(f\'"),
                    string("zeta_"),
                    string(" / e: {e}\', flush=True)"),
                ])),
                string("        return trace_calls"),
                string("import mpmath"),
                string("def fn(log, s):"),
                string("    global count"),
                string("    if log:"),
                concat(new_array(&[
                    string("        print(f\'"),
                    string("zeta_"),
                    string(" / s: {s} / count: {count}\', flush=True)"),
                ])),
                string("    s = complex(*s)"),
                string("    try:"),
                string("        if log: sys.settrace(trace_calls)"),
                v1_1,
                string("        if log:"),
                string("            sys.settrace(None)"),
                concat(new_array(&[
                    string("            print(f\'"),
                    string("zeta_"),
                    string(" / result: {s} / count: {count}\', flush=True)"),
                ])),
                string("    except ValueError as e:"),
                string("        if s.real == 1:"),
                string("            s = complex(float(\'inf\'), 0)"),
                string("    return (s.real, s.imag)"),
            ]));
            let v46: LrcPtr<(bool, LrcPtr<(f64, f64)>)> =
                LrcPtr::new((false, LrcPtr::new((v2.clone().re, v2.im))));
            let v49: pyo3::Python = Math::method7(v0_1);
            let v56: &str = &*v36;
            let v92: std::string::String = String::from(v56);
            let v123: std::ffi::CString = std::ffi::CString::new(v92).unwrap();
            let v131: &str = &*string("");
            let v167: std::string::String = String::from(v131);
            let v198: std::ffi::CString = std::ffi::CString::new(v167).unwrap();
            let v200: Result<pyo3::Bound<pyo3::types::PyModule>, pyo3::PyErr> =
                pyo3::types::PyModule::from_code(v49, &v123, &v198, &v198);
            let v202: bool = true;
            let _result_map_error__ = v200.map_err(|x| {
                //;
                let v204: pyo3::PyErr = x;
                let v211: std::string::String = format!("{}", v204);
                let v242: bool = true;
                v211
            });
            let v244: Result<pyo3::Bound<pyo3::types::PyModule>, std::string::String> =
                _result_map_error__;
            let v248: pyo3::Bound<pyo3::types::PyModule> = v244.unwrap();
            let v260: string = Math::method8();
            let v267: &str = &*v260;
            let v297: pyo3::Bound<pyo3::types::PyModule> = Math::method9(v248);
            let v299: Result<pyo3::Bound<pyo3::PyAny>, pyo3::PyErr> = v297.getattr(v267);
            let v301: bool = true;
            let _result_map_error__ = v299.map_err(|x| {
                //;
                let v303: pyo3::PyErr = x;
                let v310: std::string::String = format!("{}", v303);
                let v341: bool = true;
                v310
            });
            let v343: Result<pyo3::Bound<pyo3::PyAny>, std::string::String> = _result_map_error__;
            let v347: pyo3::Bound<pyo3::PyAny> = v343.unwrap();
            let v359: LrcPtr<(bool, LrcPtr<(f64, f64)>)> =
                Math::method10(v46.0.clone(), v46.1.clone());
            let v360: pyo3::Bound<pyo3::PyAny> = Math::method11(v347);
            let v362: Result<pyo3::Bound<pyo3::PyAny>, pyo3::PyErr> =
                pyo3::prelude::PyAnyMethods::call(&v360, ((*v359).0, *(*v359).1), None);
            let v364: bool = true;
            let _result_map_error__ = v362.map_err(|x| {
                //;
                let v366: pyo3::PyErr = x;
                let v373: std::string::String = format!("{}", v366);
                let v404: bool = true;
                v373
            });
            let v406: Result<pyo3::Bound<pyo3::PyAny>, std::string::String> = _result_map_error__;
            let v409: pyo3::Bound<pyo3::PyAny> = Math::method12(v406?);
            let v411: Result<(f64, f64), pyo3::PyErr> = v409.extract();
            let v413: bool = true;
            let _result_map_error__ = v411.map_err(|x| {
                //;
                let v415: pyo3::PyErr = x;
                let v422: std::string::String = format!("{}", v415);
                let v453: bool = true;
                v422
            });
            let v455: Result<(f64, f64), std::string::String> = _result_map_error__;
            let patternInput: (f64, f64) = v455?;
            Ok::<num_complex::Complex<f64>, std::string::String>(num_complex::Complex::new(
                patternInput.0.clone(),
                patternInput.1.clone(),
            ))
        }
        pub fn method14(v0_1: LrcPtr<Math::Mut0>) -> bool {
            (v0_1.l0.get().clone()) < 10000_i32
        }
        pub fn method15(v0_1: i32, v1_1: LrcPtr<Math::Mut2>) -> bool {
            (v1_1.l0.get().clone()) < (v0_1)
        }
        pub fn method16(
            v0_1: pyo3::Python,
            v1_1: string,
            v2: num_complex::Complex<f64>,
        ) -> Result<num_complex::Complex<f64>, std::string::String> {
            let v13: string = string(
                "            args = { k: v for k, v in frame.f_locals.items() if frame.f_code.co_name != \'make_mpc\' and k not in [\'ctx\'] and not callable(v) }",
            );
            let v14: string = string(
                "            args_str = \', \'.join([ f\"{k}={re.sub(memory_address_pattern, \' at 0x<?>\', repr(v))}\" for k, v in args.items() ])",
            );
            let v36: string = Math::method5(new_array(&[
                string("import sys"),
                string("import traceback"),
                string("import re"),
                string("count = 0"),
                string("memory_address_pattern = re.compile(r\' at 0x[0-9a-fA-F]+\')"),
                string("def trace_calls(frame, event, arg):"),
                string("    global count"),
                string("    count += 1"),
                string("    if count < 200:"),
                string("        try:"),
                v13,
                v14,
                concat(new_array(&[
                    string("            print(f\"{event}("),
                    string("gamma_"),
                    string(
                        ") / f_code.co_name: {frame.f_code.co_name} / f_locals: {args_str} / f_lineno: {frame.f_lineno} / f_code.co_filename: {frame.f_code.co_filename.split(\'site-packages\')[-1]} / f_back.f_lineno: { \'\' if frame.f_back is None else frame.f_back.f_lineno } / f_back.f_code.co_filename: { \'\' if frame.f_back is None else frame.f_back.f_code.co_filename.split(\'site-packages\')[-1] } / arg: {re.sub(memory_address_pattern, \' at 0x<?>\', repr(arg))}\", flush=True)",
                    ),
                ])),
                string("        except ValueError as e:"),
                concat(new_array(&[
                    string("            print(f\'"),
                    string("gamma_"),
                    string(" / e: {e}\', flush=True)"),
                ])),
                string("        return trace_calls"),
                string("import mpmath"),
                string("def fn(log, s):"),
                string("    global count"),
                string("    if log:"),
                concat(new_array(&[
                    string("        print(f\'"),
                    string("gamma_"),
                    string(" / s: {s} / count: {count}\', flush=True)"),
                ])),
                string("    s = complex(*s)"),
                string("    try:"),
                string("        if log: sys.settrace(trace_calls)"),
                v1_1,
                string("        if log:"),
                string("            sys.settrace(None)"),
                concat(new_array(&[
                    string("            print(f\'"),
                    string("gamma_"),
                    string(" / result: {s} / count: {count}\', flush=True)"),
                ])),
                string("    except ValueError as e:"),
                string("        if s.real == 1:"),
                string("            s = complex(float(\'inf\'), 0)"),
                string("    return (s.real, s.imag)"),
            ]));
            let v46: LrcPtr<(bool, LrcPtr<(f64, f64)>)> =
                LrcPtr::new((false, LrcPtr::new((v2.clone().re, v2.im))));
            let v49: pyo3::Python = Math::method7(v0_1);
            let v56: &str = &*v36;
            let v92: std::string::String = String::from(v56);
            let v123: std::ffi::CString = std::ffi::CString::new(v92).unwrap();
            let v131: &str = &*string("");
            let v167: std::string::String = String::from(v131);
            let v198: std::ffi::CString = std::ffi::CString::new(v167).unwrap();
            let v200: Result<pyo3::Bound<pyo3::types::PyModule>, pyo3::PyErr> =
                pyo3::types::PyModule::from_code(v49, &v123, &v198, &v198);
            let v202: bool = true;
            let _result_map_error__ = v200.map_err(|x| {
                //;
                let v204: pyo3::PyErr = x;
                let v211: std::string::String = format!("{}", v204);
                let v242: bool = true;
                v211
            });
            let v244: Result<pyo3::Bound<pyo3::types::PyModule>, std::string::String> =
                _result_map_error__;
            let v248: pyo3::Bound<pyo3::types::PyModule> = v244.unwrap();
            let v260: string = Math::method8();
            let v267: &str = &*v260;
            let v297: pyo3::Bound<pyo3::types::PyModule> = Math::method9(v248);
            let v299: Result<pyo3::Bound<pyo3::PyAny>, pyo3::PyErr> = v297.getattr(v267);
            let v301: bool = true;
            let _result_map_error__ = v299.map_err(|x| {
                //;
                let v303: pyo3::PyErr = x;
                let v310: std::string::String = format!("{}", v303);
                let v341: bool = true;
                v310
            });
            let v343: Result<pyo3::Bound<pyo3::PyAny>, std::string::String> = _result_map_error__;
            let v347: pyo3::Bound<pyo3::PyAny> = v343.unwrap();
            let v359: LrcPtr<(bool, LrcPtr<(f64, f64)>)> =
                Math::method10(v46.0.clone(), v46.1.clone());
            let v360: pyo3::Bound<pyo3::PyAny> = Math::method11(v347);
            let v362: Result<pyo3::Bound<pyo3::PyAny>, pyo3::PyErr> =
                pyo3::prelude::PyAnyMethods::call(&v360, ((*v359).0, *(*v359).1), None);
            let v364: bool = true;
            let _result_map_error__ = v362.map_err(|x| {
                //;
                let v366: pyo3::PyErr = x;
                let v373: std::string::String = format!("{}", v366);
                let v404: bool = true;
                v373
            });
            let v406: Result<pyo3::Bound<pyo3::PyAny>, std::string::String> = _result_map_error__;
            let v409: pyo3::Bound<pyo3::PyAny> = Math::method12(v406?);
            let v411: Result<(f64, f64), pyo3::PyErr> = v409.extract();
            let v413: bool = true;
            let _result_map_error__ = v411.map_err(|x| {
                //;
                let v415: pyo3::PyErr = x;
                let v422: std::string::String = format!("{}", v415);
                let v453: bool = true;
                v422
            });
            let v455: Result<(f64, f64), std::string::String> = _result_map_error__;
            let patternInput: (f64, f64) = v455?;
            Ok::<num_complex::Complex<f64>, std::string::String>(num_complex::Complex::new(
                patternInput.0.clone(),
                patternInput.1.clone(),
            ))
        }
        pub fn closure1(unitVar: (), v0_1: num_complex::Complex<f64>) -> Math::US0 {
            Math::US0::US0_0(v0_1)
        }
        pub fn method17() -> Func1<num_complex::Complex<f64>, Math::US0> {
            Func1::new(move |v: num_complex::Complex<f64>| Math::closure1((), v))
        }
        pub fn method13(
            v0_1: pyo3::Python,
            v1_1: num_complex::Complex<f64>,
        ) -> num_complex::Complex<f64> {
            println!("zeta / count: {:?} / s: {:?}", 0_i32, v1_1.clone());
            if (v1_1.clone().re) > 1.0_f64 {
                let v7: num_complex::Complex<f64> = num_complex::Complex::new(0.0_f64, 0.0_f64);
                let v8: Array<i32> = new_init(&0_i32, 10000_i32);
                let v9: LrcPtr<Math::Mut0> = LrcPtr::new(Math::Mut0 {
                    l0: MutCell::new(0_i32),
                });
                while Math::method14(v9.clone()) {
                    let v11: i32 = v9.l0.get().clone();
                    v8.get_mut()[v11 as usize] = v11;
                    {
                        let v12: i32 = (v11) + 1_i32;
                        v9.l0.set(v12);
                        ()
                    }
                }
                {
                    let v13: i32 = get_Count(v8.clone());
                    let v14: LrcPtr<Math::Mut2> = LrcPtr::new(Math::Mut2 {
                        l0: MutCell::new(0_i32),
                        l1: MutCell::new(v7),
                    });
                    while Math::method15(v13, v14.clone()) {
                        let v16: i32 = v14.l0.get().clone();
                        let v17: num_complex::Complex<f64> = v14.l1.get().clone();
                        let v18: i32 = v8[v16].clone();
                        let v20: num_complex::Complex<f64> =
                            num_complex::Complex::new(1.0_f64, 0.0_f64);
                        let v23: f64 = v18 as f64;
                        let v27: num_complex::Complex<f64> =
                            num_complex::Complex::new(v23, 0.0_f64);
                        let v29: num_complex::Complex<f64> =
                            num_complex::Complex::powc(v27, v1_1.clone());
                        let v31: num_complex::Complex<f64> = v20 / v29;
                        let v33: num_complex::Complex<f64> = v17 + v31;
                        let v34: i32 = (v16) + 1_i32;
                        v14.l0.set(v34);
                        v14.l1.set(v33);
                        ()
                    }
                    v14.l1.get().clone()
                }
            } else {
                let v37: num_complex::Complex<f64> = num_complex::Complex::new(1.0_f64, 0.0_f64);
                let v42: Result<num_complex::Complex<f64>, std::string::String> = Math::method16(
                    v0_1.clone(),
                    string("        s = mpmath.gamma(s)"),
                    Math::method3(v37 - v1_1.clone()),
                );
                let v46: Option<num_complex::Complex<f64>> = v42.ok();
                let v82: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v46));
                let v87: f64 = f64::NAN;
                let v89: f64 = f64::NAN;
                let v91: num_complex::Complex<f64> = num_complex::Complex::new(v87, v89);
                let v94: num_complex::Complex<f64> = match &v82 {
                    Math::US0::US0_0(v82_0_0) => match &v82 {
                        Math::US0::US0_0(x) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                    _ => v91.clone(),
                };
                let v96: num_complex::Complex<f64> =
                    num_complex::Complex::new(3.141592653589793_f64, 0.0_f64);
                let v98: num_complex::Complex<f64> = v96 * v1_1.clone();
                let v100: num_complex::Complex<f64> = num_complex::Complex::new(2.0_f64, 0.0_f64);
                let v102: num_complex::Complex<f64> = v98 / v100;
                let v104: num_complex::Complex<f64> = v102.sin();
                let v107: f64 = 1.0_f64 - (v1_1.clone().re);
                let v110: f64 = -v1_1.clone().im;
                let v112: num_complex::Complex<f64> = num_complex::Complex::new(v107, v110);
                let v646: num_complex::Complex<f64> = if (v112.clone().re) <= 1.0_f64 {
                    num_complex::Complex::new(0.0_f64, 0.0_f64)
                } else {
                    println!("zeta / count: {:?} / s: {:?}", 1_i32, v112.clone());
                    if (v112.clone().re) > 1.0_f64 {
                        let v123: num_complex::Complex<f64> =
                            num_complex::Complex::new(0.0_f64, 0.0_f64);
                        let v124: Array<i32> = new_init(&0_i32, 10000_i32);
                        let v125: LrcPtr<Math::Mut0> = LrcPtr::new(Math::Mut0 {
                            l0: MutCell::new(0_i32),
                        });
                        while Math::method14(v125.clone()) {
                            let v127: i32 = v125.l0.get().clone();
                            v124.get_mut()[v127 as usize] = v127;
                            {
                                let v128: i32 = (v127) + 1_i32;
                                v125.l0.set(v128);
                                ()
                            }
                        }
                        {
                            let v129: i32 = get_Count(v124.clone());
                            let v130: LrcPtr<Math::Mut2> = LrcPtr::new(Math::Mut2 {
                                l0: MutCell::new(0_i32),
                                l1: MutCell::new(v123),
                            });
                            while Math::method15(v129, v130.clone()) {
                                let v132: i32 = v130.l0.get().clone();
                                let v133: num_complex::Complex<f64> = v130.l1.get().clone();
                                let v134: i32 = v124[v132].clone();
                                let v136: num_complex::Complex<f64> =
                                    num_complex::Complex::new(1.0_f64, 0.0_f64);
                                let v139: f64 = v134 as f64;
                                let v143: num_complex::Complex<f64> =
                                    num_complex::Complex::new(v139, 0.0_f64);
                                let v145: num_complex::Complex<f64> =
                                    num_complex::Complex::powc(v143, v112.clone());
                                let v147: num_complex::Complex<f64> = v136 / v145;
                                let v149: num_complex::Complex<f64> = v133 + v147;
                                let v150: i32 = (v132) + 1_i32;
                                v130.l0.set(v150);
                                v130.l1.set(v149);
                                ()
                            }
                            v130.l1.get().clone()
                        }
                    } else {
                        let v153: num_complex::Complex<f64> =
                            num_complex::Complex::new(1.0_f64, 0.0_f64);
                        let v158: Result<num_complex::Complex<f64>, std::string::String> =
                            Math::method16(
                                v0_1.clone(),
                                string("        s = mpmath.gamma(s)"),
                                Math::method3(v153 - v112.clone()),
                            );
                        let v162: Option<num_complex::Complex<f64>> = v158.ok();
                        let v198: Math::US0 =
                            defaultValue(Math::US0::US0_1, map(Math::method17(), v162));
                        let v203: f64 = f64::NAN;
                        let v205: f64 = f64::NAN;
                        let v207: num_complex::Complex<f64> = num_complex::Complex::new(v203, v205);
                        let v210: num_complex::Complex<f64> = match &v198 {
                            Math::US0::US0_0(v198_0_0) => match &v198 {
                                Math::US0::US0_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            _ => v207.clone(),
                        };
                        let v212: num_complex::Complex<f64> =
                            num_complex::Complex::new(3.141592653589793_f64, 0.0_f64);
                        let v214: num_complex::Complex<f64> = v212 * v112.clone();
                        let v216: num_complex::Complex<f64> =
                            num_complex::Complex::new(2.0_f64, 0.0_f64);
                        let v218: num_complex::Complex<f64> = v214 / v216;
                        let v220: num_complex::Complex<f64> = v218.sin();
                        let v223: f64 = 1.0_f64 - (v112.clone().re);
                        let v226: f64 = -v112.clone().im;
                        let v228: num_complex::Complex<f64> = num_complex::Complex::new(v223, v226);
                        let v630: num_complex::Complex<f64> = if (v228.clone().re) <= 1.0_f64 {
                            num_complex::Complex::new(0.0_f64, 0.0_f64)
                        } else {
                            println!("zeta / count: {:?} / s: {:?}", 2_i32, v228.clone());
                            if (v228.clone().re) > 1.0_f64 {
                                let v239: num_complex::Complex<f64> =
                                    num_complex::Complex::new(0.0_f64, 0.0_f64);
                                let v240: Array<i32> = new_init(&0_i32, 10000_i32);
                                let v241: LrcPtr<Math::Mut0> = LrcPtr::new(Math::Mut0 {
                                    l0: MutCell::new(0_i32),
                                });
                                while Math::method14(v241.clone()) {
                                    let v243: i32 = v241.l0.get().clone();
                                    v240.get_mut()[v243 as usize] = v243;
                                    {
                                        let v244: i32 = (v243) + 1_i32;
                                        v241.l0.set(v244);
                                        ()
                                    }
                                }
                                {
                                    let v245: i32 = get_Count(v240.clone());
                                    let v246: LrcPtr<Math::Mut2> = LrcPtr::new(Math::Mut2 {
                                        l0: MutCell::new(0_i32),
                                        l1: MutCell::new(v239),
                                    });
                                    while Math::method15(v245, v246.clone()) {
                                        let v248: i32 = v246.l0.get().clone();
                                        let v249: num_complex::Complex<f64> = v246.l1.get().clone();
                                        let v250: i32 = v240[v248].clone();
                                        let v252: num_complex::Complex<f64> =
                                            num_complex::Complex::new(1.0_f64, 0.0_f64);
                                        let v255: f64 = v250 as f64;
                                        let v259: num_complex::Complex<f64> =
                                            num_complex::Complex::new(v255, 0.0_f64);
                                        let v261: num_complex::Complex<f64> =
                                            num_complex::Complex::powc(v259, v228.clone());
                                        let v263: num_complex::Complex<f64> = v252 / v261;
                                        let v265: num_complex::Complex<f64> = v249 + v263;
                                        let v266: i32 = (v248) + 1_i32;
                                        v246.l0.set(v266);
                                        v246.l1.set(v265);
                                        ()
                                    }
                                    v246.l1.get().clone()
                                }
                            } else {
                                let v269: num_complex::Complex<f64> =
                                    num_complex::Complex::new(1.0_f64, 0.0_f64);
                                let v274: Result<num_complex::Complex<f64>, std::string::String> =
                                    Math::method16(
                                        v0_1.clone(),
                                        string("        s = mpmath.gamma(s)"),
                                        Math::method3(v269 - v228.clone()),
                                    );
                                let v278: Option<num_complex::Complex<f64>> = v274.ok();
                                let v314: Math::US0 =
                                    defaultValue(Math::US0::US0_1, map(Math::method17(), v278));
                                let v319: f64 = f64::NAN;
                                let v321: f64 = f64::NAN;
                                let v323: num_complex::Complex<f64> =
                                    num_complex::Complex::new(v319, v321);
                                let v326: num_complex::Complex<f64> = match &v314 {
                                    Math::US0::US0_0(v314_0_0) => match &v314 {
                                        Math::US0::US0_0(x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone(),
                                    _ => v323.clone(),
                                };
                                let v328: num_complex::Complex<f64> =
                                    num_complex::Complex::new(3.141592653589793_f64, 0.0_f64);
                                let v330: num_complex::Complex<f64> = v328 * v228.clone();
                                let v332: num_complex::Complex<f64> =
                                    num_complex::Complex::new(2.0_f64, 0.0_f64);
                                let v334: num_complex::Complex<f64> = v330 / v332;
                                let v336: num_complex::Complex<f64> = v334.sin();
                                let v339: f64 = 1.0_f64 - (v228.clone().re);
                                let v342: f64 = -v228.clone().im;
                                let v344: num_complex::Complex<f64> =
                                    num_complex::Complex::new(v339, v342);
                                let v614: num_complex::Complex<f64> = if (v344.clone().re)
                                    <= 1.0_f64
                                {
                                    num_complex::Complex::new(0.0_f64, 0.0_f64)
                                } else {
                                    println!("zeta / count: {:?} / s: {:?}", 3_i32, v344.clone());
                                    if (v344.clone().re) > 1.0_f64 {
                                        let v355: num_complex::Complex<f64> =
                                            num_complex::Complex::new(0.0_f64, 0.0_f64);
                                        let v356: Array<i32> = new_init(&0_i32, 10000_i32);
                                        let v357: LrcPtr<Math::Mut0> = LrcPtr::new(Math::Mut0 {
                                            l0: MutCell::new(0_i32),
                                        });
                                        while Math::method14(v357.clone()) {
                                            let v359: i32 = v357.l0.get().clone();
                                            v356.get_mut()[v359 as usize] = v359;
                                            {
                                                let v360: i32 = (v359) + 1_i32;
                                                v357.l0.set(v360);
                                                ()
                                            }
                                        }
                                        {
                                            let v361: i32 = get_Count(v356.clone());
                                            let v362: LrcPtr<Math::Mut2> =
                                                LrcPtr::new(Math::Mut2 {
                                                    l0: MutCell::new(0_i32),
                                                    l1: MutCell::new(v355),
                                                });
                                            while Math::method15(v361, v362.clone()) {
                                                let v364: i32 = v362.l0.get().clone();
                                                let v365: num_complex::Complex<f64> =
                                                    v362.l1.get().clone();
                                                let v366: i32 = v356[v364].clone();
                                                let v368: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(1.0_f64, 0.0_f64);
                                                let v371: f64 = v366 as f64;
                                                let v375: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(v371, 0.0_f64);
                                                let v377: num_complex::Complex<f64> =
                                                    num_complex::Complex::powc(v375, v344.clone());
                                                let v379: num_complex::Complex<f64> = v368 / v377;
                                                let v381: num_complex::Complex<f64> = v365 + v379;
                                                let v382: i32 = (v364) + 1_i32;
                                                v362.l0.set(v382);
                                                v362.l1.set(v381);
                                                ()
                                            }
                                            v362.l1.get().clone()
                                        }
                                    } else {
                                        let v385: num_complex::Complex<f64> =
                                            num_complex::Complex::new(1.0_f64, 0.0_f64);
                                        let v390: Result<
                                            num_complex::Complex<f64>,
                                            std::string::String,
                                        > = Math::method16(
                                            v0_1.clone(),
                                            string("        s = mpmath.gamma(s)"),
                                            Math::method3(v385 - v344.clone()),
                                        );
                                        let v394: Option<num_complex::Complex<f64>> = v390.ok();
                                        let v430: Math::US0 = defaultValue(
                                            Math::US0::US0_1,
                                            map(Math::method17(), v394),
                                        );
                                        let v435: f64 = f64::NAN;
                                        let v437: f64 = f64::NAN;
                                        let v439: num_complex::Complex<f64> =
                                            num_complex::Complex::new(v435, v437);
                                        let v442: num_complex::Complex<f64> = match &v430 {
                                            Math::US0::US0_0(v430_0_0) => match &v430 {
                                                Math::US0::US0_0(x) => x.clone(),
                                                _ => unreachable!(),
                                            }
                                            .clone(),
                                            _ => v439.clone(),
                                        };
                                        let v444: num_complex::Complex<f64> =
                                            num_complex::Complex::new(
                                                3.141592653589793_f64,
                                                0.0_f64,
                                            );
                                        let v446: num_complex::Complex<f64> = v444 * v344.clone();
                                        let v448: num_complex::Complex<f64> =
                                            num_complex::Complex::new(2.0_f64, 0.0_f64);
                                        let v450: num_complex::Complex<f64> = v446 / v448;
                                        let v452: num_complex::Complex<f64> = v450.sin();
                                        let v455: f64 = 1.0_f64 - (v344.clone().re);
                                        let v458: f64 = -v344.clone().im;
                                        let v460: num_complex::Complex<f64> =
                                            num_complex::Complex::new(v455, v458);
                                        let v598: num_complex::Complex<f64> = if (v460.clone().re)
                                            <= 1.0_f64
                                        {
                                            num_complex::Complex::new(0.0_f64, 0.0_f64)
                                        } else {
                                            println!(
                                                "zeta / count: {:?} / s: {:?}",
                                                4_i32,
                                                v460.clone()
                                            );
                                            if (v460.clone().re) > 1.0_f64 {
                                                let v471: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(0.0_f64, 0.0_f64);
                                                let v472: Array<i32> = new_init(&0_i32, 10000_i32);
                                                let v473: LrcPtr<Math::Mut0> =
                                                    LrcPtr::new(Math::Mut0 {
                                                        l0: MutCell::new(0_i32),
                                                    });
                                                while Math::method14(v473.clone()) {
                                                    let v475: i32 = v473.l0.get().clone();
                                                    v472.get_mut()[v475 as usize] = v475;
                                                    {
                                                        let v476: i32 = (v475) + 1_i32;
                                                        v473.l0.set(v476);
                                                        ()
                                                    }
                                                }
                                                {
                                                    let v477: i32 = get_Count(v472.clone());
                                                    let v478: LrcPtr<Math::Mut2> =
                                                        LrcPtr::new(Math::Mut2 {
                                                            l0: MutCell::new(0_i32),
                                                            l1: MutCell::new(v471),
                                                        });
                                                    while Math::method15(v477, v478.clone()) {
                                                        let v480: i32 = v478.l0.get().clone();
                                                        let v481: num_complex::Complex<f64> =
                                                            v478.l1.get().clone();
                                                        let v482: i32 = v472[v480].clone();
                                                        let v484: num_complex::Complex<f64> =
                                                            num_complex::Complex::new(
                                                                1.0_f64, 0.0_f64,
                                                            );
                                                        let v487: f64 = v482 as f64;
                                                        let v491: num_complex::Complex<f64> =
                                                            num_complex::Complex::new(
                                                                v487, 0.0_f64,
                                                            );
                                                        let v493: num_complex::Complex<f64> =
                                                            num_complex::Complex::powc(
                                                                v491,
                                                                v460.clone(),
                                                            );
                                                        let v495: num_complex::Complex<f64> =
                                                            v484 / v493;
                                                        let v497: num_complex::Complex<f64> =
                                                            v481 + v495;
                                                        let v498: i32 = (v480) + 1_i32;
                                                        v478.l0.set(v498);
                                                        v478.l1.set(v497);
                                                        ()
                                                    }
                                                    v478.l1.get().clone()
                                                }
                                            } else {
                                                let v501: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(1.0_f64, 0.0_f64);
                                                let v506: Result<
                                                    num_complex::Complex<f64>,
                                                    std::string::String,
                                                > = Math::method16(
                                                    v0_1,
                                                    string("        s = mpmath.gamma(s)"),
                                                    Math::method3(v501 - v460.clone()),
                                                );
                                                let v510: Option<num_complex::Complex<f64>> =
                                                    v506.ok();
                                                let v546: Math::US0 = defaultValue(
                                                    Math::US0::US0_1,
                                                    map(Math::method17(), v510),
                                                );
                                                let v551: f64 = f64::NAN;
                                                let v553: f64 = f64::NAN;
                                                let v555: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(v551, v553);
                                                let v558: num_complex::Complex<f64> = match &v546 {
                                                    Math::US0::US0_0(v546_0_0) => match &v546 {
                                                        Math::US0::US0_0(x) => x.clone(),
                                                        _ => unreachable!(),
                                                    }
                                                    .clone(),
                                                    _ => v555.clone(),
                                                };
                                                let v560: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(
                                                        3.141592653589793_f64,
                                                        0.0_f64,
                                                    );
                                                let v562: num_complex::Complex<f64> =
                                                    v560 * v460.clone();
                                                let v564: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(2.0_f64, 0.0_f64);
                                                let v566: num_complex::Complex<f64> = v562 / v564;
                                                let v568: num_complex::Complex<f64> = v566.sin();
                                                let v571: f64 = 1.0_f64 - (v460.clone().re);
                                                let v574: f64 = -v460.clone().im;
                                                let v576: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(v571, v574);
                                                let v582: num_complex::Complex<f64> =
                                                    if (v576.clone().re) <= 1.0_f64 {
                                                        num_complex::Complex::new(0.0_f64, 0.0_f64)
                                                    } else {
                                                        v576
                                                    };
                                                let v584: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(2.0_f64, 0.0_f64);
                                                let v586: num_complex::Complex<f64> =
                                                    num_complex::Complex::new(
                                                        3.141592653589793_f64,
                                                        0.0_f64,
                                                    );
                                                let v588: num_complex::Complex<f64> =
                                                    num_complex::Complex::powc(v586, v460.clone());
                                                let v590: num_complex::Complex<f64> = v584 * v588;
                                                let v592: num_complex::Complex<f64> = v590 * v568;
                                                let v594: num_complex::Complex<f64> = v592 * v558;
                                                v594 * v582
                                            }
                                        };
                                        let v600: num_complex::Complex<f64> =
                                            num_complex::Complex::new(2.0_f64, 0.0_f64);
                                        let v602: num_complex::Complex<f64> =
                                            num_complex::Complex::new(
                                                3.141592653589793_f64,
                                                0.0_f64,
                                            );
                                        let v604: num_complex::Complex<f64> =
                                            num_complex::Complex::powc(v602, v344.clone());
                                        let v606: num_complex::Complex<f64> = v600 * v604;
                                        let v608: num_complex::Complex<f64> = v606 * v452;
                                        let v610: num_complex::Complex<f64> = v608 * v442;
                                        v610 * v598
                                    }
                                };
                                let v616: num_complex::Complex<f64> =
                                    num_complex::Complex::new(2.0_f64, 0.0_f64);
                                let v618: num_complex::Complex<f64> =
                                    num_complex::Complex::new(3.141592653589793_f64, 0.0_f64);
                                let v620: num_complex::Complex<f64> =
                                    num_complex::Complex::powc(v618, v228.clone());
                                let v622: num_complex::Complex<f64> = v616 * v620;
                                let v624: num_complex::Complex<f64> = v622 * v336;
                                let v626: num_complex::Complex<f64> = v624 * v326;
                                v626 * v614
                            }
                        };
                        let v632: num_complex::Complex<f64> =
                            num_complex::Complex::new(2.0_f64, 0.0_f64);
                        let v634: num_complex::Complex<f64> =
                            num_complex::Complex::new(3.141592653589793_f64, 0.0_f64);
                        let v636: num_complex::Complex<f64> =
                            num_complex::Complex::powc(v634, v112.clone());
                        let v638: num_complex::Complex<f64> = v632 * v636;
                        let v640: num_complex::Complex<f64> = v638 * v220;
                        let v642: num_complex::Complex<f64> = v640 * v210;
                        v642 * v630
                    }
                };
                let v648: num_complex::Complex<f64> = num_complex::Complex::new(2.0_f64, 0.0_f64);
                let v650: num_complex::Complex<f64> =
                    num_complex::Complex::new(3.141592653589793_f64, 0.0_f64);
                let v652: num_complex::Complex<f64> =
                    num_complex::Complex::powc(v650, v1_1.clone());
                let v654: num_complex::Complex<f64> = v648 * v652;
                let v656: num_complex::Complex<f64> = v654 * v104;
                let v658: num_complex::Complex<f64> = v656 * v94;
                v658 * v646
            }
        }
        pub fn method18(v0_1: bool) -> bool {
            v0_1
        }
        pub fn method20() -> string {
            string("")
        }
        pub fn closure2(v0_1: LrcPtr<Math::Mut3>, v1_1: string, unitVar: ()) {
            let v4: string = append((v0_1.l0.get().clone()), (v1_1));
            v0_1.l0.set(v4);
            ()
        }
        pub fn method19(v0_1: string, v1_1: f64) -> string {
            let v3: LrcPtr<Math::Mut3> = LrcPtr::new(Math::Mut3 {
                l0: MutCell::new(Math::method20()),
            });
            let v17: () = {
                Math::closure2(v3.clone(), string("{ "), ());
                ()
            };
            let v36: () = {
                Math::closure2(v3.clone(), string("name"), ());
                ()
            };
            let v55: () = {
                Math::closure2(v3.clone(), string(" = "), ());
                ()
            };
            let v72: () = {
                Math::closure2(v3.clone(), v0_1, ());
                ()
            };
            let v91: () = {
                Math::closure2(v3.clone(), string("; "), ());
                ()
            };
            let v110: () = {
                Math::closure2(v3.clone(), string("expected"), ());
                ()
            };
            let v127: () = {
                Math::closure2(v3.clone(), string(" = "), ());
                ()
            };
            let v147: () = {
                Math::closure2(v3.clone(), sprintf!("{:+.6}", v1_1), ());
                ()
            };
            let v166: () = {
                Math::closure2(v3.clone(), string(" }"), ());
                ()
            };
            v3.l0.get().clone()
        }
        pub fn closure3(v0_1: string, unitVar: ()) {
            printfn!("{0}", v0_1);
        }
        pub fn method1(v0_1: pyo3::Python) {
            let v5: Array<(num_complex::Complex<f64>, f64)> = new_array(&[
                (
                    num_complex::Complex::new(2.0_f64, 0.0_f64),
                    1.6449340668482264_f64,
                ),
                (
                    num_complex::Complex::new(-1.0_f64, 0.0_f64),
                    -0.08333333333333333_f64,
                ),
            ]);
            let v6: i32 = get_Count(v5.clone());
            let v7: LrcPtr<Math::Mut0> = LrcPtr::new(Math::Mut0 {
                l0: MutCell::new(0_i32),
            });
            while Math::method2(v6, v7.clone()) {
                let v9: i32 = v7.l0.get().clone();
                let patternInput: (num_complex::Complex<f64>, f64) = v5[v9].clone();
                let v10: num_complex::Complex<f64> = patternInput.0.clone();
                let v14: Result<num_complex::Complex<f64>, std::string::String> = Math::method4(
                    v0_1.clone(),
                    string("        s = mpmath.zeta(s)"),
                    Math::method3(v10.clone()),
                );
                let v15: num_complex::Complex<f64> = Math::method13(v0_1.clone(), v10);
                let v19: Option<num_complex::Complex<f64>> = v14.ok();
                let v55: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v19));
                let v60: f64 = f64::NAN;
                let v62: f64 = f64::NAN;
                let v64: num_complex::Complex<f64> = num_complex::Complex::new(v60, v62);
                let v67: num_complex::Complex<f64> = match &v55 {
                    Math::US0::US0_0(v55_0_0) => match &v55 {
                        Math::US0::US0_0(x) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                    _ => v64.clone(),
                };
                let v70: bool = (v67.clone().im) == 0.0_f64;
                let v72: bool = if v70 { true } else { Math::method18(v70) };
                let v75: string = Math::method19(string("__assert_eq"), 0.0_f64);
                let v79: () = {
                    Math::closure3(v75.clone(), ());
                    ()
                };
                if (v72) == false {
                    panic!("{}", v75,);
                }
                {
                    let v84: f64 = (v67.re) - (patternInput.1.clone());
                    let v85: f64 = -v84;
                    let v88: bool = (if (v84) >= (v85) { v84 } else { v85 }) < 0.0001_f64;
                    let v90: bool = if v88 { true } else { Math::method18(v88) };
                    let v93: string = Math::method19(string("__assert_lt"), 0.0001_f64);
                    let v97: () = {
                        Math::closure3(v93.clone(), ());
                        ()
                    };
                    if (v90) == false {
                        panic!("{}", v93,);
                    }
                    {
                        let v100: i32 = (v9) + 1_i32;
                        v7.l0.set(v100);
                        ()
                    }
                }
            }
            ()
        }
        pub fn method21(v0_1: Result<(), pyo3::PyErr>) -> Result<(), pyo3::PyErr> {
            v0_1
        }
        pub fn method0() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method1(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method23(v0_1: pyo3::Python) {
            let v2: num_complex::Complex<f64> = num_complex::Complex::new(2.0_f64, -2.0_f64);
            let v5: Result<num_complex::Complex<f64>, std::string::String> = Math::method4(
                v0_1.clone(),
                string("        s = mpmath.zeta(s)"),
                Math::method3(v2.clone()),
            );
            let v6: num_complex::Complex<f64> = Math::method13(v0_1, v2);
            let v10: Option<num_complex::Complex<f64>> = v5.ok();
            let v46: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v10));
            let v51: f64 = f64::NAN;
            let v53: f64 = f64::NAN;
            let v55: num_complex::Complex<f64> = num_complex::Complex::new(v51, v53);
            let v58: num_complex::Complex<f64> = match &v46 {
                Math::US0::US0_0(v46_0_0) => match &v46 {
                    Math::US0::US0_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => v55.clone(),
            };
            let v61: f64 = (v58.clone().re) - 0.8673_f64;
            let v62: f64 = -v61;
            let v65: bool = (if (v61) >= (v62) { v61 } else { v62 }) < 0.001_f64;
            let v67: bool = if v65 { true } else { Math::method18(v65) };
            let v70: string = Math::method19(string("__assert_lt"), 0.001_f64);
            let v74: () = {
                Math::closure3(v70.clone(), ());
                ()
            };
            if (v67) == false {
                panic!("{}", v70,);
            }
            {
                let v79: f64 = (v58.im) - 0.275_f64;
                let v80: f64 = -v79;
                let v83: bool = (if (v79) >= (v80) { v79 } else { v80 }) < 0.001_f64;
                let v85: bool = if v83 { true } else { Math::method18(v83) };
                let v87: string = Math::method19(string("__assert_lt"), 0.001_f64);
                let v91: () = {
                    Math::closure3(v87.clone(), ());
                    ()
                };
                if (v85) == false {
                    panic!("{}", v87,);
                }
            }
        }
        pub fn method22() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method23(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method26() -> LrcPtr<Math::UH0> {
            LrcPtr::new(Math::UH0::UH0_1(-2.0_f64,
                                         LrcPtr::new(Math::UH0::UH0_1(-4.0_f64,
                                                                      LrcPtr::new(Math::UH0::UH0_1(-6.0_f64,
                                                                                                   LrcPtr::new(Math::UH0::UH0_1(-8.0_f64,
                                                                                                                                LrcPtr::new(Math::UH0::UH0_1(-10.0_f64,
                                                                                                                                                             LrcPtr::new(Math::UH0::UH0_1(-12.0_f64,
                                                                                                                                                                                          LrcPtr::new(Math::UH0::UH0_1(-14.0_f64,
                                                                                                                                                                                                                       LrcPtr::new(Math::UH0::UH0_1(-16.0_f64,
                                                                                                                                                                                                                                                    LrcPtr::new(Math::UH0::UH0_1(-18.0_f64,
                                                                                                                                                                                                                                                                                 LrcPtr::new(Math::UH0::UH0_1(-20.0_f64,
                                                                                                                                                                                                                                                                                                              LrcPtr::new(Math::UH0::UH0_1(-22.0_f64,
                                                                                                                                                                                                                                                                                                                                           LrcPtr::new(Math::UH0::UH0_1(-24.0_f64,
                                                                                                                                                                                                                                                                                                                                                                        LrcPtr::new(Math::UH0::UH0_1(-26.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                     LrcPtr::new(Math::UH0::UH0_1(-28.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                  LrcPtr::new(Math::UH0::UH0_1(-30.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                               LrcPtr::new(Math::UH0::UH0_1(-32.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            LrcPtr::new(Math::UH0::UH0_1(-34.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         LrcPtr::new(Math::UH0::UH0_1(-36.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      LrcPtr::new(Math::UH0::UH0_1(-38.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   LrcPtr::new(Math::UH0::UH0_1(-40.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                LrcPtr::new(Math::UH0::UH0_0)))))))))))))))))))))))))))))))))))))))))
        }
        pub fn method27(v0_1: pyo3::Python, v1_1: LrcPtr<Math::UH0>) {
            let v0_1: MutCell<pyo3::Python> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<Math::UH0>> = MutCell::new(v1_1.clone());
            '_method27: loop {
                break '_method27 (match v1_1.get().clone().as_ref() {
                    Math::UH0::UH0_0 => (),
                    Math::UH0::UH0_1(v1_1_1_0, v1_1_1_1) => {
                        let v5: num_complex::Complex<f64> = num_complex::Complex::new(
                            match v1_1.get().clone().as_ref() {
                                Math::UH0::UH0_1(x, _) => x.clone(),
                                _ => unreachable!(),
                            },
                            0.0_f64,
                        );
                        let v8: Result<num_complex::Complex<f64>, std::string::String> =
                            Math::method4(
                                v0_1.get().clone(),
                                string("        s = mpmath.zeta(s)"),
                                Math::method3(v5.clone()),
                            );
                        let v9: num_complex::Complex<f64> = Math::method13(v0_1.get().clone(), v5);
                        let v13: Option<num_complex::Complex<f64>> = v8.ok();
                        let v49: Math::US0 =
                            defaultValue(Math::US0::US0_1, map(Math::method17(), v13));
                        let v54: f64 = f64::NAN;
                        let v56: f64 = f64::NAN;
                        let v58: num_complex::Complex<f64> = num_complex::Complex::new(v54, v56);
                        let v61: num_complex::Complex<f64> = match &v49 {
                            Math::US0::US0_0(v49_0_0) => match &v49 {
                                Math::US0::US0_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            _ => v58.clone(),
                        };
                        let v64: bool = (v61.clone().re) == 0.0_f64;
                        let v66: bool = if v64 { true } else { Math::method18(v64) };
                        let v69: string = Math::method19(string("__assert_eq"), 0.0_f64);
                        let v73: () = {
                            Math::closure3(v69.clone(), ());
                            ()
                        };
                        if (v66) == false {
                            panic!("{}", v69,);
                        }
                        {
                            let v78: bool = (v61.im) == 0.0_f64;
                            let v80: bool = if v78 { true } else { Math::method18(v78) };
                            let v82: string = Math::method19(string("__assert_eq"), 0.0_f64);
                            let v86: () = {
                                Math::closure3(v82.clone(), ());
                                ()
                            };
                            if (v80) == false {
                                panic!("{}", v82,);
                            }
                            {
                                let v0_1_temp: pyo3::Python = v0_1.get().clone();
                                let v1_1_temp: LrcPtr<Math::UH0> =
                                    match v1_1.get().clone().as_ref() {
                                        Math::UH0::UH0_1(_, x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone();
                                v0_1.set(v0_1_temp);
                                v1_1.set(v1_1_temp);
                                continue '_method27;
                            }
                        }
                    }
                });
            }
        }
        pub fn method25(v0_1: pyo3::Python) {
            Math::method27(v0_1, Math::method26());
        }
        pub fn method24() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method25(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method29(v0_1: pyo3::Python) {
            let v13: Array<num_complex::Complex<f64>> = new_array(&[
                num_complex::Complex::new(0.5_f64, 14.134725_f64),
                num_complex::Complex::new(0.5_f64, 21.02204_f64),
                num_complex::Complex::new(0.5_f64, 25.010857_f64),
                num_complex::Complex::new(0.5_f64, 30.424876_f64),
                num_complex::Complex::new(0.5_f64, 32.935062_f64),
                num_complex::Complex::new(0.5_f64, 37.586178_f64),
            ]);
            let v14: i32 = get_Count(v13.clone());
            let v15: LrcPtr<Math::Mut0> = LrcPtr::new(Math::Mut0 {
                l0: MutCell::new(0_i32),
            });
            while Math::method2(v14, v15.clone()) {
                let v17: i32 = v15.l0.get().clone();
                let v18: num_complex::Complex<f64> = v13[v17].clone();
                let v21: Result<num_complex::Complex<f64>, std::string::String> = Math::method4(
                    v0_1.clone(),
                    string("        s = mpmath.zeta(s)"),
                    Math::method3(v18.clone()),
                );
                let v22: num_complex::Complex<f64> = Math::method13(v0_1.clone(), v18);
                let v26: Option<num_complex::Complex<f64>> = v21.ok();
                let v62: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v26));
                let v67: f64 = f64::NAN;
                let v69: f64 = f64::NAN;
                let v71: num_complex::Complex<f64> = num_complex::Complex::new(v67, v69);
                let v74: num_complex::Complex<f64> = match &v62 {
                    Math::US0::US0_0(v62_0_0) => match &v62 {
                        Math::US0::US0_0(x) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                    _ => v71.clone(),
                };
                let v76: f64 = v74.clone().re;
                let v77: f64 = -v76;
                let v80: bool = (if (v76) >= (v77) { v76 } else { v77 }) < 0.0001_f64;
                let v82: bool = if v80 { true } else { Math::method18(v80) };
                let v85: string = Math::method19(string("__assert_lt"), 0.0001_f64);
                let v89: () = {
                    Math::closure3(v85.clone(), ());
                    ()
                };
                if (v82) == false {
                    panic!("{}", v85,);
                }
                {
                    let v93: f64 = v74.im;
                    let v94: f64 = -v93;
                    let v97: bool = (if (v93) >= (v94) { v93 } else { v94 }) < 0.0001_f64;
                    let v99: bool = if v97 { true } else { Math::method18(v97) };
                    let v101: string = Math::method19(string("__assert_lt"), 0.0001_f64);
                    let v105: () = {
                        Math::closure3(v101.clone(), ());
                        ()
                    };
                    if (v99) == false {
                        panic!("{}", v101,);
                    }
                    {
                        let v108: i32 = (v17) + 1_i32;
                        v15.l0.set(v108);
                        ()
                    }
                }
            }
            ()
        }
        pub fn method28() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method29(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method31(v0_1: pyo3::Python) {
            let v1_1: Array<f64> = new_array(&[
                2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64, 10.0_f64, 20.0_f64, 50.0_f64,
            ]);
            let v2: i32 = get_Count(v1_1.clone());
            let v3: LrcPtr<Math::Mut0> = LrcPtr::new(Math::Mut0 {
                l0: MutCell::new(0_i32),
            });
            while Math::method2(v2, v3.clone()) {
                let v5: i32 = v3.l0.get().clone();
                let v6: f64 = v1_1[v5].clone();
                let v8: num_complex::Complex<f64> = num_complex::Complex::new(v6, 0.0_f64);
                let v11: Result<num_complex::Complex<f64>, std::string::String> = Math::method4(
                    v0_1.clone(),
                    string("        s = mpmath.zeta(s)"),
                    Math::method3(v8.clone()),
                );
                let v12: num_complex::Complex<f64> = Math::method13(v0_1.clone(), v8);
                let v16: Option<num_complex::Complex<f64>> = v11.ok();
                let v52: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v16));
                let v57: f64 = f64::NAN;
                let v59: f64 = f64::NAN;
                let v61: num_complex::Complex<f64> = num_complex::Complex::new(v57, v59);
                let v64: num_complex::Complex<f64> = match &v52 {
                    Math::US0::US0_0(v52_0_0) => match &v52 {
                        Math::US0::US0_0(x) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                    _ => v61.clone(),
                };
                let v67: bool = (v64.clone().re) > 0.0_f64;
                let v69: bool = if v67 { true } else { Math::method18(v67) };
                let v72: string = Math::method19(string("__assert_gt"), 0.0_f64);
                let v76: () = {
                    Math::closure3(v72.clone(), ());
                    ()
                };
                if (v69) == false {
                    panic!("{}", v72,);
                }
                {
                    let v81: bool = (v64.im) == 0.0_f64;
                    let v83: bool = if v81 { true } else { Math::method18(v81) };
                    let v86: string = Math::method19(string("__assert_eq"), 0.0_f64);
                    let v90: () = {
                        Math::closure3(v86.clone(), ());
                        ()
                    };
                    if (v83) == false {
                        panic!("{}", v86,);
                    }
                    {
                        let v93: i32 = (v5) + 1_i32;
                        v3.l0.set(v93);
                        ()
                    }
                }
            }
            ()
        }
        pub fn method30() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method31(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method33(v0_1: pyo3::Python) {
            let v2: num_complex::Complex<f64> = num_complex::Complex::new(1.0_f64, 0.0_f64);
            let v5: Result<num_complex::Complex<f64>, std::string::String> = Math::method4(
                v0_1.clone(),
                string("        s = mpmath.zeta(s)"),
                Math::method3(v2.clone()),
            );
            let v6: num_complex::Complex<f64> = Math::method13(v0_1, v2);
            let v10: Option<num_complex::Complex<f64>> = v5.ok();
            let v46: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v10));
            let v51: f64 = f64::NAN;
            let v53: f64 = f64::NAN;
            let v55: num_complex::Complex<f64> = num_complex::Complex::new(v51, v53);
            let v58: num_complex::Complex<f64> = match &v46 {
                Math::US0::US0_0(v46_0_0) => match &v46 {
                    Math::US0::US0_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => v55.clone(),
            };
            let v61: bool = (v58.clone().re) == (f64::INFINITY);
            let v63: bool = if v61 { true } else { Math::method18(v61) };
            let v66: string = Math::method19(string("__assert_eq"), f64::INFINITY);
            let v70: () = {
                Math::closure3(v66.clone(), ());
                ()
            };
            if (v63) == false {
                panic!("{}", v66,);
            }
            {
                let v75: bool = (v58.im) == 0.0_f64;
                let v77: bool = if v75 { true } else { Math::method18(v75) };
                let v79: string = Math::method19(string("__assert_eq"), 0.0_f64);
                let v83: () = {
                    Math::closure3(v79.clone(), ());
                    ()
                };
                if (v77) == false {
                    panic!("{}", v79,);
                }
            }
        }
        pub fn method32() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method33(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method35(v0_1: pyo3::Python) {
            let v2: num_complex::Complex<f64> = num_complex::Complex::new(2.0_f64, 10.0_f64);
            let v5: Result<num_complex::Complex<f64>, std::string::String> = Math::method4(
                v0_1.clone(),
                string("        s = mpmath.zeta(s)"),
                Math::method3(v2.clone()),
            );
            let v6: num_complex::Complex<f64> = Math::method13(v0_1.clone(), v2.clone());
            let v10: Option<num_complex::Complex<f64>> = v5.ok();
            let v46: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v10));
            let v51: f64 = f64::NAN;
            let v53: f64 = f64::NAN;
            let v55: num_complex::Complex<f64> = num_complex::Complex::new(v51, v53);
            let v58: num_complex::Complex<f64> = match &v46 {
                Math::US0::US0_0(v46_0_0) => match &v46 {
                    Math::US0::US0_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => v55.clone(),
            };
            let v60: f64 = v2.clone().re;
            let v63: f64 = -v2.im;
            let v65: num_complex::Complex<f64> = num_complex::Complex::new(v60, v63);
            let v68: Result<num_complex::Complex<f64>, std::string::String> = Math::method4(
                v0_1.clone(),
                string("        s = mpmath.zeta(s)"),
                Math::method3(v65.clone()),
            );
            let v69: num_complex::Complex<f64> = Math::method13(v0_1, v65);
            let v73: Option<num_complex::Complex<f64>> = v68.ok();
            let v109: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v73));
            let v114: f64 = f64::NAN;
            let v116: f64 = f64::NAN;
            let v118: num_complex::Complex<f64> = num_complex::Complex::new(v114, v116);
            let v121: num_complex::Complex<f64> = match &v109 {
                Math::US0::US0_0(v109_0_0) => match &v109 {
                    Math::US0::US0_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => v118.clone(),
            };
            let v123: num_complex::Complex<f64> = v121.conj();
            let v125: f64 = v58.clone().re;
            let v127: f64 = v123.clone().re;
            let v128: bool = (v125) == (v127);
            let v130: bool = if v128 { true } else { Math::method18(v128) };
            let v132: string = Math::method19(string("__assert_eq"), v127);
            let v136: () = {
                Math::closure3(v132.clone(), ());
                ()
            };
            if (v130) == false {
                panic!("{}", v132,);
            }
            {
                let v140: f64 = v58.im;
                let v142: f64 = v123.im;
                let v143: bool = (v140) == (v142);
                let v145: bool = if v143 { true } else { Math::method18(v143) };
                let v146: string = Math::method19(string("__assert_eq"), v142);
                let v150: () = {
                    Math::closure3(v146.clone(), ());
                    ()
                };
                if (v145) == false {
                    panic!("{}", v146,);
                }
            }
        }
        pub fn method34() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method35(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method37(v0_1: pyo3::Python) {
            let v2: num_complex::Complex<f64> = num_complex::Complex::new(0.01_f64, 0.01_f64);
            let v5: Result<num_complex::Complex<f64>, std::string::String> = Math::method4(
                v0_1.clone(),
                string("        s = mpmath.zeta(s)"),
                Math::method3(v2.clone()),
            );
            let v6: num_complex::Complex<f64> = Math::method13(v0_1, v2);
            let v10: Option<num_complex::Complex<f64>> = v5.ok();
            let v46: Math::US0 = defaultValue(Math::US0::US0_1, map(Math::method17(), v10));
            let v51: f64 = f64::NAN;
            let v53: f64 = f64::NAN;
            let v55: num_complex::Complex<f64> = num_complex::Complex::new(v51, v53);
            let v58: num_complex::Complex<f64> = match &v46 {
                Math::US0::US0_0(v46_0_0) => match &v46 {
                    Math::US0::US0_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => v55.clone(),
            };
            let v61: bool = (v58.clone().re) < (f64::INFINITY);
            let v63: bool = if v61 { true } else { Math::method18(v61) };
            let v66: string = Math::method19(string("__assert_lt"), f64::INFINITY);
            let v70: () = {
                Math::closure3(v66.clone(), ());
                ()
            };
            if (v63) == false {
                panic!("{}", v66,);
            }
            {
                let v75: bool = (v58.im) < (f64::INFINITY);
                let v77: bool = if v75 { true } else { Math::method18(v75) };
                let v79: string = Math::method19(string("__assert_lt"), f64::INFINITY);
                let v83: () = {
                    Math::closure3(v79.clone(), ());
                    ()
                };
                if (v77) == false {
                    panic!("{}", v79,);
                }
            }
        }
        pub fn method36() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method37(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method40() -> LrcPtr<Math::UH0> {
            LrcPtr::new(Math::UH0::UH0_1(
                10.0_f64,
                LrcPtr::new(Math::UH0::UH0_1(
                    20.0_f64,
                    LrcPtr::new(Math::UH0::UH0_1(
                        30.0_f64,
                        LrcPtr::new(Math::UH0::UH0_1(
                            40.0_f64,
                            LrcPtr::new(Math::UH0::UH0_1(
                                50.0_f64,
                                LrcPtr::new(Math::UH0::UH0_1(
                                    60.0_f64,
                                    LrcPtr::new(Math::UH0::UH0_1(
                                        70.0_f64,
                                        LrcPtr::new(Math::UH0::UH0_1(
                                            80.0_f64,
                                            LrcPtr::new(Math::UH0::UH0_1(
                                                90.0_f64,
                                                LrcPtr::new(Math::UH0::UH0_1(
                                                    100.0_f64,
                                                    LrcPtr::new(Math::UH0::UH0_0),
                                                )),
                                            )),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        }
        pub fn method41(v0_1: pyo3::Python, v1_1: LrcPtr<Math::UH0>) {
            let v0_1: MutCell<pyo3::Python> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<Math::UH0>> = MutCell::new(v1_1.clone());
            '_method41: loop {
                break '_method41 (match v1_1.get().clone().as_ref() {
                    Math::UH0::UH0_0 => (),
                    Math::UH0::UH0_1(v1_1_1_0, v1_1_1_1) => {
                        let v5: num_complex::Complex<f64> = num_complex::Complex::new(
                            0.0_f64,
                            match v1_1.get().clone().as_ref() {
                                Math::UH0::UH0_1(x, _) => x.clone(),
                                _ => unreachable!(),
                            },
                        );
                        let v8: Result<num_complex::Complex<f64>, std::string::String> =
                            Math::method4(
                                v0_1.get().clone(),
                                string("        s = mpmath.zeta(s)"),
                                Math::method3(v5.clone()),
                            );
                        let v9: num_complex::Complex<f64> = Math::method13(v0_1.get().clone(), v5);
                        let v13: Option<num_complex::Complex<f64>> = v8.ok();
                        let v49: Math::US0 =
                            defaultValue(Math::US0::US0_1, map(Math::method17(), v13));
                        let v54: f64 = f64::NAN;
                        let v56: f64 = f64::NAN;
                        let v58: num_complex::Complex<f64> = num_complex::Complex::new(v54, v56);
                        let v61: num_complex::Complex<f64> = match &v49 {
                            Math::US0::US0_0(v49_0_0) => match &v49 {
                                Math::US0::US0_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            _ => v58.clone(),
                        };
                        let v65: bool = (v61.clone().re) != 0.0_f64;
                        let v69: bool = if v65 { true } else { Math::method18(v65) };
                        let v72: string = Math::method19(string("__assert_ne"), 0.0_f64);
                        let v76: () = {
                            Math::closure3(v72.clone(), ());
                            ()
                        };
                        if (v69) == false {
                            panic!("{}", v72,);
                        }
                        {
                            let v82: bool = (v61.im) != 0.0_f64;
                            let v86: bool = if v82 { true } else { Math::method18(v82) };
                            let v88: string = Math::method19(string("__assert_ne"), 0.0_f64);
                            let v92: () = {
                                Math::closure3(v88.clone(), ());
                                ()
                            };
                            if (v86) == false {
                                panic!("{}", v88,);
                            }
                            {
                                let v0_1_temp: pyo3::Python = v0_1.get().clone();
                                let v1_1_temp: LrcPtr<Math::UH0> =
                                    match v1_1.get().clone().as_ref() {
                                        Math::UH0::UH0_1(_, x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone();
                                v0_1.set(v0_1_temp);
                                v1_1.set(v1_1_temp);
                                continue '_method41;
                            }
                        }
                    }
                });
            }
        }
        pub fn method39(v0_1: pyo3::Python) {
            Math::method41(v0_1, Math::method40());
        }
        pub fn method38() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method39(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method44() -> LrcPtr<Math::UH1> {
            LrcPtr::new(Math::UH1::UH1_1(
                num_complex::Complex::new(0.5_f64, 14.134725_f64),
                LrcPtr::new(Math::UH1::UH1_1(
                    num_complex::Complex::new(0.75_f64, 20.5_f64),
                    LrcPtr::new(Math::UH1::UH1_1(
                        num_complex::Complex::new(1.25_f64, 30.1_f64),
                        LrcPtr::new(Math::UH1::UH1_1(
                            num_complex::Complex::new(0.25_f64, 40.0_f64),
                            LrcPtr::new(Math::UH1::UH1_1(
                                num_complex::Complex::new(1.0_f64, 50.0_f64),
                                LrcPtr::new(Math::UH1::UH1_0),
                            )),
                        )),
                    )),
                )),
            ))
        }
        pub fn method45(v0_1: pyo3::Python, v1_1: LrcPtr<Math::UH1>) {
            let v0_1: MutCell<pyo3::Python> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<Math::UH1>> = MutCell::new(v1_1.clone());
            '_method45: loop {
                break '_method45 (match v1_1.get().clone().as_ref() {
                    Math::UH1::UH1_0 => (),
                    Math::UH1::UH1_1(v1_1_1_0, v1_1_1_1) => {
                        let v2: num_complex::Complex<f64> = match v1_1.get().clone().as_ref() {
                            Math::UH1::UH1_1(x, _) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone();
                        let v6: Result<num_complex::Complex<f64>, std::string::String> =
                            Math::method4(
                                v0_1.get().clone(),
                                string("        s = mpmath.zeta(s)"),
                                Math::method3(v2.clone()),
                            );
                        let v7: num_complex::Complex<f64> = Math::method13(v0_1.get().clone(), v2);
                        let v11: Option<num_complex::Complex<f64>> = v6.ok();
                        let v47: Math::US0 =
                            defaultValue(Math::US0::US0_1, map(Math::method17(), v11));
                        let v52: f64 = f64::NAN;
                        let v54: f64 = f64::NAN;
                        let v56: num_complex::Complex<f64> = num_complex::Complex::new(v52, v54);
                        let v59: num_complex::Complex<f64> = match &v47 {
                            Math::US0::US0_0(v47_0_0) => match &v47 {
                                Math::US0::US0_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            _ => v56.clone(),
                        };
                        let v63: bool = (v59.clone().re) != 0.0_f64;
                        let v67: bool = if v63 { true } else { Math::method18(v63) };
                        let v70: string = Math::method19(string("__assert_ne"), 0.0_f64);
                        let v74: () = {
                            Math::closure3(v70.clone(), ());
                            ()
                        };
                        if (v67) == false {
                            panic!("{}", v70,);
                        }
                        {
                            let v80: bool = (v59.im) != 0.0_f64;
                            let v84: bool = if v80 { true } else { Math::method18(v80) };
                            let v86: string = Math::method19(string("__assert_ne"), 0.0_f64);
                            let v90: () = {
                                Math::closure3(v86.clone(), ());
                                ()
                            };
                            if (v84) == false {
                                panic!("{}", v86,);
                            }
                            {
                                let v0_1_temp: pyo3::Python = v0_1.get().clone();
                                let v1_1_temp: LrcPtr<Math::UH1> =
                                    match v1_1.get().clone().as_ref() {
                                        Math::UH1::UH1_1(_, x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone();
                                v0_1.set(v0_1_temp);
                                v1_1.set(v1_1_temp);
                                continue '_method45;
                            }
                        }
                    }
                });
            }
        }
        pub fn method43(v0_1: pyo3::Python) {
            Math::method45(v0_1, Math::method44());
        }
        pub fn method42() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method43(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method48() -> LrcPtr<Math::UH1> {
            LrcPtr::new(Math::UH1::UH1_1(
                num_complex::Complex::new(3.0_f64, 4.0_f64),
                LrcPtr::new(Math::UH1::UH1_1(
                    num_complex::Complex::new(2.5_f64, -3.5_f64),
                    LrcPtr::new(Math::UH1::UH1_1(
                        num_complex::Complex::new(1.5_f64, 2.5_f64),
                        LrcPtr::new(Math::UH1::UH1_1(
                            num_complex::Complex::new(0.5_f64, 14.134725_f64),
                            LrcPtr::new(Math::UH1::UH1_0),
                        )),
                    )),
                )),
            ))
        }
        pub fn method49(v0_1: pyo3::Python, v1_1: LrcPtr<Math::UH1>) {
            let v0_1: MutCell<pyo3::Python> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<Math::UH1>> = MutCell::new(v1_1.clone());
            '_method49: loop {
                break '_method49 (match v1_1.get().clone().as_ref() {
                    Math::UH1::UH1_0 => (),
                    Math::UH1::UH1_1(v1_1_1_0, v1_1_1_1) => {
                        let v2: num_complex::Complex<f64> = match v1_1.get().clone().as_ref() {
                            Math::UH1::UH1_1(x, _) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone();
                        let v6: Result<num_complex::Complex<f64>, std::string::String> =
                            Math::method4(
                                v0_1.get().clone(),
                                string("        s = mpmath.zeta(s)"),
                                Math::method3(v2.clone()),
                            );
                        let v7: num_complex::Complex<f64> =
                            Math::method13(v0_1.get().clone(), v2.clone());
                        let v11: Option<num_complex::Complex<f64>> = v6.ok();
                        let v47: Math::US0 =
                            defaultValue(Math::US0::US0_1, map(Math::method17(), v11));
                        let v52: f64 = f64::NAN;
                        let v54: f64 = f64::NAN;
                        let v56: num_complex::Complex<f64> = num_complex::Complex::new(v52, v54);
                        let v59: num_complex::Complex<f64> = match &v47 {
                            Math::US0::US0_0(v47_0_0) => match &v47 {
                                Math::US0::US0_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            _ => v56.clone(),
                        };
                        let v61: num_complex::Complex<f64> =
                            num_complex::Complex::new(2.0_f64, 0.0_f64);
                        let v63: num_complex::Complex<f64> =
                            num_complex::Complex::powc(v61, v2.clone());
                        let v65: num_complex::Complex<f64> =
                            num_complex::Complex::new(3.141592653589793_f64, 0.0_f64);
                        let v67: num_complex::Complex<f64> =
                            num_complex::Complex::new(1.0_f64, 0.0_f64);
                        let v69: num_complex::Complex<f64> = v2.clone() - v67;
                        let v71: num_complex::Complex<f64> = num_complex::Complex::powc(v65, v69);
                        let v73: num_complex::Complex<f64> = v63 * v71;
                        let v75: num_complex::Complex<f64> =
                            num_complex::Complex::new(3.141592653589793_f64, 0.0_f64);
                        let v77: num_complex::Complex<f64> = v75 * v2.clone();
                        let v79: num_complex::Complex<f64> =
                            num_complex::Complex::new(2.0_f64, 0.0_f64);
                        let v81: num_complex::Complex<f64> = v77 / v79;
                        let v83: num_complex::Complex<f64> = v81.sin();
                        let v85: num_complex::Complex<f64> = v73 * v83;
                        let v87: num_complex::Complex<f64> =
                            num_complex::Complex::new(1.0_f64, 0.0_f64);
                        let v92: Result<num_complex::Complex<f64>, std::string::String> =
                            Math::method16(
                                v0_1.get().clone(),
                                string("        s = mpmath.gamma(s)"),
                                Math::method3(v87 - v2.clone()),
                            );
                        let v96: Option<num_complex::Complex<f64>> = v92.ok();
                        let v132: Math::US0 =
                            defaultValue(Math::US0::US0_1, map(Math::method17(), v96));
                        let v137: f64 = f64::NAN;
                        let v139: f64 = f64::NAN;
                        let v141: num_complex::Complex<f64> = num_complex::Complex::new(v137, v139);
                        let v144: num_complex::Complex<f64> = match &v132 {
                            Math::US0::US0_0(v132_0_0) => match &v132 {
                                Math::US0::US0_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            _ => v141.clone(),
                        };
                        let v146: num_complex::Complex<f64> = v85 * v144;
                        let v149: f64 = 1.0_f64 - (v2.clone().re);
                        let v152: f64 = -v2.im;
                        let v154: num_complex::Complex<f64> = num_complex::Complex::new(v149, v152);
                        let v157: Result<num_complex::Complex<f64>, std::string::String> =
                            Math::method4(
                                v0_1.get().clone(),
                                string("        s = mpmath.zeta(s)"),
                                Math::method3(v154.clone()),
                            );
                        let v158: num_complex::Complex<f64> =
                            Math::method13(v0_1.get().clone(), v154);
                        let v162: Option<num_complex::Complex<f64>> = v157.ok();
                        let v198: Math::US0 =
                            defaultValue(Math::US0::US0_1, map(Math::method17(), v162));
                        let v203: f64 = f64::NAN;
                        let v205: f64 = f64::NAN;
                        let v207: num_complex::Complex<f64> = num_complex::Complex::new(v203, v205);
                        let v210: num_complex::Complex<f64> = match &v198 {
                            Math::US0::US0_0(v198_0_0) => match &v198 {
                                Math::US0::US0_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            _ => v207.clone(),
                        };
                        let v212: num_complex::Complex<f64> = v146 * v210;
                        let v217: f64 = (v59.clone().re) - (v212.clone().re);
                        let v218: f64 = -v217;
                        let v221: bool = (if (v217) >= (v218) { v217 } else { v218 }) < 0.0001_f64;
                        let v223: bool = if v221 { true } else { Math::method18(v221) };
                        let v226: string = Math::method19(string("__assert_lt"), 0.0001_f64);
                        let v230: () = {
                            Math::closure3(v226.clone(), ());
                            ()
                        };
                        if (v223) == false {
                            panic!("{}", v226,);
                        }
                        {
                            let v237: f64 = (v59.im) - (v212.im);
                            let v238: f64 = -v237;
                            let v241: bool =
                                (if (v237) >= (v238) { v237 } else { v238 }) < 0.0001_f64;
                            let v243: bool = if v241 { true } else { Math::method18(v241) };
                            let v245: string = Math::method19(string("__assert_lt"), 0.0001_f64);
                            let v249: () = {
                                Math::closure3(v245.clone(), ());
                                ()
                            };
                            if (v243) == false {
                                panic!("{}", v245,);
                            }
                            {
                                let v0_1_temp: pyo3::Python = v0_1.get().clone();
                                let v1_1_temp: LrcPtr<Math::UH1> =
                                    match v1_1.get().clone().as_ref() {
                                        Math::UH1::UH1_1(_, x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone();
                                v0_1.set(v0_1_temp);
                                v1_1.set(v1_1_temp);
                                continue '_method49;
                            }
                        }
                    }
                });
            }
        }
        pub fn method47(v0_1: pyo3::Python) {
            Math::method49(v0_1, Math::method48());
        }
        pub fn method46() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method47(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn method52() -> LrcPtr<Math::UH0> {
            LrcPtr::new(Math::UH0::UH0_1(
                2.0_f64,
                LrcPtr::new(Math::UH0::UH0_1(
                    2.5_f64,
                    LrcPtr::new(Math::UH0::UH0_1(
                        3.0_f64,
                        LrcPtr::new(Math::UH0::UH0_1(
                            3.5_f64,
                            LrcPtr::new(Math::UH0::UH0_1(
                                4.0_f64,
                                LrcPtr::new(Math::UH0::UH0_1(
                                    4.5_f64,
                                    LrcPtr::new(Math::UH0::UH0_1(
                                        5.0_f64,
                                        LrcPtr::new(Math::UH0::UH0_0),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            ))
        }
        pub fn method53() -> LrcPtr<Math::UH0> {
            LrcPtr::new(Math::UH0::UH0_1(2.0_f64,
                                         LrcPtr::new(Math::UH0::UH0_1(3.0_f64,
                                                                      LrcPtr::new(Math::UH0::UH0_1(5.0_f64,
                                                                                                   LrcPtr::new(Math::UH0::UH0_1(7.0_f64,
                                                                                                                                LrcPtr::new(Math::UH0::UH0_1(11.0_f64,
                                                                                                                                                             LrcPtr::new(Math::UH0::UH0_1(13.0_f64,
                                                                                                                                                                                          LrcPtr::new(Math::UH0::UH0_1(17.0_f64,
                                                                                                                                                                                                                       LrcPtr::new(Math::UH0::UH0_1(19.0_f64,
                                                                                                                                                                                                                                                    LrcPtr::new(Math::UH0::UH0_1(23.0_f64,
                                                                                                                                                                                                                                                                                 LrcPtr::new(Math::UH0::UH0_1(29.0_f64,
                                                                                                                                                                                                                                                                                                              LrcPtr::new(Math::UH0::UH0_1(31.0_f64,
                                                                                                                                                                                                                                                                                                                                           LrcPtr::new(Math::UH0::UH0_1(37.0_f64,
                                                                                                                                                                                                                                                                                                                                                                        LrcPtr::new(Math::UH0::UH0_1(41.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                     LrcPtr::new(Math::UH0::UH0_1(43.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                  LrcPtr::new(Math::UH0::UH0_1(47.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                               LrcPtr::new(Math::UH0::UH0_1(53.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            LrcPtr::new(Math::UH0::UH0_1(59.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         LrcPtr::new(Math::UH0::UH0_1(61.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      LrcPtr::new(Math::UH0::UH0_1(67.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   LrcPtr::new(Math::UH0::UH0_1(71.0_f64,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                LrcPtr::new(Math::UH0::UH0_0)))))))))))))))))))))))))))))))))))))))))
        }
        pub fn method55(v0_1: f64, v1_1: LrcPtr<Math::UH0>, v2: f64) -> f64 {
            let v0_1: MutCell<f64> = MutCell::new(v0_1);
            let v1_1: MutCell<LrcPtr<Math::UH0>> = MutCell::new(v1_1.clone());
            let v2: MutCell<f64> = MutCell::new(v2);
            '_method55: loop {
                break '_method55 (match v1_1.get().clone().as_ref() {
                    Math::UH0::UH0_0 => v2.get().clone(),
                    Math::UH0::UH0_1(v1_1_1_0, v1_1_1_1) => {
                        let v5: f64 = -v0_1.get().clone();
                        {
                            let v0_1_temp: f64 = v0_1.get().clone();
                            let v1_1_temp: LrcPtr<Math::UH0> = match v1_1.get().clone().as_ref() {
                                Math::UH0::UH0_1(_, x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone();
                            let v2_temp: f64 = (v2.get().clone())
                                / (1.0_f64
                                    - (match v1_1.get().clone().as_ref() {
                                        Math::UH0::UH0_1(x, _) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .powf(v5)));
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            continue '_method55;
                        }
                    }
                });
            }
        }
        pub fn method54(v0_1: pyo3::Python, v1_1: LrcPtr<Math::UH0>, v2: LrcPtr<Math::UH0>) {
            let v0_1: MutCell<pyo3::Python> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<Math::UH0>> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<Math::UH0>> = MutCell::new(v2.clone());
            '_method54: loop {
                break '_method54 (match v2.get().clone().as_ref() {
                    Math::UH0::UH0_0 => (),
                    Math::UH0::UH0_1(v2_1_0, v2_1_1) => {
                        let v3: f64 = match v2.get().clone().as_ref() {
                            Math::UH0::UH0_1(x, _) => x.clone(),
                            _ => unreachable!(),
                        };
                        let v6: num_complex::Complex<f64> = num_complex::Complex::new(v3, 0.0_f64);
                        let v8: f64 = Math::method55(v3, v1_1.get().clone(), 1.0_f64);
                        let v11: Result<num_complex::Complex<f64>, std::string::String> =
                            Math::method4(
                                v0_1.get().clone(),
                                string("        s = mpmath.zeta(s)"),
                                Math::method3(v6.clone()),
                            );
                        let v12: num_complex::Complex<f64> = Math::method13(v0_1.get().clone(), v6);
                        let v16: Option<num_complex::Complex<f64>> = v11.ok();
                        let v52: Math::US0 =
                            defaultValue(Math::US0::US0_1, map(Math::method17(), v16));
                        let v57: f64 = f64::NAN;
                        let v59: f64 = f64::NAN;
                        let v61: num_complex::Complex<f64> = num_complex::Complex::new(v57, v59);
                        let v64: num_complex::Complex<f64> = match &v52 {
                            Math::US0::US0_0(v52_0_0) => match &v52 {
                                Math::US0::US0_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            _ => v61.clone(),
                        };
                        let v67: f64 = (v64.clone().re) - (v8);
                        let v68: f64 = -v67;
                        let v71: bool = (if (v67) >= (v68) { v67 } else { v68 }) < 0.01_f64;
                        let v73: bool = if v71 { true } else { Math::method18(v71) };
                        let v76: string = Math::method19(string("__assert_lt"), 0.01_f64);
                        let v80: () = {
                            Math::closure3(v76.clone(), ());
                            ()
                        };
                        if (v73) == false {
                            panic!("{}", v76,);
                        }
                        {
                            let v85: bool = (v64.im) < 0.01_f64;
                            let v87: bool = if v85 { true } else { Math::method18(v85) };
                            let v89: string = Math::method19(string("__assert_lt"), 0.01_f64);
                            let v93: () = {
                                Math::closure3(v89.clone(), ());
                                ()
                            };
                            if (v87) == false {
                                panic!("{}", v89,);
                            }
                            {
                                let v0_1_temp: pyo3::Python = v0_1.get().clone();
                                let v1_1_temp: LrcPtr<Math::UH0> = v1_1.get().clone();
                                let v2_temp: LrcPtr<Math::UH0> = match v2.get().clone().as_ref() {
                                    Math::UH0::UH0_1(_, x) => x.clone(),
                                    _ => unreachable!(),
                                }
                                .clone();
                                v0_1.set(v0_1_temp);
                                v1_1.set(v1_1_temp);
                                v2.set(v2_temp);
                                continue '_method54;
                            }
                        }
                    }
                });
            }
        }
        pub fn method51(v0_1: pyo3::Python) {
            let v1_1: LrcPtr<Math::UH0> = Math::method52();
            Math::method54(v0_1, Math::method53(), v1_1)
        }
        pub fn method50() {
            pyo3::Python::initialize();
            let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> {
                //;
                Math::method51(py);
                {
                    let v8: Result<(), pyo3::PyErr> = Math::method21(Ok::<(), pyo3::PyErr>(()));
                    let v12: string = string("}}");
                    let v14: string = string("{");
                    let v19: bool = true;
                    let _fix_closure_v16 = v8;
                    let v25: string = append(
                        (append(
                            (append(
                                (append(string("true; _fix_closure_v16 "), (v12))),
                                string("); "),
                            )),
                            (v14),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v26: bool = true;
                    _fix_closure_v16
                }
            });
            {
                // rust.fix_closure';
                let v28: Result<(), pyo3::PyErr> = __run_test;
                v28.unwrap();
                ()
            }
        }
        pub fn closure0(unitVar: (), unitVar_1: ()) {
            let v1_1: bool = true;
            () //;
        } /* /*;
        {
        let v4: string =
        string("*/ #[test] fn test_zeta_at_known_values_() { //");
        let v5: bool =
         */
        #[test]
        fn test_zeta_at_known_values_() {
            //;
            Math::method0();
        } /* /*;
        {
        let v8: string =
        string("*/ #[test] fn test_zeta_at_2_minus2() { //");
        let v9: bool = */
        #[test]
        fn test_zeta_at_2_minus2() {
            //;
            Math::method22();
        } /* /*;
        {
        let v12: string =
        string("*/ #[test] fn test_trivial_zero_at_negative_even___() { //");
        let v13: bool =
         */
        #[test]
        fn test_trivial_zero_at_negative_even___() {
            //;
            Math::method24();
        } /* /*;
        {
        let v16: string =
        string("*/ #[test] fn test_non_trivial_zero___() { //");
        let v17: bool =
         */
        #[test]
        fn test_non_trivial_zero___() {
            //;
            Math::method28();
        } /* /*;
        {
        let v20: string =
        string("*/ #[test] fn test_real_part_greater_than_one___() { //");
        let v21: bool =
         */
        #[test]
        fn test_real_part_greater_than_one___() {
            //;
            Math::method30();
        } /* /*;
        {
        let v24: string =
        string("*/ #[test] fn test_zeta_at_1___() { //");
        let v25: bool =
         */
        #[test]
        fn test_zeta_at_1___() {
            //;
            Math::method32();
        } /* /*;
        {
        let v28: string =
        string("*/ #[test] fn test_symmetry_across_real_axis___() { //");
        let v29: bool =
         */
        #[test]
        fn test_symmetry_across_real_axis___() {
            //;
            Math::method34();
        } /* /*;
        {
        let v32: string =
        string("*/ #[test] fn test_behavior_near_origin___() { //");
        let v33: bool =
         */
        #[test]
        fn test_behavior_near_origin___() {
            //;
            Math::method36();
        } /* /*;
        {
        let v36: string =
        string("*/ #[test] fn test_imaginary_axis() { //");
        let v37: bool =
         */
        #[test]
        fn test_imaginary_axis() {
            //;
            Math::method38();
        } /* /*;
        {
        let v40: string =
        string("*/ #[test] fn test_critical_strip() { //");
        let v41: bool =
         */
        #[test]
        fn test_critical_strip() {
            //;
            Math::method42();
        } /* /*;
        {
        let v44: string =
        string("*/ #[test] fn test_reflection_formula_for_specific_value() { //");
        let v45: bool =
         */
        #[test]
        fn test_reflection_formula_for_specific_value() {
            //;
            Math::method46();
        } /* /*;
        {
        let v48: string =
        string("*/ #[test] fn test_euler_product_formula() { //");
        let v49: bool =
         */
        #[test]
        fn test_euler_product_formula() {
            //;
            Math::method50();
            {
                //;
                {
                    //;
                    {
                        //;
                        {
                            //;
                            {
                                //;
                                {
                                    //;
                                    {
                                        //;
                                        {
                                            //;
                                            {
                                                //;
                                                {
                                                    //;
                                                    {
                                                        //;
                                                        {
                                                            //;
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
        pub fn closure4(unitVar: (), v0_1: Array<string>) -> i32 {
            let v5: () = {
                Math::closure3(sprintf!("value: {}", 1_i32), ());
                ()
            };
            0_i32
        }
        pub fn v0() -> Func0<()> {
            static v0: OnceInit<Func0<()>> = OnceInit::new();
            v0.get_or_init(|| Func0::new(move || Math::closure0((), ())))
                .clone()
        }
        pub fn tests() {
            (Math::v0())();
        }
        pub fn v1() -> Func1<Array<string>, i32> {
            static v1: OnceInit<Func1<Array<string>, i32>> = OnceInit::new();
            v1.get_or_init(|| Func1::new(move |v: Array<string>| Math::closure4((), v)))
                .clone()
        }
        pub fn main(args: Array<string>) -> i32 {
            (Math::v1())(args)
        }
    }
}
pub use module_b7a9935b::*;
#[path = "../../deps/spiral/lib/spiral/async_.rs"]
mod module_2335f2f5;
pub use module_2335f2f5::*;
#[path = "../../deps/spiral/lib/spiral/common.rs"]
mod module_652e6d81;
pub use module_652e6d81::*;
#[path = "../../deps/spiral/lib/spiral/crypto.rs"]
mod module_dd5f95ef;
pub use module_dd5f95ef::*;
#[path = "../../deps/spiral/lib/spiral/date_time.rs"]
mod module_ca5e6cb2;
pub use module_ca5e6cb2::*;
#[path = "../../deps/spiral/lib/spiral/file_system.rs"]
mod module_5ab1faf0;
pub use module_5ab1faf0::*;
#[path = "../../deps/spiral/lib/spiral/lib.rs"]
mod module_b386774b;
pub use module_b386774b::*;
#[path = "../../deps/spiral/lib/spiral/networking.rs"]
mod module_ce497f72;
pub use module_ce497f72::*;
#[path = "../../deps/spiral/lib/spiral/platform.rs"]
mod module_9a61edd3;
pub use module_9a61edd3::*;
#[path = "../../deps/spiral/lib/spiral/runtime.rs"]
mod module_502d7e30;
pub use module_502d7e30::*;
#[path = "../../deps/spiral/lib/spiral/sm.rs"]
mod module_34f67952;
pub use module_34f67952::*;
#[path = "../../deps/spiral/lib/spiral/threading.rs"]
mod module_11c0c5c2;
pub use module_11c0c5c2::*;
#[path = "../../deps/spiral/lib/spiral/trace.rs"]
mod module_28ecba0d;
pub use module_28ecba0d::*;
#[path = "../../lib/fsharp/Common.rs"]
mod module_ad43931;
pub use module_ad43931::*;
pub mod Polyglot {
    pub use crate::module_ad43931::Polyglot::*;
}
pub fn main() {
    let args = std::env::args().skip(1).map(fromString).collect();
    Math::main(array_from(args));
}
