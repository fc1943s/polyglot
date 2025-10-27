#nowarn 0686

#if !INTERACTIVE
namespace Polyglot
#endif

module Supervisor =

#if !INTERACTIVE
    open Lib
#endif

    open Common
    open SpiralFileSystem.Operators
    open Microsoft.AspNetCore.SignalR.Client

    /// ### sendJson
    let sendJson (port : int) (json : string) = async {
        let host = "127.0.0.1"
        let! portOpen = SpiralNetworking.test_port_open host port
        if portOpen then
            try
                let connection = HubConnectionBuilder().WithUrl($"http://{host}:{port}").Build()
                do! connection.StartAsync () |> Async.AwaitTask
                let! result = connection.InvokeAsync<string> ("ClientToServerMsg", json) |> Async.AwaitTask
                do! connection.StopAsync () |> Async.AwaitTask
                trace Verbose (fun () -> $"Supervisor.sendJson / port: {port} / json: {json |> SpiralSm.ellipsis_end 200} / result: {result |> Option.ofObj |> Option.map (SpiralSm.ellipsis_end 200)}") _locals
                return Some result
            with ex ->
                trace Critical (fun () -> $"Supervisor.sendJson / port: {port} / json: {json |> SpiralSm.ellipsis_end 200} / ex: {ex |> SpiralSm.format_exception}") _locals
                return None
        else
            trace Debug (fun () -> $"Supervisor.sendJson / port: {port} / error: port not open") _locals
            return None
    }

    /// ### sendObj
    let sendObj port obj =
        obj
        |> System.Text.Json.JsonSerializer.Serialize
        |> sendJson port

    /// ### workspaceRoot
    let workspaceRoot = SpiralFileSystem.get_workspace_root ()

    /// ### getFilePathFromUri
    let getFilePathFromUri uri =
        match System.Uri.TryCreate (uri, System.UriKind.Absolute) with
        | true, uri -> uri.AbsolutePath |> System.IO.Path.GetFullPath
        | _ -> failwith "invalid uri"

    /// ### getCompilerPort
    let getCompilerPort () =
        13805

    /// ### serialize_obj
    let serializeObj obj =
            try
                obj
                |> FSharp.Json.Json.serialize
                |> SpiralSm.replace "\\\\" "\\"
                |> SpiralSm.replace "\\r\\n" "\n"
                |> SpiralSm.replace "\\n" "\n"
            with ex ->
                trace Critical (fun () -> "Supervisor.serialize_obj / obj: %A{obj}") _locals
                ""

    /// ### Backend
    type Backend =
        | Gleam
        | Fsharp
        | Python
        | Cpp




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

    /// ### awaitCompiler
    let awaitCompiler port cancellationToken = async {
        let host = "127.0.0.1"
        let struct (ct, disposable) = cancellationToken |> SpiralThreading.new_disposable_token
        let! ct = ct |> SpiralAsync.merge_cancellation_token_with_default_async

        let compiler = MailboxProcessor.Start (fun inbox -> async {
            let! availablePort = SpiralNetworking.get_available_port (Some 180) host port
            if availablePort <> port then
                inbox.Post (port, false)
            else
                let compilerDir =
                    workspaceRoot </> "deps/spiral/apps/compiler/dist"
                    |> System.IO.Path.GetFullPath

                let compilerPath = compilerDir </> $"spiral_compiler{SpiralPlatform.get_executable_suffix ()}"

                let! exitCode, result =
                    SpiralRuntime.execution_options (fun x ->
                        { x with
                            l0 = $"\"{compilerPath}\" --port {availablePort} --default-int i32 --default-float f64"
                            l1 = Some ct
                            l3 = Some (fun struct (_, line, _) -> async {
                                if line |> SpiralSm.contains $"System.IO.IOException: Failed to bind to address http://{host}:{port}: address already in use." then
                                    inbox.Post (port, false)

                                if line |> SpiralSm.contains $"Server bound to: http://localhost:{availablePort}" then
                                    let rec 루프 retry = async {
                                        do!
                                            SpiralNetworking.wait_for_port_access (Some 100) true host availablePort
                                            |> Async.runWithTimeoutAsync 500
                                            |> Async.Ignore

                                        let _locals () = $"port: {availablePort} / retry: {retry} / {_locals ()}"
                                        try
                                            let pingObj = {| Ping = true |}
                                            let! pingResult = pingObj |> sendObj availablePort
                                            trace Verbose (fun () -> $"Supervisor.awaitCompiler / Ping / result: '%A{pingResult}'") _locals

                                            match pingResult with
                                            | Some _ -> inbox.Post (availablePort, true)
                                            | None -> do! 루프 (retry + 1)
                                        with ex ->
                                            trace Verbose (fun () -> $"Supervisor.awaitCompiler / Ping / ex: {ex |> SpiralSm.format_exception}") _locals
                                            do! 루프 (retry + 1)
                                    }
                                    do! 루프 1
                            })
                            l6 = Some workspaceRoot
                        }
                    )
                    |> SpiralRuntime.execute_with_options_async

                trace Debug (fun () -> $"Supervisor.awaitCompiler / exitCode: {exitCode} / result: {result}") _locals
                disposable.Dispose ()
        }, ct)

        let! serverPort, managed = compiler.Receive ()

        let connection = HubConnectionBuilder().WithUrl($"http://{host}:{serverPort}").Build ()
        do! connection.StartAsync () |> Async.AwaitTask

        let event = Event<_> ()
        let disposable' = connection.On<string> ("ServerToClientMsg", event.Trigger)
        let stream =
            FSharp.Control.AsyncSeq.unfoldAsync
                (fun () -> async {
                    let! msg = event.Publish |> Async.AwaitEvent
                    return Some (msg |> FSharp.Json.Json.deserialize<ClientErrorsRes>, ())
                })
                ()

        let disposable' =
            new_disposable (fun () ->
                async {
                    disposable'.Dispose ()
                    do! connection.StopAsync () |> Async.AwaitTask
                    disposable.Dispose ()
                    if managed
                    then do!
                        SpiralNetworking.wait_for_port_access (Some 100) false host serverPort
                        |> Async.runWithTimeoutAsync 1500
                        |> Async.Ignore
                }
                |> Async.RunSynchronously
            )

        return
            serverPort,
            stream,
            ct,
            disposable'
    }

    open Hopac
    open Hopac.Infixes

    /// ### server
    let server1 = new_server<Job<unit>, obj, string option, Job<unit>, unit> ()
    let server2 = new_server<Job<unit>, obj, int array, Job<unit>, unit> ()

    /// ### buildFile
    let buildFile backend timeout port cancellationToken path =
        let rec 루프 retry = async {
            let fullPath = path |> System.IO.Path.GetFullPath |> SpiralFileSystem.normalize_path
            let fileDir = fullPath |> System.IO.Path.GetDirectoryName
            let fileName = fullPath |> System.IO.Path.GetFileNameWithoutExtension
            let! code = fullPath |> SpiralFileSystem.read_all_text_async

            // let stream, disposable = fileDir |> FileSystem.watchDirectory (fun _ -> true)
            // use _ = disposable

            let struct (token, disposable) = SpiralThreading.new_disposable_token cancellationToken
            use _ = disposable

            let outputFileName =
                match backend with
                | Gleam -> $"{fileName}.gleam"
                | Fsharp -> $"{fileName}.fsx"
                | Python -> $"{fileName}.py"
                | Cpp -> $"{fileName}.cpp"

            // let outputContentSeq =
            //     stream
            //     |> FSharp.Control.AsyncSeq.chooseAsync (function
            //         | _, (FileSystem.FileSystemChange.Changed (path, Some text))
            //             when (path |> System.IO.Path.GetFileName) = outputFileName
            //             ->
            //                 // fileDir </> path |> SpiralFileSystem.read_all_text_retry_async
            //                 text |> Some |> Async.init
            //         | _ -> None |> Async.init
            //     )
            //     |> FSharp.Control.AsyncSeq.map (fun content ->
            //         Some (content |> SpiralSm.replace "\r\n" "\n"), None
            //     )

            let printErrorData (data : {| uri : string; errors : RString list |}) =
                let fileName = data.uri |> System.IO.Path.GetFileName
                let errors =
                    data.errors
                    |> List.map snd
                    |> SpiralSm.concat "\n"
                $"{fileName}:\n{errors}"


            let port = port |> Option.defaultWith getCompilerPort
            // let! serverPort, errors, ct, disposable = awaitCompiler port (Some token)
            let serverPort = port
            // use _ = disposable

            let errorsSeq =
                server1.errors
                |> FSharp.Control.AsyncSeq.choose (fun error ->
                    match error with
                    | FatalError message ->
                        Some (message, error)
                    | TracedError data ->
                        Some (data.message, error)
                    | PackageErrors data when data.errors |> List.isEmpty |> not ->
                        Some (data |> printErrorData, error)
                    | TokenizerErrors data when data.errors |> List.isEmpty |> not ->
                        Some (data |> printErrorData, error)
                    | ParserErrors data when data.errors |> List.isEmpty |> not ->
                        Some (data |> printErrorData, error)
                    | TypeErrors data when data.errors |> List.isEmpty |> not ->
                        Some (data |> printErrorData, error)
                    | _ -> None
                )
                |> FSharp.Control.AsyncSeq.map (fun (message, error) ->
                    None, Some (message, error)
                )

            let timerSeq =
                500
                |> FSharp.Control.AsyncSeq.intervalMs
                |> FSharp.Control.AsyncSeq.map (fun _ -> None, None)

            let compilerEvent = Event<option<string> * option<string * ClientErrorsRes>> ()
            // let disposable' = connection.On<string> ("ServerToClientMsg", event.Trigger)
            let outputContentSeq =
                FSharp.Control.AsyncSeq.unfoldAsync
                    (fun () -> async {
                        let! msg = compilerEvent.Publish |> Async.AwaitEvent
                        trace Verbose (fun () -> $"Supervisor.buildFile / outputContentSeq unfoldAsync / msg: %A{msg}") _locals
                        return Some (msg, ())
                    })
                    ()

            let compilerEvent2 = Event<option<string> * (string * ClientErrorsRes) list * int> ()
            // let disposable' = connection.On<string> ("ServerToClientMsg", event.Trigger)
            let outputContentSeq2 =
                FSharp.Control.AsyncSeq.unfoldAsync
                    (fun () -> async {
                        let! msg = compilerEvent2.Publish |> Async.AwaitEvent
                        trace Verbose (fun () -> $"Supervisor.buildFile / outputContentSeq2 unfoldAsync / msg: %A{msg}") _locals
                        return Some (msg, ())
                    })
                    ()

            let outputSeq =
                [ outputContentSeq; errorsSeq; timerSeq ]
                // [ errorsSeq; timerSeq ]
                |> FSharp.Control.AsyncSeq.mergeAll

            let! outputChild =
                ((None, [], 0), outputSeq)
                ||> FSharp.Control.AsyncSeq.scan (
                    fun (outputContentResult, errors, typeErrorCount) (outputContent, error) ->
                        trace Verbose (fun () -> $"Supervisor.buildFile / AsyncSeq.scan / outputContent:\n'{outputContent |> Option.map (SpiralSm.ellipsis_end 1500)} / errors: {errors |> serializeObj} / outputContentResult: {outputContentResult} / typeErrorCount: {typeErrorCount} / retry: {retry} / error: {error} / path: {path}'") _locals
                        match outputContent, error with
                        | Some outputContent, None -> Some outputContent, errors, typeErrorCount
                        | None, Some (
                            _,
                            FatalError "File main has a type error somewhere in its path."
                            ) ->
                            outputContentResult, errors, typeErrorCount + 1
                        | None, Some error -> outputContentResult, error :: errors, typeErrorCount
                        | None, None when typeErrorCount >= 1 ->
                            outputContentResult, errors, typeErrorCount + 1
                        | _ -> outputContentResult, errors, typeErrorCount
                )
                |> FSharp.Control.AsyncSeq.takeWhileInclusive (fun (outputContent, errors, typeErrorCount) ->
                    trace
                        Verbose
                        (fun () -> $"Supervisor.buildFile / takeWhileInclusive / outputContent:\n'{outputContent |> Option.map (SpiralSm.ellipsis_end 1500)}' / errors: {errors |> serializeObj} / typeErrorCount: {typeErrorCount} / retry: {retry} / path: {path}")
                        _locals
        #if INTERACTIVE
                    let errorWait = 1
        #else
                    let errorWait = 1
        #endif
                    match outputContent, errors with
                    | None, [] when typeErrorCount > errorWait -> false
                    | _, [ message, TypeErrors errors ] ->
                        compilerEvent.Trigger (None, Some (message, TypeErrors errors))
                        trace Verbose (fun () -> $"Supervisor.buildFile / takeWhileInclusive / TypeErrors trigger") _locals
                        false
                    | None, [] -> true
                    | _ -> false
                )
                |> FSharp.Control.AsyncSeq.tryLast
                // |> Async.withCancellationToken ct
                |> Async.catch
                |> Async.runWithTimeoutAsync timeout
                |> Async.StartChild

            // do! Async.Sleep 60

            let fullPathUri = fullPath |> SpiralFileSystem.normalize_path |> SpiralFileSystem.new_file_uri

            trace Verbose (fun () -> $"Supervisor.buildFile / fullPathUri: %A{fullPathUri}") (fun () -> "")

            // let fileOpenObj = {| FileOpen = {| uri = fullPathUri; spiText = code |} |}

            // let! _fileOpenResult = fileOpenObj |> sendObj serverPort
            let fileOpenArgs = {| uri = fullPathUri; spiText = code |}
            let! _fileOpenResult =
                server1.job_null (server1.supervisor *<+ SupervisorReq.FileOpen fileOpenArgs)
                |> Async.AwaitTask
                |> Async.runWithTimeoutAsync 60000
                |> Async.map Option.get
            trace Verbose (fun () -> $"Supervisor.buildFile / _fileOpenResult: %A{_fileOpenResult}") (fun () -> "")

            // let! _fileOpenResult = fileOpenObj |> sendObj serverPort

            // do! Async.Sleep 60

            let backendId =
                match backend with
                | Gleam -> "Gleam"
                | Fsharp -> "Fsharp"
                | Python -> "Python + Cuda"
                | Cpp -> "Cpp + Cuda"
            // let buildFileObj = {| BuildFile = {| uri = fullPathUri; backend = backendId |} |}

            // let backend = Supervisor.Fsharp
            let buildFileArgs = {| uri = fullPathUri; backend = backendId |}
            let buildFileResultAsync =
                server1.job_val (fun res -> server1.supervisor *<+ SupervisorReq.BuildFile(buildFileArgs,res))
                |> Async.AwaitTask
                |> Async.runWithTimeoutAsync timeout
                |> Async.map Option.get

            let buildFileResultAsync = async {
                let! buildFileResult = buildFileResultAsync
                let buildFileResult =
                    if buildFileResult = "" || buildFileResult = null
                    then None
                    else buildFileResult |> SpiralSm.replace "\r\n" "\n" |> Some
                trace Verbose (fun () -> $"Supervisor.buildFile / buildFileResult: %A{buildFileResult}") (fun () -> "")
                if buildFileResult.IsSome then
                    compilerEvent2.Trigger (buildFileResult, [], 0)
                return buildFileResult, [], 0
            }

            let resultAsync =
                outputChild
                |> Async.map (fun x ->
                    trace Verbose (fun () -> $"Supervisor.buildFile / outputChild / x: %A{x}") _locals
                    match x with
                    | Some (Ok (Some (outputCode, errors, typeErrorCount))) ->
                        let x =
                            match errors with
                            | [ message, TypeErrors errors ] ->
                                trace Verbose (fun () -> $"Supervisor.buildFile / outputChild |> Async.map") _locals
                                compilerEvent2.Trigger (None, [ message, TypeErrors errors ], 0)
                                trace Verbose (fun () -> $"Supervisor.buildFile / outputChild |> Async.map") _locals
                            | _ -> ()
                        outputCode, errors |> List.distinct |> List.rev, typeErrorCount
                    | Some (Error ex) ->
                        trace Critical (fun () -> $"Supervisor.buildFile / error: {ex |> SpiralSm.format_exception} / retry: {retry}") _locals
                        None, [], 0
                    | _ -> None, [], 0
                )

            trace Verbose (fun () -> $"Supervisor.buildFile / before result: %A{()}") (fun () -> "")

            // let resultSeq =
            //     [| buildFileResultAsync; resultAsync |]
            //     |> FSharp.Control.AsyncSeq.ofSeqAsync


            let buildFileResultSeq = [| buildFileResultAsync |] |> FSharp.Control.AsyncSeq.ofSeqAsync
            let resultSeq = [| resultAsync |] |> FSharp.Control.AsyncSeq.ofSeqAsync

            let resultSeq =
                [ outputContentSeq2; buildFileResultSeq; resultSeq ]
                |> FSharp.Control.AsyncSeq.mergeAll

            let! buildFileResult, result, typeErrorCount =
                resultSeq
                // |> FSharp.Control.AsyncSeq.collect (Array.singleton >> FSharp.Control.AsyncSeq.ofSeq)
                |> FSharp.Control.AsyncSeq.collect (fun x ->
                    trace Verbose (fun () -> $"Supervisor.buildFile / ofSeqAsync / x: %A{x}") _locals
                    match x with
                    | Some _, _, _ as x -> [| x |]
                    | _, _ :: _, _ as x -> [| x |]
                    | _ -> [||]
                    |> FSharp.Control.AsyncSeq.ofSeq
                )
                |> FSharp.Control.AsyncSeq.tryFirst
                |> Async.map Option.get

            trace Verbose (fun () -> $"Supervisor.buildFile / result: %A{result} / buildFileResult: %A{buildFileResult} / typeErrorCount: {typeErrorCount}") (fun () -> "")

            match buildFileResult with
            // | None when typeErrorCount > 0 && retry = 0 ->
            //     trace Verbose (fun () -> $"Supervisor.buildFile / result: {result} / retry: {retry} / typeErrorCount: {typeErrorCount} / fileDir: {fileDir} / targetDir: {targetDir}") _locals
            //     return! 루프 (retry + 1)
            | _ ->
                let targetDir = workspaceRoot </> "target"
                trace Verbose (fun () -> $"Supervisor.buildFile / retry: {retry} / typeErrorCount: {typeErrorCount} / fileDir: {fileDir} / targetDir: {targetDir}") _locals
                if fileDir |> SpiralSm.starts_with targetDir then
                    let fileDirUri = fileDir |> SpiralFileSystem.normalize_path |> SpiralFileSystem.new_file_uri

                    // let fileDeleteObj = {| FileDelete = {| uris = [| fileDirUri |] |} |}
                    // let! _fileDeleteResult = fileDeleteObj |> sendObj serverPort
                    let fileDeleteArgs = {| uris = [| fileDirUri |] |}
                    let! _fileDeleteResult =
                        server1.job_null (server1.supervisor *<+ SupervisorReq.FileDelete fileDeleteArgs)
                        |> Async.AwaitTask
                    ()

                let outputPath = fileDir </> outputFileName
                return outputPath, buildFileResult, result
        }
        루프 0

    /// ### SpiralInput
    type SpiralInput =
        | Spi of string * string option
        | Spir of string

    /// ### persistCode
    let persistCode (input: {| backend : Backend option; input: SpiralInput; packages: string [] |}) = async {
        let targetDir = workspaceRoot </> "target/spiral_Eval"

        let packagesDir = targetDir </> "packages"

        let hashHex = $"%A{input.backend}%A{input.input}" |> SpiralCrypto.hash_text

        let packageDir = packagesDir </> hashHex

        let packageDir =
            if input.backend = Some Gleam
            then packageDir </> "src"
            else packageDir

        packageDir |> System.IO.Directory.CreateDirectory |> ignore

        let moduleName = "main"

        let spirModule, spiModule =
            match input.input with
            | Spi (_spi, Some _spir) -> true, true
            | Spi (_spi, None) -> false, true
            | Spir _spir -> true, false
            |> fun (spir, spi) ->
                (if spir then $"{moduleName}_real*-" else ""),
                if spi then moduleName else ""

        let libLinkTargetPath = workspaceRoot </> "../spiral/lib/spiral"
        let libLinkPath = packageDir </> "spiral"

        let packagesModule =
            input.packages
            |> Array.map (fun package ->
                let path = workspaceRoot </> package
                let packageName = path |> System.IO.Path.GetFileName
                let libLinkPath = packageDir </> packageName
                libLinkPath |> SpiralFileSystem.link_directory path
                $"{packageName}-"
            )
            |> String.concat "\n    "

        let packageDir' =
            if input.packages |> Array.isEmpty
            then workspaceRoot </> "../spiral/lib"
            else
                libLinkPath |> SpiralFileSystem.link_directory libLinkTargetPath
                packageDir

        let spiprojPath = packageDir </> "package.spiproj"
        let spiprojCode =
            $"""packageDir: {packageDir'}
packages:
    |core-
    spiral-
    {packagesModule}
modules:
    {spirModule}
    {spiModule}
"""
        do! spiprojCode |> SpiralFileSystem.write_all_text_exists spiprojPath

        let spirPath = packageDir </> $"{moduleName}_real.spir"
        let spiPath = packageDir </> $"{moduleName}.spi"

        let spirCode, spiCode =
            match input.input with
            | Spi (spi, Some spir) -> Some spir, Some spi
            | Spi (spi, None) -> None, Some spi
            | Spir spir -> Some spir, None

        match spirCode with
        | Some spir -> do! spir |> SpiralFileSystem.write_all_text_exists spirPath
        | None -> ()
        match spiCode with
        | Some spi -> do! spi |> SpiralFileSystem.write_all_text_exists spiPath
        | None -> ()

        let spiralPath =
            match input.input with
            | Spi _ -> spiPath
            | Spir _ -> spirPath
        match input.backend with
        | None -> return spiralPath, None
        | Some backend ->
            let outputFileName =
                let fileName =
                    match input.input with
                    | Spi _ -> moduleName
                    | Spir _ -> $"{moduleName}_real"
                match backend with
                | Gleam -> $"{fileName}.gleam"
                | Fsharp -> $"{fileName}.fsx"
                | Python -> $"{fileName}.py"
                | Cpp -> $"{fileName}.cpp"
            let outputPath = packageDir </> outputFileName
            if outputPath |> System.IO.File.Exists |> not
            then return spiralPath, None
            else
                let! oldCode = spiralPath |> SpiralFileSystem.read_all_text_async
                if oldCode <> (spiCode |> Option.defaultValue (spirCode |> Option.defaultValue ""))
                then return spiralPath, None
                else
                    let! outputCode = outputPath |> SpiralFileSystem.read_all_text_async
                    return spiralPath, Some (outputPath, outputCode |> SpiralSm.replace "\r\n" "\n")
        }

    /// ### buildCode
    let buildCode backend packages isCache timeout cancellationToken input = async {
        let! mainPath, outputCache =
            persistCode {| input = input; backend = Some backend; packages = packages |}
        match outputCache with
        | Some (outputPath, outputCode) when isCache -> return mainPath, (outputPath, Some outputCode), []
        | _ ->
            let! outputPath, outputCode, errors = mainPath |> buildFile backend timeout None cancellationToken
            return mainPath, (outputPath, outputCode), errors
    }

    /// ## getFileTokenRange
    let getFileTokenRange port cancellationToken path = async {
        let fullPath = path |> System.IO.Path.GetFullPath
        let codeDir = fullPath |> System.IO.Path.GetDirectoryName
        let tokensPath = codeDir </> "tokens.json"
        let! tokens = async {
            if tokensPath |> System.IO.File.Exists |> not
            then return None
            else
                let! text = tokensPath |> SpiralFileSystem.read_all_text_async

                return
                    if text.Length > 2
                    then text |> FSharp.Json.Json.deserialize<int array> |> Some
                    else None
        }

        match tokens with
        | Some tokens ->
            return tokens |> Some
        | None ->
            let! code = fullPath |> SpiralFileSystem.read_all_text_async
            let lines = code |> SpiralSm.split "\n"

            let struct (token, disposable) = SpiralThreading.new_disposable_token cancellationToken
            use _ = disposable

            let port = port |> Option.defaultWith getCompilerPort
            // let! serverPort, _errors, ct, disposable = awaitCompiler port (Some token)
            // use _ = disposable

            let fullPathUri = fullPath |> SpiralFileSystem.normalize_path |> SpiralFileSystem.new_file_uri

            // let fileOpenObj = {| FileOpen = {| uri = fullPathUri; spiText = code |} |}
            // let! _fileOpenResult = fileOpenObj |> sendObj serverPort
            let fileOpenArgs = {| uri = fullPathUri; spiText = code |}
            let! _fileOpenResult =
                server2.job_null (server2.supervisor *<+ SupervisorReq.FileOpen fileOpenArgs)
                |> Async.AwaitTask

            // do! Async.Sleep 60

            let fileTokenRangeArgs =
                {|
                    uri = fullPathUri
                    range =
                        {|
                            line = 0
                            character = 0
                        |},
                        {|
                            line = lines.Length - 1
                            character = lines.[lines.Length - 1].Length
                        |}
                |}
            // let! fileTokenRangeResult =
            //     fileTokenRangeObj
            //     |> sendObj serverPort
            //     |> Async.withCancellationToken ct

            // let fileTokenRangeArgs = {| uri = fullPathUri; backend = backendId |}
            let! fileTokenRangeResult =
                server2.job_val (fun res -> server2.supervisor *<+ SupervisorReq.FileTokenRange(fileTokenRangeArgs,res))
                |> Async.AwaitTask

            if codeDir |> SpiralSm.starts_with (workspaceRoot </> "target") then
                let fileDirUri = codeDir |> SpiralFileSystem.normalize_path |> SpiralFileSystem.new_file_uri
                // let fileDeleteObj = {| FileDelete = {| uris = [| fileDirUri |] |} |}
                // let! _fileDeleteResult = fileDeleteObj |> sendObj serverPort
                let fileDeleteArgs = {| uris = [| fileDirUri |] |}
                let! _fileDeleteResult =
                    server2.job_null (server2.supervisor *<+ SupervisorReq.FileDelete fileDeleteArgs)
                    |> Async.AwaitTask
                ()

            return fileTokenRangeResult |> FSharp.Json.Json.deserialize<int array> |> Some
    }

    /// ## getCodeTokenRange
    let getCodeTokenRange cancellationToken code = async {
        let! mainPath, _ =
            persistCode {| input = Spi (code, None); backend = None; packages = [||] |}
        return! mainPath |> getFileTokenRange None cancellationToken
    }

    /// ## getFileHoverAt
    let getFileHoverAt
        port
        cancellationToken
        path
        (position : {| line: int; character: int |})
        = async {
        let fullPath = path |> System.IO.Path.GetFullPath
        let! code = fullPath |> SpiralFileSystem.read_all_text_async
        let lines = code |> SpiralSm.split "\n"

        let struct (token, disposable) = SpiralThreading.new_disposable_token cancellationToken
        use _ = disposable

        let port = port |> Option.defaultWith getCompilerPort
        // let! serverPort, _errors, ct, disposable = awaitCompiler port (Some token)
        // use _ = disposable

        let fullPathUri = fullPath |> SpiralFileSystem.normalize_path |> SpiralFileSystem.new_file_uri

        // let fileOpenObj = {| FileOpen = {| uri = fullPathUri; spiText = code |} |}
        // let! _fileOpenResult = fileOpenObj |> sendObj serverPort
        let fileOpenArgs = {| uri = fullPathUri; spiText = code |}
        let! _fileOpenResult =
            server1.job_null (server1.supervisor *<+ SupervisorReq.FileOpen fileOpenArgs)
            |> Async.AwaitTask

        // do! Async.Sleep 60

        let hoverAtArgs =
                {|
                    uri = fullPathUri
                    pos = position
                |}

        let! hoverAtResult =
            server1.job_val (fun res -> server1.supervisor *<+ SupervisorReq.HoverAt(hoverAtArgs,res))
            |> Async.AwaitTask

        let fileDir = fullPath |> System.IO.Path.GetDirectoryName
        if fileDir |> SpiralSm.starts_with (workspaceRoot </> "target") then
            let fileDirUri = fileDir |> SpiralFileSystem.normalize_path |> SpiralFileSystem.new_file_uri
            // let fileDeleteObj = {| FileDelete = {| uris = [| fileDirUri |] |} |}
            // let! _fileDeleteResult = fileDeleteObj |> sendObj serverPort
            let fileDeleteArgs = {| uris = [| fileDirUri |] |}
            let! _fileDeleteResult =
                server1.job_null (server1.supervisor *<+ SupervisorReq.FileDelete fileDeleteArgs)
                |> Async.AwaitTask
                |> Async.runWithTimeoutAsync 60000
                |> Async.map Option.get
            ()

        return hoverAtResult |> Some
    }

    /// ## getCodeHoverAt
    let getCodeHoverAt cancellationToken code position = async {
        let! mainPath, _ =
            persistCode {| input = Spi (code, None); backend = None; packages = [||] |}

        let codeDir = mainPath |> System.IO.Path.GetDirectoryName
        let filePath = codeDir </> "hover.json"
        let! output = async {
            if filePath |> System.IO.File.Exists |> not
            then return None
            else
                let! text = filePath |> SpiralFileSystem.read_all_text_async

                return
                    if text.Length > 2
                    then text |> Some
                    else None
        }
        match output with
        | Some output ->
            return output |> Some
        | None -> return! getFileHoverAt None cancellationToken mainPath position
    }

    /// ## Arguments
    [<RequireQualifiedAccess>]
    type Arguments =
        | Build_File of string * string
        | File_Token_Range of string * string
        | File_Hover_At of string * string * int * int
        | Execute_Command of string
        | [<Argu.ArguAttributes.Unique>] Timeout of int
        | [<Argu.ArguAttributes.Unique>] Port of int
        | [<Argu.ArguAttributes.Unique>] Parallel
        | [<Argu.ArguAttributes.Unique>] Exit_On_Error

        interface Argu.IArgParserTemplate with
            member s.Usage =
                match s with
                | Build_File _ -> nameof Build_File
                | File_Token_Range _ -> nameof File_Token_Range
                | File_Hover_At _ -> nameof File_Hover_At
                | Execute_Command _ -> nameof Execute_Command
                | Timeout _ -> nameof Timeout
                | Port _ -> nameof Port
                | Parallel -> nameof Parallel
                | Exit_On_Error-> nameof Exit_On_Error

    /// ## main
    let main args =
        SpiralTrace.TraceLevel.US0_1 |> set_trace_level
        let argsMap = args |> Runtime.parseArgsMap<Arguments>

        let buildFileActions =
            argsMap
            |> Map.tryFind (nameof Arguments.Build_File)
            |> Option.defaultValue []
            |> List.choose (function
                | Arguments.Build_File (inputPath, outputPath) -> Some (inputPath, outputPath)
                | _ -> None
            )

        let fileTokenRangeActions =
            argsMap
            |> Map.tryFind (nameof Arguments.File_Token_Range)
            |> Option.defaultValue []
            |> List.choose (function
                | Arguments.File_Token_Range (inputPath, outputPath) -> Some (inputPath, outputPath)
                | _ -> None
            )

        let fileHoverAtActions =
            argsMap
            |> Map.tryFind (nameof Arguments.File_Hover_At)
            |> Option.defaultValue []
            |> List.choose (function
                | Arguments.File_Hover_At (inputPath, outputPath, line, character) ->
                    Some (inputPath, outputPath, line, character)
                | _ -> None
            )

        let executeCommandActions =
            argsMap
            |> Map.tryFind (nameof Arguments.Execute_Command)
            |> Option.defaultValue []
            |> List.choose (function
                | Arguments.Execute_Command command -> Some command
                | _ -> None
            )

        let timeout =
            match argsMap |> Map.tryFind (nameof Arguments.Timeout) with
            | Some [ Arguments.Timeout timeout ] -> timeout
            | _ -> 60002 * 60 * 24

        let port =
            match argsMap |> Map.tryFind (nameof Arguments.Port) with
            | Some [ Arguments.Port port ] -> Some port
            | _ -> None

        let isParallel = argsMap |> Map.containsKey (nameof Arguments.Parallel)

        let isExitOnError = argsMap |> Map.containsKey (nameof Arguments.Exit_On_Error)

        async {
            let port =
                port
                |> Option.defaultWith getCompilerPort
            let struct (localToken, disposable) = SpiralThreading.new_disposable_token None
            // let! serverPort, _errors, compilerToken, disposable = awaitCompiler port (Some localToken)
            let serverPort = port
            let struct (compilerToken, disposable) = SpiralThreading.new_disposable_token None
            use _ = disposable

            let buildFileAsync =
                buildFileActions
                |> List.map (fun (inputPath, outputPath) -> async {
                    let! _outputPath, outputCode, errors =
                        let backend =
                            if outputPath |> SpiralSm.ends_with ".gleam"
                            then Gleam
                            elif outputPath |> SpiralSm.ends_with ".fsx"
                            then Fsharp
                            elif outputPath |> SpiralSm.ends_with ".py"
                            then Python
                            elif outputPath |> SpiralSm.ends_with ".cpp"
                            then Cpp
                            else failwith $"Supervisor.main / invalid backend / outputPath: {outputPath}"
                        let isReal = inputPath |> SpiralSm.ends_with ".spir"
                        inputPath |> buildFile backend timeout (Some serverPort) None

                    errors
                    |> List.map snd
                    |> List.iter (fun error ->
                        trace Critical (fun () -> $"main / error: {error |> serializeObj}") _locals
                    )

                    match outputCode with
                    | Some outputCode ->
                        do! outputCode |> SpiralFileSystem.write_all_text_exists outputPath
                        return 0
                    | None ->
                        if isExitOnError
                        then SpiralRuntime.current_process_kill ()

                        return 1
                })

            let fileTokenRangeAsync =
                fileTokenRangeActions
                |> List.map (fun (inputPath, outputPath) -> async {
                    let! tokenRange = inputPath |> getFileTokenRange (Some serverPort) None
                    match tokenRange with
                    | Some tokenRange ->
                        do! tokenRange |> FSharp.Json.Json.serialize |> SpiralFileSystem.write_all_text_exists outputPath
                        return 0
                    | None ->
                        if isExitOnError
                        then SpiralRuntime.current_process_kill ()

                        return 1
                })

            let fileHoverAtAsync =
                fileHoverAtActions
                |> List.map (fun (inputPath, outputPath, line, character) -> async {
                    let! hoverAt =
                        getFileHoverAt
                            (Some serverPort)
                            None
                            inputPath
                            {| line = line; character = character |}
                    match hoverAt with
                    | Some hoverAt ->
                        do! hoverAt |> FSharp.Json.Json.serialize |> SpiralFileSystem.write_all_text_exists outputPath
                        return 0
                    | None ->
                        if isExitOnError
                        then SpiralRuntime.current_process_kill ()

                        return 1
                })

            let executeCommandAsync =
                executeCommandActions
                |> List.map (fun command -> async {
                    let! exitCode, result =
                        SpiralRuntime.execution_options (fun x ->
                            { x with
                                l0 = command
                                l1 = Some compilerToken
                            }
                        )
                        |> SpiralRuntime.execute_with_options_async

                    trace
                        Debug
                        (fun () ->
                            $"Supervisor.main / executeCommand / exitCode: {exitCode} / command: {command}"
                            + " / result"
                            + if exitCode = 0 then ".Length" else ""
                            + ": "
                            + if exitCode = 0 then result |> String.length |> string else result
                        )
                        _locals

                    if isExitOnError && exitCode <> 0
                    then SpiralRuntime.current_process_kill ()

                    return exitCode
                })

            return!
                [| buildFileAsync; fileTokenRangeAsync; fileHoverAtAsync; executeCommandAsync |]
                |> Seq.collect id
                |> fun x ->
                    if isParallel
                    then Async.Parallel (x, float System.Environment.ProcessorCount * 0.51 |> ceil |> int)
                    else Async.Sequential x
                |> Async.map Array.sum
        }
        |> Async.runWithTimeout timeout
        |> Option.defaultValue 1
