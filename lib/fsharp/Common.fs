#if !INTERACTIVE
namespace Polyglot
#endif

module Common =

#if !INTERACTIVE
    open Lib
#endif

    let nl = System.Environment.NewLine
    let q = @""""

    let inline cons head tail = head :: tail

    /// ## memoize
    let inline memoize fn =
        let result = lazy fn ()
        fun () -> result.Value

    /// ## TraceLevel
    type TraceLevel =
        | Verbose
        | Debug
        | Info
        | Warning
        | Critical

    let inline _locals () = ""

    /// ## trace
    let to_trace_level = function
        | Verbose -> SpiralTrace.TraceLevel.US0_0
        | Debug -> SpiralTrace.TraceLevel.US0_1
        | Info -> SpiralTrace.TraceLevel.US0_2
        | Warning -> SpiralTrace.TraceLevel.US0_3
        | Critical -> SpiralTrace.TraceLevel.US0_4

    let trace level fn locals =
        let level = level |> to_trace_level
        SpiralTrace.trace level fn locals
