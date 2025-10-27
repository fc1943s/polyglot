namespace Types


// #if FABLE_COMPILER_RUST

open Fable.Core
open Fable.Core.Rust

[<Erase; Emit("[$0]")>]
type Array<'T> = class end
[<Erase; Emit("Box<$0>")>]
type Box<'T> = class end
[<Erase; Emit("dyn $0")>]
type Dyn<'T> = class end
[<Erase; Emit("$0")>]
type EmitType<'T> = class end
[<Erase; Emit("mut $0")>]
type Mut<'T> = class end
[<Erase; Emit("RangeInclusive<$0>")>]
type RangeInclusive<'T> = class end
[<Erase; Emit("&$0")>]
type Ref<'T> = class end
[<Erase; Emit("&'static $0")>]
type StaticRef<'T> = class end
[<Erase; Emit("str")>]
type Str = class end
[<Erase; Emit("($0, $1)")>]
type Tuple<'A, 'B> = class end
[<Erase; Emit("u16")>]
type U16 = class end
[<Erase; Emit("Vec<$0>")>]
type Vec<'T> = class end

module std =
    module cell =
        [<Erase; Emit("std::cell::Ref<$0>")>]
        type CellRef<'T> = class end
        [<Erase; Emit("std::cell::RefCell<$0>")>]
        type RefCell<'T> = class end

    module collections =
        [<Erase; Emit("std::collections::VecDeque<$0>")>]
        type VecDeque<'T> = class end

    module fs =
        [<Erase; Emit("std::fs::File")>]
        type File = class end

    module io =
        [<Erase; Emit("std::io::Error")>]
        type Error = class end

    module iter =
        [<Erase; Emit("_")>]
        type Iterator<'T> = class end

    module net =
        [<Erase; Emit("std::net::SocketAddr")>]
        type SocketAddr = class end
        [<Erase; Emit("std::net::TcpListener")>]
        type TcpListener = class end

    module path =
        [<Erase; Emit("std::path::Ancestors")>]
        type Ancestors = class end
        [<Erase; Emit("std::path::Path")>]
        type Path = class end
        [<Erase; Emit("std::path::PathBuf")>]
        type PathBuf = class end
        [<Erase; Emit("std::path::StripPrefixError")>]
        type StripPrefixError = class end

    module process =
        [<Erase; Emit("std::process::Child")>]
        type Child = class end

    module rc =
        [<Erase; Emit("std::rc::Rc<$0>")>]
        type Rc<'T> = class end

    module string =
        [<Erase; Emit("std::string::String")>]
        type String = class end

    module sync =
        [<Erase; Emit("std::sync::Arc<$0>")>]
        type Arc<'T> = class end
        [<Erase; Emit("std::sync::Mutex<$0>")>]
        type Mutex<'T> = class end

    module time =
        [<Erase; Emit("std::time::SystemTime")>]
        type SystemTime = class end

module async_std =
    module sync =
        [<Erase; Emit("async_std::sync::Condvar")>]
        type Condvar = class end
        [<Erase; Emit("async_std::sync::Mutex<$0>")>]
        type Mutex<'T> = class end

module chrono =
    [<Erase; Emit("chrono::Utc")>]
    type Utc = class end
    [<Erase; Emit("chrono::Local")>]
    type Local = class end
    [<Erase; Emit("chrono::DateTime<$0>")>]
    type DateTime<'T> = class end

module clap =
    module builder =
        [<Erase; Emit("clap::builder::Arg")>]
        type Arg = class end
        [<Erase; Emit("clap::builder::Command")>]
        type Command = class end

    module parser =
        [<Erase; Emit("clap::parser::ArgMatches")>]
        type ArgMatches = class end

module colored =
    [<Erase; Emit("colored::ColoredString")>]
    type ColoredString = class end

module core =
    module future =
        [<Erase; Emit("_")>]
        type Future<'T> = class end

module dominator =
    [<Erase; Emit("dominator::Dom")>]
    type Dom = class end
    [<Erase; Emit("dominator::DomBuilder<$0>")>]
    type DomBuilder<'T> = class end

module ehttp =
    [<Erase; Emit("ehttp::Request")>]
    type Request = class end
    [<Erase; Emit("ehttp::Response")>]
    type Response = class end

module fable_library_rust =
    module Native_ =
        [<Erase; Emit("fable_library_rust::Native_::Box_<$0>")>]
        type Box<'T> = class end

module futures_signals =
    module signal =
        [<Erase; Emit("futures_signals::signal::Mutable<$0>")>]
        type Mutable<'T> = class end
        [<Erase; Emit("futures_signals::signal::MutableSignal<$0>")>]
        type MutableSignal<'T> = class end

module indexed_db_futures =
    module request =
        [<Erase; Emit("indexed_db_futures::request::OpenDbRequest")>]
        type OpenDbRequest = class end

module linereader =
    [<Erase; Emit("linereader::LineReader<$0>")>]
    type LineReader<'T> = class end

module regex =
    [<Erase; Emit("regex::Regex")>]
    type Regex = class end
    [<Erase; Emit("regex::RegexBuilder")>]
    type RegexBuilder = class end

module rocket =
    module http =
        module uri =
            [<Erase; Emit("rocket::http::uri::Segments<'_, $0>")>]
            type Segments<'T> = class end

            module fmt =
                [<Erase; Emit("rocket::http::uri::fmt::Path")>]
                type Path = class end

module wasm_bindgen =
    [<Erase; Emit("wasm_bindgen::JsValue")>]
    type JsValue = class end

module web_sys =
    [<Erase; Emit("web_sys::Element")>]
    type Element = class end
    [<Erase; Emit("web_sys::HtmlElement")>]
    type HtmlElement = class end
    [<Erase; Emit("web_sys::HtmlIFrameElement")>]
    type HtmlIFrameElement = class end
    [<Erase; Emit("web_sys::HtmlInputElement")>]
    type HtmlInputElement = class end
    [<Erase; Emit("web_sys::HtmlStyleElement")>]
    type HtmlStyleElement = class end

//// #else
////     ()
//// #endif //FABLE_COMPILER_RUST
