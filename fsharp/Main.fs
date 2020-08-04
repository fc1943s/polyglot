﻿namespace Polyglot.FSharp

open System

module Main =
    open Model

    [<EntryPoint>]
    let main _ =
        ChallengeList.challenges
        |> List.iter (fun challenge ->
            let table =
                let rows =
                    let resultList = challenge () |> Seq.toList
                    resultList
                    |> List.map (fun result ->
                        let best =
                            result.TimeList
                            |> List.mapi (fun i time ->
                                i + 1, time
                            )
                            |> List.sortBy snd
                            |> List.head
                            |> fun x -> x.ToString ()
                        let row =
                            [ result.Input
                              result.Expected
                              result.Result
                              best ]
                        let color =
                            match result.Expected = result.Result with
                            | true -> Some ConsoleColor.DarkGreen
                            | false -> Some ConsoleColor.DarkRed
                        row, color
                    )
                let header =
                    [ [ "Input"
                        "Expected"
                        "Result"
                        "Best" ]
                      [ "---"
                        "---"
                        "---"
                        "---" ] ]
                    |> List.map (fun row -> row, None)
                header @ rows

            let formattedTable =
                let lengthMap =
                    table
                    |> List.map fst
                    |> List.transpose
                    |> List.map (fun column ->
                        column
                        |> List.map String.length
                        |> List.sortDescending
                        |> List.tryHead
                        |> Option.defaultValue 0
                    )
                    |> List.indexed
                    |> Map.ofList
                table
                |> List.map (fun (row, color) ->
                    let newRow =
                        row
                        |> List.mapi (fun i cell ->
                            cell.PadRight lengthMap.[i]
                        )
                    newRow, color
                )

            printfn ""
            formattedTable
            |> List.iter (fun (row, color) ->
                match color with
                | Some color -> Console.ForegroundColor <- color
                | None -> Console.ResetColor ()

                printfn "%s" (String.Join ("\t", row))

                Console.ResetColor ()
            )
        )
        0

