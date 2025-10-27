#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("pyo3::Python")>]
#endif
type pyo3_Python = class end
#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("num_complex::Complex<$0>")>]
#endif
type num_complex_Complex<'T> = class end
#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("&$0")>]
type Ref<'T> = class end
#else
type Ref<'T> = 'T
#endif

#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("std::string::String")>]
type std_string_String = class end
#else
type std_string_String = string
#endif

#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("std::ffi::CString")>]
#endif
type std_ffi_CString = class end
#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("pyo3::PyErr")>]
#endif
type pyo3_PyErr = class end
#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("pyo3::Bound<$0>")>]
#endif
type pyo3_Bound<'T> = class end
Fable.Core.RustInterop.emitRustExpr () ");
use pyo3::prelude::PyAnyMethods;
//"
#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("str")>]
type Str = class end
#else
type Str = string
#endif

#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("pyo3::types::PyModule")>]
#endif
type pyo3_types_PyModule = class end
#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("pyo3::PyAny")>]
#endif
type pyo3_PyAny = class end
type Mut0 = {mutable l0 : int32}
and Mut1 = {mutable l0 : int32; mutable l1 : string; mutable l2 : string}
and Mut2 = {mutable l0 : int32; mutable l1 : num_complex_Complex<float>}
and [<Struct>] US0 =
    | US0_0 of f0_0 : num_complex_Complex<float>
    | US0_1
and Mut3 = {mutable l0 : string}
and UH0 =
    | UH0_0
    | UH0_1 of float * UH0
and UH1 =
    | UH1_0
    | UH1_1 of num_complex_Complex<float> * UH1
let rec method2 (v0 : int32, v1 : Mut0) : bool =
    let v2 : int32 = v1.l0
    let v3 : bool = v2 < v0
    v3
and method3 (v0 : num_complex_Complex<float>) : num_complex_Complex<float> =
    v0
and method6 (v0 : int32, v1 : Mut1) : bool =
    let v2 : int32 = v1.l0
    let v3 : bool = v2 < v0
    v3
and method5 (v0 : (string [])) : string =
    let v1 : int32 = v0.Length
    let v2 : string = ""
    let v3 : Mut1 = {l0 = 0; l1 = v2; l2 = v2} : Mut1
    while method6(v1, v3) do
        let v5 : int32 = v3.l0
        let v6 : int32 =  -v5
        let v7 : int32 = v6 + v1
        let v8 : int32 = v7 - 1
        let struct (v9 : string, v10 : string) = v3.l1, v3.l2
        let v11 : string = v0.[int v8]
        let v13 : string = v11 + v10 
        let v17 : string = v13 + v9 
        let v20 : int32 = v5 + 1
        let v21 : string = "\n"
        v3.l0 <- v20
        v3.l1 <- v17
        v3.l2 <- v21
        ()
    let struct (v22 : string, v23 : string) = v3.l1, v3.l2
    v22
and method7 (v0 : pyo3_Python) : pyo3_Python =
    v0
and method8 () : string =
    let v0 : string = "fn"
    v0
and method9 (v0 : pyo3_Bound<pyo3_types_PyModule>) : pyo3_Bound<pyo3_types_PyModule> =
    v0
and method10 (v0 : (bool * (float * float))) : (bool * (float * float)) =
    v0
and method11 (v0 : pyo3_Bound<pyo3_PyAny>) : pyo3_Bound<pyo3_PyAny> =
    v0
and method12 (v0 : pyo3_Bound<pyo3_PyAny>) : pyo3_Bound<pyo3_PyAny> =
    v0
and method4 (v0 : pyo3_Python, v1 : string, v2 : num_complex_Complex<float>) : Result<num_complex_Complex<float>, std_string_String> =
    let v3 : string = $"import sys"
    let v4 : string = $"import traceback"
    let v5 : string = $"import re"
    let v6 : string = $"count = 0"
    let v7 : string = $"memory_address_pattern = re.compile(r' at 0x[0-9a-fA-F]+')"
    let v8 : string = $"def trace_calls(frame, event, arg):"
    let v9 : string = $"    global count"
    let v10 : string = $"    count += 1"
    let v11 : string = $"    if count < 200:"
    let v12 : string = $"        try:"
    let v13 : string = $"            args = {{ k: v for k, v in frame.f_locals.items() if frame.f_code.co_name != 'make_mpc' and k not in ['ctx'] and not callable(v) }}"
    let v14 : string = $"            args_str = ', '.join([ f\"{{k}}={{re.sub(memory_address_pattern, ' at 0x<?>', repr(v))}}\" for k, v in args.items() ])"
    let v15 : string = "zeta_"
    let v16 : string = $"            print(f\"{{event}}({v15}) / f_code.co_name: {{frame.f_code.co_name}} / f_locals: {{args_str}} / f_lineno: {{frame.f_lineno}} / f_code.co_filename: {{frame.f_code.co_filename.split('site-packages')[-1]}} / f_back.f_lineno: {{ '' if frame.f_back is None else frame.f_back.f_lineno }} / f_back.f_code.co_filename: {{ '' if frame.f_back is None else frame.f_back.f_code.co_filename.split('site-packages')[-1] }} / arg: {{re.sub(memory_address_pattern, ' at 0x<?>', repr(arg))}}\", flush=True)"
    let v17 : string = $"        except ValueError as e:"
    let v18 : string = $"            print(f'{v15} / e: {{e}}', flush=True)"
    let v19 : string = $"        return trace_calls"
    let v20 : string = $"import mpmath"
    let v21 : string = $"def fn(log, s):"
    let v22 : string = $"    global count"
    let v23 : string = $"    if log:"
    let v24 : string = $"        print(f'{v15} / s: {{s}} / count: {{count}}', flush=True)"
    let v25 : string = $"    s = complex(*s)"
    let v26 : string = $"    try:"
    let v27 : string = $"        if log: sys.settrace(trace_calls)"
    let v28 : string = $"        if log:"
    let v29 : string = $"            sys.settrace(None)"
    let v30 : string = $"            print(f'{v15} / result: {{s}} / count: {{count}}', flush=True)"
    let v31 : string = $"    except ValueError as e:"
    let v32 : string = $"        if s.real == 1:"
    let v33 : string = $"            s = complex(float('inf'), 0)"
    let v34 : string = $"    return (s.real, s.imag)"
    let v35 : (string []) = [|v3; v4; v5; v6; v7; v8; v9; v10; v11; v12; v13; v14; v16; v17; v18; v19; v20; v21; v22; v23; v24; v25; v26; v27; v1; v28; v29; v30; v31; v32; v33; v34|]
    let v36 : string = method5(v35)
    let v37 : string = "$0.re"
    let v38 : float = Fable.Core.RustInterop.emitRustExpr v2 v37 
    let v39 : string = "$0.im"
    let v40 : float = Fable.Core.RustInterop.emitRustExpr v2 v39 
    let v42 : (float * float) = v38, v40 
    let v46 : (bool * (float * float)) = false, v42 
    let v49 : pyo3_Python = method7(v0)
    (* run_target_args'
    let v54 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v55 : string = "&*$0"
    let v56 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v36 v55 
    let _run_target_args'_v54 = v56 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v57 : string = "&*$0"
    let v58 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v36 v57 
    let _run_target_args'_v54 = v58 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v59 : string = "&*$0"
    let v60 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v36 v59 
    let _run_target_args'_v54 = v60 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v62 : Ref<Str> = v36 |> unbox<Ref<Str>>
    let _run_target_args'_v54 = v62 
    #endif
#if FABLE_COMPILER_PYTHON
    let v66 : Ref<Str> = v36 |> unbox<Ref<Str>>
    let _run_target_args'_v54 = v66 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v70 : Ref<Str> = v36 |> unbox<Ref<Str>>
    let _run_target_args'_v54 = v70 
    #endif
#else
    let v74 : Ref<Str> = v36 |> unbox<Ref<Str>>
    let _run_target_args'_v54 = v74 
    #endif
    let v77 : Ref<Str> = _run_target_args'_v54 
    (* run_target_args'
    let v90 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v91 : string = "String::from($0)"
    let v92 : std_string_String = Fable.Core.RustInterop.emitRustExpr v77 v91 
    let _run_target_args'_v90 = v92 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v93 : string = "String::from($0)"
    let v94 : std_string_String = Fable.Core.RustInterop.emitRustExpr v77 v93 
    let _run_target_args'_v90 = v94 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v95 : string = "String::from($0)"
    let v96 : std_string_String = Fable.Core.RustInterop.emitRustExpr v77 v95 
    let _run_target_args'_v90 = v96 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v98 : std_string_String = v77 |> unbox<std_string_String>
    let _run_target_args'_v90 = v98 
    #endif
#if FABLE_COMPILER_PYTHON
    let v102 : std_string_String = v77 |> unbox<std_string_String>
    let _run_target_args'_v90 = v102 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v106 : std_string_String = v77 |> unbox<std_string_String>
    let _run_target_args'_v90 = v106 
    #endif
#else
    let v110 : std_string_String = v77 |> unbox<std_string_String>
    let _run_target_args'_v90 = v110 
    #endif
    let v113 : std_string_String = _run_target_args'_v90 
    let v122 : string = "std::ffi::CString::new($0).unwrap()"
    let v123 : std_ffi_CString = Fable.Core.RustInterop.emitRustExpr v113 v122 
    let v124 : string = ""
    (* run_target_args'
    let v129 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v130 : string = "&*$0"
    let v131 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v124 v130 
    let _run_target_args'_v129 = v131 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v132 : string = "&*$0"
    let v133 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v124 v132 
    let _run_target_args'_v129 = v133 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v134 : string = "&*$0"
    let v135 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v124 v134 
    let _run_target_args'_v129 = v135 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v137 : Ref<Str> = v124 |> unbox<Ref<Str>>
    let _run_target_args'_v129 = v137 
    #endif
#if FABLE_COMPILER_PYTHON
    let v141 : Ref<Str> = v124 |> unbox<Ref<Str>>
    let _run_target_args'_v129 = v141 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v145 : Ref<Str> = v124 |> unbox<Ref<Str>>
    let _run_target_args'_v129 = v145 
    #endif
#else
    let v149 : Ref<Str> = v124 |> unbox<Ref<Str>>
    let _run_target_args'_v129 = v149 
    #endif
    let v152 : Ref<Str> = _run_target_args'_v129 
    (* run_target_args'
    let v165 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v166 : string = "String::from($0)"
    let v167 : std_string_String = Fable.Core.RustInterop.emitRustExpr v152 v166 
    let _run_target_args'_v165 = v167 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v168 : string = "String::from($0)"
    let v169 : std_string_String = Fable.Core.RustInterop.emitRustExpr v152 v168 
    let _run_target_args'_v165 = v169 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v170 : string = "String::from($0)"
    let v171 : std_string_String = Fable.Core.RustInterop.emitRustExpr v152 v170 
    let _run_target_args'_v165 = v171 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v173 : std_string_String = v152 |> unbox<std_string_String>
    let _run_target_args'_v165 = v173 
    #endif
#if FABLE_COMPILER_PYTHON
    let v177 : std_string_String = v152 |> unbox<std_string_String>
    let _run_target_args'_v165 = v177 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v181 : std_string_String = v152 |> unbox<std_string_String>
    let _run_target_args'_v165 = v181 
    #endif
#else
    let v185 : std_string_String = v152 |> unbox<std_string_String>
    let _run_target_args'_v165 = v185 
    #endif
    let v188 : std_string_String = _run_target_args'_v165 
    let v197 : string = "std::ffi::CString::new($0).unwrap()"
    let v198 : std_ffi_CString = Fable.Core.RustInterop.emitRustExpr v188 v197 
    let v199 : string = "pyo3::types::PyModule::from_code(v49, &$0, &v198, &v198)"
    let v200 : Result<pyo3_Bound<pyo3_types_PyModule>, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr v123 v199 
    let v201 : string = "true; let _result_map_error__ = $0.map_err(|x| { //"
    let v202 : bool = Fable.Core.RustInterop.emitRustExpr v200 v201 
    let v203 : string = "x"
    let v204 : pyo3_PyErr = Fable.Core.RustInterop.emitRustExpr () v203 
    (* run_target_args'
    let v209 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v210 : string = "format!(\"{}\", $0)"
    let v211 : std_string_String = Fable.Core.RustInterop.emitRustExpr v204 v210 
    let _run_target_args'_v209 = v211 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v212 : string = "format!(\"{}\", $0)"
    let v213 : std_string_String = Fable.Core.RustInterop.emitRustExpr v204 v212 
    let _run_target_args'_v209 = v213 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v214 : string = "format!(\"{}\", $0)"
    let v215 : std_string_String = Fable.Core.RustInterop.emitRustExpr v204 v214 
    let _run_target_args'_v209 = v215 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v217 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v209 = v217 
    #endif
#if FABLE_COMPILER_PYTHON
    let v221 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v209 = v221 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v225 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v209 = v225 
    #endif
#else
    let v229 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v209 = v229 
    #endif
    let v232 : std_string_String = _run_target_args'_v209 
    let v241 : string = "true; $0 })"
    let v242 : bool = Fable.Core.RustInterop.emitRustExpr v232 v241 
    let v243 : string = "_result_map_error__"
    let v244 : Result<pyo3_Bound<pyo3_types_PyModule>, std_string_String> = Fable.Core.RustInterop.emitRustExpr () v243 
    (* run_target_args'
    let v246 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v247 : string = "$0.unwrap()"
    let v248 : pyo3_Bound<pyo3_types_PyModule> = Fable.Core.RustInterop.emitRustExpr v244 v247 
    let _run_target_args'_v246 = v248 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v249 : string = "$0.unwrap()"
    let v250 : pyo3_Bound<pyo3_types_PyModule> = Fable.Core.RustInterop.emitRustExpr v244 v249 
    let _run_target_args'_v246 = v250 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v251 : string = "$0.unwrap()"
    let v252 : pyo3_Bound<pyo3_types_PyModule> = Fable.Core.RustInterop.emitRustExpr v244 v251 
    let _run_target_args'_v246 = v252 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v253 : pyo3_Bound<pyo3_types_PyModule> = match v244 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v246 = v253 
    #endif
#if FABLE_COMPILER_PYTHON
    let v254 : pyo3_Bound<pyo3_types_PyModule> = match v244 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v246 = v254 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v255 : pyo3_Bound<pyo3_types_PyModule> = match v244 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v246 = v255 
    #endif
#else
    let v256 : pyo3_Bound<pyo3_types_PyModule> = match v244 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v246 = v256 
    #endif
    let v257 : pyo3_Bound<pyo3_types_PyModule> = _run_target_args'_v246 
    let v260 : string = method8()
    (* run_target_args'
    let v265 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v266 : string = "&*$0"
    let v267 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v260 v266 
    let _run_target_args'_v265 = v267 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v268 : string = "&*$0"
    let v269 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v260 v268 
    let _run_target_args'_v265 = v269 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v270 : string = "&*$0"
    let v271 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v260 v270 
    let _run_target_args'_v265 = v271 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v273 : Ref<Str> = v260 |> unbox<Ref<Str>>
    let _run_target_args'_v265 = v273 
    #endif
#if FABLE_COMPILER_PYTHON
    let v277 : Ref<Str> = v260 |> unbox<Ref<Str>>
    let _run_target_args'_v265 = v277 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v281 : Ref<Str> = v260 |> unbox<Ref<Str>>
    let _run_target_args'_v265 = v281 
    #endif
#else
    let v285 : Ref<Str> = v260 |> unbox<Ref<Str>>
    let _run_target_args'_v265 = v285 
    #endif
    let v288 : Ref<Str> = _run_target_args'_v265 
    let v297 : pyo3_Bound<pyo3_types_PyModule> = method9(v257)
    let v298 : string = "v297.getattr($0)"
    let v299 : Result<pyo3_Bound<pyo3_PyAny>, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr v288 v298 
    let v300 : string = "true; let _result_map_error__ = $0.map_err(|x| { //"
    let v301 : bool = Fable.Core.RustInterop.emitRustExpr v299 v300 
    let v302 : string = "x"
    let v303 : pyo3_PyErr = Fable.Core.RustInterop.emitRustExpr () v302 
    (* run_target_args'
    let v308 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v309 : string = "format!(\"{}\", $0)"
    let v310 : std_string_String = Fable.Core.RustInterop.emitRustExpr v303 v309 
    let _run_target_args'_v308 = v310 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v311 : string = "format!(\"{}\", $0)"
    let v312 : std_string_String = Fable.Core.RustInterop.emitRustExpr v303 v311 
    let _run_target_args'_v308 = v312 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v313 : string = "format!(\"{}\", $0)"
    let v314 : std_string_String = Fable.Core.RustInterop.emitRustExpr v303 v313 
    let _run_target_args'_v308 = v314 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v316 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v308 = v316 
    #endif
#if FABLE_COMPILER_PYTHON
    let v320 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v308 = v320 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v324 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v308 = v324 
    #endif
#else
    let v328 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v308 = v328 
    #endif
    let v331 : std_string_String = _run_target_args'_v308 
    let v340 : string = "true; $0 })"
    let v341 : bool = Fable.Core.RustInterop.emitRustExpr v331 v340 
    let v342 : string = "_result_map_error__"
    let v343 : Result<pyo3_Bound<pyo3_PyAny>, std_string_String> = Fable.Core.RustInterop.emitRustExpr () v342 
    (* run_target_args'
    let v345 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v346 : string = "$0.unwrap()"
    let v347 : pyo3_Bound<pyo3_PyAny> = Fable.Core.RustInterop.emitRustExpr v343 v346 
    let _run_target_args'_v345 = v347 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v348 : string = "$0.unwrap()"
    let v349 : pyo3_Bound<pyo3_PyAny> = Fable.Core.RustInterop.emitRustExpr v343 v348 
    let _run_target_args'_v345 = v349 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v350 : string = "$0.unwrap()"
    let v351 : pyo3_Bound<pyo3_PyAny> = Fable.Core.RustInterop.emitRustExpr v343 v350 
    let _run_target_args'_v345 = v351 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v352 : pyo3_Bound<pyo3_PyAny> = match v343 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v345 = v352 
    #endif
#if FABLE_COMPILER_PYTHON
    let v353 : pyo3_Bound<pyo3_PyAny> = match v343 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v345 = v353 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v354 : pyo3_Bound<pyo3_PyAny> = match v343 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v345 = v354 
    #endif
#else
    let v355 : pyo3_Bound<pyo3_PyAny> = match v343 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v345 = v355 
    #endif
    let v356 : pyo3_Bound<pyo3_PyAny> = _run_target_args'_v345 
    let v359 : (bool * (float * float)) = method10(v46)
    let v360 : pyo3_Bound<pyo3_PyAny> = method11(v356)
    let v361 : string = "pyo3::prelude::PyAnyMethods::call(&v360, ((*v359).0, *(*v359).1), None)"
    let v362 : Result<pyo3_Bound<pyo3_PyAny>, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v361 
    let v363 : string = "true; let _result_map_error__ = $0.map_err(|x| { //"
    let v364 : bool = Fable.Core.RustInterop.emitRustExpr v362 v363 
    let v365 : string = "x"
    let v366 : pyo3_PyErr = Fable.Core.RustInterop.emitRustExpr () v365 
    (* run_target_args'
    let v371 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v372 : string = "format!(\"{}\", $0)"
    let v373 : std_string_String = Fable.Core.RustInterop.emitRustExpr v366 v372 
    let _run_target_args'_v371 = v373 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v374 : string = "format!(\"{}\", $0)"
    let v375 : std_string_String = Fable.Core.RustInterop.emitRustExpr v366 v374 
    let _run_target_args'_v371 = v375 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v376 : string = "format!(\"{}\", $0)"
    let v377 : std_string_String = Fable.Core.RustInterop.emitRustExpr v366 v376 
    let _run_target_args'_v371 = v377 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v379 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v371 = v379 
    #endif
#if FABLE_COMPILER_PYTHON
    let v383 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v371 = v383 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v387 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v371 = v387 
    #endif
#else
    let v391 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v371 = v391 
    #endif
    let v394 : std_string_String = _run_target_args'_v371 
    let v403 : string = "true; $0 })"
    let v404 : bool = Fable.Core.RustInterop.emitRustExpr v394 v403 
    let v405 : string = "_result_map_error__"
    let v406 : Result<pyo3_Bound<pyo3_PyAny>, std_string_String> = Fable.Core.RustInterop.emitRustExpr () v405 
    let v407 : string = "$0?"
    let v408 : pyo3_Bound<pyo3_PyAny> = Fable.Core.RustInterop.emitRustExpr v406 v407 
    let v409 : pyo3_Bound<pyo3_PyAny> = method12(v408)
    let v410 : string = "v409.extract()"
    let v411 : Result<struct (float * float), pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v410 
    let v412 : string = "true; let _result_map_error__ = $0.map_err(|x| { //"
    let v413 : bool = Fable.Core.RustInterop.emitRustExpr v411 v412 
    let v414 : string = "x"
    let v415 : pyo3_PyErr = Fable.Core.RustInterop.emitRustExpr () v414 
    (* run_target_args'
    let v420 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v421 : string = "format!(\"{}\", $0)"
    let v422 : std_string_String = Fable.Core.RustInterop.emitRustExpr v415 v421 
    let _run_target_args'_v420 = v422 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v423 : string = "format!(\"{}\", $0)"
    let v424 : std_string_String = Fable.Core.RustInterop.emitRustExpr v415 v423 
    let _run_target_args'_v420 = v424 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v425 : string = "format!(\"{}\", $0)"
    let v426 : std_string_String = Fable.Core.RustInterop.emitRustExpr v415 v425 
    let _run_target_args'_v420 = v426 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v428 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v420 = v428 
    #endif
#if FABLE_COMPILER_PYTHON
    let v432 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v420 = v432 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v436 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v420 = v436 
    #endif
#else
    let v440 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v420 = v440 
    #endif
    let v443 : std_string_String = _run_target_args'_v420 
    let v452 : string = "true; $0 })"
    let v453 : bool = Fable.Core.RustInterop.emitRustExpr v443 v452 
    let v454 : string = "_result_map_error__"
    let v455 : Result<struct (float * float), std_string_String> = Fable.Core.RustInterop.emitRustExpr () v454 
    let v456 : string = "$0?"
    let struct (v457 : float, v458 : float) = Fable.Core.RustInterop.emitRustExpr v455 v456 
    let v459 : string = "num_complex::Complex::new($0, $1)"
    let v460 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v457, v458) v459 
    let v462 : Result<num_complex_Complex<float>, std_string_String> = Ok v460 
    v462
and method14 (v0 : Mut0) : bool =
    let v1 : int32 = v0.l0
    let v2 : bool = v1 < 10000
    v2
and method15 (v0 : int32, v1 : Mut2) : bool =
    let v2 : int32 = v1.l0
    let v3 : bool = v2 < v0
    v3
and method16 (v0 : pyo3_Python, v1 : string, v2 : num_complex_Complex<float>) : Result<num_complex_Complex<float>, std_string_String> =
    let v3 : string = $"import sys"
    let v4 : string = $"import traceback"
    let v5 : string = $"import re"
    let v6 : string = $"count = 0"
    let v7 : string = $"memory_address_pattern = re.compile(r' at 0x[0-9a-fA-F]+')"
    let v8 : string = $"def trace_calls(frame, event, arg):"
    let v9 : string = $"    global count"
    let v10 : string = $"    count += 1"
    let v11 : string = $"    if count < 200:"
    let v12 : string = $"        try:"
    let v13 : string = $"            args = {{ k: v for k, v in frame.f_locals.items() if frame.f_code.co_name != 'make_mpc' and k not in ['ctx'] and not callable(v) }}"
    let v14 : string = $"            args_str = ', '.join([ f\"{{k}}={{re.sub(memory_address_pattern, ' at 0x<?>', repr(v))}}\" for k, v in args.items() ])"
    let v15 : string = "gamma_"
    let v16 : string = $"            print(f\"{{event}}({v15}) / f_code.co_name: {{frame.f_code.co_name}} / f_locals: {{args_str}} / f_lineno: {{frame.f_lineno}} / f_code.co_filename: {{frame.f_code.co_filename.split('site-packages')[-1]}} / f_back.f_lineno: {{ '' if frame.f_back is None else frame.f_back.f_lineno }} / f_back.f_code.co_filename: {{ '' if frame.f_back is None else frame.f_back.f_code.co_filename.split('site-packages')[-1] }} / arg: {{re.sub(memory_address_pattern, ' at 0x<?>', repr(arg))}}\", flush=True)"
    let v17 : string = $"        except ValueError as e:"
    let v18 : string = $"            print(f'{v15} / e: {{e}}', flush=True)"
    let v19 : string = $"        return trace_calls"
    let v20 : string = $"import mpmath"
    let v21 : string = $"def fn(log, s):"
    let v22 : string = $"    global count"
    let v23 : string = $"    if log:"
    let v24 : string = $"        print(f'{v15} / s: {{s}} / count: {{count}}', flush=True)"
    let v25 : string = $"    s = complex(*s)"
    let v26 : string = $"    try:"
    let v27 : string = $"        if log: sys.settrace(trace_calls)"
    let v28 : string = $"        if log:"
    let v29 : string = $"            sys.settrace(None)"
    let v30 : string = $"            print(f'{v15} / result: {{s}} / count: {{count}}', flush=True)"
    let v31 : string = $"    except ValueError as e:"
    let v32 : string = $"        if s.real == 1:"
    let v33 : string = $"            s = complex(float('inf'), 0)"
    let v34 : string = $"    return (s.real, s.imag)"
    let v35 : (string []) = [|v3; v4; v5; v6; v7; v8; v9; v10; v11; v12; v13; v14; v16; v17; v18; v19; v20; v21; v22; v23; v24; v25; v26; v27; v1; v28; v29; v30; v31; v32; v33; v34|]
    let v36 : string = method5(v35)
    let v37 : string = "$0.re"
    let v38 : float = Fable.Core.RustInterop.emitRustExpr v2 v37 
    let v39 : string = "$0.im"
    let v40 : float = Fable.Core.RustInterop.emitRustExpr v2 v39 
    let v42 : (float * float) = v38, v40 
    let v46 : (bool * (float * float)) = false, v42 
    let v49 : pyo3_Python = method7(v0)
    (* run_target_args'
    let v54 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v55 : string = "&*$0"
    let v56 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v36 v55 
    let _run_target_args'_v54 = v56 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v57 : string = "&*$0"
    let v58 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v36 v57 
    let _run_target_args'_v54 = v58 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v59 : string = "&*$0"
    let v60 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v36 v59 
    let _run_target_args'_v54 = v60 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v62 : Ref<Str> = v36 |> unbox<Ref<Str>>
    let _run_target_args'_v54 = v62 
    #endif
#if FABLE_COMPILER_PYTHON
    let v66 : Ref<Str> = v36 |> unbox<Ref<Str>>
    let _run_target_args'_v54 = v66 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v70 : Ref<Str> = v36 |> unbox<Ref<Str>>
    let _run_target_args'_v54 = v70 
    #endif
#else
    let v74 : Ref<Str> = v36 |> unbox<Ref<Str>>
    let _run_target_args'_v54 = v74 
    #endif
    let v77 : Ref<Str> = _run_target_args'_v54 
    (* run_target_args'
    let v90 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v91 : string = "String::from($0)"
    let v92 : std_string_String = Fable.Core.RustInterop.emitRustExpr v77 v91 
    let _run_target_args'_v90 = v92 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v93 : string = "String::from($0)"
    let v94 : std_string_String = Fable.Core.RustInterop.emitRustExpr v77 v93 
    let _run_target_args'_v90 = v94 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v95 : string = "String::from($0)"
    let v96 : std_string_String = Fable.Core.RustInterop.emitRustExpr v77 v95 
    let _run_target_args'_v90 = v96 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v98 : std_string_String = v77 |> unbox<std_string_String>
    let _run_target_args'_v90 = v98 
    #endif
#if FABLE_COMPILER_PYTHON
    let v102 : std_string_String = v77 |> unbox<std_string_String>
    let _run_target_args'_v90 = v102 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v106 : std_string_String = v77 |> unbox<std_string_String>
    let _run_target_args'_v90 = v106 
    #endif
#else
    let v110 : std_string_String = v77 |> unbox<std_string_String>
    let _run_target_args'_v90 = v110 
    #endif
    let v113 : std_string_String = _run_target_args'_v90 
    let v122 : string = "std::ffi::CString::new($0).unwrap()"
    let v123 : std_ffi_CString = Fable.Core.RustInterop.emitRustExpr v113 v122 
    let v124 : string = ""
    (* run_target_args'
    let v129 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v130 : string = "&*$0"
    let v131 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v124 v130 
    let _run_target_args'_v129 = v131 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v132 : string = "&*$0"
    let v133 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v124 v132 
    let _run_target_args'_v129 = v133 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v134 : string = "&*$0"
    let v135 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v124 v134 
    let _run_target_args'_v129 = v135 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v137 : Ref<Str> = v124 |> unbox<Ref<Str>>
    let _run_target_args'_v129 = v137 
    #endif
#if FABLE_COMPILER_PYTHON
    let v141 : Ref<Str> = v124 |> unbox<Ref<Str>>
    let _run_target_args'_v129 = v141 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v145 : Ref<Str> = v124 |> unbox<Ref<Str>>
    let _run_target_args'_v129 = v145 
    #endif
#else
    let v149 : Ref<Str> = v124 |> unbox<Ref<Str>>
    let _run_target_args'_v129 = v149 
    #endif
    let v152 : Ref<Str> = _run_target_args'_v129 
    (* run_target_args'
    let v165 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v166 : string = "String::from($0)"
    let v167 : std_string_String = Fable.Core.RustInterop.emitRustExpr v152 v166 
    let _run_target_args'_v165 = v167 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v168 : string = "String::from($0)"
    let v169 : std_string_String = Fable.Core.RustInterop.emitRustExpr v152 v168 
    let _run_target_args'_v165 = v169 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v170 : string = "String::from($0)"
    let v171 : std_string_String = Fable.Core.RustInterop.emitRustExpr v152 v170 
    let _run_target_args'_v165 = v171 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v173 : std_string_String = v152 |> unbox<std_string_String>
    let _run_target_args'_v165 = v173 
    #endif
#if FABLE_COMPILER_PYTHON
    let v177 : std_string_String = v152 |> unbox<std_string_String>
    let _run_target_args'_v165 = v177 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v181 : std_string_String = v152 |> unbox<std_string_String>
    let _run_target_args'_v165 = v181 
    #endif
#else
    let v185 : std_string_String = v152 |> unbox<std_string_String>
    let _run_target_args'_v165 = v185 
    #endif
    let v188 : std_string_String = _run_target_args'_v165 
    let v197 : string = "std::ffi::CString::new($0).unwrap()"
    let v198 : std_ffi_CString = Fable.Core.RustInterop.emitRustExpr v188 v197 
    let v199 : string = "pyo3::types::PyModule::from_code(v49, &$0, &v198, &v198)"
    let v200 : Result<pyo3_Bound<pyo3_types_PyModule>, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr v123 v199 
    let v201 : string = "true; let _result_map_error__ = $0.map_err(|x| { //"
    let v202 : bool = Fable.Core.RustInterop.emitRustExpr v200 v201 
    let v203 : string = "x"
    let v204 : pyo3_PyErr = Fable.Core.RustInterop.emitRustExpr () v203 
    (* run_target_args'
    let v209 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v210 : string = "format!(\"{}\", $0)"
    let v211 : std_string_String = Fable.Core.RustInterop.emitRustExpr v204 v210 
    let _run_target_args'_v209 = v211 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v212 : string = "format!(\"{}\", $0)"
    let v213 : std_string_String = Fable.Core.RustInterop.emitRustExpr v204 v212 
    let _run_target_args'_v209 = v213 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v214 : string = "format!(\"{}\", $0)"
    let v215 : std_string_String = Fable.Core.RustInterop.emitRustExpr v204 v214 
    let _run_target_args'_v209 = v215 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v217 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v209 = v217 
    #endif
#if FABLE_COMPILER_PYTHON
    let v221 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v209 = v221 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v225 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v209 = v225 
    #endif
#else
    let v229 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v209 = v229 
    #endif
    let v232 : std_string_String = _run_target_args'_v209 
    let v241 : string = "true; $0 })"
    let v242 : bool = Fable.Core.RustInterop.emitRustExpr v232 v241 
    let v243 : string = "_result_map_error__"
    let v244 : Result<pyo3_Bound<pyo3_types_PyModule>, std_string_String> = Fable.Core.RustInterop.emitRustExpr () v243 
    (* run_target_args'
    let v246 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v247 : string = "$0.unwrap()"
    let v248 : pyo3_Bound<pyo3_types_PyModule> = Fable.Core.RustInterop.emitRustExpr v244 v247 
    let _run_target_args'_v246 = v248 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v249 : string = "$0.unwrap()"
    let v250 : pyo3_Bound<pyo3_types_PyModule> = Fable.Core.RustInterop.emitRustExpr v244 v249 
    let _run_target_args'_v246 = v250 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v251 : string = "$0.unwrap()"
    let v252 : pyo3_Bound<pyo3_types_PyModule> = Fable.Core.RustInterop.emitRustExpr v244 v251 
    let _run_target_args'_v246 = v252 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v253 : pyo3_Bound<pyo3_types_PyModule> = match v244 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v246 = v253 
    #endif
#if FABLE_COMPILER_PYTHON
    let v254 : pyo3_Bound<pyo3_types_PyModule> = match v244 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v246 = v254 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v255 : pyo3_Bound<pyo3_types_PyModule> = match v244 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v246 = v255 
    #endif
#else
    let v256 : pyo3_Bound<pyo3_types_PyModule> = match v244 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v246 = v256 
    #endif
    let v257 : pyo3_Bound<pyo3_types_PyModule> = _run_target_args'_v246 
    let v260 : string = method8()
    (* run_target_args'
    let v265 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v266 : string = "&*$0"
    let v267 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v260 v266 
    let _run_target_args'_v265 = v267 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v268 : string = "&*$0"
    let v269 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v260 v268 
    let _run_target_args'_v265 = v269 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v270 : string = "&*$0"
    let v271 : Ref<Str> = Fable.Core.RustInterop.emitRustExpr v260 v270 
    let _run_target_args'_v265 = v271 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v273 : Ref<Str> = v260 |> unbox<Ref<Str>>
    let _run_target_args'_v265 = v273 
    #endif
#if FABLE_COMPILER_PYTHON
    let v277 : Ref<Str> = v260 |> unbox<Ref<Str>>
    let _run_target_args'_v265 = v277 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v281 : Ref<Str> = v260 |> unbox<Ref<Str>>
    let _run_target_args'_v265 = v281 
    #endif
#else
    let v285 : Ref<Str> = v260 |> unbox<Ref<Str>>
    let _run_target_args'_v265 = v285 
    #endif
    let v288 : Ref<Str> = _run_target_args'_v265 
    let v297 : pyo3_Bound<pyo3_types_PyModule> = method9(v257)
    let v298 : string = "v297.getattr($0)"
    let v299 : Result<pyo3_Bound<pyo3_PyAny>, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr v288 v298 
    let v300 : string = "true; let _result_map_error__ = $0.map_err(|x| { //"
    let v301 : bool = Fable.Core.RustInterop.emitRustExpr v299 v300 
    let v302 : string = "x"
    let v303 : pyo3_PyErr = Fable.Core.RustInterop.emitRustExpr () v302 
    (* run_target_args'
    let v308 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v309 : string = "format!(\"{}\", $0)"
    let v310 : std_string_String = Fable.Core.RustInterop.emitRustExpr v303 v309 
    let _run_target_args'_v308 = v310 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v311 : string = "format!(\"{}\", $0)"
    let v312 : std_string_String = Fable.Core.RustInterop.emitRustExpr v303 v311 
    let _run_target_args'_v308 = v312 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v313 : string = "format!(\"{}\", $0)"
    let v314 : std_string_String = Fable.Core.RustInterop.emitRustExpr v303 v313 
    let _run_target_args'_v308 = v314 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v316 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v308 = v316 
    #endif
#if FABLE_COMPILER_PYTHON
    let v320 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v308 = v320 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v324 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v308 = v324 
    #endif
#else
    let v328 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v308 = v328 
    #endif
    let v331 : std_string_String = _run_target_args'_v308 
    let v340 : string = "true; $0 })"
    let v341 : bool = Fable.Core.RustInterop.emitRustExpr v331 v340 
    let v342 : string = "_result_map_error__"
    let v343 : Result<pyo3_Bound<pyo3_PyAny>, std_string_String> = Fable.Core.RustInterop.emitRustExpr () v342 
    (* run_target_args'
    let v345 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v346 : string = "$0.unwrap()"
    let v347 : pyo3_Bound<pyo3_PyAny> = Fable.Core.RustInterop.emitRustExpr v343 v346 
    let _run_target_args'_v345 = v347 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v348 : string = "$0.unwrap()"
    let v349 : pyo3_Bound<pyo3_PyAny> = Fable.Core.RustInterop.emitRustExpr v343 v348 
    let _run_target_args'_v345 = v349 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v350 : string = "$0.unwrap()"
    let v351 : pyo3_Bound<pyo3_PyAny> = Fable.Core.RustInterop.emitRustExpr v343 v350 
    let _run_target_args'_v345 = v351 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v352 : pyo3_Bound<pyo3_PyAny> = match v343 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v345 = v352 
    #endif
#if FABLE_COMPILER_PYTHON
    let v353 : pyo3_Bound<pyo3_PyAny> = match v343 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v345 = v353 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v354 : pyo3_Bound<pyo3_PyAny> = match v343 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v345 = v354 
    #endif
#else
    let v355 : pyo3_Bound<pyo3_PyAny> = match v343 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    let _run_target_args'_v345 = v355 
    #endif
    let v356 : pyo3_Bound<pyo3_PyAny> = _run_target_args'_v345 
    let v359 : (bool * (float * float)) = method10(v46)
    let v360 : pyo3_Bound<pyo3_PyAny> = method11(v356)
    let v361 : string = "pyo3::prelude::PyAnyMethods::call(&v360, ((*v359).0, *(*v359).1), None)"
    let v362 : Result<pyo3_Bound<pyo3_PyAny>, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v361 
    let v363 : string = "true; let _result_map_error__ = $0.map_err(|x| { //"
    let v364 : bool = Fable.Core.RustInterop.emitRustExpr v362 v363 
    let v365 : string = "x"
    let v366 : pyo3_PyErr = Fable.Core.RustInterop.emitRustExpr () v365 
    (* run_target_args'
    let v371 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v372 : string = "format!(\"{}\", $0)"
    let v373 : std_string_String = Fable.Core.RustInterop.emitRustExpr v366 v372 
    let _run_target_args'_v371 = v373 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v374 : string = "format!(\"{}\", $0)"
    let v375 : std_string_String = Fable.Core.RustInterop.emitRustExpr v366 v374 
    let _run_target_args'_v371 = v375 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v376 : string = "format!(\"{}\", $0)"
    let v377 : std_string_String = Fable.Core.RustInterop.emitRustExpr v366 v376 
    let _run_target_args'_v371 = v377 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v379 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v371 = v379 
    #endif
#if FABLE_COMPILER_PYTHON
    let v383 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v371 = v383 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v387 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v371 = v387 
    #endif
#else
    let v391 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v371 = v391 
    #endif
    let v394 : std_string_String = _run_target_args'_v371 
    let v403 : string = "true; $0 })"
    let v404 : bool = Fable.Core.RustInterop.emitRustExpr v394 v403 
    let v405 : string = "_result_map_error__"
    let v406 : Result<pyo3_Bound<pyo3_PyAny>, std_string_String> = Fable.Core.RustInterop.emitRustExpr () v405 
    let v407 : string = "$0?"
    let v408 : pyo3_Bound<pyo3_PyAny> = Fable.Core.RustInterop.emitRustExpr v406 v407 
    let v409 : pyo3_Bound<pyo3_PyAny> = method12(v408)
    let v410 : string = "v409.extract()"
    let v411 : Result<struct (float * float), pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v410 
    let v412 : string = "true; let _result_map_error__ = $0.map_err(|x| { //"
    let v413 : bool = Fable.Core.RustInterop.emitRustExpr v411 v412 
    let v414 : string = "x"
    let v415 : pyo3_PyErr = Fable.Core.RustInterop.emitRustExpr () v414 
    (* run_target_args'
    let v420 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v421 : string = "format!(\"{}\", $0)"
    let v422 : std_string_String = Fable.Core.RustInterop.emitRustExpr v415 v421 
    let _run_target_args'_v420 = v422 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v423 : string = "format!(\"{}\", $0)"
    let v424 : std_string_String = Fable.Core.RustInterop.emitRustExpr v415 v423 
    let _run_target_args'_v420 = v424 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v425 : string = "format!(\"{}\", $0)"
    let v426 : std_string_String = Fable.Core.RustInterop.emitRustExpr v415 v425 
    let _run_target_args'_v420 = v426 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v428 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v420 = v428 
    #endif
#if FABLE_COMPILER_PYTHON
    let v432 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v420 = v432 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v436 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v420 = v436 
    #endif
#else
    let v440 : std_string_String = null |> unbox<std_string_String>
    let _run_target_args'_v420 = v440 
    #endif
    let v443 : std_string_String = _run_target_args'_v420 
    let v452 : string = "true; $0 })"
    let v453 : bool = Fable.Core.RustInterop.emitRustExpr v443 v452 
    let v454 : string = "_result_map_error__"
    let v455 : Result<struct (float * float), std_string_String> = Fable.Core.RustInterop.emitRustExpr () v454 
    let v456 : string = "$0?"
    let struct (v457 : float, v458 : float) = Fable.Core.RustInterop.emitRustExpr v455 v456 
    let v459 : string = "num_complex::Complex::new($0, $1)"
    let v460 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v457, v458) v459 
    let v462 : Result<num_complex_Complex<float>, std_string_String> = Ok v460 
    v462
and closure1 () (v0 : num_complex_Complex<float>) : US0 =
    US0_0(v0)
and method17 () : (num_complex_Complex<float> -> US0) =
    closure1()
and method13 (v0 : pyo3_Python, v1 : num_complex_Complex<float>) : num_complex_Complex<float> =
    let v2 : string = "println!(\"zeta / count: {:?} / s: {:?}\", $0, $1)"
    Fable.Core.RustInterop.emitRustExpr struct (0, v1) v2 
    let v3 : string = "$0.re"
    let v4 : float = Fable.Core.RustInterop.emitRustExpr v1 v3 
    let v5 : bool = v4 > 1.0
    if v5 then
        let v6 : string = "num_complex::Complex::new($0, $1)"
        let v7 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v6 
        let v8 : (int32 []) = Array.zeroCreate<int32> (10000)
        let v9 : Mut0 = {l0 = 0} : Mut0
        while method14(v9) do
            let v11 : int32 = v9.l0
            v8.[int v11] <- v11
            let v12 : int32 = v11 + 1
            v9.l0 <- v12
            ()
        let v13 : int32 = v8.Length
        let v14 : Mut2 = {l0 = 0; l1 = v7} : Mut2
        while method15(v13, v14) do
            let v16 : int32 = v14.l0
            let v17 : num_complex_Complex<float> = v14.l1
            let v18 : int32 = v8.[int v16]
            let v19 : string = "num_complex::Complex::new($0, $1)"
            let v20 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v19 
            let v22 : (int32 -> float) = float
            let v23 : float = v22 v18
            let v26 : string = "num_complex::Complex::new($0, $1)"
            let v27 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v23, 0.0) v26 
            let v28 : string = "num_complex::Complex::powc($0, $1)"
            let v29 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v27, v1) v28 
            let v30 : string = "$0 / $1"
            let v31 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v20, v29) v30 
            let v32 : string = "$0 + $1"
            let v33 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v17, v31) v32 
            let v34 : int32 = v16 + 1
            v14.l0 <- v34
            v14.l1 <- v33
            ()
        let v35 : num_complex_Complex<float> = v14.l1
        v35
    else
        let v36 : string = "num_complex::Complex::new($0, $1)"
        let v37 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v36 
        let v38 : string = "$0 - $1"
        let v39 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v37, v1) v38 
        let v40 : string = $"        s = mpmath.gamma(s)"
        let v41 : num_complex_Complex<float> = method3(v39)
        let v42 : Result<num_complex_Complex<float>, std_string_String> = method16(v0, v40, v41)
        (* run_target_args'
        let v44 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v45 : string = "$0.ok()"
        let v46 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v42 v45 
        let _run_target_args'_v44 = v46 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v47 : string = "$0.ok()"
        let v48 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v42 v47 
        let _run_target_args'_v44 = v48 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v49 : string = "$0.ok()"
        let v50 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v42 v49 
        let _run_target_args'_v44 = v50 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v51 : num_complex_Complex<float> option = match v42 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v44 = v51 
        #endif
#if FABLE_COMPILER_PYTHON
        let v52 : num_complex_Complex<float> option = match v42 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v44 = v52 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v53 : num_complex_Complex<float> option = match v42 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v44 = v53 
        #endif
#else
        let v54 : num_complex_Complex<float> option = match v42 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v44 = v54 
        #endif
        let v55 : num_complex_Complex<float> option = _run_target_args'_v44 
        let v60 : (num_complex_Complex<float> -> US0) = method17()
        let v61 : US0 option = v55 |> Option.map v60 
        let v81 : US0 = US0_1
        let v82 : US0 = v61 |> Option.defaultValue v81 
        let v86 : string = "f64::NAN"
        let v87 : float = Fable.Core.RustInterop.emitRustExpr () v86 
        let v88 : string = "f64::NAN"
        let v89 : float = Fable.Core.RustInterop.emitRustExpr () v88 
        let v90 : string = "num_complex::Complex::new($0, $1)"
        let v91 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v87, v89) v90 
        let v94 : num_complex_Complex<float> =
            match v82 with
            | US0_1 -> (* None *)
                v91
            | US0_0(v92) -> (* Some *)
                v92
        let v95 : string = "num_complex::Complex::new($0, $1)"
        let v96 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v95 
        let v97 : string = "$0 * $1"
        let v98 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v96, v1) v97 
        let v99 : string = "num_complex::Complex::new($0, $1)"
        let v100 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v99 
        let v101 : string = "$0 / $1"
        let v102 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v98, v100) v101 
        let v103 : string = "$0.sin()"
        let v104 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr v102 v103 
        let v105 : string = "$0.re"
        let v106 : float = Fable.Core.RustInterop.emitRustExpr v1 v105 
        let v107 : float = 1.0 - v106
        let v108 : string = "$0.im"
        let v109 : float = Fable.Core.RustInterop.emitRustExpr v1 v108 
        let v110 : float =  -v109
        let v111 : string = "num_complex::Complex::new($0, $1)"
        let v112 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v107, v110) v111 
        let v113 : string = "$0.re"
        let v114 : float = Fable.Core.RustInterop.emitRustExpr v112 v113 
        let v115 : bool = v114 <= 1.0
        let v646 : num_complex_Complex<float> =
            if v115 then
                let v116 : string = "num_complex::Complex::new($0, $1)"
                let v117 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v116 
                v117
            else
                let v118 : string = "println!(\"zeta / count: {:?} / s: {:?}\", $0, $1)"
                Fable.Core.RustInterop.emitRustExpr struct (1, v112) v118 
                let v119 : string = "$0.re"
                let v120 : float = Fable.Core.RustInterop.emitRustExpr v112 v119 
                let v121 : bool = v120 > 1.0
                if v121 then
                    let v122 : string = "num_complex::Complex::new($0, $1)"
                    let v123 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v122 
                    let v124 : (int32 []) = Array.zeroCreate<int32> (10000)
                    let v125 : Mut0 = {l0 = 0} : Mut0
                    while method14(v125) do
                        let v127 : int32 = v125.l0
                        v124.[int v127] <- v127
                        let v128 : int32 = v127 + 1
                        v125.l0 <- v128
                        ()
                    let v129 : int32 = v124.Length
                    let v130 : Mut2 = {l0 = 0; l1 = v123} : Mut2
                    while method15(v129, v130) do
                        let v132 : int32 = v130.l0
                        let v133 : num_complex_Complex<float> = v130.l1
                        let v134 : int32 = v124.[int v132]
                        let v135 : string = "num_complex::Complex::new($0, $1)"
                        let v136 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v135 
                        let v138 : (int32 -> float) = float
                        let v139 : float = v138 v134
                        let v142 : string = "num_complex::Complex::new($0, $1)"
                        let v143 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v139, 0.0) v142 
                        let v144 : string = "num_complex::Complex::powc($0, $1)"
                        let v145 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v143, v112) v144 
                        let v146 : string = "$0 / $1"
                        let v147 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v136, v145) v146 
                        let v148 : string = "$0 + $1"
                        let v149 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v133, v147) v148 
                        let v150 : int32 = v132 + 1
                        v130.l0 <- v150
                        v130.l1 <- v149
                        ()
                    let v151 : num_complex_Complex<float> = v130.l1
                    v151
                else
                    let v152 : string = "num_complex::Complex::new($0, $1)"
                    let v153 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v152 
                    let v154 : string = "$0 - $1"
                    let v155 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v153, v112) v154 
                    let v156 : string = $"        s = mpmath.gamma(s)"
                    let v157 : num_complex_Complex<float> = method3(v155)
                    let v158 : Result<num_complex_Complex<float>, std_string_String> = method16(v0, v156, v157)
                    (* run_target_args'
                    let v160 : unit = ()
                    run_target_args' *)
                    
#if FABLE_COMPILER || WASM || CONTRACT
                    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
                    let v161 : string = "$0.ok()"
                    let v162 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v158 v161 
                    let _run_target_args'_v160 = v162 
                    #endif
#if FABLE_COMPILER_RUST && WASM
                    let v163 : string = "$0.ok()"
                    let v164 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v158 v163 
                    let _run_target_args'_v160 = v164 
                    #endif
#if FABLE_COMPILER_RUST && CONTRACT
                    let v165 : string = "$0.ok()"
                    let v166 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v158 v165 
                    let _run_target_args'_v160 = v166 
                    #endif
#if FABLE_COMPILER_TYPESCRIPT
                    let v167 : num_complex_Complex<float> option = match v158 with Ok x -> Some x | Error _ -> None
                    let _run_target_args'_v160 = v167 
                    #endif
#if FABLE_COMPILER_PYTHON
                    let v168 : num_complex_Complex<float> option = match v158 with Ok x -> Some x | Error _ -> None
                    let _run_target_args'_v160 = v168 
                    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
                    let v169 : num_complex_Complex<float> option = match v158 with Ok x -> Some x | Error _ -> None
                    let _run_target_args'_v160 = v169 
                    #endif
#else
                    let v170 : num_complex_Complex<float> option = match v158 with Ok x -> Some x | Error _ -> None
                    let _run_target_args'_v160 = v170 
                    #endif
                    let v171 : num_complex_Complex<float> option = _run_target_args'_v160 
                    let v176 : (num_complex_Complex<float> -> US0) = method17()
                    let v177 : US0 option = v171 |> Option.map v176 
                    let v197 : US0 = US0_1
                    let v198 : US0 = v177 |> Option.defaultValue v197 
                    let v202 : string = "f64::NAN"
                    let v203 : float = Fable.Core.RustInterop.emitRustExpr () v202 
                    let v204 : string = "f64::NAN"
                    let v205 : float = Fable.Core.RustInterop.emitRustExpr () v204 
                    let v206 : string = "num_complex::Complex::new($0, $1)"
                    let v207 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v203, v205) v206 
                    let v210 : num_complex_Complex<float> =
                        match v198 with
                        | US0_1 -> (* None *)
                            v207
                        | US0_0(v208) -> (* Some *)
                            v208
                    let v211 : string = "num_complex::Complex::new($0, $1)"
                    let v212 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v211 
                    let v213 : string = "$0 * $1"
                    let v214 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v212, v112) v213 
                    let v215 : string = "num_complex::Complex::new($0, $1)"
                    let v216 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v215 
                    let v217 : string = "$0 / $1"
                    let v218 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v214, v216) v217 
                    let v219 : string = "$0.sin()"
                    let v220 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr v218 v219 
                    let v221 : string = "$0.re"
                    let v222 : float = Fable.Core.RustInterop.emitRustExpr v112 v221 
                    let v223 : float = 1.0 - v222
                    let v224 : string = "$0.im"
                    let v225 : float = Fable.Core.RustInterop.emitRustExpr v112 v224 
                    let v226 : float =  -v225
                    let v227 : string = "num_complex::Complex::new($0, $1)"
                    let v228 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v223, v226) v227 
                    let v229 : string = "$0.re"
                    let v230 : float = Fable.Core.RustInterop.emitRustExpr v228 v229 
                    let v231 : bool = v230 <= 1.0
                    let v630 : num_complex_Complex<float> =
                        if v231 then
                            let v232 : string = "num_complex::Complex::new($0, $1)"
                            let v233 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v232 
                            v233
                        else
                            let v234 : string = "println!(\"zeta / count: {:?} / s: {:?}\", $0, $1)"
                            Fable.Core.RustInterop.emitRustExpr struct (2, v228) v234 
                            let v235 : string = "$0.re"
                            let v236 : float = Fable.Core.RustInterop.emitRustExpr v228 v235 
                            let v237 : bool = v236 > 1.0
                            if v237 then
                                let v238 : string = "num_complex::Complex::new($0, $1)"
                                let v239 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v238 
                                let v240 : (int32 []) = Array.zeroCreate<int32> (10000)
                                let v241 : Mut0 = {l0 = 0} : Mut0
                                while method14(v241) do
                                    let v243 : int32 = v241.l0
                                    v240.[int v243] <- v243
                                    let v244 : int32 = v243 + 1
                                    v241.l0 <- v244
                                    ()
                                let v245 : int32 = v240.Length
                                let v246 : Mut2 = {l0 = 0; l1 = v239} : Mut2
                                while method15(v245, v246) do
                                    let v248 : int32 = v246.l0
                                    let v249 : num_complex_Complex<float> = v246.l1
                                    let v250 : int32 = v240.[int v248]
                                    let v251 : string = "num_complex::Complex::new($0, $1)"
                                    let v252 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v251 
                                    let v254 : (int32 -> float) = float
                                    let v255 : float = v254 v250
                                    let v258 : string = "num_complex::Complex::new($0, $1)"
                                    let v259 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v255, 0.0) v258 
                                    let v260 : string = "num_complex::Complex::powc($0, $1)"
                                    let v261 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v259, v228) v260 
                                    let v262 : string = "$0 / $1"
                                    let v263 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v252, v261) v262 
                                    let v264 : string = "$0 + $1"
                                    let v265 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v249, v263) v264 
                                    let v266 : int32 = v248 + 1
                                    v246.l0 <- v266
                                    v246.l1 <- v265
                                    ()
                                let v267 : num_complex_Complex<float> = v246.l1
                                v267
                            else
                                let v268 : string = "num_complex::Complex::new($0, $1)"
                                let v269 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v268 
                                let v270 : string = "$0 - $1"
                                let v271 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v269, v228) v270 
                                let v272 : string = $"        s = mpmath.gamma(s)"
                                let v273 : num_complex_Complex<float> = method3(v271)
                                let v274 : Result<num_complex_Complex<float>, std_string_String> = method16(v0, v272, v273)
                                (* run_target_args'
                                let v276 : unit = ()
                                run_target_args' *)
                                
#if FABLE_COMPILER || WASM || CONTRACT
                                
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
                                let v277 : string = "$0.ok()"
                                let v278 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v274 v277 
                                let _run_target_args'_v276 = v278 
                                #endif
#if FABLE_COMPILER_RUST && WASM
                                let v279 : string = "$0.ok()"
                                let v280 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v274 v279 
                                let _run_target_args'_v276 = v280 
                                #endif
#if FABLE_COMPILER_RUST && CONTRACT
                                let v281 : string = "$0.ok()"
                                let v282 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v274 v281 
                                let _run_target_args'_v276 = v282 
                                #endif
#if FABLE_COMPILER_TYPESCRIPT
                                let v283 : num_complex_Complex<float> option = match v274 with Ok x -> Some x | Error _ -> None
                                let _run_target_args'_v276 = v283 
                                #endif
#if FABLE_COMPILER_PYTHON
                                let v284 : num_complex_Complex<float> option = match v274 with Ok x -> Some x | Error _ -> None
                                let _run_target_args'_v276 = v284 
                                #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
                                let v285 : num_complex_Complex<float> option = match v274 with Ok x -> Some x | Error _ -> None
                                let _run_target_args'_v276 = v285 
                                #endif
#else
                                let v286 : num_complex_Complex<float> option = match v274 with Ok x -> Some x | Error _ -> None
                                let _run_target_args'_v276 = v286 
                                #endif
                                let v287 : num_complex_Complex<float> option = _run_target_args'_v276 
                                let v292 : (num_complex_Complex<float> -> US0) = method17()
                                let v293 : US0 option = v287 |> Option.map v292 
                                let v313 : US0 = US0_1
                                let v314 : US0 = v293 |> Option.defaultValue v313 
                                let v318 : string = "f64::NAN"
                                let v319 : float = Fable.Core.RustInterop.emitRustExpr () v318 
                                let v320 : string = "f64::NAN"
                                let v321 : float = Fable.Core.RustInterop.emitRustExpr () v320 
                                let v322 : string = "num_complex::Complex::new($0, $1)"
                                let v323 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v319, v321) v322 
                                let v326 : num_complex_Complex<float> =
                                    match v314 with
                                    | US0_1 -> (* None *)
                                        v323
                                    | US0_0(v324) -> (* Some *)
                                        v324
                                let v327 : string = "num_complex::Complex::new($0, $1)"
                                let v328 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v327 
                                let v329 : string = "$0 * $1"
                                let v330 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v328, v228) v329 
                                let v331 : string = "num_complex::Complex::new($0, $1)"
                                let v332 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v331 
                                let v333 : string = "$0 / $1"
                                let v334 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v330, v332) v333 
                                let v335 : string = "$0.sin()"
                                let v336 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr v334 v335 
                                let v337 : string = "$0.re"
                                let v338 : float = Fable.Core.RustInterop.emitRustExpr v228 v337 
                                let v339 : float = 1.0 - v338
                                let v340 : string = "$0.im"
                                let v341 : float = Fable.Core.RustInterop.emitRustExpr v228 v340 
                                let v342 : float =  -v341
                                let v343 : string = "num_complex::Complex::new($0, $1)"
                                let v344 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v339, v342) v343 
                                let v345 : string = "$0.re"
                                let v346 : float = Fable.Core.RustInterop.emitRustExpr v344 v345 
                                let v347 : bool = v346 <= 1.0
                                let v614 : num_complex_Complex<float> =
                                    if v347 then
                                        let v348 : string = "num_complex::Complex::new($0, $1)"
                                        let v349 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v348 
                                        v349
                                    else
                                        let v350 : string = "println!(\"zeta / count: {:?} / s: {:?}\", $0, $1)"
                                        Fable.Core.RustInterop.emitRustExpr struct (3, v344) v350 
                                        let v351 : string = "$0.re"
                                        let v352 : float = Fable.Core.RustInterop.emitRustExpr v344 v351 
                                        let v353 : bool = v352 > 1.0
                                        if v353 then
                                            let v354 : string = "num_complex::Complex::new($0, $1)"
                                            let v355 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v354 
                                            let v356 : (int32 []) = Array.zeroCreate<int32> (10000)
                                            let v357 : Mut0 = {l0 = 0} : Mut0
                                            while method14(v357) do
                                                let v359 : int32 = v357.l0
                                                v356.[int v359] <- v359
                                                let v360 : int32 = v359 + 1
                                                v357.l0 <- v360
                                                ()
                                            let v361 : int32 = v356.Length
                                            let v362 : Mut2 = {l0 = 0; l1 = v355} : Mut2
                                            while method15(v361, v362) do
                                                let v364 : int32 = v362.l0
                                                let v365 : num_complex_Complex<float> = v362.l1
                                                let v366 : int32 = v356.[int v364]
                                                let v367 : string = "num_complex::Complex::new($0, $1)"
                                                let v368 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v367 
                                                let v370 : (int32 -> float) = float
                                                let v371 : float = v370 v366
                                                let v374 : string = "num_complex::Complex::new($0, $1)"
                                                let v375 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v371, 0.0) v374 
                                                let v376 : string = "num_complex::Complex::powc($0, $1)"
                                                let v377 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v375, v344) v376 
                                                let v378 : string = "$0 / $1"
                                                let v379 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v368, v377) v378 
                                                let v380 : string = "$0 + $1"
                                                let v381 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v365, v379) v380 
                                                let v382 : int32 = v364 + 1
                                                v362.l0 <- v382
                                                v362.l1 <- v381
                                                ()
                                            let v383 : num_complex_Complex<float> = v362.l1
                                            v383
                                        else
                                            let v384 : string = "num_complex::Complex::new($0, $1)"
                                            let v385 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v384 
                                            let v386 : string = "$0 - $1"
                                            let v387 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v385, v344) v386 
                                            let v388 : string = $"        s = mpmath.gamma(s)"
                                            let v389 : num_complex_Complex<float> = method3(v387)
                                            let v390 : Result<num_complex_Complex<float>, std_string_String> = method16(v0, v388, v389)
                                            (* run_target_args'
                                            let v392 : unit = ()
                                            run_target_args' *)
                                            
#if FABLE_COMPILER || WASM || CONTRACT
                                            
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
                                            let v393 : string = "$0.ok()"
                                            let v394 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v390 v393 
                                            let _run_target_args'_v392 = v394 
                                            #endif
#if FABLE_COMPILER_RUST && WASM
                                            let v395 : string = "$0.ok()"
                                            let v396 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v390 v395 
                                            let _run_target_args'_v392 = v396 
                                            #endif
#if FABLE_COMPILER_RUST && CONTRACT
                                            let v397 : string = "$0.ok()"
                                            let v398 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v390 v397 
                                            let _run_target_args'_v392 = v398 
                                            #endif
#if FABLE_COMPILER_TYPESCRIPT
                                            let v399 : num_complex_Complex<float> option = match v390 with Ok x -> Some x | Error _ -> None
                                            let _run_target_args'_v392 = v399 
                                            #endif
#if FABLE_COMPILER_PYTHON
                                            let v400 : num_complex_Complex<float> option = match v390 with Ok x -> Some x | Error _ -> None
                                            let _run_target_args'_v392 = v400 
                                            #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
                                            let v401 : num_complex_Complex<float> option = match v390 with Ok x -> Some x | Error _ -> None
                                            let _run_target_args'_v392 = v401 
                                            #endif
#else
                                            let v402 : num_complex_Complex<float> option = match v390 with Ok x -> Some x | Error _ -> None
                                            let _run_target_args'_v392 = v402 
                                            #endif
                                            let v403 : num_complex_Complex<float> option = _run_target_args'_v392 
                                            let v408 : (num_complex_Complex<float> -> US0) = method17()
                                            let v409 : US0 option = v403 |> Option.map v408 
                                            let v429 : US0 = US0_1
                                            let v430 : US0 = v409 |> Option.defaultValue v429 
                                            let v434 : string = "f64::NAN"
                                            let v435 : float = Fable.Core.RustInterop.emitRustExpr () v434 
                                            let v436 : string = "f64::NAN"
                                            let v437 : float = Fable.Core.RustInterop.emitRustExpr () v436 
                                            let v438 : string = "num_complex::Complex::new($0, $1)"
                                            let v439 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v435, v437) v438 
                                            let v442 : num_complex_Complex<float> =
                                                match v430 with
                                                | US0_1 -> (* None *)
                                                    v439
                                                | US0_0(v440) -> (* Some *)
                                                    v440
                                            let v443 : string = "num_complex::Complex::new($0, $1)"
                                            let v444 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v443 
                                            let v445 : string = "$0 * $1"
                                            let v446 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v444, v344) v445 
                                            let v447 : string = "num_complex::Complex::new($0, $1)"
                                            let v448 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v447 
                                            let v449 : string = "$0 / $1"
                                            let v450 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v446, v448) v449 
                                            let v451 : string = "$0.sin()"
                                            let v452 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr v450 v451 
                                            let v453 : string = "$0.re"
                                            let v454 : float = Fable.Core.RustInterop.emitRustExpr v344 v453 
                                            let v455 : float = 1.0 - v454
                                            let v456 : string = "$0.im"
                                            let v457 : float = Fable.Core.RustInterop.emitRustExpr v344 v456 
                                            let v458 : float =  -v457
                                            let v459 : string = "num_complex::Complex::new($0, $1)"
                                            let v460 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v455, v458) v459 
                                            let v461 : string = "$0.re"
                                            let v462 : float = Fable.Core.RustInterop.emitRustExpr v460 v461 
                                            let v463 : bool = v462 <= 1.0
                                            let v598 : num_complex_Complex<float> =
                                                if v463 then
                                                    let v464 : string = "num_complex::Complex::new($0, $1)"
                                                    let v465 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v464 
                                                    v465
                                                else
                                                    let v466 : string = "println!(\"zeta / count: {:?} / s: {:?}\", $0, $1)"
                                                    Fable.Core.RustInterop.emitRustExpr struct (4, v460) v466 
                                                    let v467 : string = "$0.re"
                                                    let v468 : float = Fable.Core.RustInterop.emitRustExpr v460 v467 
                                                    let v469 : bool = v468 > 1.0
                                                    if v469 then
                                                        let v470 : string = "num_complex::Complex::new($0, $1)"
                                                        let v471 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v470 
                                                        let v472 : (int32 []) = Array.zeroCreate<int32> (10000)
                                                        let v473 : Mut0 = {l0 = 0} : Mut0
                                                        while method14(v473) do
                                                            let v475 : int32 = v473.l0
                                                            v472.[int v475] <- v475
                                                            let v476 : int32 = v475 + 1
                                                            v473.l0 <- v476
                                                            ()
                                                        let v477 : int32 = v472.Length
                                                        let v478 : Mut2 = {l0 = 0; l1 = v471} : Mut2
                                                        while method15(v477, v478) do
                                                            let v480 : int32 = v478.l0
                                                            let v481 : num_complex_Complex<float> = v478.l1
                                                            let v482 : int32 = v472.[int v480]
                                                            let v483 : string = "num_complex::Complex::new($0, $1)"
                                                            let v484 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v483 
                                                            let v486 : (int32 -> float) = float
                                                            let v487 : float = v486 v482
                                                            let v490 : string = "num_complex::Complex::new($0, $1)"
                                                            let v491 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v487, 0.0) v490 
                                                            let v492 : string = "num_complex::Complex::powc($0, $1)"
                                                            let v493 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v491, v460) v492 
                                                            let v494 : string = "$0 / $1"
                                                            let v495 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v484, v493) v494 
                                                            let v496 : string = "$0 + $1"
                                                            let v497 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v481, v495) v496 
                                                            let v498 : int32 = v480 + 1
                                                            v478.l0 <- v498
                                                            v478.l1 <- v497
                                                            ()
                                                        let v499 : num_complex_Complex<float> = v478.l1
                                                        v499
                                                    else
                                                        let v500 : string = "num_complex::Complex::new($0, $1)"
                                                        let v501 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v500 
                                                        let v502 : string = "$0 - $1"
                                                        let v503 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v501, v460) v502 
                                                        let v504 : string = $"        s = mpmath.gamma(s)"
                                                        let v505 : num_complex_Complex<float> = method3(v503)
                                                        let v506 : Result<num_complex_Complex<float>, std_string_String> = method16(v0, v504, v505)
                                                        (* run_target_args'
                                                        let v508 : unit = ()
                                                        run_target_args' *)
                                                        
#if FABLE_COMPILER || WASM || CONTRACT
                                                        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
                                                        let v509 : string = "$0.ok()"
                                                        let v510 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v506 v509 
                                                        let _run_target_args'_v508 = v510 
                                                        #endif
#if FABLE_COMPILER_RUST && WASM
                                                        let v511 : string = "$0.ok()"
                                                        let v512 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v506 v511 
                                                        let _run_target_args'_v508 = v512 
                                                        #endif
#if FABLE_COMPILER_RUST && CONTRACT
                                                        let v513 : string = "$0.ok()"
                                                        let v514 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v506 v513 
                                                        let _run_target_args'_v508 = v514 
                                                        #endif
#if FABLE_COMPILER_TYPESCRIPT
                                                        let v515 : num_complex_Complex<float> option = match v506 with Ok x -> Some x | Error _ -> None
                                                        let _run_target_args'_v508 = v515 
                                                        #endif
#if FABLE_COMPILER_PYTHON
                                                        let v516 : num_complex_Complex<float> option = match v506 with Ok x -> Some x | Error _ -> None
                                                        let _run_target_args'_v508 = v516 
                                                        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
                                                        let v517 : num_complex_Complex<float> option = match v506 with Ok x -> Some x | Error _ -> None
                                                        let _run_target_args'_v508 = v517 
                                                        #endif
#else
                                                        let v518 : num_complex_Complex<float> option = match v506 with Ok x -> Some x | Error _ -> None
                                                        let _run_target_args'_v508 = v518 
                                                        #endif
                                                        let v519 : num_complex_Complex<float> option = _run_target_args'_v508 
                                                        let v524 : (num_complex_Complex<float> -> US0) = method17()
                                                        let v525 : US0 option = v519 |> Option.map v524 
                                                        let v545 : US0 = US0_1
                                                        let v546 : US0 = v525 |> Option.defaultValue v545 
                                                        let v550 : string = "f64::NAN"
                                                        let v551 : float = Fable.Core.RustInterop.emitRustExpr () v550 
                                                        let v552 : string = "f64::NAN"
                                                        let v553 : float = Fable.Core.RustInterop.emitRustExpr () v552 
                                                        let v554 : string = "num_complex::Complex::new($0, $1)"
                                                        let v555 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v551, v553) v554 
                                                        let v558 : num_complex_Complex<float> =
                                                            match v546 with
                                                            | US0_1 -> (* None *)
                                                                v555
                                                            | US0_0(v556) -> (* Some *)
                                                                v556
                                                        let v559 : string = "num_complex::Complex::new($0, $1)"
                                                        let v560 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v559 
                                                        let v561 : string = "$0 * $1"
                                                        let v562 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v560, v460) v561 
                                                        let v563 : string = "num_complex::Complex::new($0, $1)"
                                                        let v564 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v563 
                                                        let v565 : string = "$0 / $1"
                                                        let v566 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v562, v564) v565 
                                                        let v567 : string = "$0.sin()"
                                                        let v568 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr v566 v567 
                                                        let v569 : string = "$0.re"
                                                        let v570 : float = Fable.Core.RustInterop.emitRustExpr v460 v569 
                                                        let v571 : float = 1.0 - v570
                                                        let v572 : string = "$0.im"
                                                        let v573 : float = Fable.Core.RustInterop.emitRustExpr v460 v572 
                                                        let v574 : float =  -v573
                                                        let v575 : string = "num_complex::Complex::new($0, $1)"
                                                        let v576 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v571, v574) v575 
                                                        let v577 : string = "$0.re"
                                                        let v578 : float = Fable.Core.RustInterop.emitRustExpr v576 v577 
                                                        let v579 : bool = v578 <= 1.0
                                                        let v582 : num_complex_Complex<float> =
                                                            if v579 then
                                                                let v580 : string = "num_complex::Complex::new($0, $1)"
                                                                let v581 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, 0.0) v580 
                                                                v581
                                                            else
                                                                v576
                                                        let v583 : string = "num_complex::Complex::new($0, $1)"
                                                        let v584 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v583 
                                                        let v585 : string = "num_complex::Complex::new($0, $1)"
                                                        let v586 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v585 
                                                        let v587 : string = "num_complex::Complex::powc($0, $1)"
                                                        let v588 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v586, v460) v587 
                                                        let v589 : string = "$0 * $1"
                                                        let v590 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v584, v588) v589 
                                                        let v591 : string = "$0 * $1"
                                                        let v592 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v590, v568) v591 
                                                        let v593 : string = "$0 * $1"
                                                        let v594 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v592, v558) v593 
                                                        let v595 : string = "$0 * $1"
                                                        let v596 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v594, v582) v595 
                                                        v596
                                            let v599 : string = "num_complex::Complex::new($0, $1)"
                                            let v600 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v599 
                                            let v601 : string = "num_complex::Complex::new($0, $1)"
                                            let v602 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v601 
                                            let v603 : string = "num_complex::Complex::powc($0, $1)"
                                            let v604 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v602, v344) v603 
                                            let v605 : string = "$0 * $1"
                                            let v606 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v600, v604) v605 
                                            let v607 : string = "$0 * $1"
                                            let v608 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v606, v452) v607 
                                            let v609 : string = "$0 * $1"
                                            let v610 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v608, v442) v609 
                                            let v611 : string = "$0 * $1"
                                            let v612 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v610, v598) v611 
                                            v612
                                let v615 : string = "num_complex::Complex::new($0, $1)"
                                let v616 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v615 
                                let v617 : string = "num_complex::Complex::new($0, $1)"
                                let v618 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v617 
                                let v619 : string = "num_complex::Complex::powc($0, $1)"
                                let v620 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v618, v228) v619 
                                let v621 : string = "$0 * $1"
                                let v622 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v616, v620) v621 
                                let v623 : string = "$0 * $1"
                                let v624 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v622, v336) v623 
                                let v625 : string = "$0 * $1"
                                let v626 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v624, v326) v625 
                                let v627 : string = "$0 * $1"
                                let v628 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v626, v614) v627 
                                v628
                    let v631 : string = "num_complex::Complex::new($0, $1)"
                    let v632 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v631 
                    let v633 : string = "num_complex::Complex::new($0, $1)"
                    let v634 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v633 
                    let v635 : string = "num_complex::Complex::powc($0, $1)"
                    let v636 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v634, v112) v635 
                    let v637 : string = "$0 * $1"
                    let v638 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v632, v636) v637 
                    let v639 : string = "$0 * $1"
                    let v640 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v638, v220) v639 
                    let v641 : string = "$0 * $1"
                    let v642 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v640, v210) v641 
                    let v643 : string = "$0 * $1"
                    let v644 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v642, v630) v643 
                    v644
        let v647 : string = "num_complex::Complex::new($0, $1)"
        let v648 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v647 
        let v649 : string = "num_complex::Complex::new($0, $1)"
        let v650 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v649 
        let v651 : string = "num_complex::Complex::powc($0, $1)"
        let v652 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v650, v1) v651 
        let v653 : string = "$0 * $1"
        let v654 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v648, v652) v653 
        let v655 : string = "$0 * $1"
        let v656 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v654, v104) v655 
        let v657 : string = "$0 * $1"
        let v658 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v656, v94) v657 
        let v659 : string = "$0 * $1"
        let v660 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v658, v646) v659 
        v660
and method18 (v0 : bool) : bool =
    v0
and method20 () : string =
    let v0 : string = ""
    v0
and closure2 (v0 : Mut3, v1 : string) () : unit =
    let v2 : string = v0.l0
    let v4 : string = v2 + v1 
    v0.l0 <- v4
    ()
and method19 (v0 : string, v1 : float) : string =
    let v2 : string = method20()
    let v3 : Mut3 = {l0 = v2} : Mut3
    let v6 : string = "{ "
    let v7 : string = $"{v6}"
    let v15 : unit = ()
    let v16 : (unit -> unit) = closure2(v3, v7)
    let v17 : unit = (fun () -> v16 (); v15) ()
    let v25 : string = "name"
    let v26 : string = $"{v25}"
    let v34 : unit = ()
    let v35 : (unit -> unit) = closure2(v3, v26)
    let v36 : unit = (fun () -> v35 (); v34) ()
    let v44 : string = " = "
    let v45 : string = $"{v44}"
    let v53 : unit = ()
    let v54 : (unit -> unit) = closure2(v3, v45)
    let v55 : unit = (fun () -> v54 (); v53) ()
    let v62 : string = $"{v0}"
    let v70 : unit = ()
    let v71 : (unit -> unit) = closure2(v3, v62)
    let v72 : unit = (fun () -> v71 (); v70) ()
    let v80 : string = "; "
    let v81 : string = $"{v80}"
    let v89 : unit = ()
    let v90 : (unit -> unit) = closure2(v3, v81)
    let v91 : unit = (fun () -> v90 (); v89) ()
    let v99 : string = "expected"
    let v100 : string = $"{v99}"
    let v108 : unit = ()
    let v109 : (unit -> unit) = closure2(v3, v100)
    let v110 : unit = (fun () -> v109 (); v108) ()
    let v117 : string = $"{v44}"
    let v125 : unit = ()
    let v126 : (unit -> unit) = closure2(v3, v117)
    let v127 : unit = (fun () -> v126 (); v125) ()
    let v137 : string = $"%+.6f{v1}"
    let v145 : unit = ()
    let v146 : (unit -> unit) = closure2(v3, v137)
    let v147 : unit = (fun () -> v146 (); v145) ()
    let v155 : string = " }"
    let v156 : string = $"{v155}"
    let v164 : unit = ()
    let v165 : (unit -> unit) = closure2(v3, v156)
    let v166 : unit = (fun () -> v165 (); v164) ()
    let v172 : string = v3.l0
    v172
and closure3 (v0 : string) () : unit =
    let v1 : (string -> unit) = System.Console.WriteLine
    v1 v0
and method1 (v0 : pyo3_Python) : unit =
    let v1 : string = "num_complex::Complex::new($0, $1)"
    let v2 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v1 
    let v3 : string = "num_complex::Complex::new($0, $1)"
    let v4 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (-1.0, 0.0) v3 
    let v5 : (struct (num_complex_Complex<float> * float) []) = [|struct (v2, 1.6449340668482264); struct (v4, -0.08333333333333333)|]
    let v6 : int32 = v5.Length
    let v7 : Mut0 = {l0 = 0} : Mut0
    while method2(v6, v7) do
        let v9 : int32 = v7.l0
        let struct (v10 : num_complex_Complex<float>, v11 : float) = v5.[int v9]
        let v12 : string = $"        s = mpmath.zeta(s)"
        let v13 : num_complex_Complex<float> = method3(v10)
        let v14 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v12, v13)
        let v15 : num_complex_Complex<float> = method13(v0, v10)
        (* run_target_args'
        let v17 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v18 : string = "$0.ok()"
        let v19 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v14 v18 
        let _run_target_args'_v17 = v19 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v20 : string = "$0.ok()"
        let v21 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v14 v20 
        let _run_target_args'_v17 = v21 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v22 : string = "$0.ok()"
        let v23 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v14 v22 
        let _run_target_args'_v17 = v23 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v24 : num_complex_Complex<float> option = match v14 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v17 = v24 
        #endif
#if FABLE_COMPILER_PYTHON
        let v25 : num_complex_Complex<float> option = match v14 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v17 = v25 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v26 : num_complex_Complex<float> option = match v14 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v17 = v26 
        #endif
#else
        let v27 : num_complex_Complex<float> option = match v14 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v17 = v27 
        #endif
        let v28 : num_complex_Complex<float> option = _run_target_args'_v17 
        let v33 : (num_complex_Complex<float> -> US0) = method17()
        let v34 : US0 option = v28 |> Option.map v33 
        let v54 : US0 = US0_1
        let v55 : US0 = v34 |> Option.defaultValue v54 
        let v59 : string = "f64::NAN"
        let v60 : float = Fable.Core.RustInterop.emitRustExpr () v59 
        let v61 : string = "f64::NAN"
        let v62 : float = Fable.Core.RustInterop.emitRustExpr () v61 
        let v63 : string = "num_complex::Complex::new($0, $1)"
        let v64 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v60, v62) v63 
        let v67 : num_complex_Complex<float> =
            match v55 with
            | US0_1 -> (* None *)
                v64
            | US0_0(v65) -> (* Some *)
                v65
        let v68 : string = "$0.im"
        let v69 : float = Fable.Core.RustInterop.emitRustExpr v67 v68 
        let v70 : bool = v69 = 0.0
        let v72 : bool =
            if v70 then
                true
            else
                method18(v70)
        let v73 : string = "__assert_eq"
        let v74 : float = 0.0
        let v75 : string = method19(v73, v74)
        let v77 : unit = ()
        let v78 : (unit -> unit) = closure3(v75)
        let v79 : unit = (fun () -> v78 (); v77) ()
        let v81 : bool = v72 = false
        if v81 then
            failwith<unit> v75
        let v82 : string = "$0.re"
        let v83 : float = Fable.Core.RustInterop.emitRustExpr v67 v82 
        let v84 : float = v83 - v11
        let v85 : float =  -v84
        let v86 : bool = v84 >= v85
        let v87 : float =
            if v86 then
                v84
            else
                v85
        let v88 : bool = v87 < 0.0001
        let v90 : bool =
            if v88 then
                true
            else
                method18(v88)
        let v91 : string = "__assert_lt"
        let v92 : float = 0.0001
        let v93 : string = method19(v91, v92)
        let v95 : unit = ()
        let v96 : (unit -> unit) = closure3(v93)
        let v97 : unit = (fun () -> v96 (); v95) ()
        let v99 : bool = v90 = false
        if v99 then
            failwith<unit> v93
        let v100 : int32 = v9 + 1
        v7.l0 <- v100
        ()
    ()
and method21 (v0 : Result<unit, pyo3_PyErr>) : Result<unit, pyo3_PyErr> =
    v0
and method0 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method1(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method23 (v0 : pyo3_Python) : unit =
    let v1 : string = "num_complex::Complex::new($0, $1)"
    let v2 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, -2.0) v1 
    let v3 : string = $"        s = mpmath.zeta(s)"
    let v4 : num_complex_Complex<float> = method3(v2)
    let v5 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v3, v4)
    let v6 : num_complex_Complex<float> = method13(v0, v2)
    (* run_target_args'
    let v8 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v9 : string = "$0.ok()"
    let v10 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v9 
    let _run_target_args'_v8 = v10 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v11 : string = "$0.ok()"
    let v12 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v11 
    let _run_target_args'_v8 = v12 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v13 : string = "$0.ok()"
    let v14 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v13 
    let _run_target_args'_v8 = v14 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v15 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v15 
    #endif
#if FABLE_COMPILER_PYTHON
    let v16 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v16 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v17 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v17 
    #endif
#else
    let v18 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v18 
    #endif
    let v19 : num_complex_Complex<float> option = _run_target_args'_v8 
    let v24 : (num_complex_Complex<float> -> US0) = method17()
    let v25 : US0 option = v19 |> Option.map v24 
    let v45 : US0 = US0_1
    let v46 : US0 = v25 |> Option.defaultValue v45 
    let v50 : string = "f64::NAN"
    let v51 : float = Fable.Core.RustInterop.emitRustExpr () v50 
    let v52 : string = "f64::NAN"
    let v53 : float = Fable.Core.RustInterop.emitRustExpr () v52 
    let v54 : string = "num_complex::Complex::new($0, $1)"
    let v55 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v51, v53) v54 
    let v58 : num_complex_Complex<float> =
        match v46 with
        | US0_1 -> (* None *)
            v55
        | US0_0(v56) -> (* Some *)
            v56
    let v59 : string = "$0.re"
    let v60 : float = Fable.Core.RustInterop.emitRustExpr v58 v59 
    let v61 : float = v60 - 0.8673
    let v62 : float =  -v61
    let v63 : bool = v61 >= v62
    let v64 : float =
        if v63 then
            v61
        else
            v62
    let v65 : bool = v64 < 0.001
    let v67 : bool =
        if v65 then
            true
        else
            method18(v65)
    let v68 : string = "__assert_lt"
    let v69 : float = 0.001
    let v70 : string = method19(v68, v69)
    let v72 : unit = ()
    let v73 : (unit -> unit) = closure3(v70)
    let v74 : unit = (fun () -> v73 (); v72) ()
    let v76 : bool = v67 = false
    if v76 then
        failwith<unit> v70
    let v77 : string = "$0.im"
    let v78 : float = Fable.Core.RustInterop.emitRustExpr v58 v77 
    let v79 : float = v78 - 0.275
    let v80 : float =  -v79
    let v81 : bool = v79 >= v80
    let v82 : float =
        if v81 then
            v79
        else
            v80
    let v83 : bool = v82 < 0.001
    let v85 : bool =
        if v83 then
            true
        else
            method18(v83)
    let v86 : float = 0.001
    let v87 : string = method19(v68, v86)
    let v89 : unit = ()
    let v90 : (unit -> unit) = closure3(v87)
    let v91 : unit = (fun () -> v90 (); v89) ()
    let v93 : bool = v85 = false
    if v93 then
        failwith<unit> v87
and method22 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method23(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method26 () : UH0 =
    let v0 : UH0 = UH0_0
    let v1 : UH0 = UH0_1(-40.0, v0)
    let v2 : UH0 = UH0_1(-38.0, v1)
    let v3 : UH0 = UH0_1(-36.0, v2)
    let v4 : UH0 = UH0_1(-34.0, v3)
    let v5 : UH0 = UH0_1(-32.0, v4)
    let v6 : UH0 = UH0_1(-30.0, v5)
    let v7 : UH0 = UH0_1(-28.0, v6)
    let v8 : UH0 = UH0_1(-26.0, v7)
    let v9 : UH0 = UH0_1(-24.0, v8)
    let v10 : UH0 = UH0_1(-22.0, v9)
    let v11 : UH0 = UH0_1(-20.0, v10)
    let v12 : UH0 = UH0_1(-18.0, v11)
    let v13 : UH0 = UH0_1(-16.0, v12)
    let v14 : UH0 = UH0_1(-14.0, v13)
    let v15 : UH0 = UH0_1(-12.0, v14)
    let v16 : UH0 = UH0_1(-10.0, v15)
    let v17 : UH0 = UH0_1(-8.0, v16)
    let v18 : UH0 = UH0_1(-6.0, v17)
    let v19 : UH0 = UH0_1(-4.0, v18)
    UH0_1(-2.0, v19)
and method27 (v0 : pyo3_Python, v1 : UH0) : unit =
    match v1 with
    | UH0_1(v2, v3) -> (* Cons *)
        let v4 : string = "num_complex::Complex::new($0, $1)"
        let v5 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v2, 0.0) v4 
        let v6 : string = $"        s = mpmath.zeta(s)"
        let v7 : num_complex_Complex<float> = method3(v5)
        let v8 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v6, v7)
        let v9 : num_complex_Complex<float> = method13(v0, v5)
        (* run_target_args'
        let v11 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v12 : string = "$0.ok()"
        let v13 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v8 v12 
        let _run_target_args'_v11 = v13 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v14 : string = "$0.ok()"
        let v15 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v8 v14 
        let _run_target_args'_v11 = v15 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v16 : string = "$0.ok()"
        let v17 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v8 v16 
        let _run_target_args'_v11 = v17 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v18 : num_complex_Complex<float> option = match v8 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v11 = v18 
        #endif
#if FABLE_COMPILER_PYTHON
        let v19 : num_complex_Complex<float> option = match v8 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v11 = v19 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v20 : num_complex_Complex<float> option = match v8 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v11 = v20 
        #endif
#else
        let v21 : num_complex_Complex<float> option = match v8 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v11 = v21 
        #endif
        let v22 : num_complex_Complex<float> option = _run_target_args'_v11 
        let v27 : (num_complex_Complex<float> -> US0) = method17()
        let v28 : US0 option = v22 |> Option.map v27 
        let v48 : US0 = US0_1
        let v49 : US0 = v28 |> Option.defaultValue v48 
        let v53 : string = "f64::NAN"
        let v54 : float = Fable.Core.RustInterop.emitRustExpr () v53 
        let v55 : string = "f64::NAN"
        let v56 : float = Fable.Core.RustInterop.emitRustExpr () v55 
        let v57 : string = "num_complex::Complex::new($0, $1)"
        let v58 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v54, v56) v57 
        let v61 : num_complex_Complex<float> =
            match v49 with
            | US0_1 -> (* None *)
                v58
            | US0_0(v59) -> (* Some *)
                v59
        let v62 : string = "$0.re"
        let v63 : float = Fable.Core.RustInterop.emitRustExpr v61 v62 
        let v64 : bool = v63 = 0.0
        let v66 : bool =
            if v64 then
                true
            else
                method18(v64)
        let v67 : string = "__assert_eq"
        let v68 : float = 0.0
        let v69 : string = method19(v67, v68)
        let v71 : unit = ()
        let v72 : (unit -> unit) = closure3(v69)
        let v73 : unit = (fun () -> v72 (); v71) ()
        let v75 : bool = v66 = false
        if v75 then
            failwith<unit> v69
        let v76 : string = "$0.im"
        let v77 : float = Fable.Core.RustInterop.emitRustExpr v61 v76 
        let v78 : bool = v77 = 0.0
        let v80 : bool =
            if v78 then
                true
            else
                method18(v78)
        let v81 : float = 0.0
        let v82 : string = method19(v67, v81)
        let v84 : unit = ()
        let v85 : (unit -> unit) = closure3(v82)
        let v86 : unit = (fun () -> v85 (); v84) ()
        let v88 : bool = v80 = false
        if v88 then
            failwith<unit> v82
        method27(v0, v3)
    | UH0_0 -> (* Nil *)
        ()
and method25 (v0 : pyo3_Python) : unit =
    let v1 : UH0 = method26()
    method27(v0, v1)
and method24 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method25(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method29 (v0 : pyo3_Python) : unit =
    let v1 : string = "num_complex::Complex::new($0, $1)"
    let v2 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.5, 14.134725) v1 
    let v3 : string = "num_complex::Complex::new($0, $1)"
    let v4 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.5, 21.02204) v3 
    let v5 : string = "num_complex::Complex::new($0, $1)"
    let v6 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.5, 25.010857) v5 
    let v7 : string = "num_complex::Complex::new($0, $1)"
    let v8 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.5, 30.424876) v7 
    let v9 : string = "num_complex::Complex::new($0, $1)"
    let v10 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.5, 32.935062) v9 
    let v11 : string = "num_complex::Complex::new($0, $1)"
    let v12 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.5, 37.586178) v11 
    let v13 : (num_complex_Complex<float> []) = [|v2; v4; v6; v8; v10; v12|]
    let v14 : int32 = v13.Length
    let v15 : Mut0 = {l0 = 0} : Mut0
    while method2(v14, v15) do
        let v17 : int32 = v15.l0
        let v18 : num_complex_Complex<float> = v13.[int v17]
        let v19 : string = $"        s = mpmath.zeta(s)"
        let v20 : num_complex_Complex<float> = method3(v18)
        let v21 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v19, v20)
        let v22 : num_complex_Complex<float> = method13(v0, v18)
        (* run_target_args'
        let v24 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v25 : string = "$0.ok()"
        let v26 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v21 v25 
        let _run_target_args'_v24 = v26 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v27 : string = "$0.ok()"
        let v28 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v21 v27 
        let _run_target_args'_v24 = v28 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v29 : string = "$0.ok()"
        let v30 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v21 v29 
        let _run_target_args'_v24 = v30 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v31 : num_complex_Complex<float> option = match v21 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v24 = v31 
        #endif
#if FABLE_COMPILER_PYTHON
        let v32 : num_complex_Complex<float> option = match v21 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v24 = v32 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v33 : num_complex_Complex<float> option = match v21 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v24 = v33 
        #endif
#else
        let v34 : num_complex_Complex<float> option = match v21 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v24 = v34 
        #endif
        let v35 : num_complex_Complex<float> option = _run_target_args'_v24 
        let v40 : (num_complex_Complex<float> -> US0) = method17()
        let v41 : US0 option = v35 |> Option.map v40 
        let v61 : US0 = US0_1
        let v62 : US0 = v41 |> Option.defaultValue v61 
        let v66 : string = "f64::NAN"
        let v67 : float = Fable.Core.RustInterop.emitRustExpr () v66 
        let v68 : string = "f64::NAN"
        let v69 : float = Fable.Core.RustInterop.emitRustExpr () v68 
        let v70 : string = "num_complex::Complex::new($0, $1)"
        let v71 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v67, v69) v70 
        let v74 : num_complex_Complex<float> =
            match v62 with
            | US0_1 -> (* None *)
                v71
            | US0_0(v72) -> (* Some *)
                v72
        let v75 : string = "$0.re"
        let v76 : float = Fable.Core.RustInterop.emitRustExpr v74 v75 
        let v77 : float =  -v76
        let v78 : bool = v76 >= v77
        let v79 : float =
            if v78 then
                v76
            else
                v77
        let v80 : bool = v79 < 0.0001
        let v82 : bool =
            if v80 then
                true
            else
                method18(v80)
        let v83 : string = "__assert_lt"
        let v84 : float = 0.0001
        let v85 : string = method19(v83, v84)
        let v87 : unit = ()
        let v88 : (unit -> unit) = closure3(v85)
        let v89 : unit = (fun () -> v88 (); v87) ()
        let v91 : bool = v82 = false
        if v91 then
            failwith<unit> v85
        let v92 : string = "$0.im"
        let v93 : float = Fable.Core.RustInterop.emitRustExpr v74 v92 
        let v94 : float =  -v93
        let v95 : bool = v93 >= v94
        let v96 : float =
            if v95 then
                v93
            else
                v94
        let v97 : bool = v96 < 0.0001
        let v99 : bool =
            if v97 then
                true
            else
                method18(v97)
        let v100 : float = 0.0001
        let v101 : string = method19(v83, v100)
        let v103 : unit = ()
        let v104 : (unit -> unit) = closure3(v101)
        let v105 : unit = (fun () -> v104 (); v103) ()
        let v107 : bool = v99 = false
        if v107 then
            failwith<unit> v101
        let v108 : int32 = v17 + 1
        v15.l0 <- v108
        ()
    ()
and method28 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method29(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method31 (v0 : pyo3_Python) : unit =
    let v1 : (float []) = [|2.0; 3.0; 4.0; 5.0; 10.0; 20.0; 50.0|]
    let v2 : int32 = v1.Length
    let v3 : Mut0 = {l0 = 0} : Mut0
    while method2(v2, v3) do
        let v5 : int32 = v3.l0
        let v6 : float = v1.[int v5]
        let v7 : string = "num_complex::Complex::new($0, $1)"
        let v8 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v6, 0.0) v7 
        let v9 : string = $"        s = mpmath.zeta(s)"
        let v10 : num_complex_Complex<float> = method3(v8)
        let v11 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v9, v10)
        let v12 : num_complex_Complex<float> = method13(v0, v8)
        (* run_target_args'
        let v14 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v15 : string = "$0.ok()"
        let v16 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v11 v15 
        let _run_target_args'_v14 = v16 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v17 : string = "$0.ok()"
        let v18 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v11 v17 
        let _run_target_args'_v14 = v18 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v19 : string = "$0.ok()"
        let v20 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v11 v19 
        let _run_target_args'_v14 = v20 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v21 : num_complex_Complex<float> option = match v11 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v14 = v21 
        #endif
#if FABLE_COMPILER_PYTHON
        let v22 : num_complex_Complex<float> option = match v11 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v14 = v22 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v23 : num_complex_Complex<float> option = match v11 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v14 = v23 
        #endif
#else
        let v24 : num_complex_Complex<float> option = match v11 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v14 = v24 
        #endif
        let v25 : num_complex_Complex<float> option = _run_target_args'_v14 
        let v30 : (num_complex_Complex<float> -> US0) = method17()
        let v31 : US0 option = v25 |> Option.map v30 
        let v51 : US0 = US0_1
        let v52 : US0 = v31 |> Option.defaultValue v51 
        let v56 : string = "f64::NAN"
        let v57 : float = Fable.Core.RustInterop.emitRustExpr () v56 
        let v58 : string = "f64::NAN"
        let v59 : float = Fable.Core.RustInterop.emitRustExpr () v58 
        let v60 : string = "num_complex::Complex::new($0, $1)"
        let v61 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v57, v59) v60 
        let v64 : num_complex_Complex<float> =
            match v52 with
            | US0_1 -> (* None *)
                v61
            | US0_0(v62) -> (* Some *)
                v62
        let v65 : string = "$0.re"
        let v66 : float = Fable.Core.RustInterop.emitRustExpr v64 v65 
        let v67 : bool = v66 > 0.0
        let v69 : bool =
            if v67 then
                true
            else
                method18(v67)
        let v70 : string = "__assert_gt"
        let v71 : float = 0.0
        let v72 : string = method19(v70, v71)
        let v74 : unit = ()
        let v75 : (unit -> unit) = closure3(v72)
        let v76 : unit = (fun () -> v75 (); v74) ()
        let v78 : bool = v69 = false
        if v78 then
            failwith<unit> v72
        let v79 : string = "$0.im"
        let v80 : float = Fable.Core.RustInterop.emitRustExpr v64 v79 
        let v81 : bool = v80 = 0.0
        let v83 : bool =
            if v81 then
                true
            else
                method18(v81)
        let v84 : string = "__assert_eq"
        let v85 : float = 0.0
        let v86 : string = method19(v84, v85)
        let v88 : unit = ()
        let v89 : (unit -> unit) = closure3(v86)
        let v90 : unit = (fun () -> v89 (); v88) ()
        let v92 : bool = v83 = false
        if v92 then
            failwith<unit> v86
        let v93 : int32 = v5 + 1
        v3.l0 <- v93
        ()
    ()
and method30 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method31(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method33 (v0 : pyo3_Python) : unit =
    let v1 : string = "num_complex::Complex::new($0, $1)"
    let v2 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v1 
    let v3 : string = $"        s = mpmath.zeta(s)"
    let v4 : num_complex_Complex<float> = method3(v2)
    let v5 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v3, v4)
    let v6 : num_complex_Complex<float> = method13(v0, v2)
    (* run_target_args'
    let v8 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v9 : string = "$0.ok()"
    let v10 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v9 
    let _run_target_args'_v8 = v10 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v11 : string = "$0.ok()"
    let v12 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v11 
    let _run_target_args'_v8 = v12 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v13 : string = "$0.ok()"
    let v14 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v13 
    let _run_target_args'_v8 = v14 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v15 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v15 
    #endif
#if FABLE_COMPILER_PYTHON
    let v16 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v16 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v17 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v17 
    #endif
#else
    let v18 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v18 
    #endif
    let v19 : num_complex_Complex<float> option = _run_target_args'_v8 
    let v24 : (num_complex_Complex<float> -> US0) = method17()
    let v25 : US0 option = v19 |> Option.map v24 
    let v45 : US0 = US0_1
    let v46 : US0 = v25 |> Option.defaultValue v45 
    let v50 : string = "f64::NAN"
    let v51 : float = Fable.Core.RustInterop.emitRustExpr () v50 
    let v52 : string = "f64::NAN"
    let v53 : float = Fable.Core.RustInterop.emitRustExpr () v52 
    let v54 : string = "num_complex::Complex::new($0, $1)"
    let v55 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v51, v53) v54 
    let v58 : num_complex_Complex<float> =
        match v46 with
        | US0_1 -> (* None *)
            v55
        | US0_0(v56) -> (* Some *)
            v56
    let v59 : string = "$0.re"
    let v60 : float = Fable.Core.RustInterop.emitRustExpr v58 v59 
    let v61 : bool = v60 = infinity
    let v63 : bool =
        if v61 then
            true
        else
            method18(v61)
    let v64 : string = "__assert_eq"
    let v65 : float = infinity
    let v66 : string = method19(v64, v65)
    let v68 : unit = ()
    let v69 : (unit -> unit) = closure3(v66)
    let v70 : unit = (fun () -> v69 (); v68) ()
    let v72 : bool = v63 = false
    if v72 then
        failwith<unit> v66
    let v73 : string = "$0.im"
    let v74 : float = Fable.Core.RustInterop.emitRustExpr v58 v73 
    let v75 : bool = v74 = 0.0
    let v77 : bool =
        if v75 then
            true
        else
            method18(v75)
    let v78 : float = 0.0
    let v79 : string = method19(v64, v78)
    let v81 : unit = ()
    let v82 : (unit -> unit) = closure3(v79)
    let v83 : unit = (fun () -> v82 (); v81) ()
    let v85 : bool = v77 = false
    if v85 then
        failwith<unit> v79
and method32 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method33(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method35 (v0 : pyo3_Python) : unit =
    let v1 : string = "num_complex::Complex::new($0, $1)"
    let v2 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 10.0) v1 
    let v3 : string = $"        s = mpmath.zeta(s)"
    let v4 : num_complex_Complex<float> = method3(v2)
    let v5 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v3, v4)
    let v6 : num_complex_Complex<float> = method13(v0, v2)
    (* run_target_args'
    let v8 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v9 : string = "$0.ok()"
    let v10 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v9 
    let _run_target_args'_v8 = v10 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v11 : string = "$0.ok()"
    let v12 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v11 
    let _run_target_args'_v8 = v12 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v13 : string = "$0.ok()"
    let v14 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v13 
    let _run_target_args'_v8 = v14 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v15 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v15 
    #endif
#if FABLE_COMPILER_PYTHON
    let v16 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v16 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v17 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v17 
    #endif
#else
    let v18 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v18 
    #endif
    let v19 : num_complex_Complex<float> option = _run_target_args'_v8 
    let v24 : (num_complex_Complex<float> -> US0) = method17()
    let v25 : US0 option = v19 |> Option.map v24 
    let v45 : US0 = US0_1
    let v46 : US0 = v25 |> Option.defaultValue v45 
    let v50 : string = "f64::NAN"
    let v51 : float = Fable.Core.RustInterop.emitRustExpr () v50 
    let v52 : string = "f64::NAN"
    let v53 : float = Fable.Core.RustInterop.emitRustExpr () v52 
    let v54 : string = "num_complex::Complex::new($0, $1)"
    let v55 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v51, v53) v54 
    let v58 : num_complex_Complex<float> =
        match v46 with
        | US0_1 -> (* None *)
            v55
        | US0_0(v56) -> (* Some *)
            v56
    let v59 : string = "$0.re"
    let v60 : float = Fable.Core.RustInterop.emitRustExpr v2 v59 
    let v61 : string = "$0.im"
    let v62 : float = Fable.Core.RustInterop.emitRustExpr v2 v61 
    let v63 : float =  -v62
    let v64 : string = "num_complex::Complex::new($0, $1)"
    let v65 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v60, v63) v64 
    let v66 : string = $"        s = mpmath.zeta(s)"
    let v67 : num_complex_Complex<float> = method3(v65)
    let v68 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v66, v67)
    let v69 : num_complex_Complex<float> = method13(v0, v65)
    (* run_target_args'
    let v71 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v72 : string = "$0.ok()"
    let v73 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v68 v72 
    let _run_target_args'_v71 = v73 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v74 : string = "$0.ok()"
    let v75 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v68 v74 
    let _run_target_args'_v71 = v75 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v76 : string = "$0.ok()"
    let v77 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v68 v76 
    let _run_target_args'_v71 = v77 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v78 : num_complex_Complex<float> option = match v68 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v71 = v78 
    #endif
#if FABLE_COMPILER_PYTHON
    let v79 : num_complex_Complex<float> option = match v68 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v71 = v79 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v80 : num_complex_Complex<float> option = match v68 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v71 = v80 
    #endif
#else
    let v81 : num_complex_Complex<float> option = match v68 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v71 = v81 
    #endif
    let v82 : num_complex_Complex<float> option = _run_target_args'_v71 
    let v87 : (num_complex_Complex<float> -> US0) = method17()
    let v88 : US0 option = v82 |> Option.map v87 
    let v108 : US0 = US0_1
    let v109 : US0 = v88 |> Option.defaultValue v108 
    let v113 : string = "f64::NAN"
    let v114 : float = Fable.Core.RustInterop.emitRustExpr () v113 
    let v115 : string = "f64::NAN"
    let v116 : float = Fable.Core.RustInterop.emitRustExpr () v115 
    let v117 : string = "num_complex::Complex::new($0, $1)"
    let v118 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v114, v116) v117 
    let v121 : num_complex_Complex<float> =
        match v109 with
        | US0_1 -> (* None *)
            v118
        | US0_0(v119) -> (* Some *)
            v119
    let v122 : string = "$0.conj()"
    let v123 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr v121 v122 
    let v124 : string = "$0.re"
    let v125 : float = Fable.Core.RustInterop.emitRustExpr v58 v124 
    let v126 : string = "$0.re"
    let v127 : float = Fable.Core.RustInterop.emitRustExpr v123 v126 
    let v128 : bool = v125 = v127
    let v130 : bool =
        if v128 then
            true
        else
            method18(v128)
    let v131 : string = "__assert_eq"
    let v132 : string = method19(v131, v127)
    let v134 : unit = ()
    let v135 : (unit -> unit) = closure3(v132)
    let v136 : unit = (fun () -> v135 (); v134) ()
    let v138 : bool = v130 = false
    if v138 then
        failwith<unit> v132
    let v139 : string = "$0.im"
    let v140 : float = Fable.Core.RustInterop.emitRustExpr v58 v139 
    let v141 : string = "$0.im"
    let v142 : float = Fable.Core.RustInterop.emitRustExpr v123 v141 
    let v143 : bool = v140 = v142
    let v145 : bool =
        if v143 then
            true
        else
            method18(v143)
    let v146 : string = method19(v131, v142)
    let v148 : unit = ()
    let v149 : (unit -> unit) = closure3(v146)
    let v150 : unit = (fun () -> v149 (); v148) ()
    let v152 : bool = v145 = false
    if v152 then
        failwith<unit> v146
and method34 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method35(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method37 (v0 : pyo3_Python) : unit =
    let v1 : string = "num_complex::Complex::new($0, $1)"
    let v2 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.01, 0.01) v1 
    let v3 : string = $"        s = mpmath.zeta(s)"
    let v4 : num_complex_Complex<float> = method3(v2)
    let v5 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v3, v4)
    let v6 : num_complex_Complex<float> = method13(v0, v2)
    (* run_target_args'
    let v8 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v9 : string = "$0.ok()"
    let v10 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v9 
    let _run_target_args'_v8 = v10 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v11 : string = "$0.ok()"
    let v12 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v11 
    let _run_target_args'_v8 = v12 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v13 : string = "$0.ok()"
    let v14 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v5 v13 
    let _run_target_args'_v8 = v14 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v15 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v15 
    #endif
#if FABLE_COMPILER_PYTHON
    let v16 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v16 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v17 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v17 
    #endif
#else
    let v18 : num_complex_Complex<float> option = match v5 with Ok x -> Some x | Error _ -> None
    let _run_target_args'_v8 = v18 
    #endif
    let v19 : num_complex_Complex<float> option = _run_target_args'_v8 
    let v24 : (num_complex_Complex<float> -> US0) = method17()
    let v25 : US0 option = v19 |> Option.map v24 
    let v45 : US0 = US0_1
    let v46 : US0 = v25 |> Option.defaultValue v45 
    let v50 : string = "f64::NAN"
    let v51 : float = Fable.Core.RustInterop.emitRustExpr () v50 
    let v52 : string = "f64::NAN"
    let v53 : float = Fable.Core.RustInterop.emitRustExpr () v52 
    let v54 : string = "num_complex::Complex::new($0, $1)"
    let v55 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v51, v53) v54 
    let v58 : num_complex_Complex<float> =
        match v46 with
        | US0_1 -> (* None *)
            v55
        | US0_0(v56) -> (* Some *)
            v56
    let v59 : string = "$0.re"
    let v60 : float = Fable.Core.RustInterop.emitRustExpr v58 v59 
    let v61 : bool = v60 < infinity
    let v63 : bool =
        if v61 then
            true
        else
            method18(v61)
    let v64 : string = "__assert_lt"
    let v65 : float = infinity
    let v66 : string = method19(v64, v65)
    let v68 : unit = ()
    let v69 : (unit -> unit) = closure3(v66)
    let v70 : unit = (fun () -> v69 (); v68) ()
    let v72 : bool = v63 = false
    if v72 then
        failwith<unit> v66
    let v73 : string = "$0.im"
    let v74 : float = Fable.Core.RustInterop.emitRustExpr v58 v73 
    let v75 : bool = v74 < infinity
    let v77 : bool =
        if v75 then
            true
        else
            method18(v75)
    let v78 : float = infinity
    let v79 : string = method19(v64, v78)
    let v81 : unit = ()
    let v82 : (unit -> unit) = closure3(v79)
    let v83 : unit = (fun () -> v82 (); v81) ()
    let v85 : bool = v77 = false
    if v85 then
        failwith<unit> v79
and method36 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method37(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method40 () : UH0 =
    let v0 : UH0 = UH0_0
    let v1 : UH0 = UH0_1(100.0, v0)
    let v2 : UH0 = UH0_1(90.0, v1)
    let v3 : UH0 = UH0_1(80.0, v2)
    let v4 : UH0 = UH0_1(70.0, v3)
    let v5 : UH0 = UH0_1(60.0, v4)
    let v6 : UH0 = UH0_1(50.0, v5)
    let v7 : UH0 = UH0_1(40.0, v6)
    let v8 : UH0 = UH0_1(30.0, v7)
    let v9 : UH0 = UH0_1(20.0, v8)
    UH0_1(10.0, v9)
and method41 (v0 : pyo3_Python, v1 : UH0) : unit =
    match v1 with
    | UH0_1(v2, v3) -> (* Cons *)
        let v4 : string = "num_complex::Complex::new($0, $1)"
        let v5 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.0, v2) v4 
        let v6 : string = $"        s = mpmath.zeta(s)"
        let v7 : num_complex_Complex<float> = method3(v5)
        let v8 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v6, v7)
        let v9 : num_complex_Complex<float> = method13(v0, v5)
        (* run_target_args'
        let v11 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v12 : string = "$0.ok()"
        let v13 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v8 v12 
        let _run_target_args'_v11 = v13 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v14 : string = "$0.ok()"
        let v15 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v8 v14 
        let _run_target_args'_v11 = v15 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v16 : string = "$0.ok()"
        let v17 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v8 v16 
        let _run_target_args'_v11 = v17 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v18 : num_complex_Complex<float> option = match v8 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v11 = v18 
        #endif
#if FABLE_COMPILER_PYTHON
        let v19 : num_complex_Complex<float> option = match v8 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v11 = v19 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v20 : num_complex_Complex<float> option = match v8 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v11 = v20 
        #endif
#else
        let v21 : num_complex_Complex<float> option = match v8 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v11 = v21 
        #endif
        let v22 : num_complex_Complex<float> option = _run_target_args'_v11 
        let v27 : (num_complex_Complex<float> -> US0) = method17()
        let v28 : US0 option = v22 |> Option.map v27 
        let v48 : US0 = US0_1
        let v49 : US0 = v28 |> Option.defaultValue v48 
        let v53 : string = "f64::NAN"
        let v54 : float = Fable.Core.RustInterop.emitRustExpr () v53 
        let v55 : string = "f64::NAN"
        let v56 : float = Fable.Core.RustInterop.emitRustExpr () v55 
        let v57 : string = "num_complex::Complex::new($0, $1)"
        let v58 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v54, v56) v57 
        let v61 : num_complex_Complex<float> =
            match v49 with
            | US0_1 -> (* None *)
                v58
            | US0_0(v59) -> (* Some *)
                v59
        let v62 : string = "$0.re"
        let v63 : float = Fable.Core.RustInterop.emitRustExpr v61 v62 
        let v65 : bool = v63 <> 0.0 
        let v69 : bool =
            if v65 then
                true
            else
                method18(v65)
        let v70 : string = "__assert_ne"
        let v71 : float = 0.0
        let v72 : string = method19(v70, v71)
        let v74 : unit = ()
        let v75 : (unit -> unit) = closure3(v72)
        let v76 : unit = (fun () -> v75 (); v74) ()
        let v78 : bool = v69 = false
        if v78 then
            failwith<unit> v72
        let v79 : string = "$0.im"
        let v80 : float = Fable.Core.RustInterop.emitRustExpr v61 v79 
        let v82 : bool = v80 <> 0.0 
        let v86 : bool =
            if v82 then
                true
            else
                method18(v82)
        let v87 : float = 0.0
        let v88 : string = method19(v70, v87)
        let v90 : unit = ()
        let v91 : (unit -> unit) = closure3(v88)
        let v92 : unit = (fun () -> v91 (); v90) ()
        let v94 : bool = v86 = false
        if v94 then
            failwith<unit> v88
        method41(v0, v3)
    | UH0_0 -> (* Nil *)
        ()
and method39 (v0 : pyo3_Python) : unit =
    let v1 : UH0 = method40()
    method41(v0, v1)
and method38 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method39(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method44 () : UH1 =
    let v0 : string = "num_complex::Complex::new($0, $1)"
    let v1 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.5, 14.134725) v0 
    let v2 : string = "num_complex::Complex::new($0, $1)"
    let v3 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.75, 20.5) v2 
    let v4 : string = "num_complex::Complex::new($0, $1)"
    let v5 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.25, 30.1) v4 
    let v6 : string = "num_complex::Complex::new($0, $1)"
    let v7 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.25, 40.0) v6 
    let v8 : string = "num_complex::Complex::new($0, $1)"
    let v9 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 50.0) v8 
    let v10 : UH1 = UH1_0
    let v11 : UH1 = UH1_1(v9, v10)
    let v12 : UH1 = UH1_1(v7, v11)
    let v13 : UH1 = UH1_1(v5, v12)
    let v14 : UH1 = UH1_1(v3, v13)
    UH1_1(v1, v14)
and method45 (v0 : pyo3_Python, v1 : UH1) : unit =
    match v1 with
    | UH1_1(v2, v3) -> (* Cons *)
        let v4 : string = $"        s = mpmath.zeta(s)"
        let v5 : num_complex_Complex<float> = method3(v2)
        let v6 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v4, v5)
        let v7 : num_complex_Complex<float> = method13(v0, v2)
        (* run_target_args'
        let v9 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v10 : string = "$0.ok()"
        let v11 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v6 v10 
        let _run_target_args'_v9 = v11 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v12 : string = "$0.ok()"
        let v13 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v6 v12 
        let _run_target_args'_v9 = v13 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v14 : string = "$0.ok()"
        let v15 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v6 v14 
        let _run_target_args'_v9 = v15 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v16 : num_complex_Complex<float> option = match v6 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v9 = v16 
        #endif
#if FABLE_COMPILER_PYTHON
        let v17 : num_complex_Complex<float> option = match v6 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v9 = v17 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v18 : num_complex_Complex<float> option = match v6 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v9 = v18 
        #endif
#else
        let v19 : num_complex_Complex<float> option = match v6 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v9 = v19 
        #endif
        let v20 : num_complex_Complex<float> option = _run_target_args'_v9 
        let v25 : (num_complex_Complex<float> -> US0) = method17()
        let v26 : US0 option = v20 |> Option.map v25 
        let v46 : US0 = US0_1
        let v47 : US0 = v26 |> Option.defaultValue v46 
        let v51 : string = "f64::NAN"
        let v52 : float = Fable.Core.RustInterop.emitRustExpr () v51 
        let v53 : string = "f64::NAN"
        let v54 : float = Fable.Core.RustInterop.emitRustExpr () v53 
        let v55 : string = "num_complex::Complex::new($0, $1)"
        let v56 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v52, v54) v55 
        let v59 : num_complex_Complex<float> =
            match v47 with
            | US0_1 -> (* None *)
                v56
            | US0_0(v57) -> (* Some *)
                v57
        let v60 : string = "$0.re"
        let v61 : float = Fable.Core.RustInterop.emitRustExpr v59 v60 
        let v63 : bool = v61 <> 0.0 
        let v67 : bool =
            if v63 then
                true
            else
                method18(v63)
        let v68 : string = "__assert_ne"
        let v69 : float = 0.0
        let v70 : string = method19(v68, v69)
        let v72 : unit = ()
        let v73 : (unit -> unit) = closure3(v70)
        let v74 : unit = (fun () -> v73 (); v72) ()
        let v76 : bool = v67 = false
        if v76 then
            failwith<unit> v70
        let v77 : string = "$0.im"
        let v78 : float = Fable.Core.RustInterop.emitRustExpr v59 v77 
        let v80 : bool = v78 <> 0.0 
        let v84 : bool =
            if v80 then
                true
            else
                method18(v80)
        let v85 : float = 0.0
        let v86 : string = method19(v68, v85)
        let v88 : unit = ()
        let v89 : (unit -> unit) = closure3(v86)
        let v90 : unit = (fun () -> v89 (); v88) ()
        let v92 : bool = v84 = false
        if v92 then
            failwith<unit> v86
        method45(v0, v3)
    | UH1_0 -> (* Nil *)
        ()
and method43 (v0 : pyo3_Python) : unit =
    let v1 : UH1 = method44()
    method45(v0, v1)
and method42 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method43(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method48 () : UH1 =
    let v0 : string = "num_complex::Complex::new($0, $1)"
    let v1 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.0, 4.0) v0 
    let v2 : string = "num_complex::Complex::new($0, $1)"
    let v3 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.5, -3.5) v2 
    let v4 : string = "num_complex::Complex::new($0, $1)"
    let v5 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.5, 2.5) v4 
    let v6 : string = "num_complex::Complex::new($0, $1)"
    let v7 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (0.5, 14.134725) v6 
    let v8 : UH1 = UH1_0
    let v9 : UH1 = UH1_1(v7, v8)
    let v10 : UH1 = UH1_1(v5, v9)
    let v11 : UH1 = UH1_1(v3, v10)
    UH1_1(v1, v11)
and method49 (v0 : pyo3_Python, v1 : UH1) : unit =
    match v1 with
    | UH1_1(v2, v3) -> (* Cons *)
        let v4 : string = $"        s = mpmath.zeta(s)"
        let v5 : num_complex_Complex<float> = method3(v2)
        let v6 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v4, v5)
        let v7 : num_complex_Complex<float> = method13(v0, v2)
        (* run_target_args'
        let v9 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v10 : string = "$0.ok()"
        let v11 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v6 v10 
        let _run_target_args'_v9 = v11 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v12 : string = "$0.ok()"
        let v13 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v6 v12 
        let _run_target_args'_v9 = v13 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v14 : string = "$0.ok()"
        let v15 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v6 v14 
        let _run_target_args'_v9 = v15 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v16 : num_complex_Complex<float> option = match v6 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v9 = v16 
        #endif
#if FABLE_COMPILER_PYTHON
        let v17 : num_complex_Complex<float> option = match v6 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v9 = v17 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v18 : num_complex_Complex<float> option = match v6 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v9 = v18 
        #endif
#else
        let v19 : num_complex_Complex<float> option = match v6 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v9 = v19 
        #endif
        let v20 : num_complex_Complex<float> option = _run_target_args'_v9 
        let v25 : (num_complex_Complex<float> -> US0) = method17()
        let v26 : US0 option = v20 |> Option.map v25 
        let v46 : US0 = US0_1
        let v47 : US0 = v26 |> Option.defaultValue v46 
        let v51 : string = "f64::NAN"
        let v52 : float = Fable.Core.RustInterop.emitRustExpr () v51 
        let v53 : string = "f64::NAN"
        let v54 : float = Fable.Core.RustInterop.emitRustExpr () v53 
        let v55 : string = "num_complex::Complex::new($0, $1)"
        let v56 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v52, v54) v55 
        let v59 : num_complex_Complex<float> =
            match v47 with
            | US0_1 -> (* None *)
                v56
            | US0_0(v57) -> (* Some *)
                v57
        let v60 : string = "num_complex::Complex::new($0, $1)"
        let v61 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v60 
        let v62 : string = "num_complex::Complex::powc($0, $1)"
        let v63 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v61, v2) v62 
        let v64 : string = "num_complex::Complex::new($0, $1)"
        let v65 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v64 
        let v66 : string = "num_complex::Complex::new($0, $1)"
        let v67 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v66 
        let v68 : string = "$0 - $1"
        let v69 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v2, v67) v68 
        let v70 : string = "num_complex::Complex::powc($0, $1)"
        let v71 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v65, v69) v70 
        let v72 : string = "$0 * $1"
        let v73 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v63, v71) v72 
        let v74 : string = "num_complex::Complex::new($0, $1)"
        let v75 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (3.141592653589793, 0.0) v74 
        let v76 : string = "$0 * $1"
        let v77 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v75, v2) v76 
        let v78 : string = "num_complex::Complex::new($0, $1)"
        let v79 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (2.0, 0.0) v78 
        let v80 : string = "$0 / $1"
        let v81 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v77, v79) v80 
        let v82 : string = "$0.sin()"
        let v83 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr v81 v82 
        let v84 : string = "$0 * $1"
        let v85 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v73, v83) v84 
        let v86 : string = "num_complex::Complex::new($0, $1)"
        let v87 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (1.0, 0.0) v86 
        let v88 : string = "$0 - $1"
        let v89 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v87, v2) v88 
        let v90 : string = $"        s = mpmath.gamma(s)"
        let v91 : num_complex_Complex<float> = method3(v89)
        let v92 : Result<num_complex_Complex<float>, std_string_String> = method16(v0, v90, v91)
        (* run_target_args'
        let v94 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v95 : string = "$0.ok()"
        let v96 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v92 v95 
        let _run_target_args'_v94 = v96 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v97 : string = "$0.ok()"
        let v98 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v92 v97 
        let _run_target_args'_v94 = v98 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v99 : string = "$0.ok()"
        let v100 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v92 v99 
        let _run_target_args'_v94 = v100 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v101 : num_complex_Complex<float> option = match v92 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v94 = v101 
        #endif
#if FABLE_COMPILER_PYTHON
        let v102 : num_complex_Complex<float> option = match v92 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v94 = v102 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v103 : num_complex_Complex<float> option = match v92 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v94 = v103 
        #endif
#else
        let v104 : num_complex_Complex<float> option = match v92 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v94 = v104 
        #endif
        let v105 : num_complex_Complex<float> option = _run_target_args'_v94 
        let v110 : (num_complex_Complex<float> -> US0) = method17()
        let v111 : US0 option = v105 |> Option.map v110 
        let v131 : US0 = US0_1
        let v132 : US0 = v111 |> Option.defaultValue v131 
        let v136 : string = "f64::NAN"
        let v137 : float = Fable.Core.RustInterop.emitRustExpr () v136 
        let v138 : string = "f64::NAN"
        let v139 : float = Fable.Core.RustInterop.emitRustExpr () v138 
        let v140 : string = "num_complex::Complex::new($0, $1)"
        let v141 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v137, v139) v140 
        let v144 : num_complex_Complex<float> =
            match v132 with
            | US0_1 -> (* None *)
                v141
            | US0_0(v142) -> (* Some *)
                v142
        let v145 : string = "$0 * $1"
        let v146 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v85, v144) v145 
        let v147 : string = "$0.re"
        let v148 : float = Fable.Core.RustInterop.emitRustExpr v2 v147 
        let v149 : float = 1.0 - v148
        let v150 : string = "$0.im"
        let v151 : float = Fable.Core.RustInterop.emitRustExpr v2 v150 
        let v152 : float =  -v151
        let v153 : string = "num_complex::Complex::new($0, $1)"
        let v154 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v149, v152) v153 
        let v155 : string = $"        s = mpmath.zeta(s)"
        let v156 : num_complex_Complex<float> = method3(v154)
        let v157 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v155, v156)
        let v158 : num_complex_Complex<float> = method13(v0, v154)
        (* run_target_args'
        let v160 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v161 : string = "$0.ok()"
        let v162 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v157 v161 
        let _run_target_args'_v160 = v162 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v163 : string = "$0.ok()"
        let v164 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v157 v163 
        let _run_target_args'_v160 = v164 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v165 : string = "$0.ok()"
        let v166 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v157 v165 
        let _run_target_args'_v160 = v166 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v167 : num_complex_Complex<float> option = match v157 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v160 = v167 
        #endif
#if FABLE_COMPILER_PYTHON
        let v168 : num_complex_Complex<float> option = match v157 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v160 = v168 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v169 : num_complex_Complex<float> option = match v157 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v160 = v169 
        #endif
#else
        let v170 : num_complex_Complex<float> option = match v157 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v160 = v170 
        #endif
        let v171 : num_complex_Complex<float> option = _run_target_args'_v160 
        let v176 : (num_complex_Complex<float> -> US0) = method17()
        let v177 : US0 option = v171 |> Option.map v176 
        let v197 : US0 = US0_1
        let v198 : US0 = v177 |> Option.defaultValue v197 
        let v202 : string = "f64::NAN"
        let v203 : float = Fable.Core.RustInterop.emitRustExpr () v202 
        let v204 : string = "f64::NAN"
        let v205 : float = Fable.Core.RustInterop.emitRustExpr () v204 
        let v206 : string = "num_complex::Complex::new($0, $1)"
        let v207 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v203, v205) v206 
        let v210 : num_complex_Complex<float> =
            match v198 with
            | US0_1 -> (* None *)
                v207
            | US0_0(v208) -> (* Some *)
                v208
        let v211 : string = "$0 * $1"
        let v212 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v146, v210) v211 
        let v213 : string = "$0.re"
        let v214 : float = Fable.Core.RustInterop.emitRustExpr v59 v213 
        let v215 : string = "$0.re"
        let v216 : float = Fable.Core.RustInterop.emitRustExpr v212 v215 
        let v217 : float = v214 - v216
        let v218 : float =  -v217
        let v219 : bool = v217 >= v218
        let v220 : float =
            if v219 then
                v217
            else
                v218
        let v221 : bool = v220 < 0.0001
        let v223 : bool =
            if v221 then
                true
            else
                method18(v221)
        let v224 : string = "__assert_lt"
        let v225 : float = 0.0001
        let v226 : string = method19(v224, v225)
        let v228 : unit = ()
        let v229 : (unit -> unit) = closure3(v226)
        let v230 : unit = (fun () -> v229 (); v228) ()
        let v232 : bool = v223 = false
        if v232 then
            failwith<unit> v226
        let v233 : string = "$0.im"
        let v234 : float = Fable.Core.RustInterop.emitRustExpr v59 v233 
        let v235 : string = "$0.im"
        let v236 : float = Fable.Core.RustInterop.emitRustExpr v212 v235 
        let v237 : float = v234 - v236
        let v238 : float =  -v237
        let v239 : bool = v237 >= v238
        let v240 : float =
            if v239 then
                v237
            else
                v238
        let v241 : bool = v240 < 0.0001
        let v243 : bool =
            if v241 then
                true
            else
                method18(v241)
        let v244 : float = 0.0001
        let v245 : string = method19(v224, v244)
        let v247 : unit = ()
        let v248 : (unit -> unit) = closure3(v245)
        let v249 : unit = (fun () -> v248 (); v247) ()
        let v251 : bool = v243 = false
        if v251 then
            failwith<unit> v245
        method49(v0, v3)
    | UH1_0 -> (* Nil *)
        ()
and method47 (v0 : pyo3_Python) : unit =
    let v1 : UH1 = method48()
    method49(v0, v1)
and method46 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method47(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and method52 () : UH0 =
    let v0 : UH0 = UH0_0
    let v1 : UH0 = UH0_1(5.0, v0)
    let v2 : UH0 = UH0_1(4.5, v1)
    let v3 : UH0 = UH0_1(4.0, v2)
    let v4 : UH0 = UH0_1(3.5, v3)
    let v5 : UH0 = UH0_1(3.0, v4)
    let v6 : UH0 = UH0_1(2.5, v5)
    UH0_1(2.0, v6)
and method53 () : UH0 =
    let v0 : UH0 = UH0_0
    let v1 : UH0 = UH0_1(71.0, v0)
    let v2 : UH0 = UH0_1(67.0, v1)
    let v3 : UH0 = UH0_1(61.0, v2)
    let v4 : UH0 = UH0_1(59.0, v3)
    let v5 : UH0 = UH0_1(53.0, v4)
    let v6 : UH0 = UH0_1(47.0, v5)
    let v7 : UH0 = UH0_1(43.0, v6)
    let v8 : UH0 = UH0_1(41.0, v7)
    let v9 : UH0 = UH0_1(37.0, v8)
    let v10 : UH0 = UH0_1(31.0, v9)
    let v11 : UH0 = UH0_1(29.0, v10)
    let v12 : UH0 = UH0_1(23.0, v11)
    let v13 : UH0 = UH0_1(19.0, v12)
    let v14 : UH0 = UH0_1(17.0, v13)
    let v15 : UH0 = UH0_1(13.0, v14)
    let v16 : UH0 = UH0_1(11.0, v15)
    let v17 : UH0 = UH0_1(7.0, v16)
    let v18 : UH0 = UH0_1(5.0, v17)
    let v19 : UH0 = UH0_1(3.0, v18)
    UH0_1(2.0, v19)
and method55 (v0 : float, v1 : UH0, v2 : float) : float =
    match v1 with
    | UH0_1(v3, v4) -> (* Cons *)
        let v5 : float =  -v0
        let v6 : float = v3 ** v5
        let v7 : float = 1.0 - v6
        let v8 : float = v2 / v7
        method55(v0, v4, v8)
    | UH0_0 -> (* Nil *)
        v2
and method54 (v0 : pyo3_Python, v1 : UH0, v2 : UH0) : unit =
    match v2 with
    | UH0_1(v3, v4) -> (* Cons *)
        let v5 : string = "num_complex::Complex::new($0, $1)"
        let v6 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v3, 0.0) v5 
        let v7 : float = 1.0
        let v8 : float = method55(v3, v1, v7)
        let v9 : string = $"        s = mpmath.zeta(s)"
        let v10 : num_complex_Complex<float> = method3(v6)
        let v11 : Result<num_complex_Complex<float>, std_string_String> = method4(v0, v9, v10)
        let v12 : num_complex_Complex<float> = method13(v0, v6)
        (* run_target_args'
        let v14 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v15 : string = "$0.ok()"
        let v16 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v11 v15 
        let _run_target_args'_v14 = v16 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v17 : string = "$0.ok()"
        let v18 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v11 v17 
        let _run_target_args'_v14 = v18 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v19 : string = "$0.ok()"
        let v20 : num_complex_Complex<float> option = Fable.Core.RustInterop.emitRustExpr v11 v19 
        let _run_target_args'_v14 = v20 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v21 : num_complex_Complex<float> option = match v11 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v14 = v21 
        #endif
#if FABLE_COMPILER_PYTHON
        let v22 : num_complex_Complex<float> option = match v11 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v14 = v22 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        let v23 : num_complex_Complex<float> option = match v11 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v14 = v23 
        #endif
#else
        let v24 : num_complex_Complex<float> option = match v11 with Ok x -> Some x | Error _ -> None
        let _run_target_args'_v14 = v24 
        #endif
        let v25 : num_complex_Complex<float> option = _run_target_args'_v14 
        let v30 : (num_complex_Complex<float> -> US0) = method17()
        let v31 : US0 option = v25 |> Option.map v30 
        let v51 : US0 = US0_1
        let v52 : US0 = v31 |> Option.defaultValue v51 
        let v56 : string = "f64::NAN"
        let v57 : float = Fable.Core.RustInterop.emitRustExpr () v56 
        let v58 : string = "f64::NAN"
        let v59 : float = Fable.Core.RustInterop.emitRustExpr () v58 
        let v60 : string = "num_complex::Complex::new($0, $1)"
        let v61 : num_complex_Complex<float> = Fable.Core.RustInterop.emitRustExpr struct (v57, v59) v60 
        let v64 : num_complex_Complex<float> =
            match v52 with
            | US0_1 -> (* None *)
                v61
            | US0_0(v62) -> (* Some *)
                v62
        let v65 : string = "$0.re"
        let v66 : float = Fable.Core.RustInterop.emitRustExpr v64 v65 
        let v67 : float = v66 - v8
        let v68 : float =  -v67
        let v69 : bool = v67 >= v68
        let v70 : float =
            if v69 then
                v67
            else
                v68
        let v71 : bool = v70 < 0.01
        let v73 : bool =
            if v71 then
                true
            else
                method18(v71)
        let v74 : string = "__assert_lt"
        let v75 : float = 0.01
        let v76 : string = method19(v74, v75)
        let v78 : unit = ()
        let v79 : (unit -> unit) = closure3(v76)
        let v80 : unit = (fun () -> v79 (); v78) ()
        let v82 : bool = v73 = false
        if v82 then
            failwith<unit> v76
        let v83 : string = "$0.im"
        let v84 : float = Fable.Core.RustInterop.emitRustExpr v64 v83 
        let v85 : bool = v84 < 0.01
        let v87 : bool =
            if v85 then
                true
            else
                method18(v85)
        let v88 : float = 0.01
        let v89 : string = method19(v74, v88)
        let v91 : unit = ()
        let v92 : (unit -> unit) = closure3(v89)
        let v93 : unit = (fun () -> v92 (); v91) ()
        let v95 : bool = v87 = false
        if v95 then
            failwith<unit> v89
        method54(v0, v1, v4)
    | UH0_0 -> (* Nil *)
        ()
and method51 (v0 : pyo3_Python) : unit =
    let v1 : UH0 = method52()
    let v2 : UH0 = method53()
    method54(v0, v2, v1)
and method50 () : unit =
    let v0 : string = "pyo3::Python::initialize()"
    Fable.Core.RustInterop.emitRustExpr () v0 
    let v1 : string = "let __run_test = pyo3::Python::attach(|py| -> pyo3::PyResult<()> { //"
    Fable.Core.RustInterop.emitRustExpr () v1 
    let v2 : string = "py"
    let v3 : pyo3_Python = Fable.Core.RustInterop.emitRustExpr () v2 
    method51(v3)
    let v5 : Result<unit, pyo3_PyErr> = Ok () 
    let v8 : Result<unit, pyo3_PyErr> = method21(v5)
    let v9 : string = ""
    let v10 : string = "}"
    let v11 : string = v9 + v10 
    let v12 : string = v11 + v10 
    let v13 : string = "{"
    let v14 : string = v9 + v13 
    let x = v8 //
    let v15 : _ = x
    let v16 : unit = ()
    (* run_target_args'
    let v17 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v18 : string = $"true; let _fix_closure_v16 = $0"
    let v19 : bool = Fable.Core.RustInterop.emitRustExpr v15 v18 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = $"true; let _fix_closure_v16 = $0"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr v15 v20 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v22 : string = $"true; let _fix_closure_v16 = $0"
    let v23 : bool = Fable.Core.RustInterop.emitRustExpr v15 v22 
    let _run_target_args'_v17 = true 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let _run_target_args'_v17 = false 
    #endif
#if FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v17 = false 
    #endif
#else
    let _run_target_args'_v17 = false 
    #endif
    let v24 : bool = _run_target_args'_v17 
    let v25 : string = $"true; _fix_closure_v16 " + v12 + "); " + v14 + " // rust.fix_closure'"
    let v26 : bool = Fable.Core.RustInterop.emitRustExpr () v25 
    let v27 : string = "__run_test"
    let v28 : Result<unit, pyo3_PyErr> = Fable.Core.RustInterop.emitRustExpr () v27 
    (* run_target_args'
    let v29 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v30 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v30 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v31 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v31 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v32 : string = "$0.unwrap()"
    Fable.Core.RustInterop.emitRustExpr v28 v32 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
#else
    match v28 with Ok x -> x | Error e -> failwith $"resultm.unwrap' / e: {e}"
    #endif
    // run_target_args' is_unit
    ()
and closure0 () () : unit =
    let v0 : string = "true; () //"
    let v1 : bool = Fable.Core.RustInterop.emitRustExpr () v0 
    let v2 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v2 
    let v3 : string = "test_zeta_at_known_values_"
    let v4 : string = $"*/ #[test] fn " + v3 + "() { //"
    let v5 : bool = Fable.Core.RustInterop.emitRustExpr () v4 
    method0()
    let v6 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v6 
    let v7 : string = "test_zeta_at_2_minus2"
    let v8 : string = $"*/ #[test] fn " + v7 + "() { //"
    let v9 : bool = Fable.Core.RustInterop.emitRustExpr () v8 
    method22()
    let v10 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v10 
    let v11 : string = "test_trivial_zero_at_negative_even___"
    let v12 : string = $"*/ #[test] fn " + v11 + "() { //"
    let v13 : bool = Fable.Core.RustInterop.emitRustExpr () v12 
    method24()
    let v14 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v14 
    let v15 : string = "test_non_trivial_zero___"
    let v16 : string = $"*/ #[test] fn " + v15 + "() { //"
    let v17 : bool = Fable.Core.RustInterop.emitRustExpr () v16 
    method28()
    let v18 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v18 
    let v19 : string = "test_real_part_greater_than_one___"
    let v20 : string = $"*/ #[test] fn " + v19 + "() { //"
    let v21 : bool = Fable.Core.RustInterop.emitRustExpr () v20 
    method30()
    let v22 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v22 
    let v23 : string = "test_zeta_at_1___"
    let v24 : string = $"*/ #[test] fn " + v23 + "() { //"
    let v25 : bool = Fable.Core.RustInterop.emitRustExpr () v24 
    method32()
    let v26 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v26 
    let v27 : string = "test_symmetry_across_real_axis___"
    let v28 : string = $"*/ #[test] fn " + v27 + "() { //"
    let v29 : bool = Fable.Core.RustInterop.emitRustExpr () v28 
    method34()
    let v30 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v30 
    let v31 : string = "test_behavior_near_origin___"
    let v32 : string = $"*/ #[test] fn " + v31 + "() { //"
    let v33 : bool = Fable.Core.RustInterop.emitRustExpr () v32 
    method36()
    let v34 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v34 
    let v35 : string = "test_imaginary_axis"
    let v36 : string = $"*/ #[test] fn " + v35 + "() { //"
    let v37 : bool = Fable.Core.RustInterop.emitRustExpr () v36 
    method38()
    let v38 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v38 
    let v39 : string = "test_critical_strip"
    let v40 : string = $"*/ #[test] fn " + v39 + "() { //"
    let v41 : bool = Fable.Core.RustInterop.emitRustExpr () v40 
    method42()
    let v42 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v42 
    let v43 : string = "test_reflection_formula_for_specific_value"
    let v44 : string = $"*/ #[test] fn " + v43 + "() { //"
    let v45 : bool = Fable.Core.RustInterop.emitRustExpr () v44 
    method46()
    let v46 : string = "} /* /*"
    Fable.Core.RustInterop.emitRustExpr () v46 
    let v47 : string = "test_euler_product_formula"
    let v48 : string = $"*/ #[test] fn " + v47 + "() { //"
    let v49 : bool = Fable.Core.RustInterop.emitRustExpr () v48 
    method50()
    let v50 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v50 
    let v51 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v51 
    let v52 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v52 
    let v53 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v53 
    let v54 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v54 
    let v55 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v55 
    let v56 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v56 
    let v57 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v57 
    let v58 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v58 
    let v59 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v59 
    let v60 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v60 
    let v61 : string = "{ //"
    Fable.Core.RustInterop.emitRustExpr () v61 
    ()
and closure4 () (v0 : (string [])) : int32 =
    let v1 : string = $"value: {1}"
    let v3 : unit = ()
    let v4 : (unit -> unit) = closure3(v1)
    let v5 : unit = (fun () -> v4 (); v3) ()
    0
let v0 : (unit -> unit) = closure0()
let tests () = v0 ()
let v1 : ((string []) -> int32) = closure4()
let main args = v1 args
()
