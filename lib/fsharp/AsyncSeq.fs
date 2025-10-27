#if !INTERACTIVE
namespace Polyglot
#endif

module AsyncSeq =

#if !INTERACTIVE
    open Lib
#endif

    open Common

    /// ## subscribeEvent
    let inline subscribeEvent (event: IEvent<'H, 'A>) map =
        let observable = System.Reactive.Linq.Observable.FromEventPattern<'H, 'A>(event.AddHandler, event.RemoveHandler)
        System.Reactive.Linq.Observable.Select (observable, fun event -> map event.EventArgs)
        |> FSharp.Control.AsyncSeq.ofObservableBuffered

    /// ## subscribeToken
    let subscribeToken (token : System.Threading.CancellationToken) =
        let tcs = new System.Threading.Tasks.TaskCompletionSource ()
        System.Action tcs.SetResult |> token.Register |> ignore
        let start = System.DateTime.Now.Ticks
        FSharp.Control.AsyncSeq.unfoldAsync
            (fun () -> async {
                do! tcs.Task |> Async.AwaitTask
                return Some (System.DateTime.Now.Ticks - start, ())
            })
            ()
