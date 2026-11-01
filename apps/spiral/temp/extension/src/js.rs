use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(value: &str);
}

pub fn use_interval(
    window: &web_sys::Window,
    mut f: impl FnMut(std::rc::Rc<web_sys::Document>) + 'static,
    timeout: i32,
    cancel_timeout: Option<i32>,
) -> impl Fn() {
    let document = std::rc::Rc::new(window.document().unwrap());
    let closure =
        Closure::wrap(Box::new(move || f(std::rc::Rc::clone(&document))) as Box<dyn FnMut()>);

    let id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            timeout,
        )
        .unwrap();

    let window_clone = window.clone();
    let cancel = move || {
        window_clone.clear_interval_with_handle(id);
    };

    match cancel_timeout {
        Some(cancel_timeout) => {
            let cancel_closure = Closure::wrap(Box::new(cancel.clone()) as Box<dyn FnMut()>);

            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cancel_closure.as_ref().unchecked_ref(),
                    cancel_timeout,
                )
                .unwrap();

            cancel_closure.forget();
        }
        None => {}
    }

    closure.forget();

    cancel
}
