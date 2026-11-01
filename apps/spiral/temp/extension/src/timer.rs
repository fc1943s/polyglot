#![allow(special_module_name)]

use crate::js::log;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Window};

pub fn update(document: &web_sys::Document) -> (u32, usize) {
    let inputs = document.get_elements_by_tag_name("input");
    log(&format!("inputs.length(): {}", inputs.length()));

    let valid_inputs = (0..inputs.length())
        .filter_map(|i| {
            let input = inputs.item(i)?.dyn_into::<HtmlInputElement>().unwrap();
            match input.type_().as_str() {
                "password" | "checkbox" | "number" | "hidden" | "submit" | "file" | "radio"
                | "color" | "email" | "range" => None,
                _ => Some(input),
            }
        })
        .collect::<Vec<HtmlInputElement>>();

    log(&format!("valid_inputs.len(): {}", valid_inputs.len()));

    for i in 0..valid_inputs.len() {
        let input = &valid_inputs[i];
        let cursor_position = input.selection_start().unwrap().unwrap_or(0);
        let value = input.value().to_uppercase();
        input.set_value(&value);
        input
            .set_selection_range(cursor_position, cursor_position)
            .unwrap_or_else(|x| {
                log(&format!("set_selection_range error: {:?}", x));
            });
    }

    (inputs.length(), valid_inputs.len())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct AlbumInfo {
    artist: String,
    album_name: String,
    real_row_index: usize,
    index: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum AlbumData {
    AlbumInfo(AlbumInfo),
}

#[wasm_bindgen]
pub fn set_albums() -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("should have a Window");
    let document: Document = window.document().expect("should have a Document");
    let document = std::rc::Rc::new(document);

    let album_containers = document.query_selector_all("div[data-track--page-id=mycollection_albums] .ReactVirtualized__Grid.ReactVirtualized__List .ReactVirtualized__Grid__innerScrollContainer")?;

    let _albums = (0..album_containers.length())
        .into_iter()
        .flat_map(|i| {
            let document = std::rc::Rc::clone(&document);
            let container = album_containers
                .get(i)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            let rows = container
                .query_selector_all("div [role=row][aria-rowindex]")
                .unwrap();
            (0..rows.length()).into_iter().map(move |j| {
                let row = rows.get(j).unwrap().dyn_into::<Element>().unwrap();
                let real_row_index = row
                    .get_attribute("aria-rowindex")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let cell_albums = document.query_selector_all(".cell-album").unwrap();

                (0..cell_albums.length())
                    .into_iter()
                    .map(move |k| {
                        let album = cell_albums.get(k).unwrap().dyn_into::<Element>().unwrap();
                        let album_name = album
                            .query_selector("wave-text")
                            .unwrap()
                            .unwrap()
                            .inner_html()
                            .replace("\"", "'");
                        let artist_elem = album.query_selector(".artist-link a");
                        let artist = artist_elem.map_or_else(
                            |_| album_name.clone(),
                            |el| el.unwrap().inner_html().replace("\"", "'"),
                        );

                        let index = k + 1;

                        let album_info = AlbumInfo {
                            artist,
                            album_name,
                            real_row_index,
                            index,
                        };

                        Ok(album_info)
                    })
                    .collect::<Result<Vec<AlbumInfo>, JsValue>>()
            })
        })
        .map(|x| x.unwrap())
        .flatten()
        .collect::<Vec<_>>();

    log(&format!("_albums: {:#?}", _albums));

    Ok(())
}
