#if !INTERACTIVE
namespace Polyglot
#endif

module FileSystem =

#if !INTERACTIVE
    open Lib
#endif

    open Common
    open SpiralFileSystem.Operators

    /// ## watchDirectory
    [<RequireQualifiedAccess>]
    type FileSystemChangeType =
        | Failure
        | Changed
        | Created
        | Deleted
        | Renamed

    [<RequireQualifiedAccess>]
    type FileSystemChange =
        | Failure of exn: exn
        | Changed of path: string * content: string option
        | Created of path: string * content: string option
        | Deleted of path: string
        | Renamed of oldPath: string * (string * string option)


    let inline watchDirectoryWithFilter filter shouldReadContent path =
        let fullPath = path |> System.IO.Path.GetFullPath
        let _locals () = $"filter: {filter} / {_locals ()}"

        let watcher =
            new System.IO.FileSystemWatcher (
                Path = fullPath,
                NotifyFilter = filter,
                EnableRaisingEvents = true,
                IncludeSubdirectories = true
            )

        let inline getEventPath (path : string) =
            path |> SpiralSm.trim |> SpiralSm.replace fullPath "" |> SpiralSm.trim_start [| '/'; '\\' |]

        let inline ticks () =
            System.DateTime.UtcNow.Ticks

        let changedStream =
            AsyncSeq.subscribeEvent
                watcher.Changed
                (fun event ->
                    ticks (),
                    [ FileSystemChange.Changed (getEventPath event.FullPath, None) ]
                )

        let deletedStream =
            AsyncSeq.subscribeEvent
                watcher.Deleted
                (fun event ->
                    ticks (),
                    [ FileSystemChange.Deleted (getEventPath event.FullPath) ]
                )

        let createdStream =
            AsyncSeq.subscribeEvent
                watcher.Created
                (fun event ->
                    let path = getEventPath event.FullPath
                    ticks (), [
                        FileSystemChange.Created (path, None)
                        if SpiralPlatform.is_windows () then
                            FileSystemChange.Changed (path, None)
                    ])

        let renamedStream =
            AsyncSeq.subscribeEvent
                watcher.Renamed
                (fun event ->
                    ticks (), [
                        FileSystemChange.Renamed (
                            getEventPath event.OldFullPath,
                            (getEventPath event.FullPath, None)
                        )
                    ]
                )

        let failureStream =
            AsyncSeq.subscribeEvent
                watcher.Error
                (fun event -> ticks (), [ FileSystemChange.Failure (event.GetException ()) ])

        let stream =
            [
                changedStream
                deletedStream
                createdStream
                renamedStream
                failureStream
            ]
            |> FSharp.Control.AsyncSeq.mergeAll
            |> FSharp.Control.AsyncSeq.map (fun (t, events) ->
                events
                |> List.fold
                    (fun (i, events) event ->
                        i + 1L,
                        (t + i, event) :: events)
                    (0L, [])
                |> snd
                |> List.rev
            )
            |> FSharp.Control.AsyncSeq.concatSeq
            |> FSharp.Control.AsyncSeq.mapAsyncParallel (fun (t, event) -> async {
                match shouldReadContent event, event with
                | true, FileSystemChange.Changed (path, _) ->
                    do! Async.Sleep 5
                    let! content = fullPath </> path |> SpiralFileSystem.read_all_text_retry_async
                    return t, FileSystemChange.Changed (path, content)
                | true, FileSystemChange.Created (path, _) ->
                    do! Async.Sleep 5
                    let! content = fullPath </> path |> SpiralFileSystem.read_all_text_retry_async
                    return t, FileSystemChange.Created (path, content)
                | true, FileSystemChange.Renamed (oldPath, (newPath, _)) ->
                    let! content = fullPath </> newPath |> SpiralFileSystem.read_all_text_retry_async
                    return t, FileSystemChange.Renamed (oldPath, (newPath, content))
                | _ -> return t, event
            })

        let disposable =
            new_disposable (fun () ->
                trace Debug (fun () -> "FileSystem.watchWithFilter / Disposing watch stream") _locals
                watcher.EnableRaisingEvents <- false
                watcher.Dispose ()
            )

        stream, disposable

    let inline watchDirectory path =
        watchDirectoryWithFilter
            (System.IO.NotifyFilters.FileName
            // ||| System.IO.NotifyFilters.DirectoryName
            // ||| System.IO.NotifyFilters.Attributes
            //// ||| System.IO.NotifyFilters.Size
            ||| System.IO.NotifyFilters.LastWrite
            //// ||| System.IO.NotifyFilters.LastAccess
            // ||| System.IO.NotifyFilters.CreationTime
            // ||| System.IO.NotifyFilters.Security
            )
            path
