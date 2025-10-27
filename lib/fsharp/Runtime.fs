#if !INTERACTIVE
namespace Polyglot
#endif

module Runtime =

#if !INTERACTIVE
    open Lib
#endif

    open Common

    /// ## parseArgs
    let inline parseArgs<'T when 'T :> Argu.IArgParserTemplate> args =
        let assemblyName = System.Reflection.Assembly.GetEntryAssembly().GetName().Name
        let errorHandler : Argu.IExiter =
            if [ "Microsoft.DotNet.Interactive.App"; "dotnet-repl" ] |> List.contains assemblyName
            then Argu.ExceptionExiter ()
            else Argu.ProcessExiter (function Argu.ErrorCode.HelpText -> None | _ -> Some System.ConsoleColor.Red)

        let parser =
            Argu.ArgumentParser.Create<'T> (
                programName = $"{assemblyName}{SpiralPlatform.get_executable_suffix ()}",
                errorHandler = errorHandler
            )

        parser.ParseCommandLine args

    let inline parseAllArgs<'T when 'T :> Argu.IArgParserTemplate> args =
        args
        |> parseArgs<'T>
        |> fun results -> results.GetAllResults ()

    let inline parseArgsMap<'T when 'T :> Argu.IArgParserTemplate> args =
        args
        |> parseAllArgs<'T>
        |> List.groupBy CommonFSharp.getUnionCaseName<'T>
        |> Map.ofList
