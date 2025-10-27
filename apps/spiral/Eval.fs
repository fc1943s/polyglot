#if !INTERACTIVE
namespace Polyglot
#endif

module Eval =

#if _LINUX
#else
#endif

#if _LINUX
#else
#endif

#if _LINUX
#else
#endif

#if _LINUX
#else
#endif

#if _LINUX
#else
#endif

#if _LINUX
#else
#endif

#if _LINUX
#else
#endif

#if _LINUX
#else
#endif

#if _LINUX
#else
#endif

    open spiral_compiler

#if !INTERACTIVE
    open Lib
#endif

    open Common
    open SpiralFileSystem.Operators
    open Microsoft.AspNetCore.SignalR.Client

    open System
    open System.Collections.Generic
    open System.IO
    open System.Text
    open System.Threading

    /// ## mapErrors
    let inline mapErrors (severity, errors, lastTopLevelIndex) allCode =
        let allCodeLineLength =
            allCode |> SpiralSm.split "\n" |> Array.length

        errors
        |> List.map (fun (_, error) ->
            match error with
            | FatalError message ->
                (
                    severity, message, 0, ("", (0, 0), (0, 0))
                )
                |> List.singleton
            | TracedError data ->
                data.trace
                |> List.truncate 5
                |> List.append [ data.message ]
                |> List.map (fun message ->
                    (
                        severity, message, 0, ("", (0, 0), (0, 0))
                    )
                )
            | PackageErrors data
            | TokenizerErrors data
            | ParserErrors data
            | TypeErrors data ->
                data.errors
                |> List.filter (fun ((rangeStart, _), _) ->
                    trace Debug (fun () -> $"Eval.mapErrors / rangeStart.line: {rangeStart.line} / lastTopLevelIndex: {lastTopLevelIndex} / allCodeLineLength: {allCodeLineLength} / filtered: {rangeStart.line > allCodeLineLength}") _locals
                    rangeStart.line > allCodeLineLength
                )
                |> List.map (fun ((rangeStart, rangeEnd), message) ->
                    (
                        severity,
                        message,
                        0,
                        (
                            (data.uri |> System.IO.Path.GetFileName),
                            (
                                (match lastTopLevelIndex with
                                | Some i when rangeStart.line >= i + allCodeLineLength + 3 ->
                                    rangeStart.line - allCodeLineLength - 2
                                | _ -> rangeStart.line - allCodeLineLength),
                                (match lastTopLevelIndex with
                                | Some i when rangeStart.line >= i + allCodeLineLength + 3 ->
                                    rangeStart.character - 4
                                | _ -> rangeStart.character)
                            ),
                            (
                                (match lastTopLevelIndex with
                                | Some i when rangeStart.line >= i + allCodeLineLength + 3 ->
                                    rangeEnd.line - allCodeLineLength - 2
                                | _ -> rangeEnd.line - allCodeLineLength),
                                (match lastTopLevelIndex with
                                | Some i when rangeStart.line >= i + allCodeLineLength + 3 ->
                                    rangeEnd.character - 4
                                | _ -> rangeEnd.character)
                            )
                        )
                    )
                )
        )
        |> List.collect id
        |> List.toArray

    /// ### workspaceRoot
    let workspaceRoot = SpiralFileSystem.get_workspace_root ()

    /// ### targetDir
    let targetDir = workspaceRoot </> "target/spiral_Eval"
    [ targetDir ]
    |> List.iter (fun dir -> if Directory.Exists dir |> not then Directory.CreateDirectory dir |> ignore)

    /// ## allCode
    let mutable allCode = ""

    /// ### allPackages
    let mutable allPackages : string [] = [||]

    /// ## allCodeReal
    let mutable allCodeReal = ""

    /// ## traceToggle
    let mutable traceToggle = false

    /// ## startTokenRangeWatcher
    let inline startTokenRangeWatcher () =
        if [ "dotnet-repl" ] |> List.contains spiral_compiler.assemblyName
        then new_disposable (fun () -> ())
        else
            let tokensDir = targetDir </> "tokens"

            [ tokensDir ]
            |> List.iter (fun dir -> if Directory.Exists dir |> not then Directory.CreateDirectory dir |> ignore)

            let stream, disposable = FileSystem.watchDirectory (fun _ -> false) tokensDir

            try
                let existingFilesChild =
                    tokensDir
                    |> System.IO.Directory.GetDirectories
                    |> Array.map (fun codeDir -> async {
                        try
                            let tokensPath = codeDir </> "tokens.json"
                            if tokensPath |> File.Exists |> not then
                                let spiralCodePath = codeDir </> "main.spi"
                                let spiralRealCodePath = codeDir </> "main_real.spir"
                                let spiralExists = spiralCodePath |> System.IO.File.Exists
                                let spiralRealExists = spiralRealCodePath |> System.IO.File.Exists
                                if spiralExists |> not && spiralRealExists |> not
                                then do! codeDir |> SpiralFileSystem.delete_directory_async |> Async.Ignore
                                else
                                    let! tokens =
                                        if spiralExists then spiralCodePath else spiralRealCodePath
                                        |> Supervisor.getFileTokenRange None None
                                    match tokens with
                                    | Some tokens ->
                                        do!
                                            tokens
                                            |> FSharp.Json.Json.serialize
                                            |> SpiralFileSystem.write_all_text_async tokensPath
                                    | None ->
                                        trace Verbose (fun () -> $"Eval.startTokenRangeWatcher / GetDirectories / tokens: None") _locals
                        with ex ->
                            trace Critical (fun () -> $"Eval.startTokenRangeWatcher / GetDirectories / ex: {ex |> SpiralSm.format_exception}") _locals
                    })
                    |> Async.Parallel
                    |> Async.Ignore

                let streamAsyncChild =
                    stream
                    |> FSharp.Control.AsyncSeq.iterAsyncParallel (fun (ticks, event) ->
                        match event with
                        | FileSystem.FileSystemChange.Changed (codePath, _)
                            when [ "main.spi"; "main_real.spir" ]
                                |> List.contains (System.IO.Path.GetFileName codePath)
                            ->
                            async {
                                let hashDir = codePath |> System.IO.Directory.GetParent
                                let hashHex = hashDir.Name
                                let codePath = tokensDir </> codePath
                                let tokensPath = tokensDir </> hashHex </> "tokens.json"
                                // do! Async.Sleep 30
                                let rec 루프 retry = async {
                                    let! tokens = codePath |> Supervisor.getFileTokenRange None None
                                    if retry = 3 || tokens <> Some [||]
                                    then return tokens, retry
                                    else
                                        trace Debug
                                            (fun () -> $"Eval.startTokenRangeWatcher / iterAsyncParallel")
                                            (fun () -> $"retry: {retry} / tokens: %A{tokens}")
                                        do! Async.Sleep 30
                                        return! 루프 (retry + 1)
                                }
                                let! tokens, retries = 루프 1
                                match tokens with
                                | Some tokens ->
                                    do!
                                        tokens
                                        |> FSharp.Json.Json.serialize
                                        |> SpiralFileSystem.write_all_text_exists tokensPath
                                | None ->
                                    trace Debug
                                        (fun () -> $"Eval.startTokenRangeWatcher / iterAsyncParallel")
                                        (fun () -> $"retries: {retries} / tokens: {tokens}")
                            }
                            |> Async.retryAsync 3
                            |> Async.map (Result.toOption >> Option.defaultValue ())
                        | _ -> () |> Async.init
                    )

                async {
                    do! Async.Sleep 3000
                    existingFilesChild |> Async.StartImmediate
                    streamAsyncChild |> Async.Start
                }
                |> Async.Start
            with ex ->
                trace Critical (fun () -> $"Eval.startTokenRangeWatcher / ex: {ex |> SpiralSm.format_exception}") _locals

            disposable

    /// ## startCommandsWatcher
    let startCommandsWatcher (uriServer : string) =
        let commandsDir = targetDir </> "eval_commands"
        let commandHistoryDir = targetDir </> "eval_command_history"
        [ commandsDir; commandHistoryDir ]
        |> List.iter (fun dir -> if Directory.Exists dir |> not then Directory.CreateDirectory dir |> ignore)

        Directory.EnumerateFiles commandsDir |> Seq.iter File.Delete

        let stream, disposable =
            commandsDir
            |> FileSystem.watchDirectory (function
                | FileSystem.FileSystemChange.Created _ -> true
                | _ -> false
            )

        let connection = HubConnectionBuilder().WithUrl(uriServer).Build()
        connection.StartAsync() |> Async.AwaitTask |> Async.Start
        // let _ = connection.On<string>("ServerToClientMsg", fun x ->
        //     printfn $"ServerToClientMsg: '{x}'"
        // )

        stream
        |> FSharp.Control.AsyncSeq.iterAsyncParallel (fun (ticks, event) -> async {
            let _locals () = $"ticks: {ticks} / event: {event} / {_locals ()}"
            trace Verbose (fun () -> "Eval.startCommandsWatcher / iterAsyncParallel") _locals

            match event with
            | FileSystem.FileSystemChange.Created (path, Some json) ->
                try
                    let fullPath = commandsDir </> path
                    let! result = connection.InvokeAsync<string>("ClientToServerMsg", json) |> Async.AwaitTask
                    let commandHistoryPath = commandHistoryDir </> path
                    do! fullPath |> SpiralFileSystem.move_file_async commandHistoryPath |> Async.Ignore
                    if result |> SpiralSm.trim |> String.length > 0 then
                        let resultPath = commandHistoryDir </> $"{Path.GetFileNameWithoutExtension path}_result.json"
                        do! result |> SpiralFileSystem.write_all_text_async resultPath
                with ex ->
                    let _locals () = $"ex: {ex |> SpiralSm.format_exception} / {_locals ()}"
                    trace Critical (fun () -> "Eval.startCommandsWatcher / iterAsyncParallel") _locals
            | _ -> ()
        })
        |> Async.StartChild
        |> Async.Ignore
        |> Async.Start

        new_disposable (fun () ->
            disposable.Dispose ()
        )

    /// ## prepareSpiral
    let prepareSpiral rawCellCode lines =
        let lastBlock =
            lines
            |> Array.tryFindBack (fun line ->
                line |> String.length > 0
                && line.[0] <> ' '
            )

        let hasMain =
            lastBlock
            |> Option.exists (fun line ->
                line |> SpiralSm.starts_with "inl main "
                || line |> SpiralSm.starts_with "let main "
            )

        if hasMain
        then rawCellCode, None
        else
            let lastTopLevelIndex, _ =
                (lines |> Array.indexed, (None, false))
                ||> Array.foldBack (fun (i, line) (lastTopLevelIndex, finished) ->
                    // trace Verbose (fun () -> $"Eval.prepareSpiral / i: {i} / line: '{line}' / lastTopLevelIndex: {lastTopLevelIndex} / finished: {finished}") _locals
                    match line with
                    | _ when finished -> lastTopLevelIndex, true
                    | "" -> lastTopLevelIndex, false
                    | line when
                        line |> SpiralSm.starts_with " "
                        || line |> SpiralSm.starts_with "// " -> lastTopLevelIndex, false
                    | line when
                        line |> SpiralSm.starts_with "open "
                        || line |> SpiralSm.starts_with "prototype "
                        || line |> SpiralSm.starts_with "instance "
                        || line |> SpiralSm.starts_with "type "
                        || line |> SpiralSm.starts_with "union "
                        || line |> SpiralSm.starts_with "nominal " -> lastTopLevelIndex, true
                    | line when
                        line |> SpiralSm.starts_with "inl "
                        || line |> SpiralSm.starts_with "and "
                        || line |> SpiralSm.starts_with "let " ->
                        let m =
                            System.Text.RegularExpressions.Regex.Match (
                                line,
                                @"^(?:and +)?(inl|let) +((?:[{( ]*)?[~\(\w]+[\w\d']*(?:|[\w\d']+[ }]*(?:&? *[\w\d']*\))?| *[~\w][\w\d']*\)|, *[~\w][\w\d']*)) +[:=](?! +function)"
                            )
                        trace Verbose (fun () -> $"Eval.prepareSpi / m: '{m}' / m.Groups.Count: {m.Groups.Count}") _locals
                        if m.Groups.Count = 3
                        then Some i, false
                        else lastTopLevelIndex, true
                    | _ -> Some i, false
                )
            let code =
                match lastTopLevelIndex with
                | Some lastTopLevelIndex ->
                    lines
                    |> Array.mapi (fun i line ->
                        match i with
                        | i when i < lastTopLevelIndex -> line
                        | i when i = lastTopLevelIndex -> $"\nlet main () =\n    {line}"
                        | _ when line |> SpiralSm.trim = "" -> ""
                        | _ -> $"    {line}"
                    )
                    |> SpiralSm.concat "\n"
                | None -> $"{rawCellCode}\n\ninl main () = ()\n"
            code, lastTopLevelIndex

    /// ## processSpiralOutput
    let processSpiralOutput
        (props : {|
            printCode: bool
            traceLevel: TraceLevel
            builderCommands: string array
            lastTopLevelIndex: int option
            backend: Supervisor.Backend
            cancellationToken: _
            spiralErrors: _
            code: string
            outputPath: string
            isReal: bool
        |})
        = async {
        let inline _trace (fn : unit -> string) =
            if props.traceLevel = Verbose
            then trace Info (fun () -> $"Eval.processSpiralOutput / props: {props |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400} / {fn ()}") _locals
            else fn () |> System.Console.WriteLine

        if props.printCode
            && props.backend <> Supervisor.Gleam
            && props.backend <> Supervisor.Python
            && props.backend <> Supervisor.Cpp then
            let ext = props.outputPath |> System.IO.Path.GetExtension
            _trace (fun () -> if props.builderCommands.Length > 0 then $"{ext}:\n{props.code}\n" else props.code)

        let workspaceRootExternal =
            let currentDir = System.IO.Directory.GetCurrentDirectory () |> SpiralSm.to_lower
            let workspaceRoot = workspaceRoot |> SpiralSm.to_lower
            if currentDir |> SpiralSm.starts_with workspaceRoot
            then None
            else Some workspaceRoot

        let! spiralBuilderResults =
            match props.builderCommands, props.lastTopLevelIndex with
            | [||], _ | _, None -> [||] |> Async.init
            | builderCommands, _ ->
                builderCommands
                |> Array.map (fun builderCommand ->
                    let path =
                        workspaceRoot </> $@"deps/spiral/workspace/target/release/spiral{SpiralPlatform.get_executable_suffix ()}"
                        |> System.IO.Path.GetFullPath
                    let commands =
                        if props.backend = Supervisor.Fsharp
                            && (
                                builderCommand |> SpiralSm.starts_with "rust"
                                || builderCommand |> SpiralSm.starts_with "typescript"
                                || builderCommand |> SpiralSm.starts_with "python"
                            )
                        then [| $"{path} fable --fs-path \"{props.outputPath}\" --command \"{builderCommand}\"" |]
                        elif props.backend = Supervisor.Python
                            && builderCommand |> SpiralSm.starts_with "cuda"
                        then [| $"{path} {builderCommand} --py-path \"{props.outputPath}\"" |]
                        elif props.backend = Supervisor.Cpp
                            && builderCommand |> SpiralSm.starts_with "cpp"
                        then [| $"{path} {builderCommand} --cpp-path \"{props.outputPath}\"" |]
                        elif props.backend = Supervisor.Gleam
                            && builderCommand |> SpiralSm.starts_with "gleam"
                        then [| $"{path} {builderCommand} --gleam-path \"{props.outputPath}\"" |]
                        else [||]
                    builderCommand, commands
                )
                |> Array.filter (fun (_, commands) -> commands.Length > 0)
                |> Array.collect (fun (builderCommand, commands) ->
                    commands
                    |> Array.map (fun command -> async {
                        let! exitCode, result =
                            SpiralRuntime.execution_options (fun x ->
                                { x with
                                    l0 = command
                                    l1 = props.cancellationToken
                                    l2 = [|
                                        "AUTOMATION", spiral_compiler.assemblyName = "dotnet-repl" |> string
                                        "TRACE_LEVEL", $"%A{if props.printCode then props.traceLevel else Info}"
                                    |]
                                    l6 = workspaceRootExternal
                                }
                            )
                            |> SpiralRuntime.execute_with_options_async
                        trace Debug
                            (fun () -> $"Eval.processSpiralOutput / spiral cli")
                            (fun () -> $"exitCode: {exitCode} / builderCommand: {builderCommand} / command: {command} / result: {result |> SpiralSm.ellipsis_end 400} / {_locals ()}")
                        return
                            if exitCode = 0
                            then {| code = result; eval = false; builderCommand = builderCommand |} |> Ok
                            else result |> Error
                    })
                )
                |> Async.Parallel

        let hasEval =
            props.backend = Supervisor.Fsharp
            && props.builderCommands |> Array.exists (fun x -> x |> SpiralSm.starts_with "fsharp")

        let outputResult =
            if props.builderCommands.Length > 0 && not hasEval
            then None
            else
                let code =
                    if props.builderCommands.Length > 1
                    then
                        let header = "System.Console.WriteLine \".fsx output:\"\n"
                        $"{header}{props.code}"
                    else props.code
                Some (Ok [ {| code = code; eval = true; builderCommand = "" |} ])

        match outputResult, spiralBuilderResults with
        | Some outputResult, [||] ->
            return outputResult, [||]
        | None, [||] ->
            return Ok [ {| code = "()"; eval = true; builderCommand = "" |} ], [||]
        | _, spiralBuilderResults ->
            try
                let spiralResults =
                    match outputResult with
                    | Some (Ok code) ->
                        spiralBuilderResults
                        |> Array.append (code |> List.map Ok |> List.toArray)
                    | _ -> spiralBuilderResults
                let codes =
                    spiralResults
                    |> Array.map (fun spiralBuilderResult' ->
                        let commandResult, errors =
                            match spiralBuilderResult' with
                            | Ok result when result.eval = false ->
                                let result' =
                                    result.code
                                    |> FSharp.Json.Json.deserialize<Map<string,string>>
                                let result =
                                    match result' |> Map.tryFind "command_result" with
                                    | Some result'' ->
                                        result''
                                        |> FSharp.Json.Json.deserialize<Map<string,string>>
                                        |> Map.add "builderCommand" result.builderCommand
                                    | None -> Map.empty
                                result, [||]
                            | Ok result when result.eval = true ->
                                let result =
                                    [
                                        "extension", "fsx"
                                        "code", result.code
                                        "output", ""
                                    ]
                                    |> Map.ofList
                                result, [||]
                            | Error error ->
                                Map.empty,
                                [|
                                    (
                                        TraceLevel.Critical, $"Eval.processSpiralOutput / evalResult error / errors[0] / outputPath: {props.outputPath} / builderCommands: %A{props.builderCommands} / spiralBuilderResult': %A{spiralBuilderResult'} / error: %A{error}", 0, ("", (0, 0), (0, 0))
                                    )
                                |]
                            | _ ->
                                Map.empty, [||]

                        if errors |> Array.isEmpty |> not
                        then Error (Exception $"Eval.processSpiralOutput / evalResult errors / Exception / commandResult: %A{commandResult}"), errors
                        else
                            let extension = commandResult.["extension"]
                            let code = commandResult.["code"]
                            let output = commandResult.["output"]
                            let builderCommand =
                                commandResult
                                |> Map.tryFind "builderCommand"
                                |> Option.defaultValue ""

                            let backendInfo =
                                match props.backend, builderCommand with
                                | Supervisor.Fsharp, builderCommand
                                    when builderCommand |> SpiralSm.contains " " -> $" ({builderCommand})"
                                | Supervisor.Fsharp, _ -> ""
                                | _ -> $" ({props.backend})"

                            let eval = output = "" && extension = "fsx"

                            if props.printCode && not eval
                            then _trace (fun () -> $""".{extension}{backendInfo}:{'\n'}{code}""")

                            trace Debug
                                (fun () -> $"Eval.processSpiralOutput / result")
                                (fun () -> $"builderCommand: {builderCommand} / extension: {extension} / commandResult: {commandResult |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400}/ {_locals ()}")

                            let code =
                                if props.printCode
                                    || spiralResults.Length > 1
                                    || props.builderCommands.Length > 1
                                then
                                    if eval
                                    then code
                                    else
                                        let header = $".{extension} output{backendInfo}:\n"
                                        $"""{if output |> SpiralSm.contains "\n" then "\n" else ""}{header}{output}"""
                                elif eval
                                then code
                                else output
                            Ok {| code = code; eval = eval; builderCommand = builderCommand |}, [||]
                    )
                trace Debug
                    (fun () -> $"Eval.processSpiralOutput / codes")
                    (fun () ->
                        let props = {| props with cancellationToken = None |}
                        $"codes: {codes |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400} / spiralResults: {spiralResults |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400} / spiralBuilderResults: {spiralBuilderResults |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400} / props: {props |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400} / {_locals ()}")
                return
                    (((Ok []), [||]), codes)
                    ||> Array.fold (fun (acc_code, acc_errors) (code, errors) ->
                        match code, acc_code with
                        | Ok code, Ok acc_code ->
                            let errors =
                                acc_errors
                                |> Array.append errors
                                |> Array.append props.spiralErrors
                            let errors =
                                if errors |> Array.isEmpty
                                then errors
                                else
                                    let code = $"%A{code}"
                                    errors
                                    |> Array.append [|
                                        TraceLevel.Critical, $"Eval.processSpiralOutput / errors / errors[-1] / outputPath: {props.outputPath} / builderCommands: %A{props.builderCommands} / code: {code |> SpiralSm.ellipsis_end 400}", 0, ("", (0, 0), (0, 0))
                                    |]
                            Ok (code :: acc_code), errors
                        | Error ex, _
                        | _, Error ex ->
                            Error (Exception $"Eval.processSpiralOutput / -1 / Exception / spiralBuilderResults: %A{spiralBuilderResults} / ex: {ex |> SpiralSm.format_exception}"),
                            acc_errors |> Array.append errors
                    )
            with ex ->
                trace Critical (fun () -> $"Eval.processSpiralOutput / try 2 ex / spiralBuilderResults: %A{spiralBuilderResults} / ex: {ex |> SpiralSm.format_exception}") _locals
                return
                    Error (Exception $"Eval.processSpiralOutput / try 2 ex / Exception / spiralBuilderResults: %A{spiralBuilderResults} / ex: {ex |> SpiralSm.format_exception}"),
                    [|
                        (
                            TraceLevel.Critical, $"Eval.processSpiralOutput / try 2 ex / errors[0] / spiralBuilderResults: %A{spiralBuilderResults} / ex: {ex |> SpiralSm.format_exception}", 0, ("", (0, 0), (0, 0))
                        )
                    |]
    }

    /// ## tryGetPropertyValue
    let tryGetPropertyValue (propertyName: string) (obj: obj) =
        let objType = obj.GetType ()
        let propertyInfo = propertyName |> objType.GetProperty
        if propertyInfo <> null
        then propertyInfo.GetValue (obj, null) |> Some
        else None

    /// ## evalAsync
    let rec evalAsync
        retry
        (props : {|
            rawCellCode: _
            lines: _
            isReal: _
            builderCommands: _ array
            isCache: _
            timeout: _
            cancellationToken: _
            printCode: _
            traceLevel: _
            fsi_eval: _
        |})
        = async {
        try
            let cellCode, lastTopLevelIndex = prepareSpiral props.rawCellCode props.lines
            let newAllCode =
                if props.isReal
                then $"{allCodeReal}\n\n{cellCode}"
                else $"{allCode}\n\n{cellCode}"

            let buildBackends =
                if props.builderCommands.Length = 0
                then [| Supervisor.Fsharp |]
                else
                    props.builderCommands
                    |> Array.map (fun x ->
                        if x |> SpiralSm.starts_with "gleam"
                        then Supervisor.Gleam
                        elif x |> SpiralSm.starts_with "cuda"
                        then Supervisor.Python
                        elif x |> SpiralSm.starts_with "cpp"
                        then Supervisor.Cpp
                        else Supervisor.Fsharp
                    )
                    |> Array.distinct

            trace Verbose
                (fun () -> $"Eval.eval")
                (fun () -> $"lastTopLevelIndex: {lastTopLevelIndex} / builderCommands: %A{props.builderCommands} / buildBackends: %A{buildBackends} / isReal: {props.isReal} / {_locals ()}")

            let! buildCodeResults =
                buildBackends
                |> Array.map (fun backend -> async {
                    let! result =
                        if props.isReal
                        then Supervisor.Spir newAllCode
                        else
                            Supervisor.Spi
                                (newAllCode, if allCodeReal = "" then None else Some allCodeReal)
                        |> Supervisor.buildCode backend allPackages props.isCache props.timeout props.cancellationToken
                    return backend, result
                })
                |> Async.Parallel
                |> Async.catch
                |> Async.runWithTimeoutAsync props.timeout

            match buildCodeResults with
            | Some (Ok buildCodeResults) ->
                let! result, errors =
                    ((Ok [], [||]), buildCodeResults)
                    ||> Async.fold (fun acc buildCodeResult -> async {
                        match buildCodeResult with
                        | backend, (_, (outputPath, Some code), spiralErrors) ->
                            let spiralErrors =
                                allCode |> mapErrors (Warning, spiralErrors, lastTopLevelIndex)
                            let! result =
                                processSpiralOutput
                                    {|
                                        printCode = props.printCode
                                        traceLevel = props.traceLevel
                                        builderCommands = props.builderCommands
                                        lastTopLevelIndex = lastTopLevelIndex
                                        backend = backend
                                        cancellationToken = props.cancellationToken
                                        spiralErrors = spiralErrors
                                        code = code
                                        outputPath = outputPath
                                        isReal = props.isReal
                                    |}
                            match result, acc with
                            | (Ok code, errors), (Ok acc_code, acc_errors) ->
                                return Ok (acc_code @ code), acc_errors |> Array.append errors
                            | (Error ex, errors), _ | _, (Error ex, errors) ->
                                return
                                    Error (Exception $"Eval.evalAsync / processSpiralOutput / Exception / buildCodeResult: %A{buildCodeResult |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400} / ex: {ex |> SpiralSm.format_exception}"),
                                    errors |> Array.append errors
                        | _, (_, _, errors) when errors |> List.isEmpty |> not ->
                            return errors.[0] |> fst |> Exception |> Error,
                            allCode |> mapErrors (TraceLevel.Critical, errors, lastTopLevelIndex)
                        | _ -> return acc
                    })
                let cancellationToken = defaultArg props.cancellationToken System.Threading.CancellationToken.None
                match result, errors with
                | Ok code, [||] ->
                    let code, eval =
                        code
                        |> List.map (fun code ->
                            if code.eval
                            then None, Some code.code
                            else Some code.code, None
                        )
                        |> List.unzip
                    let code = code |> List.choose id
                    let eval = eval |> List.choose id

                    trace Debug
                        (fun () -> $"Eval.eval")
                        (fun () -> $"eval: {eval |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400} / code: {code |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400} / {_locals ()}")

                    let ch, errors =
                        match eval, code with
                        | [], [] ->
                            Choice2Of2 (Exception $"Eval.evalAsync / eval=[] / code=[] / buildCodeResults: %A{buildCodeResults} / code: %A{code}"), errors
                        | [ eval ], [] ->
                            let eval =
                                if eval |> SpiralSm.contains "<script"
                                then $"{eval}, \"text/html1\""
                                else eval
                            let ch, errors2 = props.fsi_eval eval cancellationToken
                            let errors =
                                errors2
                                // |> Array.map (fun (e1, e2, e3, _) ->
                                //     (e1, e2, e3, ("", (0, 0), (0, 0)))
                                // )
                                |> Array.append errors
                            ch, errors
                        | [], _ ->
                            let code = code |> List.rev |> String.concat "\n\n"
                            let code =
                                if props.printCode
                                then $"\"\"\"{code}\n\n\"\"\""
                                else $"\"\"\"{code}\n\"\"\""
                            let code =
                                if code |> SpiralSm.contains "<script"
                                then $"{code}, \"text/html2\""
                                else code
                            let ch, errors2 = props.fsi_eval code cancellationToken
                            let errors =
                                errors2
                                // |> Array.map (fun (e1, e2, e3, _) ->
                                //     (e1, e2, e3, ("", (0, 0), (0, 0)))
                                // )
                                |> Array.append errors
                            ch, errors
                        | _ ->
                            let code, errors =
                                ((Ok (code |> List.rev), [||]), eval)
                                ||> List.fold (fun (acc, acc_errors) eval ->
                                    match acc with
                                    | Error ch -> Error ch, acc_errors
                                    | Ok acc ->
                                        let eval =
                                            if eval |> SpiralSm.contains "<script"
                                            then $"{eval}, \"text/html3\""
                                            else eval
                                        let ch, errors = props.fsi_eval eval cancellationToken
                                        let errors =
                                            errors
                                            // |> Array.map (fun (e1, e2, e3, _) ->
                                            //     (e1, e2, e3, ("", (0, 0), (0, 0)))
                                            // )
                                            |> Array.append acc_errors
                                        match ch with
                                        | Choice1Of2 v ->
                                            let v =
                                                v
                                                |> tryGetPropertyValue "ReflectionValue"
                                                |> Option.map (fun x -> $"%A{x}")
                                                |> Option.defaultValue ""
                                            Ok (v :: acc), errors
                                        | Choice2Of2 ex ->
                                            trace Critical (fun () -> $"Eval.evalAsync / fsi_eval fold Choice error / buildCodeResults: %A{buildCodeResults} / ex: {ex |> SpiralSm.format_exception}") _locals
                                            Error ch, errors
                                )
                            match code with
                            | Error ch -> ch, errors
                            | Ok code ->
                                let code =
                                    code
                                    |> List.filter ((<>) "")
                                    |> String.concat "\n\n"

                                let code =
                                    if props.builderCommands.Length > 0 && eval.Length = 0
                                    then code
                                    elif code |> SpiralSm.contains "\n\n\n"
                                    then $"{code}\n\n"
                                    else $"{code}\n"

                                let code =
                                    if props.printCode
                                    then $"\"\"\"{code}\n\n\n\"\"\""
                                    else $"\"\"\"{code}\n\"\"\""
                                let code =
                                    if code |> SpiralSm.contains "<script"
                                    then $"{code}, \"text/html4\""
                                    else code
                                let ch, errors2 = props.fsi_eval code cancellationToken
                                let errors =
                                    errors2
                                    // |> Array.map (fun (e1, e2, e3, _) ->
                                    //     (e1, e2, e3, ("", (0, 0), (0, 0)))
                                    // )
                                    |> Array.append errors
                                ch, errors
                    match ch with
                    | Choice1Of2 v ->
                        if props.isReal
                        then allCodeReal <- newAllCode
                        else allCode <- newAllCode
                        return Ok(v), errors
                    | Choice2Of2 ex ->
                        return
                            Error (Exception $"Eval.evalAsync / -2 / Exception / ex: {ex |> SpiralSm.format_exception} / buildCodeResults: {buildCodeResults |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 400}"),
                            errors
                | Ok code, errors ->
                    return
                        Error (Exception "Eval.evalAsync / errors / buildCodeResults: %A{buildCodeResults} / code: %A{code}"),
                        errors
                | Error ex, errors ->
                    let ex = ex |> SpiralSm.format_exception
                    if retry <= 3 &&
                        (ex |> SpiralSm.contains "Expected one of: inl, let, union, nominal, prototype, type, instance, and, open")
                        || (ex |> SpiralSm.contains "Unexpected end of block past this token.")
                        || (ex |> SpiralSm.contains "Unbound variable:")
                    then return! evalAsync (retry + 1) props
                    else
                        return
                            Error (Exception $"Eval.evalAsync / -1 / Exception / ex: {ex} / buildCodeResults: {buildCodeResults |> FSharp.Json.Json.serialize |> SpiralSm.ellipsis_end 1500}"),
                            errors
            | Some (Error ex) ->
                trace Critical (fun () -> $"Eval.evalAsync / buildCodeResults Error / buildCodeResults: %A{buildCodeResults} / ex: {ex |> SpiralSm.format_exception}") _locals
                return
                    Error (Exception $"Eval.evalAsync / buildCodeResults Error / Exception / buildCodeResults: %A{buildCodeResults} / ex: {ex |> SpiralSm.format_exception}"),
                    [|
                        (
                            TraceLevel.Critical, $"Eval.evalAsync / buildCodeResults Error / errors[0] / ex: {ex |> SpiralSm.format_exception} / buildCodeResults: %A{buildCodeResults}", 0, ("", (0, 0), (0, 0))
                        )
                    |]
            | _ ->
                return
                    Error (Exception $"Eval.evalAsync / buildCodeResults / Exception / buildCodeResults: %A{buildCodeResults}"),
                    [|
                        (
                            TraceLevel.Critical, $"Eval.evalAsync / buildCodeResults / errors[0] / buildCodeResults: %A{buildCodeResults}", 0, ("", (0, 0), (0, 0))
                        )
                    |]
        with ex ->
            trace Critical (fun () -> $"Eval.evalAsync / try 1 ex / ex: {ex |> SpiralSm.format_exception} / lines: %A{props.lines}") _locals
            return
                Error (Exception $"Eval.evalAsync / try 1 ex / Exception / ex: {ex |> SpiralSm.format_exception} / lines: %A{props.lines}"),
                [|
                    (
                        TraceLevel.Critical, $"Eval.evalAsync / try 1 ex / errors[0] / ex: {ex |> SpiralSm.format_exception} / lines: %A{props.lines}", 0, ("", (0, 0), (0, 0))
                    )
                |]
    }

    /// ## eval
    let inline eval
        (fsi_eval:
            string
            -> System.Threading.CancellationToken
            -> Choice<'a, Exception> * (TraceLevel * string * int * (string * (int * int) * (int * int))) array)
        (cancellationToken: Option<System.Threading.CancellationToken>)
        (code: string)
        =
        trace Verbose
            (fun () -> $"Eval.eval")
            (fun () -> $"code: {code |> SpiralSm.ellipsis_end 400} / {_locals ()}")

        let rawCellCode =
            code |> SpiralSm.replace "\r\n" "\n"

        let lines = rawCellCode |> SpiralSm.split "\n"

        if lines |> Array.exists (fun line -> line |> SpiralSm.starts_with "#r " && line |> SpiralSm.ends_with "\"") then
            let cancellationToken = defaultArg cancellationToken System.Threading.CancellationToken.None
            let code =
                if code |> SpiralSm.contains "<script"
                then $"{code}, \"text/html5\""
                else code
            let ch, errors = fsi_eval code cancellationToken
            trace Verbose (fun () -> $"Eval.eval / fsi_eval 1 / ch: %A{ch} / errors: %A{errors}") _locals
            match ch with
            | Choice1Of2 v -> Ok(v), errors
            | Choice2Of2 ex -> Error(ex), errors
        else
            let builderCommands =
                lines
                |> Array.choose (fun line ->
                    if line |> SpiralSm.starts_with "///! "
                    then line |> SpiralSm.split "///! " |> Array.tryItem 1
                    else None
                )

            let packages =
                lines
                |> Array.choose (fun line ->
                    if line |> SpiralSm.starts_with "//// package="
                    then line |> SpiralSm.split "=" |> Array.skip 1 |> SpiralSm.concat "" |> Some
                    else None
                )

            allPackages <- packages |> Array.append allPackages |> Array.distinct

            let timeout =
                lines
                |> Array.tryPick (fun line ->
                    if line |> SpiralSm.starts_with "//// timeout="
                    then line |> SpiralSm.split "=" |> Array.tryItem 1 |> Option.map int
                    else None
                )
                |> Option.defaultValue (60003 * 60 * 24)

            let boolArg def command =
                lines
                |> Array.tryPick (fun line ->
                    let text = $"//// {command}"
                    match line.[0..text.Length-1], line.[text.Length..] with
                    | head, "" when head = text ->
                        Some true
                    | head, _ when head = text ->
                        line |> SpiralSm.split "=" |> Array.tryItem 1 |> Option.map ((<>) "false")
                    | _ -> None
                )
                |> Option.defaultValue def

            let printCode = "print_code" |> boolArg false
            let isTraceToggle = "trace_toggle" |> boolArg false
            let isTrace = "trace" |> boolArg false
            let isCache = "cache" |> boolArg false
            let isReal = "real" |> boolArg false
            let timeout_continue = "timeout_continue" |> boolArg false

            if isTraceToggle
            then traceToggle <- not traceToggle

            let oldLevel = get_trace_level ()
            let traceLevel =
                if isTrace || traceToggle
                then Verbose
                else Info
            traceLevel
            |> to_trace_level
            |> set_trace_level
            use _ = (new_disposable (fun () ->
                oldLevel |> set_trace_level
            ))

            evalAsync 1
                {|
                    rawCellCode = rawCellCode
                    lines = lines
                    isReal = isReal
                    builderCommands = builderCommands
                    isCache = isCache
                    timeout = timeout
                    cancellationToken = cancellationToken
                    printCode = printCode
                    traceLevel = traceLevel
                    fsi_eval = fsi_eval
                |}
            |> Async.runWithTimeout timeout
            |> (fun x ->
                match x with
                | Some ((Ok x), a) -> Some ((Ok x), a)
                | Some ((Error x), a) ->
                    trace Info (fun () -> $"Eval.eval / error / exception: {x.GetType().FullName} / a: %A{a} / x: %A{x}") (fun () -> "")
                    Some ((Error x), a)
                | _ -> None
            )
            |> Option.defaultWith (fun () -> (
                let lines = lines |> SpiralSm.concat (string '\n') |> SpiralSm.ellipsis_end 1500
                in
                Error (Exception $"Eval.eval / Async.runWithTimeout / Exception / timeout: {timeout} / timeout_continue: {timeout_continue} / lines: {lines}"),
                [|
                    (
                        TraceLevel.Critical, $"Eval.eval / Async.runWithTimeout / errors[0] / timeout: {timeout} / lines: {lines}", 0, ("", (0, 0), (0, 0))
                    )
                |]
            ))
