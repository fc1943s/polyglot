#if !INTERACTIVE
namespace Polyglot
#endif

module Async =

#if !INTERACTIVE
    open Lib
#endif

    open Common

    /// ## choice
    let inline choice asyncs = async {
        let e = Event<_> ()
        use cts = new System.Threading.CancellationTokenSource ()
        let fn =
            asyncs
            |> Seq.map (fun a -> async {
                let! x = a
                e.Trigger x
            })
            |> Async.Parallel
            |> Async.Ignore
        Async.Start (fn, cts.Token)
        let! result = Async.AwaitEvent e.Publish
        cts.Cancel ()
        return result
    }

    /// ## map
    let inline map fn a = async {
        let! x = a
        return fn x
    }

    /// ## runWithTimeoutChoiceAsync
    let inline runWithTimeoutChoiceAsync (timeout : int) fn =
        let _locals () = $"timeout: {timeout} / {_locals ()}"

        let timeoutTask = async {
            do! Async.Sleep timeout
            trace Debug (fun () -> "runWithTimeoutChoiceAsync") _locals
            return None
        }

        let task = async {
            try
                let! result = fn
                return Some result
            with
            | :? System.AggregateException as ex when
                ex.InnerExceptions
                |> Seq.exists (function :? System.Threading.Tasks.TaskCanceledException -> true | _ -> false)
                ->
                trace Warning
                    (fun () -> "runWithTimeoutChoiceAsync")
                    (fun () -> $"ex: {ex |> SpiralSm.format_exception} / {_locals ()}")
                return None
            | ex ->
                trace Critical
                    (fun () -> "runWithTimeoutChoiceAsync")
                    (fun () -> $"ex: {ex |> SpiralSm.format_exception} / {_locals ()}")
                return None
        }

        [ timeoutTask; task ]
        |> choice

    let inline runWithTimeoutChoice timeout fn =
        fn
        |> runWithTimeoutChoiceAsync timeout
        |> Async.RunSynchronously

    /// ## catch
    let inline catch a =
        a
        |> Async.Catch
        |> map (function
            | Choice1Of2 result -> Ok result
            | Choice2Of2 ex -> Error ex
        )

    /// ## runWithTimeoutAsync
    let inline runWithTimeoutAsync (timeout : int) fn = async {
        let _locals () = $"timeout: {timeout} / {_locals ()}"
        let! child = Async.StartChild (fn, timeout)
        return!
            child
            |> catch
            |> map (function
                | Ok result -> Some result
                | Error (:? System.TimeoutException as ex) ->
                    trace Debug (fun () -> $"Async.runWithTimeoutAsync") _locals
                    None
                | Error ex ->
                    trace Critical (fun () -> $"Async.runWithTimeoutAsync** / ex: %A{ex}") _locals
                    None
            )
    }

    let inline runWithTimeout timeout fn =
        fn
        |> runWithTimeoutAsync timeout
        |> Async.RunSynchronously

    /// ## runWithTimeoutStrict
    let inline runWithTimeoutStrict (timeout : int) fn =
        let _locals () = $"timeout: {timeout} / {_locals ()}"

        let timeoutTask = async {
            do! Async.Sleep timeout
            return None, _locals
        }

        let task = async {
            try
                return Async.RunSynchronously (fn, timeout) |> Some, _locals
            with
            | :? System.TimeoutException as ex ->
                let _locals () = $"ex: {ex |> SpiralSm.format_exception} / {_locals ()}"
                return None, _locals
            | ex ->
                trace Critical
                    (fun () -> "Async.runWithTimeoutStrict / async error")
                    (fun () -> $"ex: {ex |> SpiralSm.format_exception} / {_locals ()}")
                return raise ex
        }

        try
            [| timeoutTask; task |]
            |> Array.map Async.StartAsTask
            |> System.Threading.Tasks.Task.WhenAny
            |> fun task ->
                match task.Result.Result with
                | None, _locals ->
                    trace Debug (fun () -> "runWithTimeoutStrict") _locals
                    None
                | result, _ -> result
        with
        | :? System.AggregateException as ex when
            ex.InnerExceptions
            |> Seq.exists (function :? System.Threading.Tasks.TaskCanceledException -> true | _ -> false)
            ->
            trace Warning
                (fun () -> "Async.runWithTimeoutStrict")
                (fun () -> $"ex: {ex |> SpiralSm.format_exception} / {_locals ()}")
            None
        | ex ->
            trace Critical
                (fun () -> "Async.runWithTimeoutStrict / task error")
                (fun () -> $"ex: {ex |> SpiralSm.format_exception} / {_locals ()}")
            None

    /// ## awaitValueTask
    let inline awaitValueTaskUnit (task : System.Threading.Tasks.ValueTask) =
        task.AsTask () |> Async.AwaitTask

    let inline awaitValueTask (task : System.Threading.Tasks.ValueTask<_>) =
        task.AsTask () |> Async.AwaitTask

    /// ## init
    let inline init x = async {
        return x
    }

    /// ## withCancellationToken
    let inline withCancellationToken (ct : System.Threading.CancellationToken) fn =
        Async.StartImmediateAsTask (fn, ct)
        |> Async.AwaitTask

    /// ## retryAsync
    let inline retryAsync retries fn =
        let rec 루프 retry lastError = async {
            try
                return!
                    if retry <= retries
                    then fn |> map Ok
                    else lastError |> Error |> init
            with ex ->
                trace Debug (fun () -> $"Async.retryAsync / retry: {retry}/{retries} / ex: {ex |> SpiralSm.format_exception}") _locals
                do! Async.Sleep 30
                return! 루프 (retry + 1) (ex |> SpiralSm.format_exception)
        }
        루프 1 "Async.retryAsync / invalid retries / retries: {retries}"

    /// ## fold
    let fold folder state array =
        let rec 루프 acc i =
            async {
                if i < Array.length array then
                    let! newAcc = folder acc array.[i]
                    return! 루프 newAcc (i + 1)
                else
                    return acc
            }
        루프 state 0
