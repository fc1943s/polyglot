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

    /// ### to_trace_level
    let to_trace_level = function
        | Verbose -> SpiralTrace.TraceLevel.Verbose
        | Debug -> SpiralTrace.TraceLevel.Debug
        | Info -> SpiralTrace.TraceLevel.Info
        | Warning -> SpiralTrace.TraceLevel.Warning
        | Critical -> SpiralTrace.TraceLevel.Critical

    /// ### from_trace_level
    let from_trace_level = function
        | SpiralTrace.TraceLevel.Verbose -> Verbose
        | SpiralTrace.TraceLevel.Debug -> Debug
        | SpiralTrace.TraceLevel.Info -> Info
        | SpiralTrace.TraceLevel.Warning -> Warning
        | SpiralTrace.TraceLevel.Critical -> Critical

    /// ### trace
    let trace level fn locals =
        let level = level |> to_trace_level
        SpiralTrace.trace level fn locals
