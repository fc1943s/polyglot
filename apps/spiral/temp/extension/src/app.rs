use std::collections::HashMap;

use crate::state::{State, *};

use leptos::{logging::log, *};
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::location::*;
use leptos_router::hooks::*;

#[component]
pub fn App() -> impl IntoView {
    provide_context(GlobalState {
        messages: RwSignal::new(Messages {
            outer: RwSignal::new(vec![]),
            inner: RwSignal::new(vec![]),
        }),
        state: RwSignal::new(State {
            dark_mode: RwSignal::new(true),
            account_id: RwSignal::new("".to_string()),
            network_id: RwSignal::new("".to_string()),
            s1: RwSignal::new("".to_string()),
            s2: RwSignal::new("".to_string()),
        }),
        loading: RwSignal::new(HashMap::new()),
    });

    view! {
        <Router>
            <Home />
        </Router>
    }
}

pub fn use_location_host() -> (String, String, String) {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let protocol = location.protocol().unwrap();
    let host = location.host().unwrap();
    let port = location.port().unwrap();

    let result = (protocol, host, port);
    log!("app:use_location_host () / result: {:#?}", result);

    result
}

#[component]
fn Home() -> impl IntoView {
    log!("Home ()");

    let window = web_sys::window().unwrap();

    let (_protocol, host, _port) = use_location_host();

    let _cancel = match host.as_str() {
        "libgen.is" => Some(crate::js::use_interval(
            &window,
            |document| {
                let _state = crate::timer::update(&document);
            },
            1000,
            Some(3600000),
        )),
        _ => None,
    };

    let global_state1 = use_context::<GlobalState>();
    let global_state2 = use_context::<GlobalState>();
    // let global_state_resource = LocalResource::new(
    //     async || {
    //         log!("Home () / global_state_resource create_local_resource");
    //         std::sync::Arc::new(std::sync::Mutex::new(global_state1.clone()))
    //     },
    // );

    let global_state_json = Memo::new(move |_| {
        let json = match &global_state1 {
            Some(global_state) => serde_json::to_string_pretty(&global_state).unwrap(),
            _ => "".to_string(),
        };
        log!(
            "Home () / global_state_json Memo::new / json: {:#?}",
            json
        );

        json
    });

    fn format_location(location: &Location, url_fn: impl Fn(Url) -> Url) -> String {
        let pathname = location.pathname.get();
        let search = location.search.get();
        let hash = location.hash.get();
        // let url = Url {
        //     origin: "".to_string(),
        //     search_params: ParamsMap::default(),
        //     path: pathname,
        //     search,
        //     hash,
        // };
        let url = RequestUrl::parse(&RequestUrl::new(&format!("{}{}{}", pathname, search, hash))).unwrap();
        let url = url_fn(url);
        format!(
            "{}{}{}{}",
            url.path(),
            if url.search() == "" { "" } else { "?" },
            url.search(),
            url.hash()
        )
    }

    // let router1 = use_location();
    // let router2 = use_location();
    // let router3 = use_location();
    // let route_data: Option<String> = use_route_data();

    let query_map = use_query_map();
    let params_map = use_params_map();
    let location1 = use_location();
    let location2 = use_location();
    let location3 = use_location();
    let location4 = use_location();
    // let location5 = use_location();
    let navigate = use_navigate();
    // let navigate2 = use_navigate();

    window_event_listener(ev::hashchange, move |_| {
        let hash = crate::leptos_dom::helpers::location_hash().unwrap_or("".to_string());
        log!(
            "Home () / window_event_listener ev::hashchange / location_hash(): {:#?}",
            hash
        );
    });

    let (location_text, set_location_text) = signal("".to_string());
    let set_location_text_2 = set_location_text.clone();

    Effect::new(move |_| {
        let new_location_text = format_location(&location2, |url| url);
        log!("Home () / new_location_text: {:#?}", new_location_text);
        set_location_text_2.set(new_location_text);
    });

    // Effect::new(move |_| {
    //     log!(
    //         "Home () / router1.pathname() effect: {:#?}",
    //         router1.pathname().get()
    //     )
    // });
    // Effect::new(move |_| {
    //     log!(
    //         "Home () / router2.possible_branches()** effect: {:#?}",
    //         router2.possible_branches()
    //     )
    // });
    // Effect::new(move |_| log!("Home () / route_data** effect: {:#?}", route_data));
    Effect::new(move |_| log!("Home () / query_map.get() effect: {:#?}", query_map.get()));
    Effect::new(move |_| {
        log!(
            "Home () / params_map.get()** effect: {:#?}",
            params_map.get()
        )
    });
    Effect::new(move |_| {
        log!(
            "Home () / location1.search.get() effect: {:#?}",
            location1.search.get()
        )
    });
    // Effect::new(move |_| log!("Home () / router3 effect: {:#?}", router3.base().path()));

    Effect::new(move |_| {
        let hash = location3.hash.get();
        let hash = hash.strip_prefix('#').unwrap_or("");
        log!("Home () / location3.hash.get(): {:#?}", hash);

        {
            let hash_url = <BrowserUrl as LocationProvider>::parse(hash);
            log!("Home () / hash_url: {:#?}", hash_url);
        }

        let json = js_sys::decode_uri_component(&hash)
            .unwrap()
            .as_string()
            .unwrap();
        log!("Home () / hash json: {:#?}", json);
    });

    let (location_href, set_location_href) = signal("".to_string());

    let window = web_sys::window().unwrap();

    Effect::new(move |_| {
        let new_location_href = window.location().href().ok().unwrap_or("".to_string());
        set_location_href.set(new_location_href.clone());

        crate::timer::set_albums().unwrap();

        log!(
            "Home () / Effect::new all url changes / location4.state.get(): {:#?} / new_location_href: {:#?}",
            location4.state.get(),
            new_location_href
        );
    });

    Effect::new(move |_| {
        log!(
            "Home () / location_href.get() effect: {:#?}",
            location_href.get()
        )
    });

    view! {
        <>
            {(|| {
                log!("Home () / render1");
                view! {<></>}
            })()}

            <MessagesProcessor />

            {(|| {
                log!("Home () / render2");
                view! {<></>}
            })()}

            <div class:dark={move || global_state2.clone().unwrap().state.get().dark_mode.get()}>
                <div>
                    <input
                        prop:value={move || location_text.get()}
                        on:keyup={move |ev: web_sys::KeyboardEvent| {
                            set_location_text.set(event_target_value(&ev));
                        }}
                    />
                    <button
                        on:click=move |_| navigate(&location_text.get(), Default::default())
                    >
                        navigate
                    </button>
                </div>
                <div>
                    global_state_json: <pre>{move || global_state_json}</pre>
                </div>
            </div>
        </>
    }
}
