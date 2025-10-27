#if !INTERACTIVE
namespace Polyglot
#endif

module DibParser =

#if !INTERACTIVE
    open Lib
#endif

    open Common
    open FParsec
    open SpiralFileSystem.Operators

    /// ## magicMarker
    let magicMarker : Parser<string, unit> = pstring "#!"

    /// ## magicCommand
    let magicCommand =
        magicMarker
        >>. manyTill anyChar newline
        |>> (System.String.Concat >> SpiralSm.trim)

    /// ## content
    let content =
        (newline >>. magicMarker) <|> (eof >>. preturn "")
        |> attempt
        |> lookAhead
        |> manyTill anyChar
        |>> (System.String.Concat >> SpiralSm.trim)

    /// ## Output
    type Output =
        | Fs
        | Md
        | Spi
        | Spir

    /// ## Magic
    type Magic =
        | Fsharp
        | Markdown
        | Spiral of Output
        | Magic of string

    /// ## kernelOutputs
    let inline kernelOutputs magic =
        match magic with
        | Fsharp -> [ Fs ]
        | Markdown -> [ Md ]
        | Spiral output -> [ output ]
        | _ -> []

    /// ## Block
    type Block =
        {
            magic : Magic
            content : string
        }

    /// ## block
    let block =
        pipe2
            magicCommand
            content
            (fun magic content ->
                let magic, content =
                    match magic with
                    | "fsharp" -> Fsharp, content
                    | "markdown" -> Markdown, content
                    | "spiral" ->
                        let output = if content |> SpiralSm.contains "//// real\n" then Spir else Spi
                        let content =
                            if output = Spi
                            then content
                            else
                                content
                                |> SpiralSm.replace "//// real\n\n" ""
                                |> SpiralSm.replace "//// real\n" ""
                        Spiral output, content
                    | magic -> magic |> Magic, content
                {
                    magic = magic
                    content = content
                })

    /// ## blocks
    let blocks =
        skipMany newline
        >>. sepEndBy block (skipMany1 newline)

    /// ## formatBlock
    let inline formatBlock output (block : Block) =
        match output, block with
        | output, { magic = Markdown; content = content } ->
            let markdownComment =
                match output with
                | Spi | Spir -> "/// "
                | Fs -> "/// "
                | _ -> ""
            content
            |> SpiralSm.split "\n"
            |> Array.map (SpiralSm.trim_end [||])
            |> Array.filter (SpiralSm.ends_with " (test)" >> not)
            |> Array.map (function
                | "" -> markdownComment
                | line -> System.Text.RegularExpressions.Regex.Replace (line, "^\\s*", $"$&{markdownComment}")
            )
            |> SpiralSm.concat "\n"
        | Fs, { magic = Fsharp; content = content } ->
            let trimmedContent = content |> SpiralSm.trim
            if trimmedContent |> SpiralSm.contains "//// test\n"
                || trimmedContent |> SpiralSm.contains "//// ignore\n"
            then ""
            else
                content
                |> SpiralSm.split "\n"
                |> Array.filter (SpiralSm.trim_start [||] >> SpiralSm.starts_with "#r" >> not)
                |> SpiralSm.concat "\n"
        | (Spi | Spir), { magic = Spiral output'; content = content } when output' = output ->
            let trimmedContent = content |> SpiralSm.trim
            if trimmedContent |> SpiralSm.contains "//// test\n"
                || trimmedContent |> SpiralSm.contains "//// ignore\n"
            then ""
            else content
        | _ -> ""

    /// ## formatBlocks
    let inline formatBlocks output blocks =
        blocks
        |> List.map (fun block ->
            block, formatBlock output block
        )
        |> List.filter (snd >> (<>) "")
        |> fun list ->
            (list, (None, []))
            ||> List.foldBack (fun (block, content) (lastMagic, acc) ->
                let lineBreak =
                    if block.magic = Markdown && lastMagic <> Some Markdown && lastMagic <> None
                    then ""
                    else "\n"
                Some block.magic, $"{content}{lineBreak}" :: acc
            )
        |> snd
        |> SpiralSm.concat "\n"

    /// ## indentBlock
    let inline indentBlock (block : Block) =
        { block with
            content =
                block.content
                |> SpiralSm.split "\n"
                |> Array.fold
                    (fun (lines, isMultiline) line ->
                        let trimmedLine = line |> SpiralSm.trim
                        if trimmedLine = ""
                        then "" :: lines, isMultiline
                        else
                            let inline singleQuoteLine () =
                                trimmedLine |> Seq.sumBy ((=) '"' >> System.Convert.ToInt32) = 1
                                && trimmedLine |> SpiralSm.contains @"'""'" |> not
                                && trimmedLine |> SpiralSm.ends_with "{" |> not
                                && trimmedLine |> SpiralSm.ends_with "{|" |> not
                                && trimmedLine |> SpiralSm.starts_with "}" |> not
                                && trimmedLine |> SpiralSm.starts_with "|}" |> not

                            match isMultiline, trimmedLine |> SpiralSm.split_string [| $"{q}{q}{q}" |] with
                            | false, [| _; _ |] ->
                                $"    {line}" :: lines, true

                            | true, [| _; _ |] ->
                                line :: lines, false

                            | false, _ when singleQuoteLine () ->
                                $"    {line}" :: lines, true

                            | false, _ when line |> SpiralSm.starts_with "#" && block.magic = Fsharp ->
                                line :: lines, false

                            | false, _ ->
                                $"    {line}" :: lines, false

                            | true, _ when singleQuoteLine () && line |> SpiralSm.starts_with "    " ->
                                $"    {line}" :: lines, false

                            | true, _ when singleQuoteLine () ->
                                line :: lines, false

                            | true, _ ->
                                line :: lines, true
                    )
                    ([], false)
                |> fst
                |> List.rev
                |> SpiralSm.concat "\n"
        }

    /// ## parse
    let inline parse output input =
        match run blocks input with
        | Success (blocks, _, _) ->
            blocks
            |> List.filter (fun block ->
                block.magic |> kernelOutputs |> List.contains output || block.magic = Markdown
            )
            |> List.map Some
            |> fun x -> x @ [ None ]
            |> List.pairwise
            |> List.choose (function
                | Some { magic = Markdown } as block, Some { magic = Markdown } -> block
                | Some { magic = Markdown } as block, Some { magic = magic }
                    when magic |> kernelOutputs |> List.contains output -> block
                | Some { magic = Markdown } as block, _ when output = Md -> block
                | Some { magic = Markdown }, _ -> None
                | Some block, _ -> Some block
                | _ -> None
            )
            |> List.fold
                (fun (acc, indent) -> function
                    | { magic = Markdown; content = content }
                        when output = Fs
                        && content |> SpiralSm.starts_with "# "
                        && content |> SpiralSm.ends_with ")"
                        ->
                        let moduleName, namespaceName =
                            System.Text.RegularExpressions.Regex.Match (content, @"# (.*) \((.*)\)$")
                            |> fun m -> m.Groups.[1].Value, m.Groups.[2].Value

                        let moduleBlock =
                            {
                                magic = Fsharp
                                content =
                                    $"#if !INTERACTIVE
namespace {namespaceName}
#endif

module {moduleName} ="
                            }

                        moduleBlock :: acc, (indent + 1)
                    | { magic = magic ; content = content } as block
                        when indent > 0
                        ->
                        indentBlock block :: acc, indent
                    | block -> block :: acc, indent
                )
                ([], 0)
            |> fst
            |> List.rev
            |> Result.Ok
        | Failure (errorMsg, _, _) -> Result.Error errorMsg

    /// ## parseDibCode
    let inline parseDibCode output file = async {
        trace Debug
            (fun () -> "parseDibCode")
            (fun () -> $"output: {output} / file: {file} / {_locals ()}")
        let! input = file |> SpiralFileSystem.read_all_text_async
        match parse output input with
        | Result.Ok blocks -> return blocks |> formatBlocks output
        | Result.Error msg -> return failwith msg
    }

    /// ## writeDibCode
    let inline writeDibCode output path = async {
        trace Debug
            (fun () -> "writeDibCode")
            (fun () -> $"output: {output} / path: {path} / {_locals ()}")
        let! result = parseDibCode output path
        let pathDir = path |> System.IO.Path.GetDirectoryName
        let fileNameWithoutExt =
            match output, path |> System.IO.Path.GetFileNameWithoutExtension with
            | Spir, fileNameWithoutExt -> $"{fileNameWithoutExt}_real"
            | _, fileNameWithoutExt -> fileNameWithoutExt
        let outputPath = pathDir </> $"{fileNameWithoutExt}.{output |> string |> SpiralSm.to_lower}"
        do! result |> SpiralFileSystem.write_all_text_async outputPath
    }

    /// ## Arguments
    [<RequireQualifiedAccess>]
    type Arguments =
        | [<Argu.ArguAttributes.MainCommand; Argu.ArguAttributes.Mandatory>]
            File of file : string * Output

        interface Argu.IArgParserTemplate with
            member s.Usage =
                match s with
                | File _ -> nameof File

    /// ## main
    let main args =
        let argsMap = args |> Runtime.parseArgsMap<Arguments>

        let files =
            argsMap.[nameof Arguments.File]
            |> List.map (function
                | Arguments.File (path, output) -> path, output
            )

        files
        |> List.map (fun (path, output) -> path |> writeDibCode output)
        |> Async.Parallel
        |> Async.Ignore
        |> Async.runWithTimeout 30000
        |> function
            | Some () -> 0
            | None -> 1
