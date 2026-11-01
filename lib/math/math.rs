#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
thread_local! { static SPIRAL_TEST_NAME: std::cell::RefCell<Option<std::rc::Rc<str>>> = const { std::cell::RefCell::new(None) }; }
#[cfg(test)]
fn spiral_test(name: &'static str) {
    let code = std::thread::Builder::new().stack_size(1 << 30).spawn(move || { SPIRAL_TEST_NAME.with(|x| *x.borrow_mut() = Some(std::rc::Rc::from(name))); spiral_main() }).unwrap().join().unwrap();
    assert_eq!(code, 0);
}
#[cfg(test)]
mod spiral_tests {
    macro_rules! tests { ($($name:ident),*) => { $(#[test] fn $name() { super::spiral_test(stringify!($name)) })* } }
    tests!(test_zeta_at_known_values_, test_zeta_at_2_minus2, test_trivial_zero_at_negative_even___, test_non_trivial_zero___, test_real_part_greater_than_one___, test_zeta_at_1___, test_symmetry_across_real_axis___, test_behavior_near_origin___, test_imaginary_axis, test_critical_strip, test_reflection_formula_for_specific_value, test_euler_product_formula);
}
use pyo3::prelude::PyAnyMethods;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
struct Mut0 { l0: i32 }
struct Mut1 { l0: i32, l1: num_complex::Complex<f64> }
#[derive(Clone)]
enum US1 {
    US1_Some(num_complex::Complex<f64>),
    US1_None,
}
struct Mut2 { l0: Rc<str> }
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(f64, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(num_complex::Complex<f64>, Rc<UH1>),
}
fn method0(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure0() -> Rc<dyn Fn((Rc<str>)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US0> = Rc::new(move |mut v0: (Rc<str>)| -> US0 {
        let v1: Rc<str> = (v0);
        US0::US0_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_line_argument_count_1() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn method3(mut v0: Rc<str>) -> Rc<str> {
    v0
}
fn method6(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0.clone();
    let mut v3: bool = v2 < v0;
    v3
}
fn method7(mut v0: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
    v0
}
fn method9(mut v0: Rc<str>) -> Rc<str> {
    v0
}
fn method10(mut v0: f64, mut v1: f64) -> (f64, f64) {
    (v0, v1)
}
fn method11(mut v0: (f64, f64)) -> (bool, (f64, f64)) {
    (false, v0)
}
fn method12(mut v0: pyo3::Python) -> pyo3::Python {
    v0
}
fn method13() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn"); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn method14(mut v0: pyo3::Bound<pyo3::types::PyModule>) -> pyo3::Bound<pyo3::types::PyModule> {
    v0
}
fn method15(mut v0: (bool, (f64, f64))) -> (bool, (f64, f64)) {
    v0
}
fn method16(mut v0: pyo3::Bound<pyo3::PyAny>) -> pyo3::Bound<pyo3::PyAny> {
    v0
}
fn method17(mut v0: pyo3::Bound<pyo3::PyAny>) -> pyo3::Bound<pyo3::PyAny> {
    v0
}
fn method8(mut v0: pyo3::Python, mut v1: num_complex::Complex<f64>) -> Result<num_complex::Complex<f64>, std::string::String> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import sys"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import traceback"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import re"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count = 0"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_address_pattern = re.compile(r' at 0x[0-9a-fA-F]+')"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("def trace_calls(frame, event, arg):"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    global count"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    count += 1"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    if count < 200:"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        try:"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            args = { k: v for k, v in frame.f_locals.items() if frame.f_code.co_name != 'make_mpc' and k not in ['ctx'] and not callable(v) }"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            args_str = ', '.join([ f\"{k}={re.sub(memory_address_pattern, ' at 0x<?>', repr(v))}\" for k, v in args.items() ])"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            print(f\"{event}(__NAME__) / f_code.co_name: {frame.f_code.co_name} / f_locals: {args_str} / f_lineno: {frame.f_lineno} / f_code.co_filename: {frame.f_code.co_filename.split('site-packages')[-1]} / f_back.f_lineno: { '' if frame.f_back is None else frame.f_back.f_lineno } / f_back.f_code.co_filename: { '' if frame.f_back is None else frame.f_back.f_code.co_filename.split('site-packages')[-1] } / arg: {re.sub(memory_address_pattern, ' at 0x<?>', repr(arg))}\", flush=True)"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        except ValueError as e:"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            print(f'__NAME__ / e: {e}', flush=True)"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        return trace_calls"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import mpmath"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("def fn(log, s):"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    if log:"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        print(f'__NAME__ / s: {s} / count: {count}', flush=True)"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    s = complex(*s)"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    try:"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        if log: sys.settrace(trace_calls)"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        s = mpmath.zeta(s)"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        if log:"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            sys.settrace(None)"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            print(f'__NAME__ / result: {s} / count: {count}', flush=True)"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    except ValueError as e:"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        if s.real == 1:"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            s = complex(float('inf'), 0)"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    return (s.real, s.imag)"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v8.clone(), v20.clone(), v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v29.clone(), v30.clone(), v31.clone(), v32.clone()]));
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from((*v33).borrow().iter().map(|x| &**x).collect::<Vec<&str>>().join(&*v34).as_str());
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__NAME__"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zeta_"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = Rc::<str>::from(v35.replace(&*v36, &*v37));
    let mut v39: Rc<str> = method9(v38.clone());
    let mut v41: f64 = v1.re;
    let v43: f64 = v1.im;
    let (mut v44, mut v45): (f64, f64) = method10(v41, v43);
    let v46: (f64, f64) = (v44, v45);
    let (mut v47, mut v48): (bool, (f64, f64)) = method11(v46.clone());
    let v49: (bool, (f64, f64)) = (v47, v48);
    let v50: pyo3::Python = method12(v0.clone());
    let mut v52: &str = &*v39;
    let v54: std::string::String = String::from(v52);
    let v56: std::ffi::CString = std::ffi::CString::new(v54).unwrap();
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v59: &str = &*v57;
    let v61: std::string::String = String::from(v59);
    let v63: std::ffi::CString = std::ffi::CString::new(v61).unwrap();
    let mut v65: Result<pyo3::Bound<pyo3::types::PyModule>, pyo3::PyErr> = pyo3::types::PyModule::from_code(v50, &v56, &v63, &v63);
    let v67: bool = true; let _result_map_error__ = v65.map_err(|x| { //;
    let v69: pyo3::PyErr = x;
    let v71: std::string::String = format!("{}", v69);
    let v73: bool = true; v71 });
    let v75: Result<pyo3::Bound<pyo3::types::PyModule>, std::string::String> = _result_map_error__;
    let v77: pyo3::Bound<pyo3::types::PyModule> = v75.unwrap();
    let v78: Rc<str> = method13();
    let v80: &str = &*v78;
    let mut v81: pyo3::Bound<pyo3::types::PyModule> = method14(v77.clone());
    let mut v83: Result<pyo3::Bound<pyo3::PyAny>, pyo3::PyErr> = v81.getattr(v80);
    let v85: bool = true; let _result_map_error__ = v83.map_err(|x| { //;
    let mut v87: pyo3::PyErr = x;
    let v89: std::string::String = format!("{}", v87);
    let mut v91: bool = true; v89 });
    let v93: Result<pyo3::Bound<pyo3::PyAny>, std::string::String> = _result_map_error__;
    let v95: pyo3::Bound<pyo3::PyAny> = v93.unwrap();
    let v96: (bool, (f64, f64)) = method15(v49.clone());
    let v97: pyo3::Bound<pyo3::PyAny> = method16(v95.clone());
    let mut v99: Result<pyo3::Bound<pyo3::PyAny>, pyo3::PyErr> = pyo3::prelude::PyAnyMethods::call(&v97, v96, None);
    let v101: bool = true; let _result_map_error__ = v99.map_err(|x| { //;
    let v103: pyo3::PyErr = x;
    let v105: std::string::String = format!("{}", v103);
    let v107: bool = true; v105 });
    let v109: Result<pyo3::Bound<pyo3::PyAny>, std::string::String> = _result_map_error__;
    let v111: pyo3::Bound<pyo3::PyAny> = v109?;
    let mut v112: pyo3::Bound<pyo3::PyAny> = method17(v111.clone());
    let mut v114: Result<(f64, f64), pyo3::PyErr> = v112.extract();
    let v116: bool = true; let _result_map_error__ = v114.map_err(|x| { //;
    let mut v118: pyo3::PyErr = x;
    let v120: std::string::String = format!("{}", v118);
    let v122: bool = true; v120 });
    let v124: Result<(f64, f64), std::string::String> = _result_map_error__;
    let (mut v126, mut v127): (f64, f64) = v124?;
    let v129: num_complex::Complex<f64> = num_complex::Complex::new(v126, v127);
    let v130: Result<num_complex::Complex<f64>, std::string::String> = Ok::<num_complex::Complex<f64>, std::string::String>(v129);
    v130.clone()
}
fn method19(mut v0: Rc<RefCell<Mut0>>) -> bool {
    let v1: i32 = v0.borrow().l0.clone();
    let mut v2: bool = v1 < 10000i32;
    v2
}
fn method20(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0.clone();
    let mut v3: bool = v2 < v0;
    v3
}
fn method21(mut v0: pyo3::Python, mut v1: num_complex::Complex<f64>) -> Result<num_complex::Complex<f64>, std::string::String> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import sys"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import traceback"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import re"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count = 0"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_address_pattern = re.compile(r' at 0x[0-9a-fA-F]+')"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("def trace_calls(frame, event, arg):"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    global count"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    count += 1"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    if count < 200:"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        try:"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            args = { k: v for k, v in frame.f_locals.items() if frame.f_code.co_name != 'make_mpc' and k not in ['ctx'] and not callable(v) }"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            args_str = ', '.join([ f\"{k}={re.sub(memory_address_pattern, ' at 0x<?>', repr(v))}\" for k, v in args.items() ])"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            print(f\"{event}(__NAME__) / f_code.co_name: {frame.f_code.co_name} / f_locals: {args_str} / f_lineno: {frame.f_lineno} / f_code.co_filename: {frame.f_code.co_filename.split('site-packages')[-1]} / f_back.f_lineno: { '' if frame.f_back is None else frame.f_back.f_lineno } / f_back.f_code.co_filename: { '' if frame.f_back is None else frame.f_back.f_code.co_filename.split('site-packages')[-1] } / arg: {re.sub(memory_address_pattern, ' at 0x<?>', repr(arg))}\", flush=True)"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        except ValueError as e:"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            print(f'__NAME__ / e: {e}', flush=True)"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        return trace_calls"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import mpmath"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("def fn(log, s):"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    if log:"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        print(f'__NAME__ / s: {s} / count: {count}', flush=True)"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    s = complex(*s)"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    try:"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        if log: sys.settrace(trace_calls)"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        s = mpmath.gamma(s)"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        if log:"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            sys.settrace(None)"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            print(f'__NAME__ / result: {s} / count: {count}', flush=True)"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    except ValueError as e:"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("        if s.real == 1:"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("            s = complex(float('inf'), 0)"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    return (s.real, s.imag)"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v8.clone(), v20.clone(), v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v29.clone(), v30.clone(), v31.clone(), v32.clone()]));
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from((*v33).borrow().iter().map(|x| &**x).collect::<Vec<&str>>().join(&*v34).as_str());
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__NAME__"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gamma_"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = Rc::<str>::from(v35.replace(&*v36, &*v37));
    let mut v39: Rc<str> = method9(v38.clone());
    let mut v41: f64 = v1.re;
    let v43: f64 = v1.im;
    let (mut v44, mut v45): (f64, f64) = method10(v41, v43);
    let v46: (f64, f64) = (v44, v45);
    let (mut v47, mut v48): (bool, (f64, f64)) = method11(v46.clone());
    let v49: (bool, (f64, f64)) = (v47, v48);
    let v50: pyo3::Python = method12(v0.clone());
    let mut v52: &str = &*v39;
    let v54: std::string::String = String::from(v52);
    let v56: std::ffi::CString = std::ffi::CString::new(v54).unwrap();
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v59: &str = &*v57;
    let v61: std::string::String = String::from(v59);
    let v63: std::ffi::CString = std::ffi::CString::new(v61).unwrap();
    let mut v65: Result<pyo3::Bound<pyo3::types::PyModule>, pyo3::PyErr> = pyo3::types::PyModule::from_code(v50, &v56, &v63, &v63);
    let v67: bool = true; let _result_map_error__ = v65.map_err(|x| { //;
    let v69: pyo3::PyErr = x;
    let v71: std::string::String = format!("{}", v69);
    let v73: bool = true; v71 });
    let v75: Result<pyo3::Bound<pyo3::types::PyModule>, std::string::String> = _result_map_error__;
    let v77: pyo3::Bound<pyo3::types::PyModule> = v75.unwrap();
    let v78: Rc<str> = method13();
    let v80: &str = &*v78;
    let mut v81: pyo3::Bound<pyo3::types::PyModule> = method14(v77.clone());
    let mut v83: Result<pyo3::Bound<pyo3::PyAny>, pyo3::PyErr> = v81.getattr(v80);
    let v85: bool = true; let _result_map_error__ = v83.map_err(|x| { //;
    let mut v87: pyo3::PyErr = x;
    let v89: std::string::String = format!("{}", v87);
    let mut v91: bool = true; v89 });
    let v93: Result<pyo3::Bound<pyo3::PyAny>, std::string::String> = _result_map_error__;
    let v95: pyo3::Bound<pyo3::PyAny> = v93.unwrap();
    let v96: (bool, (f64, f64)) = method15(v49.clone());
    let v97: pyo3::Bound<pyo3::PyAny> = method16(v95.clone());
    let mut v99: Result<pyo3::Bound<pyo3::PyAny>, pyo3::PyErr> = pyo3::prelude::PyAnyMethods::call(&v97, v96, None);
    let v101: bool = true; let _result_map_error__ = v99.map_err(|x| { //;
    let v103: pyo3::PyErr = x;
    let v105: std::string::String = format!("{}", v103);
    let v107: bool = true; v105 });
    let v109: Result<pyo3::Bound<pyo3::PyAny>, std::string::String> = _result_map_error__;
    let v111: pyo3::Bound<pyo3::PyAny> = v109?;
    let mut v112: pyo3::Bound<pyo3::PyAny> = method17(v111.clone());
    let mut v114: Result<(f64, f64), pyo3::PyErr> = v112.extract();
    let v116: bool = true; let _result_map_error__ = v114.map_err(|x| { //;
    let mut v118: pyo3::PyErr = x;
    let v120: std::string::String = format!("{}", v118);
    let v122: bool = true; v120 });
    let v124: Result<(f64, f64), std::string::String> = _result_map_error__;
    let (mut v126, mut v127): (f64, f64) = v124?;
    let v129: num_complex::Complex<f64> = num_complex::Complex::new(v126, v127);
    let v130: Result<num_complex::Complex<f64>, std::string::String> = Ok::<num_complex::Complex<f64>, std::string::String>(v129);
    v130.clone()
}
fn method22(mut v0: Option<num_complex::Complex<f64>>) -> Option<num_complex::Complex<f64>> {
    v0
}
fn closure1() -> Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = Rc::new(move |mut v0: (num_complex::Complex<f64>)| -> US1 {
        let v1: num_complex::Complex<f64> = (v0);
        US1::US1_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18(mut v0: pyo3::Python, mut v1: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
    println!("zeta / count: {:?} / s: {:?}", 0i32, v1);
    let v4: f64 = v1.re;
    let mut v5: bool = v4 > 1.0f64;
    if v5 {
        let mut v7: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
        let mut v8: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); 10000i32 as usize]));
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
        while method19(v9.clone()) {
            let mut v11: i32 = v9.borrow().l0.clone();
            v8.clone().borrow_mut()[v11 as usize] = v11;
            let mut v12: i32 = v11.wrapping_add(1i32);
            v9.borrow_mut().l0 = v12;
            ()
        };
        let mut v13: i32 = (v8.clone().borrow().len() as i32);
        let mut v14: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v7.clone() }));
        while method20(v13, v14.clone()) {
            let mut v16: i32 = v14.borrow().l0.clone();
            let mut v17: num_complex::Complex<f64> = v14.borrow().l1.clone();
            let mut v18: i32 = v8.clone().borrow()[v16 as usize].clone();
            let mut v20: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
            let mut v21: f64 = (v18 as f64);
            let v23: num_complex::Complex<f64> = num_complex::Complex::new(v21, 0.0f64);
            let v25: num_complex::Complex<f64> = num_complex::Complex::powc(v23, v1.clone());
            let mut v27: num_complex::Complex<f64> = v20 / v25;
            let v29: num_complex::Complex<f64> = v17 + v27;
            let v30: i32 = v16.wrapping_add(1i32);
            v14.borrow_mut().l0 = v30;
            v14.borrow_mut().l1 = v29.clone();
            ()
        };
        let v31: num_complex::Complex<f64> = v14.borrow().l1.clone();
        v31.clone()
    } else {
        let v33: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
        let v35: num_complex::Complex<f64> = v33 - v1;
        let v36: num_complex::Complex<f64> = method7(v35.clone());
        let mut v37: Result<num_complex::Complex<f64>, std::string::String> = method21(v0.clone(), v36.clone());
        let mut v39: Option<num_complex::Complex<f64>> = v37.ok();
        let v40: Option<num_complex::Complex<f64>> = method22(v39.clone());
        let mut v41: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
        let mut v42: Option<US1> = v40.map(|x| v41(x));
        let v44: US1 = US1::US1_None;
        let v45: US1 = v44.clone();
        let v46: US1 = v42.unwrap_or(v45);
        let v48: f64 = f64::NAN;
        let v50: f64 = f64::NAN;
        let mut v52: num_complex::Complex<f64> = num_complex::Complex::new(v48, v50);
        let v55: num_complex::Complex<f64> = match &v46 {
            US1::US1_None => {
                v52.clone()
            }
            US1::US1_Some(v53) => {
                let v53: num_complex::Complex<f64> = v53.clone();
                v53.clone()
            }
        };
        let v57: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
        let v59: num_complex::Complex<f64> = v57 * v1;
        let v61: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
        let v63: num_complex::Complex<f64> = v59 / v61;
        let mut v65: num_complex::Complex<f64> = v63.sin();
        let v67: f64 = v1.re;
        let v68: f64 = 1.0f64 - v67;
        let v70: f64 = v1.im;
        let v71: f64 = -(v70);
        let v73: num_complex::Complex<f64> = num_complex::Complex::new(v68, v71);
        let v75: f64 = v73.re;
        let v76: bool = v75 <= 1.0f64;
        let v447: num_complex::Complex<f64> = if v76 {
            let v78: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
            v78.clone()
        } else {
            println!("zeta / count: {:?} / s: {:?}", 1i32, v73);
            let mut v81: f64 = v73.re;
            let v82: bool = v81 > 1.0f64;
            if v82 {
                let v84: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
                let v85: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); 10000i32 as usize]));
                let mut v86: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method19(v86.clone()) {
                    let v88: i32 = v86.borrow().l0.clone();
                    v85.clone().borrow_mut()[v88 as usize] = v88;
                    let v89: i32 = v88.wrapping_add(1i32);
                    v86.borrow_mut().l0 = v89;
                    ()
                };
                let v90: i32 = (v85.clone().borrow().len() as i32);
                let mut v91: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v84.clone() }));
                while method20(v90, v91.clone()) {
                    let v93: i32 = v91.borrow().l0.clone();
                    let v94: num_complex::Complex<f64> = v91.borrow().l1.clone();
                    let v95: i32 = v85.clone().borrow()[v93 as usize].clone();
                    let v97: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                    let v98: f64 = (v95 as f64);
                    let v100: num_complex::Complex<f64> = num_complex::Complex::new(v98, 0.0f64);
                    let v102: num_complex::Complex<f64> = num_complex::Complex::powc(v100, v73.clone());
                    let v104: num_complex::Complex<f64> = v97 / v102;
                    let v106: num_complex::Complex<f64> = v94 + v104;
                    let v107: i32 = v93.wrapping_add(1i32);
                    v91.borrow_mut().l0 = v107;
                    v91.borrow_mut().l1 = v106.clone();
                    ()
                };
                let v108: num_complex::Complex<f64> = v91.borrow().l1.clone();
                v108.clone()
            } else {
                let v110: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                let mut v112: num_complex::Complex<f64> = v110 - v73;
                let v113: num_complex::Complex<f64> = method7(v112.clone());
                let mut v114: Result<num_complex::Complex<f64>, std::string::String> = method21(v0.clone(), v113.clone());
                let v116: Option<num_complex::Complex<f64>> = v114.ok();
                let v117: Option<num_complex::Complex<f64>> = method22(v116.clone());
                let mut v118: Option<US1> = v117.map(|x| v41(x));
                let v120: US1 = US1::US1_None;
                let v121: US1 = v120.clone();
                let v122: US1 = v118.unwrap_or(v121);
                let v124: f64 = f64::NAN;
                let v126: f64 = f64::NAN;
                let v128: num_complex::Complex<f64> = num_complex::Complex::new(v124, v126);
                let v131: num_complex::Complex<f64> = match &v122 {
                    US1::US1_None => {
                        v128.clone()
                    }
                    US1::US1_Some(v129) => {
                        let v129: num_complex::Complex<f64> = v129.clone();
                        v129.clone()
                    }
                };
                let v133: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                let v135: num_complex::Complex<f64> = v133 * v73;
                let v137: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                let v139: num_complex::Complex<f64> = v135 / v137;
                let v141: num_complex::Complex<f64> = v139.sin();
                let v143: f64 = v73.re;
                let v144: f64 = 1.0f64 - v143;
                let v146: f64 = v73.im;
                let v147: f64 = -(v146);
                let v149: num_complex::Complex<f64> = num_complex::Complex::new(v144, v147);
                let v151: f64 = v149.re;
                let v152: bool = v151 <= 1.0f64;
                let v431: num_complex::Complex<f64> = if v152 {
                    let v154: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
                    v154.clone()
                } else {
                    println!("zeta / count: {:?} / s: {:?}", 2i32, v149);
                    let v157: f64 = v149.re;
                    let v158: bool = v157 > 1.0f64;
                    if v158 {
                        let v160: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
                        let v161: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); 10000i32 as usize]));
                        let mut v162: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                        while method19(v162.clone()) {
                            let v164: i32 = v162.borrow().l0.clone();
                            v161.clone().borrow_mut()[v164 as usize] = v164;
                            let v165: i32 = v164.wrapping_add(1i32);
                            v162.borrow_mut().l0 = v165;
                            ()
                        };
                        let v166: i32 = (v161.clone().borrow().len() as i32);
                        let mut v167: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v160.clone() }));
                        while method20(v166, v167.clone()) {
                            let v169: i32 = v167.borrow().l0.clone();
                            let v170: num_complex::Complex<f64> = v167.borrow().l1.clone();
                            let v171: i32 = v161.clone().borrow()[v169 as usize].clone();
                            let v173: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                            let v174: f64 = (v171 as f64);
                            let v176: num_complex::Complex<f64> = num_complex::Complex::new(v174, 0.0f64);
                            let v178: num_complex::Complex<f64> = num_complex::Complex::powc(v176, v149.clone());
                            let v180: num_complex::Complex<f64> = v173 / v178;
                            let v182: num_complex::Complex<f64> = v170 + v180;
                            let v183: i32 = v169.wrapping_add(1i32);
                            v167.borrow_mut().l0 = v183;
                            v167.borrow_mut().l1 = v182.clone();
                            ()
                        };
                        let v184: num_complex::Complex<f64> = v167.borrow().l1.clone();
                        v184.clone()
                    } else {
                        let v186: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                        let v188: num_complex::Complex<f64> = v186 - v149;
                        let v189: num_complex::Complex<f64> = method7(v188.clone());
                        let mut v190: Result<num_complex::Complex<f64>, std::string::String> = method21(v0.clone(), v189.clone());
                        let v192: Option<num_complex::Complex<f64>> = v190.ok();
                        let v193: Option<num_complex::Complex<f64>> = method22(v192.clone());
                        let mut v194: Option<US1> = v193.map(|x| v41(x));
                        let v196: US1 = US1::US1_None;
                        let v197: US1 = v196.clone();
                        let v198: US1 = v194.unwrap_or(v197);
                        let v200: f64 = f64::NAN;
                        let v202: f64 = f64::NAN;
                        let v204: num_complex::Complex<f64> = num_complex::Complex::new(v200, v202);
                        let v207: num_complex::Complex<f64> = match &v198 {
                            US1::US1_None => {
                                v204.clone()
                            }
                            US1::US1_Some(v205) => {
                                let v205: num_complex::Complex<f64> = v205.clone();
                                v205.clone()
                            }
                        };
                        let v209: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                        let v211: num_complex::Complex<f64> = v209 * v149;
                        let v213: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                        let v215: num_complex::Complex<f64> = v211 / v213;
                        let v217: num_complex::Complex<f64> = v215.sin();
                        let v219: f64 = v149.re;
                        let v220: f64 = 1.0f64 - v219;
                        let v222: f64 = v149.im;
                        let v223: f64 = -(v222);
                        let v225: num_complex::Complex<f64> = num_complex::Complex::new(v220, v223);
                        let v227: f64 = v225.re;
                        let v228: bool = v227 <= 1.0f64;
                        let v415: num_complex::Complex<f64> = if v228 {
                            let v230: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
                            v230.clone()
                        } else {
                            println!("zeta / count: {:?} / s: {:?}", 3i32, v225);
                            let v233: f64 = v225.re;
                            let v234: bool = v233 > 1.0f64;
                            if v234 {
                                let v236: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
                                let v237: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); 10000i32 as usize]));
                                let mut v238: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                                while method19(v238.clone()) {
                                    let v240: i32 = v238.borrow().l0.clone();
                                    v237.clone().borrow_mut()[v240 as usize] = v240;
                                    let v241: i32 = v240.wrapping_add(1i32);
                                    v238.borrow_mut().l0 = v241;
                                    ()
                                };
                                let v242: i32 = (v237.clone().borrow().len() as i32);
                                let mut v243: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v236.clone() }));
                                while method20(v242, v243.clone()) {
                                    let v245: i32 = v243.borrow().l0.clone();
                                    let v246: num_complex::Complex<f64> = v243.borrow().l1.clone();
                                    let v247: i32 = v237.clone().borrow()[v245 as usize].clone();
                                    let v249: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                                    let v250: f64 = (v247 as f64);
                                    let v252: num_complex::Complex<f64> = num_complex::Complex::new(v250, 0.0f64);
                                    let v254: num_complex::Complex<f64> = num_complex::Complex::powc(v252, v225.clone());
                                    let v256: num_complex::Complex<f64> = v249 / v254;
                                    let v258: num_complex::Complex<f64> = v246 + v256;
                                    let v259: i32 = v245.wrapping_add(1i32);
                                    v243.borrow_mut().l0 = v259;
                                    v243.borrow_mut().l1 = v258.clone();
                                    ()
                                };
                                let v260: num_complex::Complex<f64> = v243.borrow().l1.clone();
                                v260.clone()
                            } else {
                                let v262: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                                let v264: num_complex::Complex<f64> = v262 - v225;
                                let v265: num_complex::Complex<f64> = method7(v264.clone());
                                let mut v266: Result<num_complex::Complex<f64>, std::string::String> = method21(v0.clone(), v265.clone());
                                let v268: Option<num_complex::Complex<f64>> = v266.ok();
                                let v269: Option<num_complex::Complex<f64>> = method22(v268.clone());
                                let mut v270: Option<US1> = v269.map(|x| v41(x));
                                let v272: US1 = US1::US1_None;
                                let v273: US1 = v272.clone();
                                let v274: US1 = v270.unwrap_or(v273);
                                let v276: f64 = f64::NAN;
                                let v278: f64 = f64::NAN;
                                let v280: num_complex::Complex<f64> = num_complex::Complex::new(v276, v278);
                                let v283: num_complex::Complex<f64> = match &v274 {
                                    US1::US1_None => {
                                        v280.clone()
                                    }
                                    US1::US1_Some(v281) => {
                                        let v281: num_complex::Complex<f64> = v281.clone();
                                        v281.clone()
                                    }
                                };
                                let v285: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                                let v287: num_complex::Complex<f64> = v285 * v225;
                                let v289: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                                let v291: num_complex::Complex<f64> = v287 / v289;
                                let v293: num_complex::Complex<f64> = v291.sin();
                                let v295: f64 = v225.re;
                                let v296: f64 = 1.0f64 - v295;
                                let v298: f64 = v225.im;
                                let v299: f64 = -(v298);
                                let v301: num_complex::Complex<f64> = num_complex::Complex::new(v296, v299);
                                let v303: f64 = v301.re;
                                let v304: bool = v303 <= 1.0f64;
                                let v399: num_complex::Complex<f64> = if v304 {
                                    let v306: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
                                    v306.clone()
                                } else {
                                    println!("zeta / count: {:?} / s: {:?}", 4i32, v301);
                                    let v309: f64 = v301.re;
                                    let v310: bool = v309 > 1.0f64;
                                    if v310 {
                                        let v312: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
                                        let v313: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); 10000i32 as usize]));
                                        let mut v314: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                                        while method19(v314.clone()) {
                                            let v316: i32 = v314.borrow().l0.clone();
                                            v313.clone().borrow_mut()[v316 as usize] = v316;
                                            let v317: i32 = v316.wrapping_add(1i32);
                                            v314.borrow_mut().l0 = v317;
                                            ()
                                        };
                                        let v318: i32 = (v313.clone().borrow().len() as i32);
                                        let mut v319: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v312.clone() }));
                                        while method20(v318, v319.clone()) {
                                            let v321: i32 = v319.borrow().l0.clone();
                                            let v322: num_complex::Complex<f64> = v319.borrow().l1.clone();
                                            let v323: i32 = v313.clone().borrow()[v321 as usize].clone();
                                            let v325: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                                            let v326: f64 = (v323 as f64);
                                            let v328: num_complex::Complex<f64> = num_complex::Complex::new(v326, 0.0f64);
                                            let v330: num_complex::Complex<f64> = num_complex::Complex::powc(v328, v301.clone());
                                            let v332: num_complex::Complex<f64> = v325 / v330;
                                            let v334: num_complex::Complex<f64> = v322 + v332;
                                            let v335: i32 = v321.wrapping_add(1i32);
                                            v319.borrow_mut().l0 = v335;
                                            v319.borrow_mut().l1 = v334.clone();
                                            ()
                                        };
                                        let v336: num_complex::Complex<f64> = v319.borrow().l1.clone();
                                        v336.clone()
                                    } else {
                                        let v338: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                                        let v340: num_complex::Complex<f64> = v338 - v301;
                                        let v341: num_complex::Complex<f64> = method7(v340.clone());
                                        let mut v342: Result<num_complex::Complex<f64>, std::string::String> = method21(v0.clone(), v341.clone());
                                        let v344: Option<num_complex::Complex<f64>> = v342.ok();
                                        let v345: Option<num_complex::Complex<f64>> = method22(v344.clone());
                                        let mut v346: Option<US1> = v345.map(|x| v41(x));
                                        let v348: US1 = US1::US1_None;
                                        let v349: US1 = v348.clone();
                                        let v350: US1 = v346.unwrap_or(v349);
                                        let v352: f64 = f64::NAN;
                                        let v354: f64 = f64::NAN;
                                        let v356: num_complex::Complex<f64> = num_complex::Complex::new(v352, v354);
                                        let v359: num_complex::Complex<f64> = match &v350 {
                                            US1::US1_None => {
                                                v356.clone()
                                            }
                                            US1::US1_Some(v357) => {
                                                let v357: num_complex::Complex<f64> = v357.clone();
                                                v357.clone()
                                            }
                                        };
                                        let v361: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                                        let v363: num_complex::Complex<f64> = v361 * v301;
                                        let v365: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                                        let v367: num_complex::Complex<f64> = v363 / v365;
                                        let v369: num_complex::Complex<f64> = v367.sin();
                                        let v371: f64 = v301.re;
                                        let v372: f64 = 1.0f64 - v371;
                                        let v374: f64 = v301.im;
                                        let v375: f64 = -(v374);
                                        let v377: num_complex::Complex<f64> = num_complex::Complex::new(v372, v375);
                                        let v379: f64 = v377.re;
                                        let v380: bool = v379 <= 1.0f64;
                                        let v383: num_complex::Complex<f64> = if v380 {
                                            let v382: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, 0.0f64);
                                            v382.clone()
                                        } else {
                                            v377.clone()
                                        };
                                        let v385: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                                        let v387: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                                        let v389: num_complex::Complex<f64> = num_complex::Complex::powc(v387, v301);
                                        let v391: num_complex::Complex<f64> = v385 * v389;
                                        let v393: num_complex::Complex<f64> = v391 * v369;
                                        let v395: num_complex::Complex<f64> = v393 * v359;
                                        let v397: num_complex::Complex<f64> = v395 * v383;
                                        v397.clone()
                                    }
                                };
                                let v401: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                                let v403: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                                let v405: num_complex::Complex<f64> = num_complex::Complex::powc(v403, v225);
                                let v407: num_complex::Complex<f64> = v401 * v405;
                                let v409: num_complex::Complex<f64> = v407 * v293;
                                let v411: num_complex::Complex<f64> = v409 * v283;
                                let v413: num_complex::Complex<f64> = v411 * v399;
                                v413.clone()
                            }
                        };
                        let v417: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                        let v419: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                        let v421: num_complex::Complex<f64> = num_complex::Complex::powc(v419, v149);
                        let v423: num_complex::Complex<f64> = v417 * v421;
                        let v425: num_complex::Complex<f64> = v423 * v217;
                        let v427: num_complex::Complex<f64> = v425 * v207;
                        let v429: num_complex::Complex<f64> = v427 * v415;
                        v429.clone()
                    }
                };
                let v433: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                let v435: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                let v437: num_complex::Complex<f64> = num_complex::Complex::powc(v435, v73);
                let v439: num_complex::Complex<f64> = v433 * v437;
                let v441: num_complex::Complex<f64> = v439 * v141;
                let v443: num_complex::Complex<f64> = v441 * v131;
                let v445: num_complex::Complex<f64> = v443 * v431;
                v445.clone()
            }
        };
        let v449: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
        let v451: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
        let v453: num_complex::Complex<f64> = num_complex::Complex::powc(v451, v1);
        let v455: num_complex::Complex<f64> = v449 * v453;
        let v457: num_complex::Complex<f64> = v455 * v65;
        let v459: num_complex::Complex<f64> = v457 * v55;
        let v461: num_complex::Complex<f64> = v459 * v447;
        v461.clone()
    }
}
fn method23(mut v0: bool) -> bool {
    v0
}
fn position_of_25(mut v0: u8, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return -1i32;
        } else {
            let mut v5: u8 = v1.clone().as_bytes()[v2 as usize];
            let mut v6: bool = v5 == v0;
            if v6 {
                return v2;
            } else {
                let mut v7: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0, v1.clone(), v7);
                continue;
            }
        }
    }
}
fn digits_value_26(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v1.wrapping_add(1i32);
            let mut v6: i32 = v2.wrapping_mul(10i32);
            let mut v7: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = 0i32;
            let v10: i32 = position_of_25(v7, v8.clone(), v9);
            let mut v11: i32 = v6.wrapping_add(v10);
            (v0, v1, v2) = (v0.clone(), v5, v11);
            continue;
        }
    }
}
fn zeros_from_27(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 < v2;
        let mut v6: bool = if v3 {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v5: bool = v4 == b'0';
            v5
        } else {
            false
        };
        if v6 {
            let mut v7: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn zeros_before_28(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 > 0i32;
        let mut v6: bool = if v2 {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b'0';
            v5
        } else {
            false
        };
        if v6 {
            let mut v7: i32 = v1.wrapping_sub(1i32);
            (v0, v1) = (v0.clone(), v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn repeated_zeros_29(mut v0: i32, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let mut v2: bool = v0 <= 0i32;
        if v2 {
            return v1.clone();
        } else {
            let mut v3: i32 = v0.wrapping_sub(1i32);
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }));
            (v0, v1) = (v3, v4.clone());
            continue;
        }
    }
}
fn digits_text_30(mut v0: i32, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.wrapping_rem(10i32);
        let mut v3: i32 = v2.wrapping_add(1i32);
        let v4: i32 = v3.wrapping_sub(1i32);
        let mut v5: Rc<str> = string_slice(&{ thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) }, v2 as i64, v4 as i64);
        let mut v6: bool = v0 < 10i32;
        if v6 {
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), v1.clone()));
            return v7.clone();
        } else {
            let mut v8: i32 = v0.wrapping_div(10i32);
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), v1.clone()));
            (v0, v1) = (v8, v9.clone());
            continue;
        }
    }
}
fn method31(mut v0: Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_24(mut v0: f64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1.clone() }));
    () /* backend_switch: no Rust arm (lib/spiral) */;
    let mut v3: bool = v0.is_nan();
    let mut v13: Rc<str> = if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NaN"); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        () /* backend_switch: no Rust arm (lib/spiral) */;
        let mut v5: bool = v0.is_infinite();
        if v5 {
            let mut v6: f64 = -(v0);
            let mut v7: bool = v0 > v6;
            if v7 {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Infinity"); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-Infinity"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            }
        } else {
            let mut v11: Rc<str> = Rc::<str>::from(format!("{:?}", v0));
            v11.clone()
        }
    };
    let mut v14: bool = &*v13 == "NaN";
    let mut v16: bool = if v14 {
        true
    } else {
        let mut v15: bool = &*v13 == "Infinity";
        v15
    };
    let mut v18: bool = if v16 {
        true
    } else {
        let mut v17: bool = &*v13 == "-Infinity";
        v17
    };
    let v166: Rc<str> = if v18 {
        v13.clone()
    } else {
        let v19: i32 = (v13.clone().len() as i32);
        let mut v20: bool = v19 > 0i32;
        let v23: bool = if v20 {
            let mut v21: u8 = v13.clone().as_bytes()[0i32 as usize];
            let mut v22: bool = v21 == b'-';
            v22
        } else {
            false
        };
        let v25: Rc<str> = if v23 {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        } else {
            v1.clone()
        };
        let v32: Rc<str> = if v23 {
            let v26: i32 = v19.wrapping_sub(1i32);
            let mut v27: bool = v26 <= 0i32;
            if v27 {
                v1.clone()
            } else {
                let mut v28: i32 = 1i32.wrapping_add(v26);
                let v29: i32 = v28.wrapping_sub(1i32);
                let v30: Rc<str> = string_slice(&v13.clone(), 1i32 as i64, v29 as i64);
                v30.clone()
            }
        } else {
            v13.clone()
        };
        let v33: u8 = b'e';
        let mut v34: i32 = 0i32;
        let v35: i32 = position_of_25(v33, v32.clone(), v34);
        let v36: bool = v35 >= 0i32;
        let v40: i32 = if v36 {
            v35
        } else {
            let mut v37: u8 = b'E';
            let v38: i32 = 0i32;
            position_of_25(v37, v32.clone(), v38)
        };
        let mut v41: bool = v40 < 0i32;
        let v46: Rc<str> = if v41 {
            v32.clone()
        } else {
            let mut v42: bool = v40 <= 0i32;
            if v42 {
                v1.clone()
            } else {
                let v43: i32 = v40.wrapping_sub(1i32);
                let v44: Rc<str> = string_slice(&v32.clone(), 0i32 as i64, v43 as i64);
                v44.clone()
            }
        };
        let v74: i32 = if v41 {
            0i32
        } else {
            let v47: i32 = v40.wrapping_add(1i32);
            let v48: i32 = (v32.clone().len() as i32);
            let v49: i32 = v48.wrapping_sub(v40);
            let v50: i32 = v49.wrapping_sub(1i32);
            let v51: bool = v50 <= 0i32;
            let v55: Rc<str> = if v51 {
                v1.clone()
            } else {
                let mut v52: i32 = v47.wrapping_add(v50);
                let v53: i32 = v52.wrapping_sub(1i32);
                let v54: Rc<str> = string_slice(&v32.clone(), v47 as i64, v53 as i64);
                v54.clone()
            };
            let v56: i32 = (v55.clone().len() as i32);
            let v57: bool = v56 == 0i32;
            if v57 {
                0i32
            } else {
                let mut v58: u8 = v55.clone().as_bytes()[0i32 as usize];
                let v59: bool = v58 == b'-';
                if v59 {
                    let v60: i32 = 1i32;
                    let v61: i32 = 0i32;
                    let mut v62: i32 = digits_value_26(v55.clone(), v60, v61);
                    let v63: i32 = v62.wrapping_neg();
                    v63
                } else {
                    let v64: bool = v58 == b'+';
                    if v64 {
                        let mut v65: i32 = 1i32;
                        let v66: i32 = 0i32;
                        digits_value_26(v55.clone(), v65, v66)
                    } else {
                        let v68: i32 = 0i32;
                        let v69: i32 = 0i32;
                        digits_value_26(v55.clone(), v68, v69)
                    }
                }
            }
        };
        let v75: u8 = b'.';
        let v76: i32 = 0i32;
        let v77: i32 = position_of_25(v75, v46.clone(), v76);
        let v78: bool = v77 < 0i32;
        let mut v83: Rc<str> = if v78 {
            v46.clone()
        } else {
            let v79: bool = v77 <= 0i32;
            if v79 {
                v1.clone()
            } else {
                let v80: i32 = v77.wrapping_sub(1i32);
                let mut v81: Rc<str> = string_slice(&v46.clone(), 0i32 as i64, v80 as i64);
                v81.clone()
            }
        };
        let v93: Rc<str> = if v78 {
            v1.clone()
        } else {
            let v84: i32 = v77.wrapping_add(1i32);
            let v85: i32 = (v46.clone().len() as i32);
            let mut v86: i32 = v85.wrapping_sub(v77);
            let mut v87: i32 = v86.wrapping_sub(1i32);
            let v88: bool = v87 <= 0i32;
            if v88 {
                v1.clone()
            } else {
                let v89: i32 = v84.wrapping_add(v87);
                let v90: i32 = v89.wrapping_sub(1i32);
                let mut v91: Rc<str> = string_slice(&v46.clone(), v84 as i64, v90 as i64);
                v91.clone()
            }
        };
        let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v83.clone(), v93.clone()));
        let v95: i32 = 0i32;
        let v96: i32 = zeros_from_27(v94.clone(), v95);
        let v97: i32 = (v94.clone().len() as i32);
        let v98: i32 = zeros_before_28(v94.clone(), v97);
        let mut v99: bool = v96 >= v98;
        if v99 {
            let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0.0"); } LIT.with(|lit| lit.clone()) }));
            v100.clone()
        } else {
            let v101: i32 = v98.wrapping_sub(v96);
            let v102: bool = v101 <= 0i32;
            let v106: Rc<str> = if v102 {
                v1.clone()
            } else {
                let v103: i32 = v96.wrapping_add(v101);
                let v104: i32 = v103.wrapping_sub(1i32);
                let v105: Rc<str> = string_slice(&v94.clone(), v96 as i64, v104 as i64);
                v105.clone()
            };
            let v107: i32 = (v83.clone().len() as i32);
            let v108: i32 = v107.wrapping_add(v74);
            let v109: i32 = v108.wrapping_sub(v96);
            let v110: i32 = (v106.clone().len() as i32);
            let v111: i32 = v109.wrapping_sub(1i32);
            let mut v112: bool = -4i32 <= v111;
            let mut v114: bool = if v112 {
                let v113: bool = v111 < 16i32;
                v113
            } else {
                false
            };
            if v114 {
                let v115: bool = v109 <= 0i32;
                if v115 {
                    let v116: i32 = v109.wrapping_neg();
                    let v117: Rc<str> = repeated_zeros_29(v116, v1.clone());
                    let mut v118: Rc<str> = Rc::<str>::from(format!("{}{}", v117.clone(), v106.clone()));
                    let v119: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0."); } LIT.with(|lit| lit.clone()) }, v118.clone()));
                    let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v119.clone()));
                    v120.clone()
                } else {
                    let v121: bool = v110 <= v109;
                    if v121 {
                        let v122: i32 = v109.wrapping_sub(v110);
                        let v123: Rc<str> = repeated_zeros_29(v122, v1.clone());
                        let v124: Rc<str> = Rc::<str>::from(format!("{}{}", v123.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".0"); } LIT.with(|lit| lit.clone()) }));
                        let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v106.clone(), v124.clone()));
                        let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v125.clone()));
                        v126.clone()
                    } else {
                        let v127: Rc<str> = string_slice(&v106.clone(), 0i32 as i64, v111 as i64);
                        let v128: i32 = v110.wrapping_sub(v109);
                        let v129: bool = v128 <= 0i32;
                        let v133: Rc<str> = if v129 {
                            v1.clone()
                        } else {
                            let v130: i32 = v109.wrapping_add(v128);
                            let v131: i32 = v130.wrapping_sub(1i32);
                            let v132: Rc<str> = string_slice(&v106.clone(), v109 as i64, v131 as i64);
                            v132.clone()
                        };
                        let v134: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }, v133.clone()));
                        let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v127.clone(), v134.clone()));
                        let v136: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v135.clone()));
                        v136.clone()
                    }
                }
            } else {
                let v139: bool = v110 > 1i32;
                let v147: Rc<str> = if v139 {
                    let v140: i32 = v110.wrapping_sub(1i32);
                    let v141: bool = v140 <= 0i32;
                    let v145: Rc<str> = if v141 {
                        v1.clone()
                    } else {
                        let v142: i32 = 1i32.wrapping_add(v140);
                        let v143: i32 = v142.wrapping_sub(1i32);
                        let v144: Rc<str> = string_slice(&v106.clone(), 1i32 as i64, v143 as i64);
                        v144.clone()
                    };
                    let v146: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }, v145.clone()));
                    v146.clone()
                } else {
                    v1.clone()
                };
                let v148: bool = v111 < 0i32;
                let v150: i32 = if v148 {
                    let v149: i32 = v111.wrapping_neg();
                    v149
                } else {
                    v111
                };
                let v151: bool = v150 < 10i32;
                let v155: Rc<str> = if v151 {
                    let v152: Rc<str> = digits_text_30(v150, v1.clone());
                    let v153: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v152.clone()));
                    v153.clone()
                } else {
                    digits_text_30(v150, v1.clone())
                };
                let v158: Rc<str> = if v148 {
                    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("e-"); } LIT.with(|lit| lit.clone()) };
                    v156.clone()
                } else {
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("e+"); } LIT.with(|lit| lit.clone()) };
                    v157.clone()
                };
                let v159: Rc<str> = string_slice(&v106.clone(), 0i32 as i64, 0i32 as i64);
                let v160: Rc<str> = Rc::<str>::from(format!("{}{}", v158.clone(), v155.clone()));
                let v161: Rc<str> = Rc::<str>::from(format!("{}{}", v147.clone(), v160.clone()));
                let mut v162: Rc<str> = Rc::<str>::from(format!("{}{}", v159.clone(), v161.clone()));
                let v163: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v162.clone()));
                v163.clone()
            }
        }
    };
    method31(v2.clone(), v166.clone());
    let mut v167: Rc<str> = v2.borrow().l0.clone();
    v167.clone()
}
fn method5(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let v4: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
    let mut v6: num_complex::Complex<f64> = num_complex::Complex::new(-1.0f64, 0.0f64);
    let mut v7: Rc<RefCell<Vec<(num_complex::Complex<f64>, f64)>>> = Rc::new(RefCell::new(vec![(v4.clone(), 1.6449340668482264f64), (v6.clone(), -0.08333333333333333f64)]));
    let mut v8: i32 = (v7.clone().borrow().len() as i32);
    let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method6(v8, v9.clone()) {
        let mut v11: i32 = v9.borrow().l0.clone();
        let (mut v12, mut v13): (num_complex::Complex<f64>, f64) = v7.clone().borrow()[v11 as usize].clone();
        let mut v14: num_complex::Complex<f64> = method7(v12.clone());
        let mut v15: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v14.clone());
        let mut v16: num_complex::Complex<f64> = method18(v2.clone(), v12.clone());
        let mut v18: Option<num_complex::Complex<f64>> = v15.ok();
        let v19: Option<num_complex::Complex<f64>> = method22(v18.clone());
        let mut v20: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
        let mut v21: Option<US1> = v19.map(|x| v20(x));
        let v23: US1 = US1::US1_None;
        let v24: US1 = v23.clone();
        let v25: US1 = v21.unwrap_or(v24);
        let mut v27: f64 = f64::NAN;
        let v29: f64 = f64::NAN;
        let v31: num_complex::Complex<f64> = num_complex::Complex::new(v27, v29);
        let mut v34: num_complex::Complex<f64> = match &v25 {
            US1::US1_None => {
                v31.clone()
            }
            US1::US1_Some(v32) => {
                let v32: num_complex::Complex<f64> = v32.clone();
                v32.clone()
            }
        };
        let v36: f64 = v34.im;
        let mut v37: bool = v36 == 0.0f64;
        let mut v39: bool = if v37 {
            true
        } else {
            method23(v37)
        };
        let v73: Rc<str> = if v37 {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v46: f64 = 0.0f64;
            let v47: Rc<str> = format_real_24(v46);
            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v47));
            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v48));
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
            v52.clone()
        } else {
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v59: Rc<str> = format_real_24(v36);
            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v62, v63)), v57));
            let v66: f64 = 0.0f64;
            let v67: Rc<str> = format_real_24(v66);
            let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v67));
            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v68));
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
            v72.clone()
        };
        let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq"); } LIT.with(|lit| lit.clone()) };
        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq "); } LIT.with(|lit| lit.clone()) };
        let v110: Rc<str> = if v37 {
            let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let mut v83: f64 = 0.0f64;
            let v84: Rc<str> = format_real_24(v83);
            let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v84));
            let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let mut v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v85));
            let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v88));
            v89.clone()
        } else {
            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v96: Rc<str> = format_real_24(v36);
            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
            let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v98));
            let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v102: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v99, v100)), v94));
            let v103: f64 = 0.0f64;
            let v104: Rc<str> = format_real_24(v103);
            let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v104));
            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v105));
            let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v108));
            v109.clone()
        };
        let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v110));
        println!("{}", v111);
        let mut v112: bool = v39 == false;
        if v112 {
            std::panic::panic_any::<std::string::String>(format!("{}", v111.clone()))
        };
        let mut v114: f64 = v34.re;
        let v115: f64 = v114 - v13;
        let v116: f64 = -(v115);
        let v117: bool = v115 >= v116;
        let mut v118: f64 = if v117 {
            v115
        } else {
            v116
        };
        let v119: bool = v118 < 0.0001f64;
        let v121: bool = if v119 {
            true
        } else {
            method23(v119)
        };
        let v155: Rc<str> = if v119 {
            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v128: f64 = 0.0001f64;
            let v129: Rc<str> = format_real_24(v128);
            let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v129));
            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v130));
            let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v133));
            v134.clone()
        } else {
            let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v141: Rc<str> = format_real_24(v118);
            let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v141));
            let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v144: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v143));
            let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v147: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v144, v145)), v139));
            let v148: f64 = 0.0001f64;
            let v149: Rc<str> = format_real_24(v148);
            let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v149));
            let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v150));
            let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v153));
            v154.clone()
        };
        let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt"); } LIT.with(|lit| lit.clone()) };
        let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
        let v191: Rc<str> = if v119 {
            let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v164: f64 = 0.0001f64;
            let v165: Rc<str> = format_real_24(v164);
            let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v163, v165));
            let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v167, v166));
            let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v170: Rc<str> = Rc::<str>::from(format!("{}{}", v168, v169));
            v170.clone()
        } else {
            let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v177: Rc<str> = format_real_24(v118);
            let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v176, v177));
            let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v180: Rc<str> = Rc::<str>::from(format!("{}{}", v178, v179));
            let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v183: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v180, v181)), v175));
            let v184: f64 = 0.0001f64;
            let v185: Rc<str> = format_real_24(v184);
            let v186: Rc<str> = Rc::<str>::from(format!("{}{}", v183, v185));
            let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v188: Rc<str> = Rc::<str>::from(format!("{}{}", v187, v186));
            let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let mut v190: Rc<str> = Rc::<str>::from(format!("{}{}", v188, v189));
            v190.clone()
        };
        let v192: Rc<str> = Rc::<str>::from(format!("{}{}", v157, v191));
        println!("{}", v192);
        let v193: bool = v121 == false;
        if v193 {
            std::panic::panic_any::<std::string::String>(format!("{}", v192.clone()))
        };
        let mut v194: i32 = v11.wrapping_add(1i32);
        v9.borrow_mut().l0 = v194;
        ()
    };
    ()
}
fn method4() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method5(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method33(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let v4: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, -2.0f64);
    let mut v5: num_complex::Complex<f64> = method7(v4.clone());
    let mut v6: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v5.clone());
    let mut v7: num_complex::Complex<f64> = method18(v2.clone(), v4.clone());
    let mut v9: Option<num_complex::Complex<f64>> = v6.ok();
    let v10: Option<num_complex::Complex<f64>> = method22(v9.clone());
    let mut v11: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let mut v14: US1 = US1::US1_None;
    let mut v15: US1 = v14.clone();
    let mut v16: US1 = v12.unwrap_or(v15);
    let mut v18: f64 = f64::NAN;
    let mut v20: f64 = f64::NAN;
    let mut v22: num_complex::Complex<f64> = num_complex::Complex::new(v18, v20);
    let v25: num_complex::Complex<f64> = match &v16 {
        US1::US1_None => {
            v22.clone()
        }
        US1::US1_Some(v23) => {
            let v23: num_complex::Complex<f64> = v23.clone();
            v23.clone()
        }
    };
    let mut v27: f64 = v25.re;
    let mut v28: f64 = v27 - 0.8673f64;
    let v29: f64 = -(v28);
    let v30: bool = v28 >= v29;
    let v31: f64 = if v30 {
        v28
    } else {
        v29
    };
    let v32: bool = v31 < 0.001f64;
    let mut v34: bool = if v32 {
        true
    } else {
        method23(v32)
    };
    let v68: Rc<str> = if v32 {
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let mut v41: f64 = 0.001f64;
        let mut v42: Rc<str> = format_real_24(v41);
        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v42));
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v43));
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
        v47.clone()
    } else {
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v54: Rc<str> = format_real_24(v31);
        let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v57, v58)), v52));
        let v61: f64 = 0.001f64;
        let mut v62: Rc<str> = format_real_24(v61);
        let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v62));
        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v63));
        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
        v67.clone()
    };
    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt"); } LIT.with(|lit| lit.clone()) };
    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
    let v105: Rc<str> = if v32 {
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v78: f64 = 0.001f64;
        let v79: Rc<str> = format_real_24(v78);
        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v79));
        let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v80));
        let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v83));
        v84.clone()
    } else {
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let mut v91: Rc<str> = format_real_24(v31);
        let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
        let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v97: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v94, v95)), v89));
        let v98: f64 = 0.001f64;
        let mut v99: Rc<str> = format_real_24(v98);
        let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v99));
        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v100));
        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v103));
        v104.clone()
    };
    let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v105));
    println!("{}", v106);
    let v107: bool = v34 == false;
    if v107 {
        std::panic::panic_any::<std::string::String>(format!("{}", v106.clone()))
    };
    let v109: f64 = v25.im;
    let v110: f64 = v109 - 0.275f64;
    let v111: f64 = -(v110);
    let mut v112: bool = v110 >= v111;
    let v113: f64 = if v112 {
        v110
    } else {
        v111
    };
    let mut v114: bool = v113 < 0.001f64;
    let v116: bool = if v114 {
        true
    } else {
        method23(v114)
    };
    let v150: Rc<str> = if v114 {
        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v123: f64 = 0.001f64;
        let v124: Rc<str> = format_real_24(v123);
        let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v124));
        let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v125));
        let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v128));
        v129.clone()
    } else {
        let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v136: Rc<str> = format_real_24(v113);
        let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v137, v138));
        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v142: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v139, v140)), v134));
        let v143: f64 = 0.001f64;
        let v144: Rc<str> = format_real_24(v143);
        let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v144));
        let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v146, v145));
        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v149: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v148));
        v149.clone()
    };
    let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
    let v185: Rc<str> = if v114 {
        let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v158: f64 = 0.001f64;
        let v159: Rc<str> = format_real_24(v158);
        let v160: Rc<str> = Rc::<str>::from(format!("{}{}", v157, v159));
        let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let mut v162: Rc<str> = Rc::<str>::from(format!("{}{}", v161, v160));
        let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
        v164.clone()
    } else {
        let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v171: Rc<str> = format_real_24(v113);
        let v172: Rc<str> = Rc::<str>::from(format!("{}{}", v170, v171));
        let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v174: Rc<str> = Rc::<str>::from(format!("{}{}", v172, v173));
        let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v177: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v174, v175)), v169));
        let v178: f64 = 0.001f64;
        let v179: Rc<str> = format_real_24(v178);
        let v180: Rc<str> = Rc::<str>::from(format!("{}{}", v177, v179));
        let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v182: Rc<str> = Rc::<str>::from(format!("{}{}", v181, v180));
        let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v184: Rc<str> = Rc::<str>::from(format!("{}{}", v182, v183));
        v184.clone()
    };
    let v186: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v185));
    println!("{}", v186);
    let v187: bool = v116 == false;
    if v187 {
        std::panic::panic_any::<std::string::String>(format!("{}", v186.clone()))
    }
}
fn method32() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method33(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method36(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: Rc<UH0>) -> () {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: f64 = *v3;
                let v4: Rc<UH0> = v4.clone();
                let mut v6: num_complex::Complex<f64> = num_complex::Complex::new(v3, 0.0f64);
                let mut v7: num_complex::Complex<f64> = method7(v6.clone());
                let mut v8: Result<num_complex::Complex<f64>, std::string::String> = method8(v0.clone(), v7.clone());
                let mut v9: num_complex::Complex<f64> = method18(v1.clone(), v6.clone());
                let mut v11: Option<num_complex::Complex<f64>> = v8.ok();
                let mut v12: Option<num_complex::Complex<f64>> = method22(v11.clone());
                let mut v13: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
                let mut v14: Option<US1> = v12.map(|x| v13(x));
                let mut v16: US1 = US1::US1_None;
                let mut v17: US1 = v16.clone();
                let mut v18: US1 = v14.unwrap_or(v17);
                let mut v20: f64 = f64::NAN;
                let mut v22: f64 = f64::NAN;
                let v24: num_complex::Complex<f64> = num_complex::Complex::new(v20, v22);
                let mut v27: num_complex::Complex<f64> = match &v18 {
                    US1::US1_None => {
                        v24.clone()
                    }
                    US1::US1_Some(v25) => {
                        let v25: num_complex::Complex<f64> = v25.clone();
                        v25.clone()
                    }
                };
                let v29: f64 = v27.re;
                let v30: bool = v29 == 0.0f64;
                let v32: bool = if v30 {
                    true
                } else {
                    method23(v30)
                };
                let v66: Rc<str> = if v30 {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let mut v39: f64 = 0.0f64;
                    let v40: Rc<str> = format_real_24(v39);
                    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v40));
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v41));
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
                    v45.clone()
                } else {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let mut v52: Rc<str> = format_real_24(v29);
                    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v55, v56)), v50));
                    let v59: f64 = 0.0f64;
                    let v60: Rc<str> = format_real_24(v59);
                    let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v60));
                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v61));
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
                    v65.clone()
                };
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq"); } LIT.with(|lit| lit.clone()) };
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq "); } LIT.with(|lit| lit.clone()) };
                let v103: Rc<str> = if v30 {
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v76: f64 = 0.0f64;
                    let v77: Rc<str> = format_real_24(v76);
                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v77));
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v78));
                    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
                    v82.clone()
                } else {
                    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v89: Rc<str> = format_real_24(v29);
                    let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v89));
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v95: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v92, v93)), v87));
                    let v96: f64 = 0.0f64;
                    let v97: Rc<str> = format_real_24(v96);
                    let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v97));
                    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v98));
                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v101));
                    v102.clone()
                };
                let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v103));
                println!("{}", v104);
                let v105: bool = v32 == false;
                if v105 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v104.clone()))
                };
                let v107: f64 = v27.im;
                let v108: bool = v107 == 0.0f64;
                let v110: bool = if v108 {
                    true
                } else {
                    method23(v108)
                };
                let v144: Rc<str> = if v108 {
                    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v117: f64 = 0.0f64;
                    let mut v118: Rc<str> = format_real_24(v117);
                    let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v118));
                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v119));
                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
                    v123.clone()
                } else {
                    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = format_real_24(v107);
                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v132));
                    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v136: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v133, v134)), v128));
                    let v137: f64 = 0.0f64;
                    let v138: Rc<str> = format_real_24(v137);
                    let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v138));
                    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v139));
                    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v142));
                    v143.clone()
                };
                let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq "); } LIT.with(|lit| lit.clone()) };
                let v179: Rc<str> = if v108 {
                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v152: f64 = 0.0f64;
                    let v153: Rc<str> = format_real_24(v152);
                    let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v153));
                    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v155, v154));
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v158: Rc<str> = Rc::<str>::from(format!("{}{}", v156, v157));
                    v158.clone()
                } else {
                    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v165: Rc<str> = format_real_24(v107);
                    let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v165));
                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v166, v167));
                    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v171: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v168, v169)), v163));
                    let v172: f64 = 0.0f64;
                    let v173: Rc<str> = format_real_24(v172);
                    let v174: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v173));
                    let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v174));
                    let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v176, v177));
                    v178.clone()
                };
                let v180: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v179));
                println!("{}", v180);
                let v181: bool = v110 == false;
                if v181 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v180.clone()))
                };
                (v0, v1, v2) = (v0.clone(), v1.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method37() -> Rc<UH0> {
    let mut v0: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v1: Rc<UH0> = Rc::new(UH0::UH0_Cons(-40.0f64, v0.clone()));
    let mut v2: Rc<UH0> = Rc::new(UH0::UH0_Cons(-38.0f64, v1.clone()));
    let mut v3: Rc<UH0> = Rc::new(UH0::UH0_Cons(-36.0f64, v2.clone()));
    let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(-34.0f64, v3.clone()));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(-32.0f64, v4.clone()));
    let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(-30.0f64, v5.clone()));
    let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(-28.0f64, v6.clone()));
    let mut v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(-26.0f64, v7.clone()));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(-24.0f64, v8.clone()));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(-22.0f64, v9.clone()));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(-20.0f64, v10.clone()));
    let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(-18.0f64, v11.clone()));
    let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(-16.0f64, v12.clone()));
    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(-14.0f64, v13.clone()));
    let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(-12.0f64, v14.clone()));
    let mut v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(-10.0f64, v15.clone()));
    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(-8.0f64, v16.clone()));
    let mut v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(-6.0f64, v17.clone()));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(-4.0f64, v18.clone()));
    Rc::new(UH0::UH0_Cons(-2.0f64, v19.clone()))
}
fn method35(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let mut v3: Rc<UH0> = method37();
    method36(v1, v2, v3)
}
fn method34() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method35(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method39(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let v4: num_complex::Complex<f64> = num_complex::Complex::new(0.5f64, 14.134725f64);
    let mut v6: num_complex::Complex<f64> = num_complex::Complex::new(0.5f64, 21.02204f64);
    let mut v8: num_complex::Complex<f64> = num_complex::Complex::new(0.5f64, 25.010857f64);
    let v10: num_complex::Complex<f64> = num_complex::Complex::new(0.5f64, 30.424876f64);
    let mut v12: num_complex::Complex<f64> = num_complex::Complex::new(0.5f64, 32.935062f64);
    let mut v14: num_complex::Complex<f64> = num_complex::Complex::new(0.5f64, 37.586178f64);
    let mut v15: Rc<RefCell<Vec<num_complex::Complex<f64>>>> = Rc::new(RefCell::new(vec![v4.clone(), v6.clone(), v8.clone(), v10.clone(), v12.clone(), v14.clone()]));
    let mut v16: i32 = (v15.clone().borrow().len() as i32);
    let mut v17: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method6(v16, v17.clone()) {
        let v19: i32 = v17.borrow().l0.clone();
        let mut v20: num_complex::Complex<f64> = v15.clone().borrow()[v19 as usize].clone();
        let mut v21: num_complex::Complex<f64> = method7(v20.clone());
        let mut v22: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v21.clone());
        let v23: num_complex::Complex<f64> = method18(v2.clone(), v20.clone());
        let v25: Option<num_complex::Complex<f64>> = v22.ok();
        let v26: Option<num_complex::Complex<f64>> = method22(v25.clone());
        let mut v27: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
        let mut v28: Option<US1> = v26.map(|x| v27(x));
        let v30: US1 = US1::US1_None;
        let v31: US1 = v30.clone();
        let v32: US1 = v28.unwrap_or(v31);
        let mut v34: f64 = f64::NAN;
        let v36: f64 = f64::NAN;
        let v38: num_complex::Complex<f64> = num_complex::Complex::new(v34, v36);
        let mut v41: num_complex::Complex<f64> = match &v32 {
            US1::US1_None => {
                v38.clone()
            }
            US1::US1_Some(v39) => {
                let mut v39: num_complex::Complex<f64> = v39.clone();
                v39.clone()
            }
        };
        let v43: f64 = v41.re;
        let v44: f64 = -(v43);
        let v45: bool = v43 >= v44;
        let v46: f64 = if v45 {
            v43
        } else {
            v44
        };
        let v47: bool = v46 < 0.0001f64;
        let v49: bool = if v47 {
            true
        } else {
            method23(v47)
        };
        let mut v83: Rc<str> = if v47 {
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v56: f64 = 0.0001f64;
            let v57: Rc<str> = format_real_24(v56);
            let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v57));
            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v58));
            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
            v62.clone()
        } else {
            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v69: Rc<str> = format_real_24(v46);
            let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v69));
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
            let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v72, v73)), v67));
            let v76: f64 = 0.0001f64;
            let v77: Rc<str> = format_real_24(v76);
            let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v77));
            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v78));
            let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
            v82.clone()
        };
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt"); } LIT.with(|lit| lit.clone()) };
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
        let v120: Rc<str> = if v47 {
            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v93: f64 = 0.0001f64;
            let v94: Rc<str> = format_real_24(v93);
            let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v94));
            let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v95));
            let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v98));
            v99.clone()
        } else {
            let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v106: Rc<str> = format_real_24(v46);
            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v106));
            let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v108));
            let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v112: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v109, v110)), v104));
            let v113: f64 = 0.0001f64;
            let mut v114: Rc<str> = format_real_24(v113);
            let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v114));
            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v115));
            let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v118));
            v119.clone()
        };
        let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v120));
        println!("{}", v121);
        let v122: bool = v49 == false;
        if v122 {
            std::panic::panic_any::<std::string::String>(format!("{}", v121.clone()))
        };
        let v124: f64 = v41.im;
        let v125: f64 = -(v124);
        let v126: bool = v124 >= v125;
        let v127: f64 = if v126 {
            v124
        } else {
            v125
        };
        let v128: bool = v127 < 0.0001f64;
        let v130: bool = if v128 {
            true
        } else {
            method23(v128)
        };
        let v164: Rc<str> = if v128 {
            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v137: f64 = 0.0001f64;
            let v138: Rc<str> = format_real_24(v137);
            let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v138));
            let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v139));
            let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v142));
            v143.clone()
        } else {
            let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v150: Rc<str> = format_real_24(v127);
            let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v150));
            let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v153: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v152));
            let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v156: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v153, v154)), v148));
            let v157: f64 = 0.0001f64;
            let v158: Rc<str> = format_real_24(v157);
            let v159: Rc<str> = Rc::<str>::from(format!("{}{}", v156, v158));
            let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v161: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v159));
            let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v163: Rc<str> = Rc::<str>::from(format!("{}{}", v161, v162));
            v163.clone()
        };
        let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
        let v199: Rc<str> = if v128 {
            let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v172: f64 = 0.0001f64;
            let v173: Rc<str> = format_real_24(v172);
            let v174: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v173));
            let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v174));
            let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v176, v177));
            v178.clone()
        } else {
            let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v185: Rc<str> = format_real_24(v127);
            let v186: Rc<str> = Rc::<str>::from(format!("{}{}", v184, v185));
            let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v188: Rc<str> = Rc::<str>::from(format!("{}{}", v186, v187));
            let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v191: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v188, v189)), v183));
            let v192: f64 = 0.0001f64;
            let v193: Rc<str> = format_real_24(v192);
            let mut v194: Rc<str> = Rc::<str>::from(format!("{}{}", v191, v193));
            let v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v196: Rc<str> = Rc::<str>::from(format!("{}{}", v195, v194));
            let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v198: Rc<str> = Rc::<str>::from(format!("{}{}", v196, v197));
            v198.clone()
        };
        let v200: Rc<str> = Rc::<str>::from(format!("{}{}", v165, v199));
        println!("{}", v200);
        let v201: bool = v130 == false;
        if v201 {
            std::panic::panic_any::<std::string::String>(format!("{}", v200.clone()))
        };
        let v202: i32 = v19.wrapping_add(1i32);
        v17.borrow_mut().l0 = v202;
        ()
    };
    ()
}
fn method38() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method39(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method41(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let mut v3: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(vec![2.0f64, 3.0f64, 4.0f64, 5.0f64, 10.0f64, 20.0f64, 50.0f64]));
    let v4: i32 = (v3.clone().borrow().len() as i32);
    let mut v5: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method6(v4, v5.clone()) {
        let mut v7: i32 = v5.borrow().l0.clone();
        let mut v8: f64 = v3.clone().borrow()[v7 as usize].clone();
        let v10: num_complex::Complex<f64> = num_complex::Complex::new(v8, 0.0f64);
        let mut v11: num_complex::Complex<f64> = method7(v10.clone());
        let mut v12: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v11.clone());
        let mut v13: num_complex::Complex<f64> = method18(v2.clone(), v10.clone());
        let mut v15: Option<num_complex::Complex<f64>> = v12.ok();
        let mut v16: Option<num_complex::Complex<f64>> = method22(v15.clone());
        let mut v17: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
        let mut v18: Option<US1> = v16.map(|x| v17(x));
        let mut v20: US1 = US1::US1_None;
        let mut v21: US1 = v20.clone();
        let mut v22: US1 = v18.unwrap_or(v21);
        let v24: f64 = f64::NAN;
        let v26: f64 = f64::NAN;
        let mut v28: num_complex::Complex<f64> = num_complex::Complex::new(v24, v26);
        let v31: num_complex::Complex<f64> = match &v22 {
            US1::US1_None => {
                v28.clone()
            }
            US1::US1_Some(v29) => {
                let v29: num_complex::Complex<f64> = v29.clone();
                v29.clone()
            }
        };
        let v33: f64 = v31.re;
        let mut v34: bool = v33 > 0.0f64;
        let v36: bool = if v34 {
            true
        } else {
            method23(v34)
        };
        let v70: Rc<str> = if v34 {
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v43: f64 = 0.0f64;
            let v44: Rc<str> = format_real_24(v43);
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v44));
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v45));
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
            v49.clone()
        } else {
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v56: Rc<str> = format_real_24(v33);
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
            let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v58));
            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v59, v60)), v54));
            let v63: f64 = 0.0f64;
            let v64: Rc<str> = format_real_24(v63);
            let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v64));
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v65));
            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
            v69.clone()
        };
        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_gt"); } LIT.with(|lit| lit.clone()) };
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_gt "); } LIT.with(|lit| lit.clone()) };
        let v107: Rc<str> = if v34 {
            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v80: f64 = 0.0f64;
            let mut v81: Rc<str> = format_real_24(v80);
            let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v81));
            let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v82));
            let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let mut v86: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v85));
            v86.clone()
        } else {
            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v93: Rc<str> = format_real_24(v33);
            let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v95));
            let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v96, v97)), v91));
            let v100: f64 = 0.0f64;
            let v101: Rc<str> = format_real_24(v100);
            let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v101));
            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v102));
            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
            v106.clone()
        };
        let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v107));
        println!("{}", v108);
        let v109: bool = v36 == false;
        if v109 {
            std::panic::panic_any::<std::string::String>(format!("{}", v108.clone()))
        };
        let v111: f64 = v31.im;
        let mut v112: bool = v111 == 0.0f64;
        let mut v114: bool = if v112 {
            true
        } else {
            method23(v112)
        };
        let v148: Rc<str> = if v112 {
            let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v121: f64 = 0.0f64;
            let v122: Rc<str> = format_real_24(v121);
            let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v122));
            let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v124, v123));
            let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v126));
            v127.clone()
        } else {
            let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v134: Rc<str> = format_real_24(v111);
            let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v134));
            let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
            let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v140: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v137, v138)), v132));
            let v141: f64 = 0.0f64;
            let v142: Rc<str> = format_real_24(v141);
            let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v142));
            let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v143));
            let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v146));
            v147.clone()
        };
        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq"); } LIT.with(|lit| lit.clone()) };
        let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq "); } LIT.with(|lit| lit.clone()) };
        let v184: Rc<str> = if v112 {
            let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
            let v157: f64 = 0.0f64;
            let v158: Rc<str> = format_real_24(v157);
            let v159: Rc<str> = Rc::<str>::from(format!("{}{}", v156, v158));
            let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v161: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v159));
            let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v163: Rc<str> = Rc::<str>::from(format!("{}{}", v161, v162));
            v163.clone()
        } else {
            let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
            let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
            let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
            let v170: Rc<str> = format_real_24(v111);
            let v171: Rc<str> = Rc::<str>::from(format!("{}{}", v169, v170));
            let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v173: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v172));
            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v176: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v173, v174)), v168));
            let v177: f64 = 0.0f64;
            let v178: Rc<str> = format_real_24(v177);
            let v179: Rc<str> = Rc::<str>::from(format!("{}{}", v176, v178));
            let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
            let v181: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v179));
            let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
            let v183: Rc<str> = Rc::<str>::from(format!("{}{}", v181, v182));
            v183.clone()
        };
        let v185: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v184));
        println!("{}", v185);
        let v186: bool = v114 == false;
        if v186 {
            std::panic::panic_any::<std::string::String>(format!("{}", v185.clone()))
        };
        let v187: i32 = v7.wrapping_add(1i32);
        v5.borrow_mut().l0 = v187;
        ()
    };
    ()
}
fn method40() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method41(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method43(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let v4: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
    let mut v5: num_complex::Complex<f64> = method7(v4.clone());
    let mut v6: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v5.clone());
    let mut v7: num_complex::Complex<f64> = method18(v2.clone(), v4.clone());
    let mut v9: Option<num_complex::Complex<f64>> = v6.ok();
    let v10: Option<num_complex::Complex<f64>> = method22(v9.clone());
    let mut v11: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let mut v14: US1 = US1::US1_None;
    let mut v15: US1 = v14.clone();
    let mut v16: US1 = v12.unwrap_or(v15);
    let mut v18: f64 = f64::NAN;
    let mut v20: f64 = f64::NAN;
    let mut v22: num_complex::Complex<f64> = num_complex::Complex::new(v18, v20);
    let v25: num_complex::Complex<f64> = match &v16 {
        US1::US1_None => {
            v22.clone()
        }
        US1::US1_Some(v23) => {
            let v23: num_complex::Complex<f64> = v23.clone();
            v23.clone()
        }
    };
    let mut v27: f64 = v25.re;
    let mut v28: bool = v27 == f64::INFINITY;
    let v30: bool = if v28 {
        true
    } else {
        method23(v28)
    };
    let v64: Rc<str> = if v28 {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let mut v37: f64 = f64::INFINITY;
        let v38: Rc<str> = format_real_24(v37);
        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v38));
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v39));
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
        v43.clone()
    } else {
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = format_real_24(v27);
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v53, v54)), v48));
        let v57: f64 = f64::INFINITY;
        let mut v58: Rc<str> = format_real_24(v57);
        let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v58));
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v59));
        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
        v63.clone()
    };
    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq"); } LIT.with(|lit| lit.clone()) };
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq "); } LIT.with(|lit| lit.clone()) };
    let v101: Rc<str> = if v28 {
        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v74: f64 = f64::INFINITY;
        let v75: Rc<str> = format_real_24(v74);
        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v75));
        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v76));
        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
        v80.clone()
    } else {
        let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let mut v87: Rc<str> = format_real_24(v27);
        let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
        let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v89));
        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v93: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v90, v91)), v85));
        let v94: f64 = f64::INFINITY;
        let v95: Rc<str> = format_real_24(v94);
        let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v95));
        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v96));
        let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
        v100.clone()
    };
    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v101));
    println!("{}", v102);
    let v103: bool = v30 == false;
    if v103 {
        std::panic::panic_any::<std::string::String>(format!("{}", v102.clone()))
    };
    let v105: f64 = v25.im;
    let v106: bool = v105 == 0.0f64;
    let v108: bool = if v106 {
        true
    } else {
        method23(v106)
    };
    let v142: Rc<str> = if v106 {
        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v115: f64 = 0.0f64;
        let v116: Rc<str> = format_real_24(v115);
        let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v116));
        let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v117));
        let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v120));
        v121.clone()
    } else {
        let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v128: Rc<str> = format_real_24(v105);
        let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v128));
        let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
        let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v134: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v131, v132)), v126));
        let v135: f64 = 0.0f64;
        let v136: Rc<str> = format_real_24(v135);
        let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v136));
        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v137));
        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v140));
        v141.clone()
    };
    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq "); } LIT.with(|lit| lit.clone()) };
    let v177: Rc<str> = if v106 {
        let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v150: f64 = 0.0f64;
        let v151: Rc<str> = format_real_24(v150);
        let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v151));
        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v153, v152));
        let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v154, v155));
        v156.clone()
    } else {
        let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v163: Rc<str> = format_real_24(v105);
        let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
        let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v165));
        let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v169: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v166, v167)), v161));
        let v170: f64 = 0.0f64;
        let v171: Rc<str> = format_real_24(v170);
        let v172: Rc<str> = Rc::<str>::from(format!("{}{}", v169, v171));
        let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v174: Rc<str> = Rc::<str>::from(format!("{}{}", v173, v172));
        let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v174, v175));
        v176.clone()
    };
    let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v177));
    println!("{}", v178);
    let v179: bool = v108 == false;
    if v179 {
        std::panic::panic_any::<std::string::String>(format!("{}", v178.clone()))
    }
}
fn method42() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method43(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method45(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let v4: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 10.0f64);
    let mut v5: num_complex::Complex<f64> = method7(v4.clone());
    let mut v6: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v5.clone());
    let mut v7: num_complex::Complex<f64> = method18(v2.clone(), v4.clone());
    let mut v9: Option<num_complex::Complex<f64>> = v6.ok();
    let v10: Option<num_complex::Complex<f64>> = method22(v9.clone());
    let mut v11: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let mut v14: US1 = US1::US1_None;
    let mut v15: US1 = v14.clone();
    let mut v16: US1 = v12.unwrap_or(v15);
    let mut v18: f64 = f64::NAN;
    let mut v20: f64 = f64::NAN;
    let mut v22: num_complex::Complex<f64> = num_complex::Complex::new(v18, v20);
    let v25: num_complex::Complex<f64> = match &v16 {
        US1::US1_None => {
            v22.clone()
        }
        US1::US1_Some(v23) => {
            let v23: num_complex::Complex<f64> = v23.clone();
            v23.clone()
        }
    };
    let mut v27: f64 = v4.re;
    let v29: f64 = v4.im;
    let v30: f64 = -(v29);
    let v32: num_complex::Complex<f64> = num_complex::Complex::new(v27, v30);
    let v33: num_complex::Complex<f64> = method7(v32.clone());
    let mut v34: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v33.clone());
    let v35: num_complex::Complex<f64> = method18(v2.clone(), v32.clone());
    let mut v37: Option<num_complex::Complex<f64>> = v34.ok();
    let v38: Option<num_complex::Complex<f64>> = method22(v37.clone());
    let mut v39: Option<US1> = v38.map(|x| v11(x));
    let mut v41: US1 = US1::US1_None;
    let mut v42: US1 = v41.clone();
    let v43: US1 = v39.unwrap_or(v42);
    let v45: f64 = f64::NAN;
    let v47: f64 = f64::NAN;
    let v49: num_complex::Complex<f64> = num_complex::Complex::new(v45, v47);
    let mut v52: num_complex::Complex<f64> = match &v43 {
        US1::US1_None => {
            v49.clone()
        }
        US1::US1_Some(v50) => {
            let v50: num_complex::Complex<f64> = v50.clone();
            v50.clone()
        }
    };
    let v54: num_complex::Complex<f64> = v52.conj();
    let v56: f64 = v25.re;
    let mut v58: f64 = v54.re;
    let v59: bool = v56 == v58;
    let v61: bool = if v59 {
        true
    } else {
        method23(v59)
    };
    let v93: Rc<str> = if v59 {
        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v68: Rc<str> = format_real_24(v58);
        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v69));
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v72));
        v73.clone()
    } else {
        let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v80: Rc<str> = format_real_24(v56);
        let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
        let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let mut v86: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v83, v84)), v78));
        let mut v87: Rc<str> = format_real_24(v58);
        let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
        let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v88));
        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
        v92.clone()
    };
    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq"); } LIT.with(|lit| lit.clone()) };
    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq "); } LIT.with(|lit| lit.clone()) };
    let v128: Rc<str> = if v59 {
        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v103: Rc<str> = format_real_24(v58);
        let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v103));
        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v104));
        let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v107));
        v108.clone()
    } else {
        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v115: Rc<str> = format_real_24(v56);
        let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v115));
        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let mut v118: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v117));
        let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v121: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v118, v119)), v113));
        let v122: Rc<str> = format_real_24(v58);
        let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
        let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v124, v123));
        let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v126));
        v127.clone()
    };
    let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v128));
    println!("{}", v129);
    let v130: bool = v61 == false;
    if v130 {
        std::panic::panic_any::<std::string::String>(format!("{}", v129.clone()))
    };
    let v132: f64 = v25.im;
    let v134: f64 = v54.im;
    let v135: bool = v132 == v134;
    let v137: bool = if v135 {
        true
    } else {
        method23(v135)
    };
    let v169: Rc<str> = if v135 {
        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v144: Rc<str> = format_real_24(v134);
        let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v144));
        let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v146, v145));
        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v149: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v148));
        v149.clone()
    } else {
        let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v156: Rc<str> = format_real_24(v132);
        let v157: Rc<str> = Rc::<str>::from(format!("{}{}", v155, v156));
        let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v159: Rc<str> = Rc::<str>::from(format!("{}{}", v157, v158));
        let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let mut v162: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v159, v160)), v154));
        let v163: Rc<str> = format_real_24(v134);
        let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
        let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v165, v164));
        let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v166, v167));
        v168.clone()
    };
    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_eq "); } LIT.with(|lit| lit.clone()) };
    let v202: Rc<str> = if v135 {
        let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v177: Rc<str> = format_real_24(v134);
        let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v176, v177));
        let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v180: Rc<str> = Rc::<str>::from(format!("{}{}", v179, v178));
        let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v182: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v181));
        v182.clone()
    } else {
        let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v189: Rc<str> = format_real_24(v132);
        let mut v190: Rc<str> = Rc::<str>::from(format!("{}{}", v188, v189));
        let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v192: Rc<str> = Rc::<str>::from(format!("{}{}", v190, v191));
        let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v195: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v192, v193)), v187));
        let v196: Rc<str> = format_real_24(v134);
        let v197: Rc<str> = Rc::<str>::from(format!("{}{}", v195, v196));
        let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v199: Rc<str> = Rc::<str>::from(format!("{}{}", v198, v197));
        let v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v201: Rc<str> = Rc::<str>::from(format!("{}{}", v199, v200));
        v201.clone()
    };
    let v203: Rc<str> = Rc::<str>::from(format!("{}{}", v170, v202));
    println!("{}", v203);
    let v204: bool = v137 == false;
    if v204 {
        std::panic::panic_any::<std::string::String>(format!("{}", v203.clone()))
    }
}
fn method44() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method45(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method47(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let v4: num_complex::Complex<f64> = num_complex::Complex::new(0.01f64, 0.01f64);
    let mut v5: num_complex::Complex<f64> = method7(v4.clone());
    let mut v6: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v5.clone());
    let mut v7: num_complex::Complex<f64> = method18(v2.clone(), v4.clone());
    let mut v9: Option<num_complex::Complex<f64>> = v6.ok();
    let v10: Option<num_complex::Complex<f64>> = method22(v9.clone());
    let mut v11: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let mut v14: US1 = US1::US1_None;
    let mut v15: US1 = v14.clone();
    let mut v16: US1 = v12.unwrap_or(v15);
    let mut v18: f64 = f64::NAN;
    let mut v20: f64 = f64::NAN;
    let mut v22: num_complex::Complex<f64> = num_complex::Complex::new(v18, v20);
    let v25: num_complex::Complex<f64> = match &v16 {
        US1::US1_None => {
            v22.clone()
        }
        US1::US1_Some(v23) => {
            let v23: num_complex::Complex<f64> = v23.clone();
            v23.clone()
        }
    };
    let mut v27: f64 = v25.re;
    let mut v28: bool = v27 < f64::INFINITY;
    let v30: bool = if v28 {
        true
    } else {
        method23(v28)
    };
    let v64: Rc<str> = if v28 {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let mut v37: f64 = f64::INFINITY;
        let v38: Rc<str> = format_real_24(v37);
        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v38));
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v39));
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
        v43.clone()
    } else {
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = format_real_24(v27);
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v53, v54)), v48));
        let v57: f64 = f64::INFINITY;
        let mut v58: Rc<str> = format_real_24(v57);
        let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v58));
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v59));
        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
        v63.clone()
    };
    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt"); } LIT.with(|lit| lit.clone()) };
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
    let v101: Rc<str> = if v28 {
        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v74: f64 = f64::INFINITY;
        let v75: Rc<str> = format_real_24(v74);
        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v75));
        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v76));
        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
        v80.clone()
    } else {
        let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let mut v87: Rc<str> = format_real_24(v27);
        let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
        let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v89));
        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v93: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v90, v91)), v85));
        let v94: f64 = f64::INFINITY;
        let v95: Rc<str> = format_real_24(v94);
        let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v95));
        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v96));
        let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
        v100.clone()
    };
    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v101));
    println!("{}", v102);
    let v103: bool = v30 == false;
    if v103 {
        std::panic::panic_any::<std::string::String>(format!("{}", v102.clone()))
    };
    let v105: f64 = v25.im;
    let v106: bool = v105 < f64::INFINITY;
    let v108: bool = if v106 {
        true
    } else {
        method23(v106)
    };
    let v142: Rc<str> = if v106 {
        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v115: f64 = f64::INFINITY;
        let v116: Rc<str> = format_real_24(v115);
        let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v116));
        let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v117));
        let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v120));
        v121.clone()
    } else {
        let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v128: Rc<str> = format_real_24(v105);
        let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v128));
        let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
        let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v134: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v131, v132)), v126));
        let v135: f64 = f64::INFINITY;
        let v136: Rc<str> = format_real_24(v135);
        let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v136));
        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v137));
        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v140));
        v141.clone()
    };
    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
    let v177: Rc<str> = if v106 {
        let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
        let v150: f64 = f64::INFINITY;
        let v151: Rc<str> = format_real_24(v150);
        let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v151));
        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v153, v152));
        let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v154, v155));
        v156.clone()
    } else {
        let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
        let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
        let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
        let v163: Rc<str> = format_real_24(v105);
        let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
        let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v165));
        let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
        let v169: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v166, v167)), v161));
        let v170: f64 = f64::INFINITY;
        let v171: Rc<str> = format_real_24(v170);
        let v172: Rc<str> = Rc::<str>::from(format!("{}{}", v169, v171));
        let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
        let v174: Rc<str> = Rc::<str>::from(format!("{}{}", v173, v172));
        let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
        let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v174, v175));
        v176.clone()
    };
    let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v177));
    println!("{}", v178);
    let v179: bool = v108 == false;
    if v179 {
        std::panic::panic_any::<std::string::String>(format!("{}", v178.clone()))
    }
}
fn method46() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method47(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method50(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: Rc<UH0>) -> () {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: f64 = *v3;
                let v4: Rc<UH0> = v4.clone();
                let mut v6: num_complex::Complex<f64> = num_complex::Complex::new(0.0f64, v3);
                let mut v7: num_complex::Complex<f64> = method7(v6.clone());
                let mut v8: Result<num_complex::Complex<f64>, std::string::String> = method8(v0.clone(), v7.clone());
                let mut v9: num_complex::Complex<f64> = method18(v1.clone(), v6.clone());
                let mut v11: Option<num_complex::Complex<f64>> = v8.ok();
                let mut v12: Option<num_complex::Complex<f64>> = method22(v11.clone());
                let mut v13: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
                let mut v14: Option<US1> = v12.map(|x| v13(x));
                let mut v16: US1 = US1::US1_None;
                let mut v17: US1 = v16.clone();
                let mut v18: US1 = v14.unwrap_or(v17);
                let mut v20: f64 = f64::NAN;
                let mut v22: f64 = f64::NAN;
                let v24: num_complex::Complex<f64> = num_complex::Complex::new(v20, v22);
                let mut v27: num_complex::Complex<f64> = match &v18 {
                    US1::US1_None => {
                        v24.clone()
                    }
                    US1::US1_Some(v25) => {
                        let v25: num_complex::Complex<f64> = v25.clone();
                        v25.clone()
                    }
                };
                let v29: f64 = v27.re;
                let v30: bool = v29 != 0.0f64 ;
                let v32: bool = if v30 {
                    true
                } else {
                    method23(v30)
                };
                let v66: Rc<str> = if v30 {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let mut v39: f64 = 0.0f64;
                    let v40: Rc<str> = format_real_24(v39);
                    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v40));
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v41));
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
                    v45.clone()
                } else {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let mut v52: Rc<str> = format_real_24(v29);
                    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v55, v56)), v50));
                    let v59: f64 = 0.0f64;
                    let v60: Rc<str> = format_real_24(v59);
                    let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v60));
                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v61));
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
                    v65.clone()
                };
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_ne"); } LIT.with(|lit| lit.clone()) };
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_ne "); } LIT.with(|lit| lit.clone()) };
                let v103: Rc<str> = if v30 {
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v76: f64 = 0.0f64;
                    let v77: Rc<str> = format_real_24(v76);
                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v77));
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v78));
                    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
                    v82.clone()
                } else {
                    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v89: Rc<str> = format_real_24(v29);
                    let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v89));
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v95: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v92, v93)), v87));
                    let v96: f64 = 0.0f64;
                    let v97: Rc<str> = format_real_24(v96);
                    let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v97));
                    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v98));
                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v101));
                    v102.clone()
                };
                let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v103));
                println!("{}", v104);
                let v105: bool = v32 == false;
                if v105 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v104.clone()))
                };
                let v107: f64 = v27.im;
                let v108: bool = v107 != 0.0f64 ;
                let v110: bool = if v108 {
                    true
                } else {
                    method23(v108)
                };
                let v144: Rc<str> = if v108 {
                    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v117: f64 = 0.0f64;
                    let mut v118: Rc<str> = format_real_24(v117);
                    let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v118));
                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v119));
                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
                    v123.clone()
                } else {
                    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = format_real_24(v107);
                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v132));
                    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v136: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v133, v134)), v128));
                    let v137: f64 = 0.0f64;
                    let v138: Rc<str> = format_real_24(v137);
                    let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v138));
                    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v139));
                    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v142));
                    v143.clone()
                };
                let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_ne "); } LIT.with(|lit| lit.clone()) };
                let v179: Rc<str> = if v108 {
                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v152: f64 = 0.0f64;
                    let v153: Rc<str> = format_real_24(v152);
                    let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v153));
                    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v155, v154));
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v158: Rc<str> = Rc::<str>::from(format!("{}{}", v156, v157));
                    v158.clone()
                } else {
                    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v165: Rc<str> = format_real_24(v107);
                    let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v165));
                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v166, v167));
                    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v171: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v168, v169)), v163));
                    let v172: f64 = 0.0f64;
                    let v173: Rc<str> = format_real_24(v172);
                    let v174: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v173));
                    let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v174));
                    let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v176, v177));
                    v178.clone()
                };
                let v180: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v179));
                println!("{}", v180);
                let v181: bool = v110 == false;
                if v181 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v180.clone()))
                };
                (v0, v1, v2) = (v0.clone(), v1.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method51() -> Rc<UH0> {
    let mut v0: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v1: Rc<UH0> = Rc::new(UH0::UH0_Cons(100.0f64, v0.clone()));
    let mut v2: Rc<UH0> = Rc::new(UH0::UH0_Cons(90.0f64, v1.clone()));
    let mut v3: Rc<UH0> = Rc::new(UH0::UH0_Cons(80.0f64, v2.clone()));
    let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(70.0f64, v3.clone()));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(60.0f64, v4.clone()));
    let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(50.0f64, v5.clone()));
    let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(40.0f64, v6.clone()));
    let mut v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(30.0f64, v7.clone()));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(20.0f64, v8.clone()));
    Rc::new(UH0::UH0_Cons(10.0f64, v9.clone()))
}
fn method49(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let mut v3: Rc<UH0> = method51();
    method50(v1, v2, v3)
}
fn method48() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method49(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method54(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: Rc<UH1>) -> () {
    loop {
        match &*v2 {
            UH1::UH1_Cons(v3, v4) => {
                let mut v3: num_complex::Complex<f64> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                let mut v5: num_complex::Complex<f64> = method7(v3.clone());
                let mut v6: Result<num_complex::Complex<f64>, std::string::String> = method8(v0.clone(), v5.clone());
                let mut v7: num_complex::Complex<f64> = method18(v1.clone(), v3.clone());
                let mut v9: Option<num_complex::Complex<f64>> = v6.ok();
                let v10: Option<num_complex::Complex<f64>> = method22(v9.clone());
                let mut v11: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
                let mut v12: Option<US1> = v10.map(|x| v11(x));
                let mut v14: US1 = US1::US1_None;
                let mut v15: US1 = v14.clone();
                let mut v16: US1 = v12.unwrap_or(v15);
                let mut v18: f64 = f64::NAN;
                let mut v20: f64 = f64::NAN;
                let mut v22: num_complex::Complex<f64> = num_complex::Complex::new(v18, v20);
                let v25: num_complex::Complex<f64> = match &v16 {
                    US1::US1_None => {
                        v22.clone()
                    }
                    US1::US1_Some(v23) => {
                        let v23: num_complex::Complex<f64> = v23.clone();
                        v23.clone()
                    }
                };
                let mut v27: f64 = v25.re;
                let mut v28: bool = v27 != 0.0f64 ;
                let v30: bool = if v28 {
                    true
                } else {
                    method23(v28)
                };
                let v64: Rc<str> = if v28 {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let mut v37: f64 = 0.0f64;
                    let v38: Rc<str> = format_real_24(v37);
                    let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v38));
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v39));
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
                    v43.clone()
                } else {
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = format_real_24(v27);
                    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v56: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v53, v54)), v48));
                    let v57: f64 = 0.0f64;
                    let mut v58: Rc<str> = format_real_24(v57);
                    let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v58));
                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v59));
                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
                    v63.clone()
                };
                let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_ne"); } LIT.with(|lit| lit.clone()) };
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_ne "); } LIT.with(|lit| lit.clone()) };
                let v101: Rc<str> = if v28 {
                    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v74: f64 = 0.0f64;
                    let v75: Rc<str> = format_real_24(v74);
                    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v75));
                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v76));
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
                    v80.clone()
                } else {
                    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let mut v87: Rc<str> = format_real_24(v27);
                    let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
                    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v89));
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v93: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v90, v91)), v85));
                    let v94: f64 = 0.0f64;
                    let v95: Rc<str> = format_real_24(v94);
                    let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v95));
                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v96));
                    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
                    v100.clone()
                };
                let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v101));
                println!("{}", v102);
                let v103: bool = v30 == false;
                if v103 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v102.clone()))
                };
                let v105: f64 = v25.im;
                let v106: bool = v105 != 0.0f64 ;
                let v108: bool = if v106 {
                    true
                } else {
                    method23(v106)
                };
                let v142: Rc<str> = if v106 {
                    let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v115: f64 = 0.0f64;
                    let v116: Rc<str> = format_real_24(v115);
                    let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v116));
                    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v117));
                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v120));
                    v121.clone()
                } else {
                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v128: Rc<str> = format_real_24(v105);
                    let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v128));
                    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v131, v132)), v126));
                    let v135: f64 = 0.0f64;
                    let v136: Rc<str> = format_real_24(v135);
                    let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v136));
                    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v137));
                    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v140));
                    v141.clone()
                };
                let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_ne "); } LIT.with(|lit| lit.clone()) };
                let v177: Rc<str> = if v106 {
                    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v150: f64 = 0.0f64;
                    let v151: Rc<str> = format_real_24(v150);
                    let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v151));
                    let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v153, v152));
                    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v154, v155));
                    v156.clone()
                } else {
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v163: Rc<str> = format_real_24(v105);
                    let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
                    let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v165));
                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v169: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v166, v167)), v161));
                    let v170: f64 = 0.0f64;
                    let v171: Rc<str> = format_real_24(v170);
                    let v172: Rc<str> = Rc::<str>::from(format!("{}{}", v169, v171));
                    let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v174: Rc<str> = Rc::<str>::from(format!("{}{}", v173, v172));
                    let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v174, v175));
                    v176.clone()
                };
                let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v177));
                println!("{}", v178);
                let v179: bool = v108 == false;
                if v179 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v178.clone()))
                };
                (v0, v1, v2) = (v0.clone(), v1.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method55() -> Rc<UH1> {
    let v1: num_complex::Complex<f64> = num_complex::Complex::new(0.5f64, 14.134725f64);
    let mut v3: num_complex::Complex<f64> = num_complex::Complex::new(0.75f64, 20.5f64);
    let mut v5: num_complex::Complex<f64> = num_complex::Complex::new(1.25f64, 30.1f64);
    let mut v7: num_complex::Complex<f64> = num_complex::Complex::new(0.25f64, 40.0f64);
    let mut v9: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 50.0f64);
    let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v9.clone(), v10.clone()));
    let mut v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7.clone(), v11.clone()));
    let mut v13: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5.clone(), v12.clone()));
    let mut v14: Rc<UH1> = Rc::new(UH1::UH1_Cons(v3.clone(), v13.clone()));
    Rc::new(UH1::UH1_Cons(v1.clone(), v14.clone()))
}
fn method53(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let mut v3: Rc<UH1> = method55();
    method54(v1, v2, v3)
}
fn method52() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method53(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method58(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python, mut v3: Rc<UH1>) -> () {
    loop {
        match &*v3 {
            UH1::UH1_Cons(v4, v5) => {
                let v4: num_complex::Complex<f64> = v4.clone();
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: num_complex::Complex<f64> = method7(v4.clone());
                let mut v7: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v6.clone());
                let mut v8: num_complex::Complex<f64> = method18(v2.clone(), v4.clone());
                let v10: Option<num_complex::Complex<f64>> = v7.ok();
                let mut v11: Option<num_complex::Complex<f64>> = method22(v10.clone());
                let mut v12: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
                let mut v13: Option<US1> = v11.map(|x| v12(x));
                let mut v15: US1 = US1::US1_None;
                let mut v16: US1 = v15.clone();
                let mut v17: US1 = v13.unwrap_or(v16);
                let v19: f64 = f64::NAN;
                let mut v21: f64 = f64::NAN;
                let v23: num_complex::Complex<f64> = num_complex::Complex::new(v19, v21);
                let v26: num_complex::Complex<f64> = match &v17 {
                    US1::US1_None => {
                        v23.clone()
                    }
                    US1::US1_Some(v24) => {
                        let v24: num_complex::Complex<f64> = v24.clone();
                        v24.clone()
                    }
                };
                let mut v28: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                let v30: num_complex::Complex<f64> = num_complex::Complex::powc(v28, v4.clone());
                let v32: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                let mut v34: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                let v36: num_complex::Complex<f64> = v4 - v34;
                let v38: num_complex::Complex<f64> = num_complex::Complex::powc(v32, v36);
                let v40: num_complex::Complex<f64> = v30 * v38;
                let mut v42: num_complex::Complex<f64> = num_complex::Complex::new(3.141592653589793f64, 0.0f64);
                let v44: num_complex::Complex<f64> = v42 * v4;
                let v46: num_complex::Complex<f64> = num_complex::Complex::new(2.0f64, 0.0f64);
                let v48: num_complex::Complex<f64> = v44 / v46;
                let v50: num_complex::Complex<f64> = v48.sin();
                let mut v52: num_complex::Complex<f64> = v40 * v50;
                let v54: num_complex::Complex<f64> = num_complex::Complex::new(1.0f64, 0.0f64);
                let v56: num_complex::Complex<f64> = v54 - v4;
                let v57: num_complex::Complex<f64> = method7(v56.clone());
                let mut v58: Result<num_complex::Complex<f64>, std::string::String> = method21(v0.clone(), v57.clone());
                let v60: Option<num_complex::Complex<f64>> = v58.ok();
                let v61: Option<num_complex::Complex<f64>> = method22(v60.clone());
                let mut v62: Option<US1> = v61.map(|x| v12(x));
                let v64: US1 = US1::US1_None;
                let mut v65: US1 = v64.clone();
                let v66: US1 = v62.unwrap_or(v65);
                let v68: f64 = f64::NAN;
                let v70: f64 = f64::NAN;
                let v72: num_complex::Complex<f64> = num_complex::Complex::new(v68, v70);
                let v75: num_complex::Complex<f64> = match &v66 {
                    US1::US1_None => {
                        v72.clone()
                    }
                    US1::US1_Some(v73) => {
                        let v73: num_complex::Complex<f64> = v73.clone();
                        v73.clone()
                    }
                };
                let v77: num_complex::Complex<f64> = v52 * v75;
                let v79: f64 = v4.re;
                let v80: f64 = 1.0f64 - v79;
                let v82: f64 = v4.im;
                let mut v83: f64 = -(v82);
                let v85: num_complex::Complex<f64> = num_complex::Complex::new(v80, v83);
                let mut v86: num_complex::Complex<f64> = method7(v85.clone());
                let mut v87: Result<num_complex::Complex<f64>, std::string::String> = method8(v1.clone(), v86.clone());
                let v88: num_complex::Complex<f64> = method18(v2.clone(), v85.clone());
                let v90: Option<num_complex::Complex<f64>> = v87.ok();
                let mut v91: Option<num_complex::Complex<f64>> = method22(v90.clone());
                let mut v92: Option<US1> = v91.map(|x| v12(x));
                let v94: US1 = US1::US1_None;
                let v95: US1 = v94.clone();
                let v96: US1 = v92.unwrap_or(v95);
                let v98: f64 = f64::NAN;
                let v100: f64 = f64::NAN;
                let v102: num_complex::Complex<f64> = num_complex::Complex::new(v98, v100);
                let v105: num_complex::Complex<f64> = match &v96 {
                    US1::US1_None => {
                        v102.clone()
                    }
                    US1::US1_Some(v103) => {
                        let v103: num_complex::Complex<f64> = v103.clone();
                        v103.clone()
                    }
                };
                let v107: num_complex::Complex<f64> = v77 * v105;
                let v109: f64 = v26.re;
                let v111: f64 = v107.re;
                let mut v112: f64 = v109 - v111;
                let v113: f64 = -(v112);
                let mut v114: bool = v112 >= v113;
                let v115: f64 = if v114 {
                    v112
                } else {
                    v113
                };
                let v116: bool = v115 < 0.0001f64;
                let mut v118: bool = if v116 {
                    true
                } else {
                    method23(v116)
                };
                let v152: Rc<str> = if v116 {
                    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v125: f64 = 0.0001f64;
                    let v126: Rc<str> = format_real_24(v125);
                    let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v124, v126));
                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v127));
                    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
                    v131.clone()
                } else {
                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v138: Rc<str> = format_real_24(v115);
                    let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v137, v138));
                    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v140));
                    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v144: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v141, v142)), v136));
                    let v145: f64 = 0.0001f64;
                    let v146: Rc<str> = format_real_24(v145);
                    let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v146));
                    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v149: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v147));
                    let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v150));
                    v151.clone()
                };
                let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt"); } LIT.with(|lit| lit.clone()) };
                let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
                let v189: Rc<str> = if v116 {
                    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let mut v162: f64 = 0.0001f64;
                    let v163: Rc<str> = format_real_24(v162);
                    let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v161, v163));
                    let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v165, v164));
                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v166, v167));
                    v168.clone()
                } else {
                    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v175: Rc<str> = format_real_24(v115);
                    let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v174, v175));
                    let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v176, v177));
                    let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v181: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v178, v179)), v173));
                    let v182: f64 = 0.0001f64;
                    let v183: Rc<str> = format_real_24(v182);
                    let v184: Rc<str> = Rc::<str>::from(format!("{}{}", v181, v183));
                    let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v186: Rc<str> = Rc::<str>::from(format!("{}{}", v185, v184));
                    let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v188: Rc<str> = Rc::<str>::from(format!("{}{}", v186, v187));
                    v188.clone()
                };
                let mut v190: Rc<str> = Rc::<str>::from(format!("{}{}", v155, v189));
                println!("{}", v190);
                let v191: bool = v118 == false;
                if v191 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v190.clone()))
                };
                let v193: f64 = v26.im;
                let v195: f64 = v107.im;
                let v196: f64 = v193 - v195;
                let v197: f64 = -(v196);
                let v198: bool = v196 >= v197;
                let v199: f64 = if v198 {
                    v196
                } else {
                    v197
                };
                let v200: bool = v199 < 0.0001f64;
                let v202: bool = if v200 {
                    true
                } else {
                    method23(v200)
                };
                let v236: Rc<str> = if v200 {
                    let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v209: f64 = 0.0001f64;
                    let v210: Rc<str> = format_real_24(v209);
                    let v211: Rc<str> = Rc::<str>::from(format!("{}{}", v208, v210));
                    let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v213: Rc<str> = Rc::<str>::from(format!("{}{}", v212, v211));
                    let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v215: Rc<str> = Rc::<str>::from(format!("{}{}", v213, v214));
                    v215.clone()
                } else {
                    let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v221: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v222: Rc<str> = format_real_24(v199);
                    let v223: Rc<str> = Rc::<str>::from(format!("{}{}", v221, v222));
                    let v224: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v225: Rc<str> = Rc::<str>::from(format!("{}{}", v223, v224));
                    let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v228: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v225, v226)), v220));
                    let v229: f64 = 0.0001f64;
                    let v230: Rc<str> = format_real_24(v229);
                    let v231: Rc<str> = Rc::<str>::from(format!("{}{}", v228, v230));
                    let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v233: Rc<str> = Rc::<str>::from(format!("{}{}", v232, v231));
                    let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v235: Rc<str> = Rc::<str>::from(format!("{}{}", v233, v234));
                    v235.clone()
                };
                let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
                let v271: Rc<str> = if v200 {
                    let mut v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let mut v243: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v244: f64 = 0.0001f64;
                    let v245: Rc<str> = format_real_24(v244);
                    let v246: Rc<str> = Rc::<str>::from(format!("{}{}", v243, v245));
                    let v247: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v248: Rc<str> = Rc::<str>::from(format!("{}{}", v247, v246));
                    let v249: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v250: Rc<str> = Rc::<str>::from(format!("{}{}", v248, v249));
                    v250.clone()
                } else {
                    let v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v255: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v257: Rc<str> = format_real_24(v199);
                    let v258: Rc<str> = Rc::<str>::from(format!("{}{}", v256, v257));
                    let v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v260: Rc<str> = Rc::<str>::from(format!("{}{}", v258, v259));
                    let v261: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v263: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v260, v261)), v255));
                    let v264: f64 = 0.0001f64;
                    let v265: Rc<str> = format_real_24(v264);
                    let mut v266: Rc<str> = Rc::<str>::from(format!("{}{}", v263, v265));
                    let v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v268: Rc<str> = Rc::<str>::from(format!("{}{}", v267, v266));
                    let v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let mut v270: Rc<str> = Rc::<str>::from(format!("{}{}", v268, v269));
                    v270.clone()
                };
                let v272: Rc<str> = Rc::<str>::from(format!("{}{}", v237, v271));
                println!("{}", v272);
                let v273: bool = v202 == false;
                if v273 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v272.clone()))
                };
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2.clone(), v5.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method59() -> Rc<UH1> {
    let v1: num_complex::Complex<f64> = num_complex::Complex::new(3.0f64, 4.0f64);
    let mut v3: num_complex::Complex<f64> = num_complex::Complex::new(2.5f64, -3.5f64);
    let mut v5: num_complex::Complex<f64> = num_complex::Complex::new(1.5f64, 2.5f64);
    let mut v7: num_complex::Complex<f64> = num_complex::Complex::new(0.5f64, 14.134725f64);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7.clone(), v8.clone()));
    let v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5.clone(), v9.clone()));
    let mut v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v3.clone(), v10.clone()));
    Rc::new(UH1::UH1_Cons(v1.clone(), v11.clone()))
}
fn method57(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let mut v3: Rc<UH1> = method59();
    method58(v0, v1, v2, v3)
}
fn method56() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method57(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn method63(mut v0: f64, mut v1: Rc<UH0>, mut v2: f64) -> f64 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: f64 = *v3;
                let v4: Rc<UH0> = v4.clone();
                let mut v5: f64 = -(v0);
                let mut v6: f64 = v3.powf(v5);
                let mut v7: f64 = 1.0f64 - v6;
                let mut v8: f64 = v2 / v7;
                (v0, v1, v2) = (v0, v4.clone(), v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method62(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> () {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: f64 = *v4;
                let mut v5: Rc<UH0> = v5.clone();
                let mut v7: num_complex::Complex<f64> = num_complex::Complex::new(v4, 0.0f64);
                let mut v8: f64 = 1.0f64;
                let mut v9: f64 = method63(v4, v2.clone(), v8);
                let v10: num_complex::Complex<f64> = method7(v7.clone());
                let mut v11: Result<num_complex::Complex<f64>, std::string::String> = method8(v0.clone(), v10.clone());
                let mut v12: num_complex::Complex<f64> = method18(v1.clone(), v7.clone());
                let mut v14: Option<num_complex::Complex<f64>> = v11.ok();
                let mut v15: Option<num_complex::Complex<f64>> = method22(v14.clone());
                let mut v16: Rc<dyn Fn((num_complex::Complex<f64>)) -> US1> = closure1();
                let mut v17: Option<US1> = v15.map(|x| v16(x));
                let v19: US1 = US1::US1_None;
                let mut v20: US1 = v19.clone();
                let mut v21: US1 = v17.unwrap_or(v20);
                let v23: f64 = f64::NAN;
                let v25: f64 = f64::NAN;
                let mut v27: num_complex::Complex<f64> = num_complex::Complex::new(v23, v25);
                let v30: num_complex::Complex<f64> = match &v21 {
                    US1::US1_None => {
                        v27.clone()
                    }
                    US1::US1_Some(v28) => {
                        let mut v28: num_complex::Complex<f64> = v28.clone();
                        v28.clone()
                    }
                };
                let v32: f64 = v30.re;
                let v33: f64 = v32 - v9;
                let mut v34: f64 = -(v33);
                let v35: bool = v33 >= v34;
                let v36: f64 = if v35 {
                    v33
                } else {
                    v34
                };
                let mut v37: bool = v36 < 0.01f64;
                let mut v39: bool = if v37 {
                    true
                } else {
                    method23(v37)
                };
                let v73: Rc<str> = if v37 {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v46: f64 = 0.01f64;
                    let v47: Rc<str> = format_real_24(v46);
                    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v47));
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v48));
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
                    v52.clone()
                } else {
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<str> = format_real_24(v36);
                    let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v62, v63)), v57));
                    let v66: f64 = 0.01f64;
                    let v67: Rc<str> = format_real_24(v66);
                    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v67));
                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v68));
                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
                    v72.clone()
                };
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt"); } LIT.with(|lit| lit.clone()) };
                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
                let v110: Rc<str> = if v37 {
                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let mut v83: f64 = 0.01f64;
                    let v84: Rc<str> = format_real_24(v83);
                    let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v84));
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let mut v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v85));
                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v88));
                    v89.clone()
                } else {
                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v96: Rc<str> = format_real_24(v36);
                    let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
                    let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v98));
                    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v99, v100)), v94));
                    let v103: f64 = 0.01f64;
                    let v104: Rc<str> = format_real_24(v103);
                    let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v104));
                    let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v105));
                    let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v108));
                    v109.clone()
                };
                let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v110));
                println!("{}", v111);
                let mut v112: bool = v39 == false;
                if v112 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v111.clone()))
                };
                let mut v114: f64 = v30.im;
                let v115: bool = v114 < 0.01f64;
                let v117: bool = if v115 {
                    true
                } else {
                    method23(v115)
                };
                let v151: Rc<str> = if v115 {
                    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v124: f64 = 0.01f64;
                    let v125: Rc<str> = format_real_24(v124);
                    let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v125));
                    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v128: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v126));
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v129));
                    v130.clone()
                } else {
                    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v137: Rc<str> = format_real_24(v114);
                    let v138: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v137));
                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v139));
                    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v143: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v140, v141)), v135));
                    let v144: f64 = 0.01f64;
                    let v145: Rc<str> = format_real_24(v144);
                    let v146: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v145));
                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v148: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v146));
                    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v149));
                    v150.clone()
                };
                let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__assert_lt "); } LIT.with(|lit| lit.clone()) };
                let v186: Rc<str> = if v115 {
                    let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected = "); } LIT.with(|lit| lit.clone()) };
                    let v159: f64 = 0.01f64;
                    let v160: Rc<str> = format_real_24(v159);
                    let v161: Rc<str> = Rc::<str>::from(format!("{}{}", v158, v160));
                    let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v163: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v161));
                    let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v165: Rc<str> = Rc::<str>::from(format!("{}{}", v163, v164));
                    v165.clone()
                } else {
                    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual"); } LIT.with(|lit| lit.clone()) };
                    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
                    let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("actual = "); } LIT.with(|lit| lit.clone()) };
                    let v172: Rc<str> = format_real_24(v114);
                    let v173: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v172));
                    let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                    let v175: Rc<str> = Rc::<str>::from(format!("{}{}", v173, v174));
                    let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
                    let v178: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v175, v176)), v170));
                    let v179: f64 = 0.01f64;
                    let v180: Rc<str> = format_real_24(v179);
                    let v181: Rc<str> = Rc::<str>::from(format!("{}{}", v178, v180));
                    let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
                    let v183: Rc<str> = Rc::<str>::from(format!("{}{}", v182, v181));
                    let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
                    let v185: Rc<str> = Rc::<str>::from(format!("{}{}", v183, v184));
                    v185.clone()
                };
                let v187: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v186));
                println!("{}", v187);
                let v188: bool = v117 == false;
                if v188 {
                    std::panic::panic_any::<std::string::String>(format!("{}", v187.clone()))
                };
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2.clone(), v5.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method64() -> Rc<UH0> {
    let mut v0: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v1: Rc<UH0> = Rc::new(UH0::UH0_Cons(5.0f64, v0.clone()));
    let mut v2: Rc<UH0> = Rc::new(UH0::UH0_Cons(4.5f64, v1.clone()));
    let mut v3: Rc<UH0> = Rc::new(UH0::UH0_Cons(4.0f64, v2.clone()));
    let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(3.5f64, v3.clone()));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(3.0f64, v4.clone()));
    let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(2.5f64, v5.clone()));
    Rc::new(UH0::UH0_Cons(2.0f64, v6.clone()))
}
fn method65() -> Rc<UH0> {
    let mut v0: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v1: Rc<UH0> = Rc::new(UH0::UH0_Cons(71.0f64, v0.clone()));
    let mut v2: Rc<UH0> = Rc::new(UH0::UH0_Cons(67.0f64, v1.clone()));
    let mut v3: Rc<UH0> = Rc::new(UH0::UH0_Cons(61.0f64, v2.clone()));
    let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(59.0f64, v3.clone()));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(53.0f64, v4.clone()));
    let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(47.0f64, v5.clone()));
    let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(43.0f64, v6.clone()));
    let mut v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(41.0f64, v7.clone()));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(37.0f64, v8.clone()));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(31.0f64, v9.clone()));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(29.0f64, v10.clone()));
    let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(23.0f64, v11.clone()));
    let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(19.0f64, v12.clone()));
    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(17.0f64, v13.clone()));
    let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(13.0f64, v14.clone()));
    let mut v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(11.0f64, v15.clone()));
    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(7.0f64, v16.clone()));
    let mut v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(5.0f64, v17.clone()));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(3.0f64, v18.clone()));
    Rc::new(UH0::UH0_Cons(2.0f64, v19.clone()))
}
fn method61(mut v0: pyo3::Python, mut v1: pyo3::Python, mut v2: pyo3::Python) -> () {
    let mut v3: Rc<UH0> = method64();
    let v4: Rc<UH0> = method65();
    method62(v1, v2, v4, v3)
}
fn method60() -> () {
    pyo3::Python::initialize();
    let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //;
    let mut v3: pyo3::Python = py;
    method61(v3.clone(), v3.clone(), v3.clone());
    let v4: Result<(), pyo3::PyErr> = Ok::<(), pyo3::PyErr>(());
    let mut v5: bool = true; (v4) }); //;
    let mut v7: Result<(), pyo3::PyErr> = __run_test;
    v7.unwrap();
    ()
}
fn spiral_main() -> i32 {
    let mut v0: Option<Rc<str>> = SPIRAL_TEST_NAME.with(|x| x.borrow().clone());
    let v1: Option<Rc<str>> = method0(v0.clone());
    let mut v2: Rc<dyn Fn((Rc<str>)) -> US0> = closure0();
    let mut v3: Option<US0> = v1.map(|x| v2(x));
    let mut v5: US0 = US0::US0_None;
    let mut v6: US0 = v5.clone();
    let mut v7: US0 = v3.unwrap_or(v6);
    let mut v21: Rc<str> = match &v7 {
        US0::US0_None => {
            let mut v9: i32 = command_line_argument_count_1();
            let v10: bool = 0i32 < v9;
            let mut v15: US0 = if v10 {
                let mut v11: i32 = 0i32;
                let mut v12: Rc<str> = command_line_argument_2(v11);
                US0::US0_Some(v12.clone())
            } else {
                US0::US0_None
            };
            match &v15 {
                US0::US0_None => {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v17.clone()
                }
                US0::US0_Some(v16) => {
                    let mut v16: Rc<str> = v16.clone();
                    v16.clone()
                }
            }
        }
        US0::US0_Some(v8) => {
            let mut v8: Rc<str> = v8.clone();
            v8.clone()
        }
    };
    let mut v22: Rc<str> = method3(v21.clone());
    let v23: bool = &*v22 == "";
    let v25: bool = if v23 {
        true
    } else {
        let v24: bool = &*v22 == "test_zeta_at_known_values_";
        v24
    };
    let v26: bool = if v25 {
        method4();
        true
    } else {
        false
    };
    let mut v28: bool = if v23 {
        true
    } else {
        let mut v27: bool = &*v22 == "test_zeta_at_2_minus2";
        v27
    };
    let v29: bool = if v28 {
        method32();
        true
    } else {
        v26
    };
    let v31: bool = if v23 {
        true
    } else {
        let v30: bool = &*v22 == "test_trivial_zero_at_negative_even___";
        v30
    };
    let v32: bool = if v31 {
        method34();
        true
    } else {
        v29
    };
    let mut v34: bool = if v23 {
        true
    } else {
        let v33: bool = &*v22 == "test_non_trivial_zero___";
        v33
    };
    let v35: bool = if v34 {
        method38();
        true
    } else {
        v32
    };
    let mut v37: bool = if v23 {
        true
    } else {
        let v36: bool = &*v22 == "test_real_part_greater_than_one___";
        v36
    };
    let v38: bool = if v37 {
        method40();
        true
    } else {
        v35
    };
    let v40: bool = if v23 {
        true
    } else {
        let mut v39: bool = &*v22 == "test_zeta_at_1___";
        v39
    };
    let mut v41: bool = if v40 {
        method42();
        true
    } else {
        v38
    };
    let v43: bool = if v23 {
        true
    } else {
        let mut v42: bool = &*v22 == "test_symmetry_across_real_axis___";
        v42
    };
    let v44: bool = if v43 {
        method44();
        true
    } else {
        v41
    };
    let v46: bool = if v23 {
        true
    } else {
        let v45: bool = &*v22 == "test_behavior_near_origin___";
        v45
    };
    let v47: bool = if v46 {
        method46();
        true
    } else {
        v44
    };
    let v49: bool = if v23 {
        true
    } else {
        let v48: bool = &*v22 == "test_imaginary_axis";
        v48
    };
    let v50: bool = if v49 {
        method48();
        true
    } else {
        v47
    };
    let mut v52: bool = if v23 {
        true
    } else {
        let v51: bool = &*v22 == "test_critical_strip";
        v51
    };
    let v53: bool = if v52 {
        method52();
        true
    } else {
        v50
    };
    let v55: bool = if v23 {
        true
    } else {
        let v54: bool = &*v22 == "test_reflection_formula_for_specific_value";
        v54
    };
    let v56: bool = if v55 {
        method56();
        true
    } else {
        v53
    };
    let mut v58: bool = if v23 {
        true
    } else {
        let v57: bool = &*v22 == "test_euler_product_formula";
        v57
    };
    let v59: bool = if v58 {
        method60();
        true
    } else {
        v56
    };
    let mut v62: i32 = if v59 {
        0i32
    } else {
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown test: "); } LIT.with(|lit| lit.clone()) };
        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v22));
        println!("{}", v61);
        1i32
    };
    if v62 != 0 { std::process::exit(v62) };
    0
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
