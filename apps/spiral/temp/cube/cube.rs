#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
struct Mut0 { l0: i32 }
struct Mut1 { l0: f64, l1: f64, l2: f64 }
struct Mut2 { l0: i32 }
struct Mut3 { l0: f64 }
fn method0(mut v0: Rc<RefCell<Mut0>>) -> bool {
    let mut v1: i32 = v0.borrow().l0.clone();
    let mut v2: bool = v1 < 7040i32;
    v2
}
fn method1(mut v0: Rc<RefCell<Mut0>>) -> bool {
    let mut v1: i32 = v0.borrow().l0.clone();
    let mut v2: bool = v1 < 60i32;
    v2
}
fn method4(mut v0: f64, mut v1: Rc<RefCell<Mut3>>) -> bool {
    let mut v2: f64 = v1.borrow().l0.clone();
    let mut v3: bool = v2 < v0;
    v3
}
fn method5(mut v0: f64, mut v1: Rc<RefCell<Mut3>>) -> bool {
    let mut v2: f64 = v1.borrow().l0.clone();
    let mut v3: bool = v2 < v0;
    v3
}
fn method6(mut v0: Rc<RefCell<Vec<f64>>>, mut v1: Rc<RefCell<Vec<i32>>>, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: f64, mut v6: f64, mut v7: f64, mut v8: f64, mut v9: i32) -> () {
    let mut v10: f64 = (v2).sin();
    let mut v11: f64 = v7 * v10;
    let mut v12: f64 = (v3).sin();
    let mut v13: f64 = v11 * v12;
    let mut v14: f64 = (v4).cos();
    let mut v15: f64 = v13 * v14;
    let mut v16: f64 = (v2).cos();
    let mut v17: f64 = v8 * v16;
    let mut v18: f64 = v17 * v12;
    let mut v19: f64 = v18 * v14;
    let mut v20: f64 = v15 - v19;
    let mut v21: f64 = v7 * v16;
    let mut v22: f64 = (v4).sin();
    let mut v23: f64 = v21 * v22;
    let mut v24: f64 = v20 + v23;
    let mut v25: f64 = v8 * v10;
    let mut v26: f64 = v25 * v22;
    let mut v27: f64 = v24 + v26;
    let mut v28: f64 = (v3).cos();
    let mut v29: f64 = v6 * v28;
    let mut v30: f64 = v29 * v14;
    let mut v31: f64 = v27 + v30;
    let mut v32: f64 = v21 * v14;
    let mut v33: f64 = v25 * v14;
    let mut v34: f64 = v32 + v33;
    let mut v35: f64 = v13 * v22;
    let mut v36: f64 = v34 - v35;
    let mut v37: f64 = v18 * v22;
    let mut v38: f64 = v36 + v37;
    let mut v39: f64 = v29 * v22;
    let mut v40: f64 = v38 - v39;
    let mut v41: f64 = v17 * v28;
    let mut v42: f64 = v11 * v28;
    let mut v43: f64 = v41 - v42;
    let mut v44: f64 = v6 * v12;
    let mut v45: f64 = v43 + v44;
    let mut v46: f64 = v45 + 100.0f64;
    let mut v47: f64 = 1.0f64 / v46;
    let mut v48: f64 = 80.0f64 + v5;
    let mut v49: f64 = 40.0f64 * v47;
    let mut v50: f64 = v49 * v31;
    let mut v51: f64 = v50 * 2.0f64;
    let mut v52: f64 = v48 + v51;
    let mut v53: i32 = (v52 as i32);
    let mut v54: f64 = v49 * v40;
    let mut v55: f64 = 22.0f64 + v54;
    let mut v56: i32 = (v55 as i32);
    let mut v57: i32 = v56.wrapping_mul(160i32);
    let mut v58: i32 = v53.wrapping_add(v57);
    let mut v59: bool = v58 >= 0i32;
    let mut v61: bool = if v59 {
        let mut v60: bool = v58 < 7040i32;
        v60
    } else {
        false
    };
    if v61 {
        let mut v62: f64 = v0.clone().borrow()[v58 as usize].clone();
        let mut v63: bool = v47 > v62;
        if v63 {
            v0.clone().borrow_mut()[v58 as usize] = v47;
            v1.clone().borrow_mut()[v58 as usize] = v9;
            ()
        }
    }
}
fn method3(mut v0: Rc<RefCell<Vec<f64>>>, mut v1: Rc<RefCell<Vec<i32>>>, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: f64, mut v6: f64) -> () {
    let mut v7: f64 = -(v5);
    let mut v8: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v7 }));
    while method4(v5, v8.clone()) {
        let mut v10: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v7 }));
        while method5(v5, v10.clone()) {
            let mut v12: f64 = v8.borrow().l0.clone();
            let mut v13: f64 = v10.borrow().l0.clone();
            let mut v14: i32 = 59i32;
            method6(v0.clone(), v1.clone(), v2, v3, v4, v6, v12, v13, v7, v14);
            let mut v15: i32 = 92i32;
            method6(v0.clone(), v1.clone(), v2, v3, v4, v6, v5, v13, v12, v15);
            let mut v16: f64 = -(v12);
            let mut v17: i32 = 47i32;
            method6(v0.clone(), v1.clone(), v2, v3, v4, v6, v7, v13, v16, v17);
            let mut v18: i32 = 61i32;
            method6(v0.clone(), v1.clone(), v2, v3, v4, v6, v16, v13, v5, v18);
            let mut v19: f64 = -(v13);
            let mut v20: i32 = 62i32;
            method6(v0.clone(), v1.clone(), v2, v3, v4, v6, v12, v7, v19, v20);
            let mut v21: i32 = 60i32;
            method6(v0.clone(), v1.clone(), v2, v3, v4, v6, v12, v5, v13, v21);
            let mut v22: f64 = v10.borrow().l0.clone();
            let mut v23: f64 = v22 + 0.6f64;
            v10.borrow_mut().l0 = v23;
            ()
        };
        let mut v24: f64 = v8.borrow().l0.clone();
        let mut v25: f64 = v24 + 0.6f64;
        v8.borrow_mut().l0 = v25;
        ()
    };
    ()
}
fn method7(mut v0: Rc<RefCell<Mut0>>) -> bool {
    let mut v1: i32 = v0.borrow().l0.clone();
    let mut v2: bool = v1 < 44i32;
    v2
}
fn method8(mut v0: Rc<RefCell<Mut0>>) -> bool {
    let mut v1: i32 = v0.borrow().l0.clone();
    let mut v2: bool = v1 < 160i32;
    v2
}
fn method2(mut v0: Rc<RefCell<Vec<f64>>>, mut v1: Rc<RefCell<Vec<i32>>>, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: i32) -> i32 {
    let mut v6: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method0(v6.clone()) {
        let mut v8: i32 = v6.borrow().l0.clone();
        v0.clone().borrow_mut()[v8 as usize] = 0.0f64;
        v1.clone().borrow_mut()[v8 as usize] = 46i32;
        let mut v9: i32 = v8.wrapping_add(1i32);
        v6.borrow_mut().l0 = v9;
        ()
    };
    let mut v10: f64 = 20.0f64;
    let mut v11: f64 = -40.0f64;
    method3(v0.clone(), v1.clone(), v2, v3, v4, v10, v11);
    let mut v12: f64 = 10.0f64;
    let mut v13: f64 = 10.0f64;
    method3(v0.clone(), v1.clone(), v2, v3, v4, v12, v13);
    let mut v14: f64 = 5.0f64;
    let mut v15: f64 = 40.0f64;
    method3(v0.clone(), v1.clone(), v2, v3, v4, v14, v15);
    print!("u001b[H");
    let mut v16: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v5 }));
    let mut v17: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method7(v17.clone()) {
        let mut v19: i32 = v17.borrow().l0.clone();
        let mut v20: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
        while method8(v20.clone()) {
            let mut v22: i32 = v20.borrow().l0.clone();
            let mut v23: i32 = v19.wrapping_mul(160i32);
            let mut v24: i32 = v22.wrapping_add(v23);
            let mut v25: i32 = v1.clone().borrow()[v24 as usize].clone();
            let mut v26: bool = v25 == 59i32;
            let mut v44: Rc<str> = if v26 {
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                v27.clone()
            } else {
                let mut v28: bool = v25 == 92i32;
                if v28 {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    v29.clone()
                } else {
                    let mut v30: bool = v25 == 47i32;
                    if v30 {
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        v31.clone()
                    } else {
                        let mut v32: bool = v25 == 61i32;
                        if v32 {
                            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                            v33.clone()
                        } else {
                            let mut v34: bool = v25 == 62i32;
                            if v34 {
                                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                v35.clone()
                            } else {
                                let mut v36: bool = v25 == 60i32;
                                if v36 {
                                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                                    v37.clone()
                                } else {
                                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                    v38.clone()
                                }
                            }
                        }
                    }
                }
            };
            print!("{}", v44.clone());
            let mut v45: i32 = v16.borrow().l0.clone();
            let mut v46: i32 = v45.wrapping_mul(31i32);
            let mut v47: i32 = v46.wrapping_add(v25);
            let mut v48: i32 = v47.wrapping_rem(1000003i32);
            v16.borrow_mut().l0 = v48;
            let mut v49: i32 = v22.wrapping_add(1i32);
            v20.borrow_mut().l0 = v49;
            ()
        };
        print!("\n");
        let mut v50: i32 = v19.wrapping_add(1i32);
        v17.borrow_mut().l0 = v50;
        ()
    };
    let mut v51: i32 = v16.borrow().l0.clone();
    v51
}
fn spiral_main() -> i32 {
    let mut v0: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(vec![<f64>::default(); 7040i32 as usize]));
    let mut v1: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method0(v1.clone()) {
        let mut v3: i32 = v1.borrow().l0.clone();
        v0.clone().borrow_mut()[v3 as usize] = 0.0f64;
        let mut v4: i32 = v3.wrapping_add(1i32);
        v1.borrow_mut().l0 = v4;
        ()
    };
    let mut v5: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); 7040i32 as usize]));
    let mut v6: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method0(v6.clone()) {
        let mut v8: i32 = v6.borrow().l0.clone();
        v5.clone().borrow_mut()[v8 as usize] = 46i32;
        let mut v9: i32 = v8.wrapping_add(1i32);
        v6.borrow_mut().l0 = v9;
        ()
    };
    let mut v10: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0.0f64, l1: 0.0f64, l2: 0.0f64 }));
    let mut v11: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 0i32 }));
    let mut v12: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method1(v12.clone()) {
        let mut v14: i32 = v12.borrow().l0.clone();
        let (mut v15, mut v16, mut v17): (f64, f64, f64) = (v10.borrow().l0.clone(), v10.borrow().l1.clone(), v10.borrow().l2.clone());
        let mut v18: i32 = v11.borrow().l0.clone();
        let mut v19: i32 = method2(v0.clone(), v5.clone(), v15, v16, v17, v18);
        v11.borrow_mut().l0 = v19;
        let (mut v20, mut v21, mut v22): (f64, f64, f64) = (v10.borrow().l0.clone(), v10.borrow().l1.clone(), v10.borrow().l2.clone());
        let mut v23: f64 = v20 + 0.05f64;
        let mut v24: f64 = v21 + 0.05f64;
        let mut v25: f64 = v22 + 0.01f64;
        v10.borrow_mut().l0 = v23;
        v10.borrow_mut().l1 = v24;
        v10.borrow_mut().l2 = v25;
        let mut v26: i32 = v14.wrapping_add(1i32);
        v12.borrow_mut().l0 = v26;
        ()
    };
    let mut v27: i32 = v11.borrow().l0.clone();
    print!("cube: {} frames, checksum {}\n", 60i32, v27);
    0i32
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 101 });
}
