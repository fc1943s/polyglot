#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[near_sdk::near_bindgen]
#[derive(near_sdk::PanicOnDefault, borsh::BorshDeserialize, borsh::BorshSerialize)]
pub struct State((u32, near_sdk::store::IterableSet<String>, near_sdk::store::IterableSet<String>, near_sdk::store::LookupMap<String, String>, near_sdk::store::LookupMap<String, std::collections::HashMap<String, (u64, u32)>>));
#[near_sdk::near_bindgen]
impl State {
    #[init]
    pub fn new() -> Self {
        Self(chat_contract_new())
    }
    pub fn is_valid_alias(&self, alias: String) -> bool {
        chat_contract_is_valid_alias(alias)
    }
    pub fn generate_cid(&self, content: Vec<u8>) -> String {
        chat_contract_generate_cid(content)
    }
    #[result_serializer(borsh)]
    pub fn generate_cid_borsh(&self, #[serializer(borsh)] content: Vec<u8>) -> String {
        self.generate_cid(content)
    }
    pub fn claim_alias(&mut self, alias: String) {
        let state = &mut self.0;
        chat_contract_claim_alias(&mut state.1, &mut state.2, &mut state.3, &mut state.4, alias)
    }
    pub fn get_account_info(&self, account_id: near_sdk::AccountId) -> Option<(String, u64, u32)> {
        chat_contract_get_account_info(&self.0.3, &self.0.4, account_id.to_string())
    }
    pub fn get_alias_map(&self, alias: String) -> Option<std::collections::HashMap<String, (u64, u32)>> {
        chat_contract_get_alias_map(&self.0.4, alias)
    }
    #[result_serializer(borsh)]
    pub fn get_alias_map_borsh(&self, #[serializer(borsh)] alias: String) -> Option<std::collections::HashMap<String, (u64, u32)>> {
        self.get_alias_map(alias)
    }
}
fn spiral_trace_hold<T: Clone>(fresh: &std::rc::Rc<dyn Fn() -> T>) -> T {
    use std::sync::OnceLock;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let raw = *SLOT.get_or_init(|| Box::into_raw(Box::new((*fresh)())) as usize);
    unsafe { (*(raw as *const T)).clone() }
}
fn spiral_out_varint_u64(n: u64) -> Vec<u8> {
    let mut buffer = unsigned_varint::encode::u64_buffer();
    unsigned_varint::encode::u64(n, &mut buffer).to_vec()
}
struct SpiralDebug<T>(T);
trait SpiralDebugShow {
    fn show(&self) -> String;
}
impl<T: std::fmt::Debug> SpiralDebugShow for SpiralDebug<T> {
    fn show(&self) -> String { format!("{:?}", self.0) }
}
trait SpiralTypeShow {
    fn show(&self) -> String;
}
impl<T> SpiralTypeShow for &SpiralDebug<T> {
    fn show(&self) -> String { String::from(std::any::type_name::<T>()) }
}
#[cfg(target_arch = "wasm32")]
fn spiral_cfg_local_clock() -> Option<[i64; 3]> { None }
#[cfg(all(windows, not(target_arch = "wasm32")))]
fn spiral_cfg_local_clock() -> Option<[i64; 3]> {
    #[repr(C)] struct St([u16; 8]);
    unsafe extern "system" { fn GetLocalTime(t: *mut St); }
    let mut t = St([0; 8]);
    unsafe { GetLocalTime(&mut t) };
    Some([i64::from(t.0[4]), i64::from(t.0[5]), i64::from(t.0[6])])
}
#[cfg(all(unix, not(target_arch = "wasm32")))]
fn spiral_cfg_local_clock() -> Option<[i64; 3]> {
    #[repr(C, align(8))] struct Tm([i32; 16]);
    unsafe extern "C" { fn localtime_r(t: *const std::os::raw::c_long, tm: *mut Tm) -> *mut Tm; }
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs());
    let secs = std::os::raw::c_long::try_from(now.unwrap_or(0)).unwrap_or(0);
    let mut tm = Tm([0; 16]);
    unsafe { localtime_r(&secs, &mut tm) };
    Some([i64::from(tm.0[2]), i64::from(tm.0[1]), i64::from(tm.0[0])])
}
#[cfg(not(any(windows, unix, target_arch = "wasm32")))]
fn spiral_cfg_local_clock() -> Option<[i64; 3]> { None }
#[cfg(target_arch = "wasm32")]
fn spiral_cfg_epoch_nanos() -> u64 { near_sdk::env::block_timestamp() }
#[cfg(not(target_arch = "wasm32"))]
fn spiral_cfg_epoch_nanos() -> u64 {
    let since = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    since.map(|d| u64::try_from(d.as_nanos()).unwrap_or(0)).unwrap_or(0)
}
#[cfg(target_arch = "wasm32")]
fn spiral_trace_near_log(text: &str) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static LOGGED: AtomicUsize = AtomicUsize::new(0);
    let cs: Vec<char> = text.chars().collect();
    for c in cs.chunks(15000) {
        let s: String = c.iter().collect();
        let used = LOGGED.load(Ordering::Relaxed);
        let budget = 16000usize.saturating_sub(used);
        if budget < 13 {
            break;
        }
        let s = if s.len() <= budget {
            s
        } else {
            let mut end = budget - 12;
            while !s.is_char_boundary(end) {
                end -= 1;
            }
            format!("{} [truncated]", &s[..end])
        };
        LOGGED.store(used + s.len(), Ordering::Relaxed);
        near_sdk::env::log_str(&s);
    }
}
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
struct Mut0 { l0: i64 }
struct Mut1 { l0: Rc<dyn Fn(Rc<str>) -> ()> }
struct Mut2 { l0: bool }
struct Mut3 { l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_Verbose,
    US0_Debug,
    US0_Info,
    US0_Warning,
    US0_Critical,
}
struct Mut4 { l0: US0 }
#[derive(Clone)]
enum US1 {
    US1_Some(US0),
    US1_None,
}
struct Mut5 { l0: i32, l1: US1 }
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some([i64; 3]),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(std::string::String),
    US4_None,
}
fn get_environment_variable_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn method2(mut v0: Rc<RefCell<Mut5>>) -> bool {
    let mut v1: i32 = v0.borrow().l0.clone();
    let v2: bool = v1 > 0i32;
    v2
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        ()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn new_trace_state_0(mut v0: US0) -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TRACE_LEVEL"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = get_environment_variable_1(v1.clone());
    ;
    ;
    ;
    ;
    ;
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Critical"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v3.to_lowercase());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Warning"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v5.to_lowercase());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Info"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Debug"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from(v9.to_lowercase());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verbose"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v11.to_lowercase());
    let mut v13: Rc<RefCell<Vec<(Rc<str>, US0)>>> = Rc::new(RefCell::new(Vec::new()));
    let v14: US0 = US0::US0_Verbose;
    v13.borrow_mut().push((v11.clone(), v14.clone()));
    let mut v15: US0 = US0::US0_Debug;
    v13.borrow_mut().push((v9.clone(), v15.clone()));
    let v16: US0 = US0::US0_Info;
    v13.borrow_mut().push((v7.clone(), v16.clone()));
    let v17: US0 = US0::US0_Warning;
    v13.borrow_mut().push((v5.clone(), v17.clone()));
    let v18: US0 = US0::US0_Critical;
    v13.borrow_mut().push((v3.clone(), v18.clone()));
    let v19: US0 = US0::US0_Verbose;
    v13.borrow_mut().push((v12.clone(), v19.clone()));
    let v20: US0 = US0::US0_Debug;
    v13.borrow_mut().push((v10.clone(), v20.clone()));
    let v21: US0 = US0::US0_Info;
    v13.borrow_mut().push((v8.clone(), v21.clone()));
    let v22: US0 = US0::US0_Warning;
    v13.borrow_mut().push((v6.clone(), v22.clone()));
    let mut v23: US0 = US0::US0_Critical;
    v13.borrow_mut().push((v4.clone(), v23.clone()));
    let v24: Rc<Vec<(Rc<str>, US0)>> = Rc::new(v13.borrow().clone());
    let v25: Rc<RefCell<Vec<(Rc<str>, US0)>>> = Rc::new(RefCell::new((v24).as_ref().clone()));
    let v26: i32 = (v25.clone().borrow().len() as i32);
    let v27: US1 = US1::US1_None;
    let mut v28: Rc<RefCell<Mut5>> = Rc::new(RefCell::new(Mut5 { l0: v26, l1: v27.clone() }));
    while method2(v28.clone()) {
        let v30: i32 = v28.borrow().l0.clone();
        let v31: i32 = v30.wrapping_sub(1i32);
        let mut v32: US1 = v28.borrow().l1.clone();
        let (mut v33, mut v34): (Rc<str>, US0) = v25.clone().borrow()[v31 as usize].clone();
        let mut v41: US1 = match &v32 {
            US1::US1_None => {
                let v36: bool = v33 == v2 ;
                if v36 {
                    US1::US1_Some(v34.clone())
                } else {
                    US1::US1_None
                }
            }
            US1::US1_Some(v35) => {
                let v35: US0 = v35.clone();
                v32.clone()
            }
        };
        v28.borrow_mut().l0 = v31;
        v28.borrow_mut().l1 = v41.clone();
        ()
    };
    let v42: US1 = v28.borrow().l1.clone();
    let v43: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i64 }));
    let v44: Rc<dyn Fn(Rc<str>) -> ()> = closure2();
    let v45: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v44.clone() }));
    let mut v46: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: true }));
    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v48: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v47.clone() }));
    let v51: US0 = match &v42 {
        US1::US1_None => {
            v0.clone()
        }
        US1::US1_Some(v49) => {
            let mut v49: US0 = v49.clone();
            v49.clone()
        }
    };
    let mut v52: Rc<RefCell<Mut4>> = Rc::new(RefCell::new(Mut4 { l0: v51.clone() }));
    let v53: Option<i64> = None;
    (v43.clone(), v45.clone(), v46.clone(), v48.clone(), v52.clone(), v53.clone())
}
fn fresh_c1() -> Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = Rc::new(move || -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) {
        let mut v0: US0 = US0::US0_Verbose;
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = new_trace_state_0(v0.clone());
        (v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fresh_c3() -> Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = Rc::new(move || -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) {
        let mut v0: US0 = US0::US0_Verbose;
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = new_trace_state_0(v0.clone());
        (v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure0() -> Rc<dyn Fn() -> (u32, near_sdk::store::IterableSet<String>, near_sdk::store::IterableSet<std::string::String>, near_sdk::store::LookupMap<String, std::string::String>, near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (u32, near_sdk::store::IterableSet<String>, near_sdk::store::IterableSet<std::string::String>, near_sdk::store::LookupMap<String, std::string::String>, near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>)> = Rc::new(move || -> (u32, near_sdk::store::IterableSet<String>, near_sdk::store::IterableSet<std::string::String>, near_sdk::store::LookupMap<String, std::string::String>, near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>) {
        let mut v1: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c1();
        { let _ = spiral_trace_hold(&v1); };
        let v3: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c3();
        let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v3);
        let mut v10: US0 = US0::US0_Info;
        v8.borrow_mut().l0 = v10.clone();
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_set"); } LIT.with(|lit| lit.clone()) };
        let v12: &[u8] = Box::leak((v11).as_bytes().to_vec().into_boxed_slice());
        let v14: near_sdk::store::IterableSet<String> = near_sdk::store::IterableSet::new(v12);
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alias_set"); } LIT.with(|lit| lit.clone()) };
        let v16: &[u8] = Box::leak((v15).as_bytes().to_vec().into_boxed_slice());
        let v18: near_sdk::store::IterableSet<std::string::String> = near_sdk::store::IterableSet::new(v16);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_map"); } LIT.with(|lit| lit.clone()) };
        let v20: &[u8] = Box::leak((v19).as_bytes().to_vec().into_boxed_slice());
        let v22: near_sdk::store::LookupMap<String, std::string::String> = near_sdk::store::LookupMap::new(v20);
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alias_map"); } LIT.with(|lit| lit.clone()) };
        let v24: &[u8] = Box::leak((v23).as_bytes().to_vec().into_boxed_slice());
        let v26: near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>> = near_sdk::store::LookupMap::new(v24);
        (2u32, v14, v18, v22, v26)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(std::string::String) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::String) -> bool> = Rc::new(move |mut v0: std::string::String| -> bool {
        let v2: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c1();
        { let _ = spiral_trace_hold(&v2); };
        let v4: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c3();
        let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v4);
        let mut v11: US0 = US0::US0_Info;
        v9.borrow_mut().l0 = v11.clone();
        let v12: bool = { let alias: &str = &(v0); alias.len() > 0 && alias.len() < 64 && !alias.starts_with('-') && !alias.ends_with('-') && alias.chars().all(|c| c.is_alphanumeric() || c == '-') };
        v12
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method3() -> u8 {
    18u8
}
fn method4(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method5() -> u8 {
    32u8
}
fn method6(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method7(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method8() -> u8 {
    1u8
}
fn closure5() -> Rc<dyn Fn(Vec<u8>) -> std::string::String> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<u8>) -> std::string::String> = Rc::new(move |mut v0: Vec<u8>| -> std::string::String {
        let v2: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c1();
        { let _ = spiral_trace_hold(&v2); };
        let v4: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c3();
        let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v4);
        let mut v11: US0 = US0::US0_Info;
        v9.borrow_mut().l0 = v11.clone();
        let v12: Vec<u8> = spiral_out_varint_u64(85u64);
        let mut v13: Vec<u8> = Vec::new();
        let v14: u8 = method3();
        let mut v15: Vec<u8> = method4(v13.clone());
        let v17: bool = true; let mut v15 = v15;
        let v19: bool = true; v15.push(v14);
        let v21: Vec<u8> = v15;
        let v22: u8 = method5();
        let mut v23: Vec<u8> = method4(v21.clone());
        let v25: bool = true; let mut v23 = v23;
        let v27: bool = true; v23.push(v22);
        let v29: Vec<u8> = v23;
        let v30: Vec<u8> = <sha2::Sha256 as sha2::Digest>::digest(&v0).to_vec();
        let v31: Vec<u8> = method6(v30.clone());
        let mut v32: Vec<u8> = method7(v29.clone());
        let v34: bool = true; let mut v32 = v32;
        let v36: bool = true; v32.extend(v31);
        let v38: Vec<u8> = v32;
        let v39: Vec<u8> = Vec::new();
        let v40: u8 = method8();
        let mut v41: Vec<u8> = method4(v39.clone());
        let v43: bool = true; let mut v41 = v41;
        let v45: bool = true; v41.push(v40);
        let v47: Vec<u8> = v41;
        let v48: Vec<u8> = method6(v12.clone());
        let mut v49: Vec<u8> = method7(v47.clone());
        let v51: bool = true; let mut v49 = v49;
        let v53: bool = true; v49.extend(v48);
        let v55: Vec<u8> = v49;
        let mut v56: Vec<u8> = method6(v38.clone());
        let mut v57: Vec<u8> = method7(v55.clone());
        let mut v59: bool = true; let mut v57 = v57;
        let v61: bool = true; v57.extend(v56);
        let v63: Vec<u8> = v57;
        let v64: std::string::String = multibase::encode(multibase::Base::Base32Lower, &v63);
        v64.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10(mut v0: Option<[i64; 3]>) -> Option<[i64; 3]> {
    v0
}
fn closure7() -> Rc<dyn Fn(([i64; 3])) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(([i64; 3])) -> US3> = Rc::new(move |mut v0: ([i64; 3])| -> US3 {
        let mut v1: [i64; 3] = (v0);
        US3::US3_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<RefCell<Mut1>>, mut v2: Rc<RefCell<Mut2>>, mut v3: Rc<RefCell<Mut3>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>) -> Rc<str> {
    let v6: Option<[i64; 3]> = spiral_cfg_local_clock();
    let v7: Option<[i64; 3]> = method10(v6.clone());
    let mut v8: Rc<dyn Fn(([i64; 3])) -> US3> = closure7();
    let mut v9: Option<US3> = v7.map(|x| v8(x));
    let mut v11: US3 = US3::US3_None;
    let v12: US3 = v11.clone();
    let mut v13: US3 = v9.unwrap_or(v12);
    let v27: u64 = match &v13 {
        US3::US3_None => {
            let mut v23: u64 = spiral_cfg_epoch_nanos();
            let v24: u64 = v23.wrapping_div(1000000000u64);
            let v25: u64 = v24.wrapping_rem(86400u64);
            v25
        }
        US3::US3_Some(v14) => {
            let v14: [i64; 3] = v14.clone();
            let mut v15: i64 = v14[0];
            let v16: i64 = v15.wrapping_mul(3600i64);
            let v17: i64 = v14[1];
            let v18: i64 = v17.wrapping_mul(60i64);
            let v19: i64 = v16.wrapping_add(v18);
            let v20: i64 = v14[2];
            let v21: i64 = v19.wrapping_add(v20);
            let v22: u64 = (v21 as u64);
            v22
        }
    };
    let mut v28: u64 = v27.wrapping_div(3600u64);
    let v29: bool = v28 < 10u64;
    let mut v32: Rc<str> = if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v30.clone()
    } else {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v31.clone()
    };
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, Rc::<str>::from(format!("{:?}", v28))));
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
    let v37: u64 = v27.wrapping_div(60u64);
    let v38: u64 = v37.wrapping_rem(60u64);
    let v39: bool = v38 < 10u64;
    let v42: Rc<str> = if v39 {
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v40.clone()
    } else {
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v41.clone()
    };
    let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, Rc::<str>::from(format!("{}{}", v42, Rc::<str>::from(format!("{:?}", v38)))))), v35));
    let v47: u64 = v27.wrapping_rem(60u64);
    let v48: bool = v47 < 10u64;
    let v51: Rc<str> = if v48 {
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v49.clone()
    } else {
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v50.clone()
    };
    let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v46, Rc::<str>::from(format!("{}{}", v51, Rc::<str>::from(format!("{:?}", v47))))));
    v54.clone()
}
fn method13(mut v0: Rc<RefCell<Mut3>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_12(mut v0: u8) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0 as char));
    method13(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method11() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[94m"); } LIT.with(|lit| lit.clone()) };
    ;
    ;
    ;
    ;
    ;
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Debug"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let v3: u8 = v2.clone().as_bytes()[0i32 as usize];
    let v4: Rc<str> = format_real_12(v3);
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[0m"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    v7.clone()
}
fn method16(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v7: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'/';
                v6
            };
            if v7 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = 0i32;
    let v3: i32 = method16(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method17(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    v8
}
fn format_real_18(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method13(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn format_real_19(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method13(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method14(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<RefCell<Mut1>>, mut v2: Rc<RefCell<Mut2>>, mut v3: Rc<RefCell<Mut3>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: std::string::String, mut v9: u64, mut v10: std::string::String, mut v11: std::string::String) -> Rc<str> {
    let v12: i64 = v0.borrow().l0.clone();
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v13));
    let mut v15: Rc<str> = format_real_18(v12);
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v14, v15)), v7)), v13));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chat_contract.claim_alias"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" / "); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alias"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alias"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alias = "); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::string::String>") } } (&&W(&v8)).s() });
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("block_timestamp"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v32, v33)), v27));
    let v36: Rc<str> = format_real_19(v9);
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v36)), v31));
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("signer_account_id"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v38, v39)), v27));
    let v42: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::string::String>") } } (&&W(&v10)).s() });
    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v41, v42)), v31));
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("predecessor_account_id"); } LIT.with(|lit| lit.clone()) };
    let v47: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v44, v45)), v27));
    let v48: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::string::String>") } } (&&W(&v11)).s() });
    let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v49));
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
    let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v22, Rc::<str>::from(format!("{}{}", v51, v52))));
    method15(v54.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        println!("{}", v0);
        ()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20(mut v0: Option<std::string::String>) -> Option<std::string::String> {
    v0
}
fn closure9() -> Rc<dyn Fn((std::string::String)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((std::string::String)) -> US4> = Rc::new(move |mut v0: (std::string::String)| -> US4 {
        let mut v1: std::string::String = (v0);
        US4::US4_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method21() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[93m"); } LIT.with(|lit| lit.clone()) };
    ;
    ;
    ;
    ;
    ;
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Warning"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let v3: u8 = v2.clone().as_bytes()[0i32 as usize];
    let v4: Rc<str> = format_real_12(v3);
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[0m"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    v7.clone()
}
fn method22(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<RefCell<Mut1>>, mut v2: Rc<RefCell<Mut2>>, mut v3: Rc<RefCell<Mut3>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: i64 = v0.borrow().l0.clone();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v10));
    let v12: Rc<str> = format_real_18(v9);
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v12)), v7)), v10));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chat_contract.claim_alias / alias already claimed"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" / "); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_alias"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_alias"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v23, v24)), v8));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v26));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v19, Rc::<str>::from(format!("{}{}", v28, v29))));
    method15(v31.clone())
}
fn method23(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<RefCell<Mut1>>, mut v2: Rc<RefCell<Mut2>>, mut v3: Rc<RefCell<Mut3>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: i64 = v0.borrow().l0.clone();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v10));
    let v12: Rc<str> = format_real_18(v9);
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v12)), v7)), v10));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chat_contract.claim_alias"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" / "); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_alias"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_alias"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v23, v24)), v8));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v26));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v19, Rc::<str>::from(format!("{}{}", v28, v29))));
    method15(v31.clone())
}
fn closure6() -> Rc<dyn Fn(&mut near_sdk::store::IterableSet<String>, &mut near_sdk::store::IterableSet<std::string::String>, &mut near_sdk::store::LookupMap<String, std::string::String>, &mut near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, std::string::String) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(&mut near_sdk::store::IterableSet<String>, &mut near_sdk::store::IterableSet<std::string::String>, &mut near_sdk::store::LookupMap<String, std::string::String>, &mut near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, std::string::String) -> ()> = Rc::new(move |mut v0: &mut near_sdk::store::IterableSet<String>, mut v1: &mut near_sdk::store::IterableSet<std::string::String>, mut v2: &mut near_sdk::store::LookupMap<String, std::string::String>, mut v3: &mut near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, mut v4: std::string::String| -> () {
        let v6: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c1();
        { let _ = spiral_trace_hold(&v6); };
        let mut v8: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c3();
        let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
        let mut v15: US0 = US0::US0_Info;
        v13.borrow_mut().l0 = v15.clone();
        let v16: String = { #[cfg(target_arch = "wasm32")] let v = near_sdk::env::signer_account_id().to_string(); #[cfg(not(target_arch = "wasm32"))] let v = String::from("a"); v };
        let v17: String = { #[cfg(target_arch = "wasm32")] let v = near_sdk::env::predecessor_account_id().to_string(); #[cfg(not(target_arch = "wasm32"))] let v = String::from("a"); v };
        let v18: u64 = { #[cfg(target_arch = "wasm32")] let v = near_sdk::env::block_timestamp(); #[cfg(not(target_arch = "wasm32"))] let v = 1u64; v };
        { let _ = spiral_trace_hold(&v6); };
        let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
        let v27: US0 = v25.borrow().l0.clone();
        let mut v32: i32 = match &v27 {
            US0::US0_Critical => {
                50i32
            }
            US0::US0_Debug => {
                20i32
            }
            US0::US0_Info => {
                30i32
            }
            US0::US0_Verbose => {
                10i32
            }
            US0::US0_Warning => {
                40i32
            }
        };
        let v33: bool = v23.borrow().l0.clone();
        let v34: bool = v33 == false;
        let v36: bool = if v34 {
            false
        } else {
            let v35: bool = 20i32 >= v32;
            v35
        };
        let v37: bool = v36 == false;
        let v89: US2 = if v37 {
            US2::US2_None
        } else {
            { let _ = spiral_trace_hold(&v6); };
            let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
            let v47: Rc<str> = method9(v41.clone(), v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone());
            let v48: Rc<str> = method11();
            let v50: std::string::String = v16.to_string();
            let mut v52: std::string::String = v17.to_string();
            let v53: Rc<str> = method14(v41.clone(), v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone(), v47.clone(), v48.clone(), v4.clone(), v18, v50.clone(), v52.clone());
            { let _ = spiral_trace_hold(&v6); };
            let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
            let v62: i64 = v56.borrow().l0.clone();
            let v63: i64 = v62.wrapping_add(1i64);
            v56.borrow_mut().l0 = v63;
            let v64: Rc<dyn Fn(Rc<str>) -> ()> = closure8();
            let v65: bool = cfg!(target_arch = "wasm32");
            if v65 {
                let v66: Rc<str> = v59.borrow().l0.clone();
                let v67: bool = &*v66 == "";
                let v75: Rc<str> = if v67 {
                    v53.clone()
                } else {
                    let v68: bool = &*v53 == "";
                    if v68 {
                        let v69: Rc<str> = v59.borrow().l0.clone();
                        v69.clone()
                    } else {
                        let v70: Rc<str> = v59.borrow().l0.clone();
                        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v73: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v70, v71)), v53));
                        v73.clone()
                    }
                };
                let mut v77: usize = v75.chars().count();
                let v78: i32 = ((v77) as i32);
                let v79: i32 = v78.wrapping_add(14999i32);
                let mut v80: i32 = v79.wrapping_div(15000i32);
                let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v82: bool = v53 != v81 ;
                let v84: bool = if v82 {
                    let v83: bool = v80 <= 1i32;
                    v83
                } else {
                    false
                };
                if v84 {
                    v59.borrow_mut().l0 = v75.clone();
                    ()
                } else {
                    v59.borrow_mut().l0 = v81.clone();
                    { #[cfg(target_arch = "wasm32")] spiral_trace_near_log(&v75); };
                    ()
                }
            } else {
                println!("{}", v53);
                ()
            };
            let mut v87: Rc<dyn Fn(Rc<str>) -> ()> = v57.borrow().l0.clone();
            v87(v53.clone());
            US2::US2_Some(v56.clone(), v57.clone(), v58.clone(), v59.clone(), v60.clone(), v61.clone())
        };
        let v90: bool = { let alias: &str = &(v4.clone()); alias.len() > 0 && alias.len() < 64 && !alias.starts_with('-') && !alias.ends_with('-') && alias.chars().all(|c| c.is_alphanumeric() || c == '-') };
        let v91: bool = v90 == false;
        if v91 {
            let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chat_contract.claim_alias / invalid alias"); } LIT.with(|lit| lit.clone()) };
            near_sdk::env::panic_str(&*v92);
            ()
        };
        let v93: Option<std::string::String> = (v2).get(&(v16.clone())).cloned();
        let v94: Option<std::string::String> = method20(v93.clone());
        let mut v95: Rc<dyn Fn((std::string::String)) -> US4> = closure9();
        let mut v96: Option<US4> = v94.map(|x| v95(x));
        let v98: US4 = US4::US4_None;
        let v99: US4 = v98.clone();
        let v100: US4 = v96.unwrap_or(v99);
        match &v100 {
            US4::US4_Some(v101) => {
                let v101: std::string::String = v101.clone();
                let v102: bool = v101 == v4 ;
                if v102 {
                    { let _ = spiral_trace_hold(&v6); };
                    let (mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                    let v111: US0 = v109.borrow().l0.clone();
                    let v116: i32 = match &v111 {
                        US0::US0_Critical => {
                            50i32
                        }
                        US0::US0_Debug => {
                            20i32
                        }
                        US0::US0_Info => {
                            30i32
                        }
                        US0::US0_Verbose => {
                            10i32
                        }
                        US0::US0_Warning => {
                            40i32
                        }
                    };
                    let v117: bool = v107.borrow().l0.clone();
                    let v118: bool = v117 == false;
                    let v120: bool = if v118 {
                        false
                    } else {
                        let v119: bool = 40i32 >= v116;
                        v119
                    };
                    let v121: bool = v120 == false;
                    let v170: US2 = if v121 {
                        US2::US2_None
                    } else {
                        { let _ = spiral_trace_hold(&v6); };
                        let (mut v125, mut v126, mut v127, mut v128, mut v129, mut v130): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                        let v131: Rc<str> = method9(v125.clone(), v126.clone(), v127.clone(), v128.clone(), v129.clone(), v130.clone());
                        let v132: Rc<str> = method21();
                        let v133: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::string::String>") } } (&&W(&v101)).s() });
                        let v134: Rc<str> = method22(v125.clone(), v126.clone(), v127.clone(), v128.clone(), v129.clone(), v130.clone(), v131.clone(), v132.clone(), v133.clone());
                        { let _ = spiral_trace_hold(&v6); };
                        let (mut v137, mut v138, mut v139, mut v140, mut v141, mut v142): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                        let v143: i64 = v137.borrow().l0.clone();
                        let v144: i64 = v143.wrapping_add(1i64);
                        v137.borrow_mut().l0 = v144;
                        let v145: Rc<dyn Fn(Rc<str>) -> ()> = closure8();
                        let v146: bool = cfg!(target_arch = "wasm32");
                        if v146 {
                            let v147: Rc<str> = v140.borrow().l0.clone();
                            let v148: bool = &*v147 == "";
                            let v156: Rc<str> = if v148 {
                                v134.clone()
                            } else {
                                let v149: bool = &*v134 == "";
                                if v149 {
                                    let v150: Rc<str> = v140.borrow().l0.clone();
                                    v150.clone()
                                } else {
                                    let v151: Rc<str> = v140.borrow().l0.clone();
                                    let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let v154: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v151, v152)), v134));
                                    v154.clone()
                                }
                            };
                            let v158: usize = v156.chars().count();
                            let v159: i32 = ((v158) as i32);
                            let v160: i32 = v159.wrapping_add(14999i32);
                            let v161: i32 = v160.wrapping_div(15000i32);
                            let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v163: bool = v134 != v162 ;
                            let v165: bool = if v163 {
                                let v164: bool = v161 <= 1i32;
                                v164
                            } else {
                                false
                            };
                            if v165 {
                                v140.borrow_mut().l0 = v156.clone();
                                ()
                            } else {
                                v140.borrow_mut().l0 = v162.clone();
                                { #[cfg(target_arch = "wasm32")] spiral_trace_near_log(&v156); };
                                ()
                            }
                        } else {
                            println!("{}", v134);
                            ()
                        };
                        let mut v168: Rc<dyn Fn(Rc<str>) -> ()> = v138.borrow().l0.clone();
                        v168(v134.clone());
                        US2::US2_Some(v137.clone(), v138.clone(), v139.clone(), v140.clone(), v141.clone(), v142.clone())
                    };
                    ()
                } else {
                    { let _ = spiral_trace_hold(&v6); };
                    let (mut v173, mut v174, mut v175, mut v176, mut v177, mut v178): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                    let v179: US0 = v177.borrow().l0.clone();
                    let v184: i32 = match &v179 {
                        US0::US0_Critical => {
                            50i32
                        }
                        US0::US0_Debug => {
                            20i32
                        }
                        US0::US0_Info => {
                            30i32
                        }
                        US0::US0_Verbose => {
                            10i32
                        }
                        US0::US0_Warning => {
                            40i32
                        }
                    };
                    let v185: bool = v175.borrow().l0.clone();
                    let v186: bool = v185 == false;
                    let v188: bool = if v186 {
                        false
                    } else {
                        let v187: bool = 20i32 >= v184;
                        v187
                    };
                    let v189: bool = v188 == false;
                    let v247: US2 = if v189 {
                        US2::US2_None
                    } else {
                        { let _ = spiral_trace_hold(&v6); };
                        let (mut v193, mut v194, mut v195, mut v196, mut v197, mut v198): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                        let v199: Rc<str> = method9(v193.clone(), v194.clone(), v195.clone(), v196.clone(), v197.clone(), v198.clone());
                        let v200: Rc<str> = method11();
                        let v201: Option<std::string::String> = Some(v101.clone());
                        let v202: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v201.clone().into_iter().collect::<Vec<_>>()));
                        let v203: u64 = (v202.clone().borrow().len() as u64);
                        let v204: bool = v203 == 0u64;
                        let v210: Rc<str> = if v204 {
                            let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
                            v205.clone()
                        } else {
                            let v206: std::string::String = v202.clone().borrow()[0u64 as usize].clone();
                            let v207: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::string::String>") } } (&&W(&v206)).s() });
                            let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Some "); } LIT.with(|lit| lit.clone()) };
                            let v209: Rc<str> = Rc::<str>::from(format!("{}{}", v208, v207));
                            v209.clone()
                        };
                        let v211: Rc<str> = method23(v193.clone(), v194.clone(), v195.clone(), v196.clone(), v197.clone(), v198.clone(), v199.clone(), v200.clone(), v210.clone());
                        { let _ = spiral_trace_hold(&v6); };
                        let (mut v214, mut v215, mut v216, mut v217, mut v218, mut v219): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                        let v220: i64 = v214.borrow().l0.clone();
                        let v221: i64 = v220.wrapping_add(1i64);
                        v214.borrow_mut().l0 = v221;
                        let v222: Rc<dyn Fn(Rc<str>) -> ()> = closure8();
                        let v223: bool = cfg!(target_arch = "wasm32");
                        if v223 {
                            let v224: Rc<str> = v217.borrow().l0.clone();
                            let v225: bool = &*v224 == "";
                            let v233: Rc<str> = if v225 {
                                v211.clone()
                            } else {
                                let v226: bool = &*v211 == "";
                                if v226 {
                                    let v227: Rc<str> = v217.borrow().l0.clone();
                                    v227.clone()
                                } else {
                                    let v228: Rc<str> = v217.borrow().l0.clone();
                                    let v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let v231: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v228, v229)), v211));
                                    v231.clone()
                                }
                            };
                            let v235: usize = v233.chars().count();
                            let v236: i32 = ((v235) as i32);
                            let v237: i32 = v236.wrapping_add(14999i32);
                            let v238: i32 = v237.wrapping_div(15000i32);
                            let v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v240: bool = v211 != v239 ;
                            let v242: bool = if v240 {
                                let v241: bool = v238 <= 1i32;
                                v241
                            } else {
                                false
                            };
                            if v242 {
                                v217.borrow_mut().l0 = v233.clone();
                                ()
                            } else {
                                v217.borrow_mut().l0 = v239.clone();
                                { #[cfg(target_arch = "wasm32")] spiral_trace_near_log(&v233); };
                                ()
                            }
                        } else {
                            println!("{}", v211);
                            ()
                        };
                        let mut v245: Rc<dyn Fn(Rc<str>) -> ()> = v215.borrow().l0.clone();
                        v245(v211.clone());
                        US2::US2_Some(v214.clone(), v215.clone(), v216.clone(), v217.clone(), v218.clone(), v219.clone())
                    };
                    { if let Some(accounts) = (v3).get_mut(&(v101)) { accounts.remove(&(v16.clone())); } };
                    { (v2).insert((v16).clone(), (v4).clone()); };
                    let v249: bool = v0.insert(v16.clone());
                    let v251: bool = v1.insert(v4.clone());
                    { let accounts = match (v3).get(&(v4.clone())) { None => { let mut accounts = std::collections::HashMap::new(); accounts.insert((v16).clone(), (v18, 0u32)); accounts } Some(previous) => { let mut ordered = previous.iter().collect::<Vec<_>>(); ordered.sort_unstable_by_key(|(_, (_, index))| *index); let mut accounts = ordered.iter().enumerate().map(|(i, (account_id, (timestamp, _)))| ((*account_id).clone(), (*timestamp, i as u32))).collect::<std::collections::HashMap<_, _>>(); accounts.insert((v16).clone(), (v18, ordered.len() as u32)); accounts } }; (v3).insert((v4).clone(), accounts); };
                    ()
                }
            }
            _ => {
                { let _ = spiral_trace_hold(&v6); };
                let (mut v254, mut v255, mut v256, mut v257, mut v258, mut v259): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                let v260: US0 = v258.borrow().l0.clone();
                let v265: i32 = match &v260 {
                    US0::US0_Critical => {
                        50i32
                    }
                    US0::US0_Debug => {
                        20i32
                    }
                    US0::US0_Info => {
                        30i32
                    }
                    US0::US0_Verbose => {
                        10i32
                    }
                    US0::US0_Warning => {
                        40i32
                    }
                };
                let v266: bool = v256.borrow().l0.clone();
                let v267: bool = v266 == false;
                let v269: bool = if v267 {
                    false
                } else {
                    let v268: bool = 20i32 >= v265;
                    v268
                };
                let v270: bool = v269 == false;
                let v329: US2 = if v270 {
                    US2::US2_None
                } else {
                    { let _ = spiral_trace_hold(&v6); };
                    let (mut v274, mut v275, mut v276, mut v277, mut v278, mut v279): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                    let v280: Rc<str> = method9(v274.clone(), v275.clone(), v276.clone(), v277.clone(), v278.clone(), v279.clone());
                    let v281: Rc<str> = method11();
                    let v283: Option<std::string::String> = match &v100 {
                        US4::US4_None => {
                            let v282: Option<std::string::String> = None;
                            v282.clone()
                        }
                        _ => unreachable!(),
                    };
                    let v284: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v283.clone().into_iter().collect::<Vec<_>>()));
                    let v285: u64 = (v284.clone().borrow().len() as u64);
                    let v286: bool = v285 == 0u64;
                    let v292: Rc<str> = if v286 {
                        let v287: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
                        v287.clone()
                    } else {
                        let v288: std::string::String = v284.clone().borrow()[0u64 as usize].clone();
                        let v289: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::string::String>") } } (&&W(&v288)).s() });
                        let v290: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Some "); } LIT.with(|lit| lit.clone()) };
                        let v291: Rc<str> = Rc::<str>::from(format!("{}{}", v290, v289));
                        v291.clone()
                    };
                    let v293: Rc<str> = method23(v274.clone(), v275.clone(), v276.clone(), v277.clone(), v278.clone(), v279.clone(), v280.clone(), v281.clone(), v292.clone());
                    { let _ = spiral_trace_hold(&v6); };
                    let (mut v296, mut v297, mut v298, mut v299, mut v300, mut v301): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v8);
                    let v302: i64 = v296.borrow().l0.clone();
                    let v303: i64 = v302.wrapping_add(1i64);
                    v296.borrow_mut().l0 = v303;
                    let v304: Rc<dyn Fn(Rc<str>) -> ()> = closure8();
                    let v305: bool = cfg!(target_arch = "wasm32");
                    if v305 {
                        let v306: Rc<str> = v299.borrow().l0.clone();
                        let v307: bool = &*v306 == "";
                        let v315: Rc<str> = if v307 {
                            v293.clone()
                        } else {
                            let v308: bool = &*v293 == "";
                            if v308 {
                                let v309: Rc<str> = v299.borrow().l0.clone();
                                v309.clone()
                            } else {
                                let v310: Rc<str> = v299.borrow().l0.clone();
                                let v311: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v313: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v310, v311)), v293));
                                v313.clone()
                            }
                        };
                        let v317: usize = v315.chars().count();
                        let v318: i32 = ((v317) as i32);
                        let v319: i32 = v318.wrapping_add(14999i32);
                        let v320: i32 = v319.wrapping_div(15000i32);
                        let v321: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v322: bool = v293 != v321 ;
                        let v324: bool = if v322 {
                            let v323: bool = v320 <= 1i32;
                            v323
                        } else {
                            false
                        };
                        if v324 {
                            v299.borrow_mut().l0 = v315.clone();
                            ()
                        } else {
                            v299.borrow_mut().l0 = v321.clone();
                            { #[cfg(target_arch = "wasm32")] spiral_trace_near_log(&v315); };
                            ()
                        }
                    } else {
                        println!("{}", v293);
                        ()
                    };
                    let mut v327: Rc<dyn Fn(Rc<str>) -> ()> = v297.borrow().l0.clone();
                    v327(v293.clone());
                    US2::US2_Some(v296.clone(), v297.clone(), v298.clone(), v299.clone(), v300.clone(), v301.clone())
                };
                { (v2).insert((v16).clone(), (v4).clone()); };
                let v331: bool = v0.insert(v16.clone());
                let v333: bool = v1.insert(v4.clone());
                { let accounts = match (v3).get(&(v4.clone())) { None => { let mut accounts = std::collections::HashMap::new(); accounts.insert((v16).clone(), (v18, 0u32)); accounts } Some(previous) => { let mut ordered = previous.iter().collect::<Vec<_>>(); ordered.sort_unstable_by_key(|(_, (_, index))| *index); let mut accounts = ordered.iter().enumerate().map(|(i, (account_id, (timestamp, _)))| ((*account_id).clone(), (*timestamp, i as u32))).collect::<std::collections::HashMap<_, _>>(); accounts.insert((v16).clone(), (v18, ordered.len() as u32)); accounts } }; (v3).insert((v4).clone(), accounts); };
                ()
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method24(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<RefCell<Mut1>>, mut v2: Rc<RefCell<Mut2>>, mut v3: Rc<RefCell<Mut3>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: String, mut v9: Rc<str>) -> Rc<str> {
    let mut v10: i64 = v0.borrow().l0.clone();
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v11));
    let mut v13: Rc<str> = format_real_18(v10);
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v7)), v11));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chat_contract.get_account_info"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" / "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_id"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_id"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("account_id = "); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<String>") } } (&&W(&v8)).s() });
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("result"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v30, v31)), v25)), v9));
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v20, Rc::<str>::from(format!("{}{}", v36, v37))));
    method15(v39.clone())
}
fn closure10() -> Rc<dyn Fn(&near_sdk::store::LookupMap<String, std::string::String>, &near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, String) -> Option<(String, u64, u32)>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(&near_sdk::store::LookupMap<String, std::string::String>, &near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, String) -> Option<(String, u64, u32)>> = Rc::new(move |mut v0: &near_sdk::store::LookupMap<String, std::string::String>, mut v1: &near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, mut v2: String| -> Option<(String, u64, u32)> {
        let v4: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c1();
        { let _ = spiral_trace_hold(&v4); };
        let v6: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c3();
        let (mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v6);
        let mut v13: US0 = US0::US0_Info;
        v11.borrow_mut().l0 = v13.clone();
        let v14: Option<(String, u64, u32)> = (v0).get(&(v2.clone())).and_then(|alias| (v1).get(alias).and_then(|accounts| accounts.get(&(v2.clone())).map(|(timestamp, index)| (alias.clone(), *timestamp, *index))));
        { let _ = spiral_trace_hold(&v4); };
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v6);
        let mut v23: US0 = v21.borrow().l0.clone();
        let mut v28: i32 = match &v23 {
            US0::US0_Critical => {
                50i32
            }
            US0::US0_Debug => {
                20i32
            }
            US0::US0_Info => {
                30i32
            }
            US0::US0_Verbose => {
                10i32
            }
            US0::US0_Warning => {
                40i32
            }
        };
        let v29: bool = v19.borrow().l0.clone();
        let v30: bool = v29 == false;
        let mut v32: bool = if v30 {
            false
        } else {
            let v31: bool = 20i32 >= v28;
            v31
        };
        let v33: bool = v32 == false;
        let v82: US2 = if v33 {
            US2::US2_None
        } else {
            { let _ = spiral_trace_hold(&v4); };
            let (mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v6);
            let v43: Rc<str> = method9(v37.clone(), v38.clone(), v39.clone(), v40.clone(), v41.clone(), v42.clone());
            let v44: Rc<str> = method11();
            let v45: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<Option<(String, u64, u32)>>") } } (&&W(&v14)).s() });
            let mut v46: Rc<str> = method24(v37.clone(), v38.clone(), v39.clone(), v40.clone(), v41.clone(), v42.clone(), v43.clone(), v44.clone(), v2.clone(), v45.clone());
            { let _ = spiral_trace_hold(&v4); };
            let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v6);
            let v55: i64 = v49.borrow().l0.clone();
            let mut v56: i64 = v55.wrapping_add(1i64);
            v49.borrow_mut().l0 = v56;
            let mut v57: Rc<dyn Fn(Rc<str>) -> ()> = closure8();
            let v58: bool = cfg!(target_arch = "wasm32");
            if v58 {
                let mut v59: Rc<str> = v52.borrow().l0.clone();
                let v60: bool = &*v59 == "";
                let v68: Rc<str> = if v60 {
                    v46.clone()
                } else {
                    let v61: bool = &*v46 == "";
                    if v61 {
                        let v62: Rc<str> = v52.borrow().l0.clone();
                        v62.clone()
                    } else {
                        let v63: Rc<str> = v52.borrow().l0.clone();
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v63, v64)), v46));
                        v66.clone()
                    }
                };
                let v70: usize = v68.chars().count();
                let v71: i32 = ((v70) as i32);
                let v72: i32 = v71.wrapping_add(14999i32);
                let v73: i32 = v72.wrapping_div(15000i32);
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v75: bool = v46 != v74 ;
                let mut v77: bool = if v75 {
                    let v76: bool = v73 <= 1i32;
                    v76
                } else {
                    false
                };
                if v77 {
                    v52.borrow_mut().l0 = v68.clone();
                    ()
                } else {
                    v52.borrow_mut().l0 = v74.clone();
                    { #[cfg(target_arch = "wasm32")] spiral_trace_near_log(&v68); };
                    ()
                }
            } else {
                println!("{}", v46);
                ()
            };
            let mut v80: Rc<dyn Fn(Rc<str>) -> ()> = v50.borrow().l0.clone();
            v80(v46.clone());
            US2::US2_Some(v49.clone(), v50.clone(), v51.clone(), v52.clone(), v53.clone(), v54.clone())
        };
        v14.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method25(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<RefCell<Mut1>>, mut v2: Rc<RefCell<Mut2>>, mut v3: Rc<RefCell<Mut3>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: std::string::String) -> Rc<str> {
    let mut v9: i64 = v0.borrow().l0.clone();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v10));
    let v12: Rc<str> = format_real_18(v9);
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v12)), v7)), v10));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chat_contract.get_alias_map"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" / "); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alias"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alias"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alias = "); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::string::String>") } } (&&W(&v8)).s() });
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v27));
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v19, Rc::<str>::from(format!("{}{}", v29, v30))));
    method15(v32.clone())
}
fn closure11() -> Rc<dyn Fn(&near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, std::string::String) -> Option<std::collections::HashMap<String, (u64, u32)>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(&near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, std::string::String) -> Option<std::collections::HashMap<String, (u64, u32)>>> = Rc::new(move |mut v0: &near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, mut v1: std::string::String| -> Option<std::collections::HashMap<String, (u64, u32)>> {
        let v3: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c1();
        { let _ = spiral_trace_hold(&v3); };
        let v5: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c3();
        let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v5);
        let v12: US0 = US0::US0_Info;
        v10.borrow_mut().l0 = v12.clone();
        { let _ = spiral_trace_hold(&v3); };
        let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v5);
        let v21: US0 = v19.borrow().l0.clone();
        let v26: i32 = match &v21 {
            US0::US0_Critical => {
                50i32
            }
            US0::US0_Debug => {
                20i32
            }
            US0::US0_Info => {
                30i32
            }
            US0::US0_Verbose => {
                10i32
            }
            US0::US0_Warning => {
                40i32
            }
        };
        let v27: bool = v17.borrow().l0.clone();
        let mut v28: bool = v27 == false;
        let v30: bool = if v28 {
            false
        } else {
            let v29: bool = 20i32 >= v26;
            v29
        };
        let v31: bool = v30 == false;
        let v79: US2 = if v31 {
            US2::US2_None
        } else {
            { let _ = spiral_trace_hold(&v3); };
            let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v5);
            let mut v41: Rc<str> = method9(v35.clone(), v36.clone(), v37.clone(), v38.clone(), v39.clone(), v40.clone());
            let v42: Rc<str> = method11();
            let v43: Rc<str> = method25(v35.clone(), v36.clone(), v37.clone(), v38.clone(), v39.clone(), v40.clone(), v41.clone(), v42.clone(), v1.clone());
            { let _ = spiral_trace_hold(&v3); };
            let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = spiral_trace_hold(&v5);
            let mut v52: i64 = v46.borrow().l0.clone();
            let v53: i64 = v52.wrapping_add(1i64);
            v46.borrow_mut().l0 = v53;
            let v54: Rc<dyn Fn(Rc<str>) -> ()> = closure8();
            let v55: bool = cfg!(target_arch = "wasm32");
            if v55 {
                let mut v56: Rc<str> = v49.borrow().l0.clone();
                let mut v57: bool = &*v56 == "";
                let v65: Rc<str> = if v57 {
                    v43.clone()
                } else {
                    let v58: bool = &*v43 == "";
                    if v58 {
                        let mut v59: Rc<str> = v49.borrow().l0.clone();
                        v59.clone()
                    } else {
                        let v60: Rc<str> = v49.borrow().l0.clone();
                        let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v63: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v60, v61)), v43));
                        v63.clone()
                    }
                };
                let v67: usize = v65.chars().count();
                let v68: i32 = ((v67) as i32);
                let v69: i32 = v68.wrapping_add(14999i32);
                let v70: i32 = v69.wrapping_div(15000i32);
                let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v72: bool = v43 != v71 ;
                let v74: bool = if v72 {
                    let v73: bool = v70 <= 1i32;
                    v73
                } else {
                    false
                };
                if v74 {
                    v49.borrow_mut().l0 = v65.clone();
                    ()
                } else {
                    v49.borrow_mut().l0 = v71.clone();
                    { #[cfg(target_arch = "wasm32")] spiral_trace_near_log(&v65); };
                    ()
                }
            } else {
                println!("{}", v43);
                ()
            };
            let mut v77: Rc<dyn Fn(Rc<str>) -> ()> = v47.borrow().l0.clone();
            v77(v43.clone());
            US2::US2_Some(v46.clone(), v47.clone(), v48.clone(), v49.clone(), v50.clone(), v51.clone())
        };
        let mut v80: Option<std::collections::HashMap<String, (u64, u32)>> = (v0).get(&(v1)).cloned();
        v80.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn chat_contract_new() -> (u32, near_sdk::store::IterableSet<String>, near_sdk::store::IterableSet<std::string::String>, near_sdk::store::LookupMap<String, std::string::String>, near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>) {
    closure0()()
}
pub fn chat_contract_is_valid_alias(v0: std::string::String) -> bool {
    closure4()(v0)
}
pub fn chat_contract_generate_cid(v0: Vec<u8>) -> std::string::String {
    closure5()(v0)
}
pub fn chat_contract_claim_alias(v0: &mut near_sdk::store::IterableSet<String>, v1: &mut near_sdk::store::IterableSet<std::string::String>, v2: &mut near_sdk::store::LookupMap<String, std::string::String>, v3: &mut near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, v4: std::string::String) -> () {
    closure6()(v0, v1, v2, v3, v4)
}
pub fn chat_contract_get_account_info(v0: &near_sdk::store::LookupMap<String, std::string::String>, v1: &near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, v2: String) -> Option<(String, u64, u32)> {
    closure10()(v0, v1, v2)
}
pub fn chat_contract_get_alias_map(v0: &near_sdk::store::LookupMap<std::string::String, std::collections::HashMap<String, (u64, u32)>>, v1: std::string::String) -> Option<std::collections::HashMap<String, (u64, u32)>> {
    closure11()(v0, v1)
}
