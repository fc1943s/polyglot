#if !INTERACTIVE
namespace Polyglot
#endif

module DirTreeHtml =

#if !INTERACTIVE
    open Lib
#endif

    open SpiralFileSystem.Operators
    open Falco.Markup

    type FileSystemNode =
        | File of string * string * int64
        | Folder of string * string * FileSystemNode list
        | Root of FileSystemNode list

    let rec scanDirectory isRoot (basePath : string) (path : string) =
        let relativePath =
            path
            |> SpiralSm.replace basePath ""
            |> SpiralSm.replace "\\" "/"
            |> SpiralSm.replace "//" "/"
            |> SpiralSm.trim_start [| '/' |]

        let directories =
            path
            |> System.IO.Directory.GetDirectories
            |> Array.toList
            |> List.sort
            |> List.map (scanDirectory false basePath)
        let files =
            path
            |> System.IO.Directory.GetFiles
            |> Array.toList
            |> List.sort
            |> List.map (fun f -> File (System.IO.Path.GetFileName f, relativePath, System.IO.FileInfo(f).Length))

        let children = directories @ files
        if isRoot
        then Root children
        else Folder (path |> System.IO.Path.GetFileName, relativePath, children)

    let rec generateHtml fsNode =
        let sizeLabel size =
            match float size with
            | size when size > 1024.0 * 1024.0 -> $"%.2f{size / 1024.0 / 1024.0} MB"
            | size when size > 1024.0 -> $"%.2f{size / 1024.0} KB"
            | size -> $"%.2f{size} B"
        match fsNode with
        | File (fileName, relativePath, size) ->
            Elem.div [] [
                Text.raw "&#128196; "
                Elem.a [
                    Attr.href $"""{relativePath}{if relativePath = "" then "" else "/"}{fileName}"""
                ] [
                    Text.raw fileName
                ]
                Elem.span [] [
                    Text.raw $" ({size |> sizeLabel})"
                ]
            ]
        | Folder (folderName, relativePath, children) ->
            let size =
                let rec 루프 children =
                    children
                    |> List.sumBy (function
                        | File (_, _, size) -> size
                        | Folder (_, _, children)
                        | Root children -> 루프 children
                    )
                루프 children
            Elem.details [
                Attr.open' "true"
            ] [
                Elem.summary [] [
                    Text.raw "&#128194; "
                    Elem.a [
                        Attr.href relativePath
                    ] [
                        Text.raw folderName
                    ]
                    Elem.span [] [
                        Text.raw $" ({size |> sizeLabel})"
                    ]
                ]
                Elem.div [] [
                    yield! children |> List.map generateHtml
                ]
            ]
        | Root children ->
            Elem.div [] [
                yield! children |> List.map generateHtml
            ]

    let generateHtmlForFileSystem root =
        $"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <style>
body {{
    background-color: #222;
    color: #ccc;
}}
a {{
  color: #777;
  font-size: 15px;
}}
span {{
  font-size: 11px;
}}
div > div {{
  padding-left: 10px;
}}
details > div {{
  padding-left: 19px;
}}
  </style>
</head>
<body>
  {root |> generateHtml |> renderNode}
</body>
</html>
"""

    /// ## Arguments
    [<RequireQualifiedAccess>]
    type Arguments =
        | [<Argu.ArguAttributes.ExactlyOnce>] Dir of string
        | [<Argu.ArguAttributes.ExactlyOnce>] Html of string

        interface Argu.IArgParserTemplate with
            member s.Usage =
                match s with
                | Dir _ -> nameof Dir
                | Html _ -> nameof Html

    /// ## main
    let main args =
        let argsMap = args |> Runtime.parseArgsMap<Arguments>

        let dir =
            match argsMap.[nameof Arguments.Dir] with
            | [ Arguments.Dir dir ] -> Some dir
            | _ -> None
            |> Option.get

        let htmlPath =
            match argsMap.[nameof Arguments.Html] with
            | [ Arguments.Html html ] -> Some html
            | _ -> None
            |> Option.get

        let fileSystem = scanDirectory true dir dir
        let html = generateHtmlForFileSystem fileSystem

        html |> SpiralFileSystem.write_all_text_async htmlPath
        |> Async.runWithTimeout 30000
        |> function
            | Some () -> 0
            | None -> 1
