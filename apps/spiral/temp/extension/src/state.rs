use leptos::{logging::log, *};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use leptos::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OuterMessage {
    SetS1 { s: String },
    SetAccountId { account_id: String },
    SetNetworkId { network_id: String },
    SetStorage { key: String, value: Option<String> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InnerMessage {
    Ack,
    SetS2 { s: String },
    SetStorage { key: String, value: String },
    GetStorage { key: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Messages {
    pub outer: RwSignal<Vec<(u64, OuterMessage)>>,
    pub inner: RwSignal<Vec<(u64, InnerMessage)>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub dark_mode: RwSignal<bool>,
    pub account_id: RwSignal<String>,
    pub network_id: RwSignal<String>,
    pub s1: RwSignal<String>,
    pub s2: RwSignal<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalState {
    pub messages: RwSignal<Messages>,
    pub state: RwSignal<State>,
    pub loading: RwSignal<HashMap<String, RwSignal<bool>>>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    fn setOuterMessagesAsync(messagesJson: &str) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch)]
    fn setInnerMessagesAsync(messagesJson: &str) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch)]
    fn setOuterMessages(messagesJson: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    fn setInnerMessages(messagesJson: &str) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
pub async fn set_outer_messages_async(messages_json: &str) -> Result<JsValue, JsValue> {
    let promise = setOuterMessagesAsync(messages_json)?;
    Ok(wasm_bindgen_futures::JsFuture::from(promise).await?)
}

#[wasm_bindgen]
pub async fn set_inner_messages_async(messages_json: &str) -> Result<JsValue, JsValue> {
    let promise = setInnerMessagesAsync(messages_json)?;
    Ok(wasm_bindgen_futures::JsFuture::from(promise).await?)
}

#[wasm_bindgen]
pub fn set_outer_messages(messages_json: &str) -> Result<JsValue, JsValue> {
    setOuterMessages(messages_json)
}

#[wasm_bindgen]
pub fn set_inner_messages(messages_json: &str) -> Result<JsValue, JsValue> {
    setInnerMessages(messages_json)
}

#[wasm_bindgen]
pub async fn set_outer_messages_state(messages_json: String) -> Result<JsValue, JsValue> {
    let global_state = use_context::<GlobalState>();

    match global_state {
        Some(global_state) => {
            global_state
                .messages
                .get_untracked()
                .outer
                .update(|outer| *outer = serde_json::from_str(&messages_json).unwrap());
        }
        None => (),
    }

    Ok(JsValue::from_str(&format!(
        "set_outer_messages_state: {}",
        messages_json
    )))
}

#[wasm_bindgen]
pub async fn set_inner_messages_state(messages_json: String) -> Result<JsValue, JsValue> {
    let global_state = use_context::<GlobalState>();

    match global_state {
        Some(global_state) => {
            global_state
                .messages
                .get_untracked()
                .inner
                .update(|inner| *inner = serde_json::from_str(&messages_json).unwrap());
        }
        None => (),
    }

    Ok(JsValue::from_str(&format!(
        "set_inner_messages_state: {}",
        messages_json
    )))
}

// fn _use_set_inner_messages_async() -> leptos::prelude::Action<String, Result<JsValue, JsValue>> {
//     Action::new(|json: &String| {
//         let json = json.to_owned();
//         async move { set_inner_messages_async(&json).await }
//     })
// }

// fn _use_set_outer_messages_async() -> leptos::prelude::Action<String, Result<JsValue, JsValue>> {
//     Action::new(|json: &String| {
//         let json = json.to_owned();
//         async move { set_outer_messages_async(&json).await }
//     })
// }

pub fn use_set_inner_messages() -> impl Fn(String) -> Result<JsValue, JsValue> {
    |json| set_inner_messages(&json)
}

pub fn _use_set_outer_messages() -> impl Fn(String) -> Result<JsValue, JsValue> {
    |json| set_outer_messages(&json)
}

pub const DARK_MODE_KEY: &str = "dark-mode";

#[component]
pub fn MessagesProcessor() -> impl IntoView {
    log!("MessagesProcessor ()");

    // let set_outer_messages = use_set_outer_messages_async();
    // let set_inner_messages = use_set_inner_messages_async();
    // let set_outer_messages = use_set_outer_messages();
    let set_inner_messages = use_set_inner_messages();

    let global_state = use_context::<GlobalState>();
    Effect::new(move |_| {
        log!("MessagesProcessor () / create_effect");
        let global_state = global_state.clone().unwrap();
        let messages = global_state.messages.get();
        let outer_messages = messages.outer.get();
        let outer_messages_len = outer_messages.len();

        let new_messages = if outer_messages.len() == 0 {
            (vec![], vec![])
        } else {
            outer_messages
                .into_iter()
                .map(|message| match message.clone() {
                    (id, OuterMessage::SetS1 { s }) => {
                        let s2 = s.to_uppercase();
                        global_state.state.update(|state| {
                            state.s1.set(s);
                            state.s2.set(s2.clone());
                        });
                        (None, vec![(id, InnerMessage::SetS2 { s: s2 })])
                    }
                    (id, OuterMessage::SetStorage { key, value }) => {
                        match key.as_str() {
                            DARK_MODE_KEY => {
                                if let Some(value) = value {
                                    let dark_mode = serde_json::from_str::<bool>(&value).unwrap();
                                    global_state.state.get().dark_mode.set(dark_mode);
                                }
                            }
                            _ => log!(
                                "MessageProcessor () / create_effect / invalid message / message: {:#?}",
                                message
                            ),
                        }
                        global_state.loading.update(|loading| {
                            match loading.get(&key) {
                                Some(loading) => loading.set(false),
                                _ => (),
                            };
                        });
                        (None, vec![(id, InnerMessage::Ack)])
                    }
                    (id, OuterMessage::SetAccountId { account_id }) => {
                        global_state.state.get().account_id.set(account_id);
                        (None, vec![(id, InnerMessage::Ack)])
                    }
                    (id, OuterMessage::SetNetworkId { network_id }) => {
                        global_state.state.get().network_id.set(network_id);
                        (None, vec![(id, InnerMessage::Ack)])
                    }
                })
                .fold((vec![], vec![]), |(outer, inner), (outer2, inner2)| {
                    (
                        outer.into_iter().chain(outer2.into_iter()).collect(),
                        inner.into_iter().chain(inner2.into_iter()).collect(),
                    )
                })
        };

        if new_messages.1.len() > 0 {
            messages.inner.update(|inner| {
                inner.extend(new_messages.1.clone());
            });

            let json = serde_json::to_string(&new_messages.1).unwrap();
            // set_inner_messages.dispatch(json);
            // let result = set_inner_messages.value().get();
            let result = set_inner_messages(json);
            log!(
                "MessagesProcessor () / create_effect / set_inner_messages / result: {:#?}",
                result
            );
        }

        if new_messages.0.len() != outer_messages_len {
            messages.outer.set(new_messages.0.clone());

            // let json = serde_json::to_string(&new_messages.0).unwrap();
            //// set_outer_messages.dispatch(json);
            //// let result = set_outer_messages.value().get();
            // let result = set_outer_messages(json);
            // log!(
            //     "MessagesProcessor / set_outer_messages / result: {:#?}",
            //     result
            // );
        }
    });

    view! {
        <>
            {(|| {
                log!("MessagesProcessor () / render");
                view! {<></>}
            })()}
        </>
    }
}
