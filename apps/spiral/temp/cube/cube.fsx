#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("std::string::String")>]
type std_string_String = class end
#else
type std_string_String = string
#endif

type IOsEnviron = abstract environ: x: unit -> obj
#if FABLE_COMPILER
[<Fable.Core.Erase; Fable.Core.Emit("std::env::VarError")>]
#endif
type std_env_VarError = class end
type [<Struct>] US0 =
    | US0_0
    | US0_1
    | US0_2
and [<Struct>] US1 =
    | US1_0 of f0_0 : US0
    | US1_1 of f1_0 : US0
    | US1_2 of f2_0 : US0
    | US1_3 of f3_0 : US0
    | US1_4 of f4_0 : US0
    | US1_5 of f5_0 : US0
    | US1_6 of f6_0 : US0
and [<Struct>] US2 =
    | US2_0 of f0_0 : string
    | US2_1
and Mut0 = {mutable l0 : float}
and [<Struct>] US3 =
    | US3_0 of f0_0 : int32 * f0_1 : float * f0_2 : char
    | US3_1
and [<Struct>] US4 =
    | US4_0 of f0_0 : float * f0_1 : char
    | US4_1
let rec method0 () : string =
    let v0 : string = "VSCODE_PID"
    v0
and method2 (v0 : string) : string =
    v0
and method3 () : string =
    let v0 : string = ""
    v0
and closure1 () (v0 : string) : US2 =
    US2_0(v0)
and method4 () : (string -> US2) =
    closure1()
and method1 (v0 : string) : string =
    (* run_target_args'
    let v2 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v3 : string = method2(v0)
    let v4 : string = "std::env::var(&*$0)"
    let v5 : Result<std_string_String, std_env_VarError> = Fable.Core.RustInterop.emitRustExpr v3 v4 
    let v6 : string = "true; let _result_map_ = $0.map(|x| { //"
    let v7 : bool = Fable.Core.RustInterop.emitRustExpr v5 v6 
    let v8 : string = "x"
    let v9 : std_string_String = Fable.Core.RustInterop.emitRustExpr () v8 
    let v10 : string = "fable_library_rust::String_::fromString($0)"
    let v11 : string = Fable.Core.RustInterop.emitRustExpr v9 v10 
    let v12 : string = "true; $0 })"
    let v13 : bool = Fable.Core.RustInterop.emitRustExpr v11 v12 
    let v14 : string = "_result_map_"
    let v15 : Result<string, std_env_VarError> = Fable.Core.RustInterop.emitRustExpr () v14 
    let v16 : string = method3()
    let v17 : string = "$0.unwrap_or($1)"
    let v18 : string = Fable.Core.RustInterop.emitRustExpr struct (v15, v16) v17 
    let _run_target_args'_v2 = v18 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v19 : US0 = US0_1
    let v20 : US1 = US1_4(v19)
    let v21 : string = $"env.get_environment_variable / target: {v20} / var: {v0}"
    let v22 : string = failwith<string> v21
    let _run_target_args'_v2 = v22 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v23 : US0 = US0_2
    let v24 : US1 = US1_4(v23)
    let v25 : string = $"env.get_environment_variable / target: {v24} / var: {v0}"
    let v26 : string = failwith<string> v25
    let _run_target_args'_v2 = v26 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v27 : string = "process.env[$0] ?? \"\""
    let v28 : string = Fable.Core.JsInterop.emitJsExpr v0 v27 
    let _run_target_args'_v2 = v28 
    #endif
#if FABLE_COMPILER_PYTHON
    let v30 : string = "os"
    let v31 : IOsEnviron = Fable.Core.PyInterop.importAll v30 
    let v32 : string = "v31.environ"
    let v33 : obj = Fable.Core.PyInterop.emitPyExpr () v32 
    let v37 : string = "v33.get($0)"
    let v38 : string = Fable.Core.PyInterop.emitPyExpr v0 v37 
    let mutable _v38 = None
    #if !FABLE_COMPILER && !WASM && !CONTRACT
    let v42 : (string -> string option) = Option.ofObj
    let v43 : string option = v42 v38
    v43 
    #else
    Some v38 
    #endif
    |> fun x -> _v38 <- Some x
    let v44 : string option = match _v38 with Some x -> x | None -> failwith "optionm'.of_obj / _v38=None"
    let v49 : (string -> US2) = method4()
    let v50 : US2 option = v44 |> Option.map v49 
    let v70 : US2 = US2_1
    let v71 : US2 = v50 |> Option.defaultValue v70 
    let v78 : string =
        match v71 with
        | US2_1 -> (* None *)
            let v76 : string = ""
            v76
        | US2_0(v75) -> (* Some *)
            v75
    let _run_target_args'_v2 = v78 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v79 : US0 = US0_1
    let v80 : US1 = US1_1(v79)
    let v81 : string = $"env.get_environment_variable / target: {v80} / var: {v0}"
    let v82 : string = failwith<string> v81
    let _run_target_args'_v2 = v82 
    #endif
#else
    let v83 : (string -> string) = System.Environment.GetEnvironmentVariable
    let v84 : string = v83 v0
    let mutable _v84 = None
    #if !FABLE_COMPILER && !WASM && !CONTRACT
    let v86 : (string -> string option) = Option.ofObj
    let v87 : string option = v86 v84
    v87 
    #else
    Some v84 
    #endif
    |> fun x -> _v84 <- Some x
    let v88 : string option = match _v84 with Some x -> x | None -> failwith "optionm'.of_obj / _v84=None"
    let v93 : (string -> US2) = method4()
    let v94 : US2 option = v88 |> Option.map v93 
    let v114 : US2 = US2_1
    let v115 : US2 = v94 |> Option.defaultValue v114 
    let v122 : string =
        match v115 with
        | US2_1 -> (* None *)
            let v120 : string = ""
            v120
        | US2_0(v119) -> (* Some *)
            v119
    let _run_target_args'_v2 = v122 
    #endif
    let v123 : string = _run_target_args'_v2 
    v123
and method7 (v0 : float, v1 : float, v2 : float) : struct (float * float * float) =
    struct (v0, v1, v2)
and method9 (v0 : float, v1 : Mut0) : bool =
    let v2 : float = v1.l0
    let v3 : bool = v2 < v0
    v3
and method10 (v0 : float seq) : float seq =
    v0
and method12 (v0 : float, v1 : float, v2 : float, v3 : char, v4 : float, v5 : float, v6 : float, v7 : float) : US3 =
    let v8 : float = sin v4
    let v9 : float = v1 * v8
    let v10 : float = sin v5
    let v11 : float = v9 * v10
    let v12 : float = cos v6
    let v13 : float = v11 * v12
    let v14 : float = cos v4
    let v15 : float = v2 * v14
    let v16 : float = v15 * v10
    let v17 : float = v16 * v12
    let v18 : float = v13 - v17
    let v19 : float = v1 * v14
    let v20 : float = sin v6
    let v21 : float = v19 * v20
    let v22 : float = v18 + v21
    let v23 : float = v2 * v8
    let v24 : float = v23 * v20
    let v25 : float = v22 + v24
    let v26 : float = cos v5
    let v27 : float = v0 * v26
    let v28 : float = v27 * v12
    let v29 : float = v25 + v28
    let v30 : float = v19 * v12
    let v31 : float = v23 * v12
    let v32 : float = v30 + v31
    let v33 : float = v11 * v20
    let v34 : float = v32 - v33
    let v35 : float = v16 * v20
    let v36 : float = v34 + v35
    let v37 : float = v27 * v20
    let v38 : float = v36 - v37
    let v39 : float = v15 * v26
    let v40 : float = v9 * v26
    let v41 : float = v39 - v40
    let v42 : float = v0 * v10
    let v43 : float = v41 + v42
    let v44 : float = v43 + 100.0
    let v45 : float = 1.0 / v44
    let v47 : (int32 -> float) = float
    let v48 : float = v47 160
    let v51 : float = v48 / 2.0
    let v52 : float = v51 + v7
    let v53 : float = 40.0 * v45
    let v54 : float = v53 * v29
    let v55 : float = v54 * 2.0
    let v56 : float = v52 + v55
    let v62 : int32 = v56 |> int32 
    let v71 : (int32 -> float) = float
    let v72 : float = v71 44
    let v75 : float = v72 / 2.0
    let v76 : float = v53 * v38
    let v77 : float = v75 + v76
    let v83 : int32 = v77 |> int32 
    let v91 : int32 = v83 * 160
    let v92 : int32 = v62 + v91
    let v93 : bool = v92 >= 0
    let v95 : bool =
        if v93 then
            let v94 : bool = v92 < 7040
            v94
        else
            false
    if v95 then
        US3_0(v92, v45, v3)
    else
        US3_1
and closure4 (v0 : float, v1 : float, v2 : float, v3 : float, v4 : float, v5 : float) (v6 : float) : US3 list =
    let v7 : float =  -v3
    let v8 : char = ';'
    let v9 : US3 = method12(v5, v6, v7, v8, v0, v1, v2, v4)
    let v10 : char = '\\'
    let v11 : US3 = method12(v3, v6, v5, v10, v0, v1, v2, v4)
    let v12 : float =  -v5
    let v13 : char = '/'
    let v14 : US3 = method12(v7, v6, v12, v13, v0, v1, v2, v4)
    let v15 : char = '='
    let v16 : US3 = method12(v12, v6, v3, v15, v0, v1, v2, v4)
    let v17 : float =  -v6
    let v18 : char = '>'
    let v19 : US3 = method12(v5, v7, v17, v18, v0, v1, v2, v4)
    let v20 : char = '<'
    let v21 : US3 = method12(v5, v3, v6, v20, v0, v1, v2, v4)
    let v22 : US3 list = []
    let v24 : US3 list = v21 :: v22 
    let v28 : US3 list = v19 :: v24 
    let v32 : US3 list = v16 :: v28 
    let v36 : US3 list = v14 :: v32 
    let v40 : US3 list = v11 :: v36 
    let v44 : US3 list = v9 :: v40 
    v44
and closure3 (v0 : float, v1 : float, v2 : float, v3 : float, v4 : float) (v5 : float) : (float -> US3 list) =
    closure4(v0, v1, v2, v3, v4, v5)
and method11 (v0 : float, v1 : float, v2 : float, v3 : float, v4 : float) : (float -> (float -> US3 list)) =
    closure3(v0, v1, v2, v3, v4)
and closure5 () (v0 : US3) : struct (int32 * float * char) option =
    match v0 with
    | US3_1 -> (* None *)
        let v9 : struct (int32 * float * char) option = None
        v9
    | US3_0(v1, v2, v3) -> (* Some *)
        let v5 : struct (int32 * float * char) option = Some struct (v1, v2, v3) 
        v5
and method13 () : (US3 -> struct (int32 * float * char) option) =
    closure5()
and closure2 (v0 : float, v1 : float, v2 : float) struct (v3 : float, v4 : float) : struct (int32 * float * char) seq =
    let v5 : float =  -v3
    let v9 : unit = ()
    let _let'_v9 =
        seq {
            let v77 : Mut0 = {l0 = v5} : Mut0
            while method9(v3, v77) do
                let v79 : float = v77.l0
                yield v79 
                let v80 : float = v77.l0
                let v81 : float = v80 + 0.6
                v77.l0 <- v81
                ()
            (* indent
            ()
        indent *)
        }
        (* indent
        ()
    indent *)
    let v147 : float seq = _let'_v9 
    let v151 : float seq = method10(v147)
    let v152 : (float -> (float -> US3 list)) = method11(v0, v1, v2, v3, v4)
    let v153 : (US3 -> struct (int32 * float * char) option) = method13()
    let v157 : unit = ()
    let _let'_v157 =
        seq {
            for cube_x in v151 do
            for cube_y in v151 do
            let x = v152 cube_x cube_y |> Seq.choose v153 
            yield! x
            (* indent
            ()
        indent *)
        }
        (* indent
        ()
    indent *)
    let v160 : struct (int32 * float * char) seq = _let'_v157 
    v160
and method8 (v0 : float, v1 : float, v2 : float) : (struct (float * float) -> struct (int32 * float * char) seq) =
    closure2(v0, v1, v2)
and closure7 (v0 : (US4 []), v1 : int32) struct (v2 : float, v3 : char) : unit =
    let v5 : US4 = v0.[int v1]
    match v5 with
    | US4_0(v8, v9) -> (* Some *)
        let v10 : bool = v8 >= v2
        if v10 then
            ()
        else
            let v11 : US4 = US4_0(v2, v3)
            v0.[v1] <- v11 
            ()
    | _ ->
        let v13 : US4 = US4_0(v2, v3)
        v0.[v1] <- v13 
        ()
and closure6 (v0 : (US4 [])) (v1 : int32) : (struct (float * char) -> unit) =
    closure7(v0, v1)
and method14 () : string =
    let v0 : string = ""
    v0
and closure9 (v0 : (US4 []), v1 : System.Text.StringBuilder, v2 : int32) (v3 : int32) : unit =
    let v4 : int32 = v2 * 160
    let v5 : int32 = v3 + v4
    let v7 : US4 = v0.[int v5]
    let v13 : char =
        match v7 with
        | US4_1 -> (* None *)
            '.'
        | US4_0(v10, v11) -> (* Some *)
            v11
    let v15 : (char -> string) = _.ToString()
    let v16 : string = v15 v13
    let v20 : (string -> System.Text.StringBuilder) = v1.Append
    let v21 : System.Text.StringBuilder = v20 v16
    ()
and closure8 (v0 : (US4 []), v1 : System.Text.StringBuilder) (v2 : int32) : unit =
    let v3 : (int32 -> unit) = closure9(v0, v1, v2)
    for col in 0 .. (160 - 1) do v3 col
    let v11 : System.Text.StringBuilder = v1.AppendLine ()
    ()
and closure10 (v0 : string) () : unit =
    let v1 : (string -> unit) = System.Console.WriteLine
    v1 v0
and method6 (v0 : int32, v1 : int32, v2 : float, v3 : float, v4 : float) : Async<unit> =
    (* run_target_args'
    let v9 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v11 : Async<unit> = null |> unbox<Async<unit>>
    let _run_target_args'_v9 = v11 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v15 : Async<unit> = null |> unbox<Async<unit>>
    let _run_target_args'_v9 = v15 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v19 : Async<unit> = null |> unbox<Async<unit>>
    let _run_target_args'_v9 = v19 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v23 : unit = ()
    let _let'_v23 =
        async {
            let struct (v1196 : float, v1197 : float, v1198 : float) = method7(v2, v3, v4)
            let v1199 : struct (float * float) list = []
            let v1201 : struct (float * float) list = struct (5.0, 40.0) :: v1199 
            let v1205 : struct (float * float) list = struct (10.0, 10.0) :: v1201 
            let v1209 : struct (float * float) list = struct (20.0, -40.0) :: v1205 
            let v1213 : (struct (float * float) -> struct (int32 * float * char) seq) = method8(v1196, v1197, v1198)
            let v1214 : (struct (float * float) list -> struct (int32 * float * char) seq) = Seq.collect v1213 
            let v1215 : struct (int32 * float * char) seq = v1214 v1209
            let v1220 : US4 = US4_1
            let v1221 : (US4 []) = Array.create (160 * 44) v1220 
            let v1225 : (int32 -> (struct (float * char) -> unit)) = closure6(v1221)
            let v1226 : (struct (int32 * float * char) seq -> unit) = Seq.iter (fun (struct (idx, ooz, ch)) -> v1225 idx (ooz, ch))
            v1226 v1215
            let v1228 : string = method14()
            let v1230 : (string -> System.Text.StringBuilder) = System.Text.StringBuilder 
            let v1231 : System.Text.StringBuilder = v1230 v1228
            let v1234 : (int32 -> unit) = closure8(v1221, v1231)
            for row in 0 .. (44 - 1) do v1234 row
            let v1237 : (System.Text.StringBuilder -> string) = _.ToString()
            let v1238 : string = v1237 v1231
            let v1241 : bool = v0 < 0
            if v1241 then
                (* run_target_args'
                let v1242 : unit = ()
                run_target_args' *)
                
#if FABLE_COMPILER || WASM || CONTRACT
                
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
                let v1243 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v1243 
                #endif
#if FABLE_COMPILER_RUST && WASM
                let v1244 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v1244 
                #endif
#if FABLE_COMPILER_RUST && CONTRACT
                let v1245 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v1245 
                #endif
#if FABLE_COMPILER_TYPESCRIPT
                let v1246 : string = $"process.stdout.write('\\u001B[1;1H')"
                Fable.Core.JsInterop.emitJsExpr () v1246 
                #endif
#if FABLE_COMPILER_PYTHON
                let v1247 : string = $"sys.stdout.write(\"\\033[1;1H\")"
                Fable.Core.PyInterop.emitPyExpr () v1247 
                #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
                #endif
#else
                System.Console.SetCursorPosition (0, 0)
                #endif
                // run_target_args' is_unit
                ()
            let v1249 : unit = ()
            let v1250 : (unit -> unit) = closure10(v1238)
            let v1251 : unit = (fun () -> v1250 (); v1249) ()
            (* run_target_args'
            let v1257 : unit = ()
            run_target_args' *)
            
#if FABLE_COMPILER || WASM || CONTRACT
            
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
            let v1258 : (int32 -> Async<unit>) = Async.Sleep
            let v1259 : Async<unit> = v1258 1
            let _run_target_args'_v1257 = v1259 
            #endif
#if FABLE_COMPILER_RUST && WASM
            let v1260 : (int32 -> Async<unit>) = Async.Sleep
            let v1261 : Async<unit> = v1260 1
            let _run_target_args'_v1257 = v1261 
            #endif
#if FABLE_COMPILER_RUST && CONTRACT
            let v1262 : (int32 -> Async<unit>) = Async.Sleep
            let v1263 : Async<unit> = v1262 1
            let _run_target_args'_v1257 = v1263 
            #endif
#if FABLE_COMPILER_TYPESCRIPT
            let v1264 : (int32 -> Async<unit>) = Async.Sleep
            let v1265 : Async<unit> = v1264 1
            let _run_target_args'_v1257 = v1265 
            #endif
#if FABLE_COMPILER_PYTHON
            let v1266 : (int32 -> Async<unit>) = Async.Sleep
            let v1267 : Async<unit> = v1266 1
            let _run_target_args'_v1257 = v1267 
            #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
            let v1268 : (int32 -> Async<unit>) = Async.Sleep
            let v1269 : Async<unit> = v1268 1
            let _run_target_args'_v1257 = v1269 
            #endif
#else
            let v1270 : (int32 -> Async<unit>) = Async.Sleep
            let v1271 : Async<unit> = v1270 1
            let _run_target_args'_v1257 = v1271 
            #endif
            let v1272 : Async<unit> = _run_target_args'_v1257 
            do! v1272 
            let v1278 : bool = v0 > 0
            let v1280 : bool =
                if v1278 then
                    let v1279 : bool = v1 >= v0
                    v1279
                else
                    false
            if v1280 then
                ()
            else
                let v1281 : float = v1196 + 0.05
                let v1282 : float = v1197 + 0.05
                let v1283 : float = v1198 + 0.01
                let v1284 : int32 = v1 + 1
                let v1285 : Async<unit> = method5(v0, v1284, v1281, v1282, v1283)
                return! v1285 
                ()
            ()
            (* indent
            ()
        indent *)
        }
        (* indent
        ()
    indent *)
    let v2456 : Async<unit> = _let'_v23 
    let _run_target_args'_v9 = v2456 
    #endif
#if FABLE_COMPILER_PYTHON
    let v2458 : unit = ()
    let _let'_v2458 =
        async {
            let struct (v3631 : float, v3632 : float, v3633 : float) = method7(v2, v3, v4)
            let v3634 : struct (float * float) list = []
            let v3636 : struct (float * float) list = struct (5.0, 40.0) :: v3634 
            let v3640 : struct (float * float) list = struct (10.0, 10.0) :: v3636 
            let v3644 : struct (float * float) list = struct (20.0, -40.0) :: v3640 
            let v3648 : (struct (float * float) -> struct (int32 * float * char) seq) = method8(v3631, v3632, v3633)
            let v3649 : (struct (float * float) list -> struct (int32 * float * char) seq) = Seq.collect v3648 
            let v3650 : struct (int32 * float * char) seq = v3649 v3644
            let v3655 : US4 = US4_1
            let v3656 : (US4 []) = Array.create (160 * 44) v3655 
            let v3660 : (int32 -> (struct (float * char) -> unit)) = closure6(v3656)
            let v3661 : (struct (int32 * float * char) seq -> unit) = Seq.iter (fun (struct (idx, ooz, ch)) -> v3660 idx (ooz, ch))
            v3661 v3650
            let v3663 : string = method14()
            let v3665 : (string -> System.Text.StringBuilder) = System.Text.StringBuilder 
            let v3666 : System.Text.StringBuilder = v3665 v3663
            let v3669 : (int32 -> unit) = closure8(v3656, v3666)
            for row in 0 .. (44 - 1) do v3669 row
            let v3672 : (System.Text.StringBuilder -> string) = _.ToString()
            let v3673 : string = v3672 v3666
            let v3676 : bool = v0 < 0
            if v3676 then
                (* run_target_args'
                let v3677 : unit = ()
                run_target_args' *)
                
#if FABLE_COMPILER || WASM || CONTRACT
                
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
                let v3678 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v3678 
                #endif
#if FABLE_COMPILER_RUST && WASM
                let v3679 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v3679 
                #endif
#if FABLE_COMPILER_RUST && CONTRACT
                let v3680 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v3680 
                #endif
#if FABLE_COMPILER_TYPESCRIPT
                let v3681 : string = $"process.stdout.write('\\u001B[1;1H')"
                Fable.Core.JsInterop.emitJsExpr () v3681 
                #endif
#if FABLE_COMPILER_PYTHON
                let v3682 : string = $"sys.stdout.write(\"\\033[1;1H\")"
                Fable.Core.PyInterop.emitPyExpr () v3682 
                #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
                #endif
#else
                System.Console.SetCursorPosition (0, 0)
                #endif
                // run_target_args' is_unit
                ()
            let v3684 : unit = ()
            let v3685 : (unit -> unit) = closure10(v3673)
            let v3686 : unit = (fun () -> v3685 (); v3684) ()
            (* run_target_args'
            let v3692 : unit = ()
            run_target_args' *)
            
#if FABLE_COMPILER || WASM || CONTRACT
            
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
            let v3693 : (int32 -> Async<unit>) = Async.Sleep
            let v3694 : Async<unit> = v3693 1
            let _run_target_args'_v3692 = v3694 
            #endif
#if FABLE_COMPILER_RUST && WASM
            let v3695 : (int32 -> Async<unit>) = Async.Sleep
            let v3696 : Async<unit> = v3695 1
            let _run_target_args'_v3692 = v3696 
            #endif
#if FABLE_COMPILER_RUST && CONTRACT
            let v3697 : (int32 -> Async<unit>) = Async.Sleep
            let v3698 : Async<unit> = v3697 1
            let _run_target_args'_v3692 = v3698 
            #endif
#if FABLE_COMPILER_TYPESCRIPT
            let v3699 : (int32 -> Async<unit>) = Async.Sleep
            let v3700 : Async<unit> = v3699 1
            let _run_target_args'_v3692 = v3700 
            #endif
#if FABLE_COMPILER_PYTHON
            let v3701 : (int32 -> Async<unit>) = Async.Sleep
            let v3702 : Async<unit> = v3701 1
            let _run_target_args'_v3692 = v3702 
            #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
            let v3703 : (int32 -> Async<unit>) = Async.Sleep
            let v3704 : Async<unit> = v3703 1
            let _run_target_args'_v3692 = v3704 
            #endif
#else
            let v3705 : (int32 -> Async<unit>) = Async.Sleep
            let v3706 : Async<unit> = v3705 1
            let _run_target_args'_v3692 = v3706 
            #endif
            let v3707 : Async<unit> = _run_target_args'_v3692 
            do! v3707 
            let v3713 : bool = v0 > 0
            let v3715 : bool =
                if v3713 then
                    let v3714 : bool = v1 >= v0
                    v3714
                else
                    false
            if v3715 then
                ()
            else
                let v3716 : float = v3631 + 0.05
                let v3717 : float = v3632 + 0.05
                let v3718 : float = v3633 + 0.01
                let v3719 : int32 = v1 + 1
                let v3720 : Async<unit> = method5(v0, v3719, v3716, v3717, v3718)
                return! v3720 
                ()
            ()
            (* indent
            ()
        indent *)
        }
        (* indent
        ()
    indent *)
    let v4891 : Async<unit> = _let'_v2458 
    let _run_target_args'_v9 = v4891 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v4893 : unit = ()
    let _let'_v4893 =
        async {
            let struct (v6066 : float, v6067 : float, v6068 : float) = method7(v2, v3, v4)
            let v6069 : struct (float * float) list = []
            let v6071 : struct (float * float) list = struct (5.0, 40.0) :: v6069 
            let v6075 : struct (float * float) list = struct (10.0, 10.0) :: v6071 
            let v6079 : struct (float * float) list = struct (20.0, -40.0) :: v6075 
            let v6083 : (struct (float * float) -> struct (int32 * float * char) seq) = method8(v6066, v6067, v6068)
            let v6084 : (struct (float * float) list -> struct (int32 * float * char) seq) = Seq.collect v6083 
            let v6085 : struct (int32 * float * char) seq = v6084 v6079
            let v6090 : US4 = US4_1
            let v6091 : (US4 []) = Array.create (160 * 44) v6090 
            let v6095 : (int32 -> (struct (float * char) -> unit)) = closure6(v6091)
            let v6096 : (struct (int32 * float * char) seq -> unit) = Seq.iter (fun (struct (idx, ooz, ch)) -> v6095 idx (ooz, ch))
            v6096 v6085
            let v6098 : string = method14()
            let v6100 : (string -> System.Text.StringBuilder) = System.Text.StringBuilder 
            let v6101 : System.Text.StringBuilder = v6100 v6098
            let v6104 : (int32 -> unit) = closure8(v6091, v6101)
            for row in 0 .. (44 - 1) do v6104 row
            let v6107 : (System.Text.StringBuilder -> string) = _.ToString()
            let v6108 : string = v6107 v6101
            let v6111 : bool = v0 < 0
            if v6111 then
                (* run_target_args'
                let v6112 : unit = ()
                run_target_args' *)
                
#if FABLE_COMPILER || WASM || CONTRACT
                
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
                let v6113 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v6113 
                #endif
#if FABLE_COMPILER_RUST && WASM
                let v6114 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v6114 
                #endif
#if FABLE_COMPILER_RUST && CONTRACT
                let v6115 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v6115 
                #endif
#if FABLE_COMPILER_TYPESCRIPT
                let v6116 : string = $"process.stdout.write('\\u001B[1;1H')"
                Fable.Core.JsInterop.emitJsExpr () v6116 
                #endif
#if FABLE_COMPILER_PYTHON
                let v6117 : string = $"sys.stdout.write(\"\\033[1;1H\")"
                Fable.Core.PyInterop.emitPyExpr () v6117 
                #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
                #endif
#else
                System.Console.SetCursorPosition (0, 0)
                #endif
                // run_target_args' is_unit
                ()
            let v6119 : unit = ()
            let v6120 : (unit -> unit) = closure10(v6108)
            let v6121 : unit = (fun () -> v6120 (); v6119) ()
            (* run_target_args'
            let v6127 : unit = ()
            run_target_args' *)
            
#if FABLE_COMPILER || WASM || CONTRACT
            
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
            let v6128 : (int32 -> Async<unit>) = Async.Sleep
            let v6129 : Async<unit> = v6128 1
            let _run_target_args'_v6127 = v6129 
            #endif
#if FABLE_COMPILER_RUST && WASM
            let v6130 : (int32 -> Async<unit>) = Async.Sleep
            let v6131 : Async<unit> = v6130 1
            let _run_target_args'_v6127 = v6131 
            #endif
#if FABLE_COMPILER_RUST && CONTRACT
            let v6132 : (int32 -> Async<unit>) = Async.Sleep
            let v6133 : Async<unit> = v6132 1
            let _run_target_args'_v6127 = v6133 
            #endif
#if FABLE_COMPILER_TYPESCRIPT
            let v6134 : (int32 -> Async<unit>) = Async.Sleep
            let v6135 : Async<unit> = v6134 1
            let _run_target_args'_v6127 = v6135 
            #endif
#if FABLE_COMPILER_PYTHON
            let v6136 : (int32 -> Async<unit>) = Async.Sleep
            let v6137 : Async<unit> = v6136 1
            let _run_target_args'_v6127 = v6137 
            #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
            let v6138 : (int32 -> Async<unit>) = Async.Sleep
            let v6139 : Async<unit> = v6138 1
            let _run_target_args'_v6127 = v6139 
            #endif
#else
            let v6140 : (int32 -> Async<unit>) = Async.Sleep
            let v6141 : Async<unit> = v6140 1
            let _run_target_args'_v6127 = v6141 
            #endif
            let v6142 : Async<unit> = _run_target_args'_v6127 
            do! v6142 
            let v6148 : bool = v0 > 0
            let v6150 : bool =
                if v6148 then
                    let v6149 : bool = v1 >= v0
                    v6149
                else
                    false
            if v6150 then
                ()
            else
                let v6151 : float = v6066 + 0.05
                let v6152 : float = v6067 + 0.05
                let v6153 : float = v6068 + 0.01
                let v6154 : int32 = v1 + 1
                let v6155 : Async<unit> = method5(v0, v6154, v6151, v6152, v6153)
                return! v6155 
                ()
            ()
            (* indent
            ()
        indent *)
        }
        (* indent
        ()
    indent *)
    let v7326 : Async<unit> = _let'_v4893 
    let _run_target_args'_v9 = v7326 
    #endif
#else
    let v7328 : unit = ()
    let _let'_v7328 =
        async {
            let struct (v8501 : float, v8502 : float, v8503 : float) = method7(v2, v3, v4)
            let v8504 : struct (float * float) list = []
            let v8506 : struct (float * float) list = struct (5.0, 40.0) :: v8504 
            let v8510 : struct (float * float) list = struct (10.0, 10.0) :: v8506 
            let v8514 : struct (float * float) list = struct (20.0, -40.0) :: v8510 
            let v8518 : (struct (float * float) -> struct (int32 * float * char) seq) = method8(v8501, v8502, v8503)
            let v8519 : (struct (float * float) list -> struct (int32 * float * char) seq) = Seq.collect v8518 
            let v8520 : struct (int32 * float * char) seq = v8519 v8514
            let v8525 : US4 = US4_1
            let v8526 : (US4 []) = Array.create (160 * 44) v8525 
            let v8530 : (int32 -> (struct (float * char) -> unit)) = closure6(v8526)
            let v8531 : (struct (int32 * float * char) seq -> unit) = Seq.iter (fun (struct (idx, ooz, ch)) -> v8530 idx (ooz, ch))
            v8531 v8520
            let v8533 : string = method14()
            let v8535 : (string -> System.Text.StringBuilder) = System.Text.StringBuilder 
            let v8536 : System.Text.StringBuilder = v8535 v8533
            let v8539 : (int32 -> unit) = closure8(v8526, v8536)
            for row in 0 .. (44 - 1) do v8539 row
            let v8542 : (System.Text.StringBuilder -> string) = _.ToString()
            let v8543 : string = v8542 v8536
            let v8546 : bool = v0 < 0
            if v8546 then
                (* run_target_args'
                let v8547 : unit = ()
                run_target_args' *)
                
#if FABLE_COMPILER || WASM || CONTRACT
                
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
                let v8548 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v8548 
                #endif
#if FABLE_COMPILER_RUST && WASM
                let v8549 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v8549 
                #endif
#if FABLE_COMPILER_RUST && CONTRACT
                let v8550 : string = $"print!(\"\\x1B[1;1H\")"
                Fable.Core.RustInterop.emitRustExpr () v8550 
                #endif
#if FABLE_COMPILER_TYPESCRIPT
                let v8551 : string = $"process.stdout.write('\\u001B[1;1H')"
                Fable.Core.JsInterop.emitJsExpr () v8551 
                #endif
#if FABLE_COMPILER_PYTHON
                let v8552 : string = $"sys.stdout.write(\"\\033[1;1H\")"
                Fable.Core.PyInterop.emitPyExpr () v8552 
                #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
                #endif
#else
                System.Console.SetCursorPosition (0, 0)
                #endif
                // run_target_args' is_unit
                ()
            let v8554 : unit = ()
            let v8555 : (unit -> unit) = closure10(v8543)
            let v8556 : unit = (fun () -> v8555 (); v8554) ()
            (* run_target_args'
            let v8562 : unit = ()
            run_target_args' *)
            
#if FABLE_COMPILER || WASM || CONTRACT
            
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
            let v8563 : (int32 -> Async<unit>) = Async.Sleep
            let v8564 : Async<unit> = v8563 1
            let _run_target_args'_v8562 = v8564 
            #endif
#if FABLE_COMPILER_RUST && WASM
            let v8565 : (int32 -> Async<unit>) = Async.Sleep
            let v8566 : Async<unit> = v8565 1
            let _run_target_args'_v8562 = v8566 
            #endif
#if FABLE_COMPILER_RUST && CONTRACT
            let v8567 : (int32 -> Async<unit>) = Async.Sleep
            let v8568 : Async<unit> = v8567 1
            let _run_target_args'_v8562 = v8568 
            #endif
#if FABLE_COMPILER_TYPESCRIPT
            let v8569 : (int32 -> Async<unit>) = Async.Sleep
            let v8570 : Async<unit> = v8569 1
            let _run_target_args'_v8562 = v8570 
            #endif
#if FABLE_COMPILER_PYTHON
            let v8571 : (int32 -> Async<unit>) = Async.Sleep
            let v8572 : Async<unit> = v8571 1
            let _run_target_args'_v8562 = v8572 
            #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
            let v8573 : (int32 -> Async<unit>) = Async.Sleep
            let v8574 : Async<unit> = v8573 1
            let _run_target_args'_v8562 = v8574 
            #endif
#else
            let v8575 : (int32 -> Async<unit>) = Async.Sleep
            let v8576 : Async<unit> = v8575 1
            let _run_target_args'_v8562 = v8576 
            #endif
            let v8577 : Async<unit> = _run_target_args'_v8562 
            do! v8577 
            let v8583 : bool = v0 > 0
            let v8585 : bool =
                if v8583 then
                    let v8584 : bool = v1 >= v0
                    v8584
                else
                    false
            if v8585 then
                ()
            else
                let v8586 : float = v8501 + 0.05
                let v8587 : float = v8502 + 0.05
                let v8588 : float = v8503 + 0.01
                let v8589 : int32 = v1 + 1
                let v8590 : Async<unit> = method5(v0, v8589, v8586, v8587, v8588)
                return! v8590 
                ()
            ()
            (* indent
            ()
        indent *)
        }
        (* indent
        ()
    indent *)
    let v9761 : Async<unit> = _let'_v7328 
    let _run_target_args'_v9 = v9761 
    #endif
    let v9762 : Async<unit> = _run_target_args'_v9 
    v9762
and method5 (v0 : int32, v1 : int32, v2 : float, v3 : float, v4 : float) : Async<unit> =
    method6(v0, v1, v2, v3, v4)
and closure0 () (v0 : (string [])) : unit =
    (* run_target_args'
    let v10 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v11 : string = method0()
    let v12 : string = method1(v11)
    let v13 : int32 = v12.Length
    let v14 : bool = 0 = v13
    let v19 : bool =
        if v14 then
            let v15 : string = "AUTOMATION"
            let v16 : string = method1(v15)
            let v17 : int32 = v16.Length
            let v18 : bool = 0 = v17
            v18
        else
            false
    let _run_target_args'_v10 = v19 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v20 : string = method0()
    let v21 : string = method1(v20)
    let v22 : int32 = v21.Length
    let v23 : bool = 0 = v22
    let v28 : bool =
        if v23 then
            let v24 : string = "AUTOMATION"
            let v25 : string = method1(v24)
            let v26 : int32 = v25.Length
            let v27 : bool = 0 = v26
            v27
        else
            false
    let _run_target_args'_v10 = v28 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v29 : string = method0()
    let v30 : string = method1(v29)
    let v31 : int32 = v30.Length
    let v32 : bool = 0 = v31
    let v37 : bool =
        if v32 then
            let v33 : string = "AUTOMATION"
            let v34 : string = method1(v33)
            let v35 : int32 = v34.Length
            let v36 : bool = 0 = v35
            v36
        else
            false
    let _run_target_args'_v10 = v37 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v38 : string = method0()
    let v39 : string = method1(v38)
    let v40 : int32 = v39.Length
    let v41 : bool = 0 = v40
    let v46 : bool =
        if v41 then
            let v42 : string = "AUTOMATION"
            let v43 : string = method1(v42)
            let v44 : int32 = v43.Length
            let v45 : bool = 0 = v44
            v45
        else
            false
    let _run_target_args'_v10 = v46 
    #endif
#if FABLE_COMPILER_PYTHON
    let v47 : string = method0()
    let v48 : string = method1(v47)
    let v49 : int32 = v48.Length
    let v50 : bool = 0 = v49
    let v55 : bool =
        if v50 then
            let v51 : string = "AUTOMATION"
            let v52 : string = method1(v51)
            let v53 : int32 = v52.Length
            let v54 : bool = 0 = v53
            v54
        else
            false
    let _run_target_args'_v10 = v55 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let _run_target_args'_v10 = false 
    #endif
#else
    let v56 : string = method0()
    let v57 : string = method1(v56)
    let v58 : int32 = v57.Length
    let v59 : bool = 0 = v58
    let v64 : bool =
        if v59 then
            let v60 : string = "AUTOMATION"
            let v61 : string = method1(v60)
            let v62 : int32 = v61.Length
            let v63 : bool = 0 = v62
            v63
        else
            false
    let _run_target_args'_v10 = v64 
    #endif
    let v65 : bool = _run_target_args'_v10 
    if v65 then
        (* run_target_args'
        let v84 : unit = ()
        run_target_args' *)
        
#if FABLE_COMPILER || WASM || CONTRACT
        
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
        let v85 : string = $"print!(\"\\x1B[?25l\")"
        Fable.Core.RustInterop.emitRustExpr () v85 
        #endif
#if FABLE_COMPILER_RUST && WASM
        let v86 : string = $"print!(\"\\x1B[?25l\")"
        Fable.Core.RustInterop.emitRustExpr () v86 
        #endif
#if FABLE_COMPILER_RUST && CONTRACT
        let v87 : string = $"print!(\"\\x1B[?25l\")"
        Fable.Core.RustInterop.emitRustExpr () v87 
        #endif
#if FABLE_COMPILER_TYPESCRIPT
        let v88 : string = $"process.stdout.write('\\u001B[?25l')"
        Fable.Core.JsInterop.emitJsExpr () v88 
        #endif
#if FABLE_COMPILER_PYTHON
        let v89 : string = "sys"
        Fable.Core.PyInterop.importAll v89 
        let v90 : string = $"sys.stdout.write(\"\\033[?25l\")"
        Fable.Core.PyInterop.emitPyExpr () v90 
        #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
        #endif
#else
        System.Console.CursorVisible <- false
        #endif
        // run_target_args' is_unit
        ()
    let v91 : int32 =
        if v65 then
            -1
        else
            50
    let v92 : int32 = 1
    let v93 : float = 0.0
    let v94 : float = 0.0
    let v95 : float = 0.0
    let v96 : Async<unit> = method5(v91, v92, v93, v94, v95)
    (* run_target_args'
    let v101 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    (* run_target_args'
    let v102 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && WASM
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_PYTHON
    let v103 : (Async<unit> -> unit) = Async.RunSynchronously
    v103 v96
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v104 : (Async<unit> -> unit) = Async.RunSynchronously
    v104 v96
    #endif
#else
    let v105 : (Async<unit> -> unit) = Async.RunSynchronously
    v105 v96
    #endif
    // run_target_args' is_unit
    #endif
#if FABLE_COMPILER_RUST && WASM
    (* run_target_args'
    let v106 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && WASM
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_PYTHON
    let v107 : (Async<unit> -> unit) = Async.RunSynchronously
    v107 v96
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v108 : (Async<unit> -> unit) = Async.RunSynchronously
    v108 v96
    #endif
#else
    let v109 : (Async<unit> -> unit) = Async.RunSynchronously
    v109 v96
    #endif
    // run_target_args' is_unit
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    (* run_target_args'
    let v110 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && WASM
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_PYTHON
    let v111 : (Async<unit> -> unit) = Async.RunSynchronously
    v111 v96
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v112 : (Async<unit> -> unit) = Async.RunSynchronously
    v112 v96
    #endif
#else
    let v113 : (Async<unit> -> unit) = Async.RunSynchronously
    v113 v96
    #endif
    // run_target_args' is_unit
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    (* run_target_args'
    let v118 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v120 : Async<Async<unit>> = null |> unbox<Async<Async<unit>>>
    let _run_target_args'_v118 = v120 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v124 : Async<Async<unit>> = null |> unbox<Async<Async<unit>>>
    let _run_target_args'_v118 = v124 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v128 : Async<Async<unit>> = null |> unbox<Async<Async<unit>>>
    let _run_target_args'_v118 = v128 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v131 : (Async<unit> -> Async<Async<unit>>) = Async.StartChild
    let v132 : Async<Async<unit>> = v131 v96
    let _run_target_args'_v118 = v132 
    #endif
#if FABLE_COMPILER_PYTHON
    let v133 : (Async<unit> -> Async<Async<unit>>) = Async.StartChild
    let v134 : Async<Async<unit>> = v133 v96
    let _run_target_args'_v118 = v134 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v135 : (Async<unit> -> Async<Async<unit>>) = Async.StartChild
    let v136 : Async<Async<unit>> = v135 v96
    let _run_target_args'_v118 = v136 
    #endif
#else
    let v137 : (Async<unit> -> Async<Async<unit>>) = Async.StartChild
    let v138 : Async<Async<unit>> = v137 v96
    let _run_target_args'_v118 = v138 
    #endif
    let v139 : Async<Async<unit>> = _run_target_args'_v118 
    #endif
#if FABLE_COMPILER_PYTHON
    (* run_target_args'
    let v148 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && WASM
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v149 : (Async<unit> -> unit) = Async.StartImmediate
    v149 v96
    #endif
#if FABLE_COMPILER_PYTHON
    let v150 : (Async<unit> -> unit) = Async.StartImmediate
    v150 v96
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v151 : (Async<unit> -> unit) = Async.StartImmediate
    v151 v96
    #endif
#else
    let v152 : (Async<unit> -> unit) = Async.StartImmediate
    v152 v96
    #endif
    // run_target_args' is_unit
    (* run_target_args'
    let v153 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v154 : (int32 -> unit) = System.Threading.Thread.Sleep
    v154 2000
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v155 : (int32 -> unit) = System.Threading.Thread.Sleep
    v155 2000
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v156 : (int32 -> unit) = System.Threading.Thread.Sleep
    v156 2000
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    #endif
#if FABLE_COMPILER_PYTHON
    let v157 : (int32 -> unit) = System.Threading.Thread.Sleep
    v157 2000
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    #endif
#else
    let v158 : (int32 -> unit) = System.Threading.Thread.Sleep
    v158 2000
    #endif
    // run_target_args' is_unit
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    (* run_target_args'
    let v163 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    let v165 : Async<Async<unit>> = null |> unbox<Async<Async<unit>>>
    let _run_target_args'_v163 = v165 
    #endif
#if FABLE_COMPILER_RUST && WASM
    let v169 : Async<Async<unit>> = null |> unbox<Async<Async<unit>>>
    let _run_target_args'_v163 = v169 
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    let v173 : Async<Async<unit>> = null |> unbox<Async<Async<unit>>>
    let _run_target_args'_v163 = v173 
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    let v176 : (Async<unit> -> Async<Async<unit>>) = Async.StartChild
    let v177 : Async<Async<unit>> = v176 v96
    let _run_target_args'_v163 = v177 
    #endif
#if FABLE_COMPILER_PYTHON
    let v178 : (Async<unit> -> Async<Async<unit>>) = Async.StartChild
    let v179 : Async<Async<unit>> = v178 v96
    let _run_target_args'_v163 = v179 
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v180 : (Async<unit> -> Async<Async<unit>>) = Async.StartChild
    let v181 : Async<Async<unit>> = v180 v96
    let _run_target_args'_v163 = v181 
    #endif
#else
    let v182 : (Async<unit> -> Async<Async<unit>>) = Async.StartChild
    let v183 : Async<Async<unit>> = v182 v96
    let _run_target_args'_v163 = v183 
    #endif
    let v184 : Async<Async<unit>> = _run_target_args'_v163 
    #endif
#else
    (* run_target_args'
    let v193 : unit = ()
    run_target_args' *)
    
#if FABLE_COMPILER || WASM || CONTRACT
    
#if FABLE_COMPILER_RUST && !WASM && !CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && WASM
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_RUST && CONTRACT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_TYPESCRIPT
    null |> unbox<unit>
    #endif
#if FABLE_COMPILER_PYTHON
    let v194 : (Async<unit> -> unit) = Async.RunSynchronously
    v194 v96
    #endif
#if !FABLE_COMPILER_RUST && !FABLE_COMPILER_TYPESCRIPT && !FABLE_COMPILER_PYTHON
    let v195 : (Async<unit> -> unit) = Async.RunSynchronously
    v195 v96
    #endif
#else
    let v196 : (Async<unit> -> unit) = Async.RunSynchronously
    v196 v96
    #endif
    // run_target_args' is_unit
    #endif
    // run_target_args' is_unit
    ()
let v0 : ((string []) -> unit) = closure0()
let main_ = v0 
#if !FABLE_COMPILER_RUST
main_ [||]
#else
let main args = main_ [||]; 0
#endif
()
