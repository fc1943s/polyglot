#if !INTERACTIVE
namespace Polyglot
#endif

module Parser =

    open Common

    /// ### TextInput
    type Position =
        {
            line : int
            column : int
        }

    let initialPos = { line = 0; column = 0 }

    let inline incrCol (pos : Position) =
        { pos with column = pos.column + 1 }

    let inline incrLine pos =
        { line = pos.line + 1; column = 0 }

    type InputState =
        {
            lines : string[]
            position : Position
        }

    let inline fromStr str =
        {
            lines =
                if str |> String.IsNullOrEmpty
                then [||]
                else str |> SpiralSm.split_string [| "\r\n"; "\n" |]
            position = initialPos
        }

    let inline currentLine inputState =
        let linePos = inputState.position.line
        if linePos < inputState.lines.Length
        then inputState.lines.[linePos]
        else "end of file"

    let inline nextChar input =
        let linePos = input.position.line
        let colPos = input.position.column

        if linePos >= input.lines.Length
        then input, None
        else
            let currentLine = currentLine input
            if colPos < currentLine.Length then
                let char = currentLine.[colPos]
                let newPos = incrCol input.position
                let newState = { input with position = newPos }
                newState, Some char
            else
                let char = '\n'
                let newPos = incrLine input.position
                let newState = { input with position = newPos }
                newState, Some char

    /// ### Parser
    type Input = InputState
    type ParserLabel = string
    type ParserError = string

    type ParserPosition =
        {
            currentLine : string
            line : int
            column : int
        }

    type ParseResult<'a> =
        | Success of 'a
        | Failure of ParserLabel * ParserError * ParserPosition

    type Parser<'a> =
        {
            label : ParserLabel
            parseFn : Input -> ParseResult<'a * Input>
        }

    let inline printResult result =
        match result with
        | Success (value, input) ->
            printfn $"%A{value}"
        | Failure (label, error, parserPos) ->
            let errorLine = parserPos.currentLine
            let colPos = parserPos.column
            let linePos = parserPos.line
            let failureCaret = $"{' ' |> string |> String.replicate colPos}^{error}"
            printfn $"Line:%i{linePos} Col:%i{colPos} Error parsing %s{label}\n%s{errorLine}\n%s{failureCaret}"

    let inline runOnInput parser input =
        parser.parseFn input

    let inline run parser inputStr =
        runOnInput parser (fromStr inputStr)

    let inline parserPositionFromInputState (inputState : Input) =
        {
            currentLine = currentLine inputState
            line = inputState.position.line
            column = inputState.position.column
        }

    let inline getLabel parser =
        parser.label

    let inline setLabel parser newLabel =
        {
            label = newLabel
            parseFn = fun input ->
                match parser.parseFn input with
                | Success s -> Success s
                | Failure (oldLabel, err, pos) -> Failure (newLabel, err, pos)
        }

    let (<?>) = setLabel

    let inline satisfy predicate label =
        {
            label = label
            parseFn = fun input ->
                let remainingInput, charOpt = nextChar input
                match charOpt with
                | None ->
                    let err = "No more input"
                    let pos = parserPositionFromInputState input
                    Failure (label, err, pos)
                | Some first ->
                    if predicate first
                    then Success (first, remainingInput)
                    else
                        let err = $"Unexpected '%c{first}'"
                        let pos = parserPositionFromInputState input
                        Failure (label, err, pos)
        }

    let inline bindP f p =
        {
            label = "unknown"
            parseFn = fun input ->
                match runOnInput p input with
                | Failure (label, err, pos) -> Failure (label, err, pos)
                | Success (value1, remainingInput) -> runOnInput (f value1) remainingInput
        }

    let inline (>>=) p f = bindP f p

    let inline returnP x =
        {
            label = $"%A{x}"
            parseFn = fun input -> Success (x, input)
        }

    let inline mapP f =
        bindP (f >> returnP)

    let (<!>) = mapP

    let inline (|>>) x f = f <!> x

    let inline applyP fP xP =
        fP >>=
            fun f ->
                xP >>=
                    fun x ->
                        returnP (f x)

    let (<*>) = applyP

    let inline lift2 f xP yP =
        returnP f <*> xP <*> yP

    let inline andThen p1 p2 =
        p1 >>=
            fun p1Result ->
                p2 >>=
                    fun p2Result ->
                        returnP (p1Result, p2Result)
        <?> $"{getLabel p1} andThen {getLabel p2}"

    let (.>>.) = andThen

    let inline orElse p1 p2 =
        {
            label = $"{getLabel p1} orElse {getLabel p2}"
            parseFn = fun input ->
                match runOnInput p1 input with
                | Success _ as result -> result
                | Failure _ -> runOnInput p2 input
        }

    let (<|>) = orElse

    let inline choice listOfParsers =
        listOfParsers |> List.reduce (<|>)

    let rec sequence parserList =
        match parserList with
        | [] -> returnP []
        | head :: tail -> (lift2 cons) head (sequence tail)

    let rec parseZeroOrMore parser input =
        match runOnInput parser input with
        | Failure (_, _, _) ->
            [], input
        | Success (firstValue, inputAfterFirstParse) ->
            let subsequentValues, remainingInput = parseZeroOrMore parser inputAfterFirstParse
            firstValue :: subsequentValues, remainingInput

    let inline many parser =
        {
            label = $"many {getLabel parser}"
            parseFn = fun input -> Success (parseZeroOrMore parser input)
        }

    let inline many1 p =
        p >>=
            fun head ->
                many p >>=
                    fun tail ->
                        returnP (head :: tail)
        <?> $"many1 {getLabel p}"

    let inline opt p =
        let some = p |>> Some
        let none = returnP None
        (some <|> none)
        <?> $"opt {getLabel p}"

    let inline (.>>) p1 p2 =
        p1 .>>. p2
        |> mapP fst

    let inline (>>.) p1 p2 =
        p1 .>>. p2
        |> mapP snd

    let inline between p1 p2 p3 =
        p1 >>. p2 .>> p3

    let inline sepBy1 p sep =
        let sepThenP = sep >>. p
        p .>>. many sepThenP
        |>> fun (p, pList) -> p :: pList

    let inline sepBy p sep =
        sepBy1 p sep <|> returnP []

    let inline pchar charToMatch =
        satisfy ((=) charToMatch) $"%c{charToMatch}"

    let inline anyOf listOfChars =
        listOfChars
        |> List.map pchar
        |> choice
        <?> $"anyOf %A{listOfChars}"

    let inline charListToStr charList =
        charList |> List.toArray |> String

    let inline manyChars cp =
        many cp
        |>> charListToStr

    let inline manyChars1 cp =
        many1 cp
        |>> charListToStr

    let inline pstring str =
        str
        |> List.ofSeq
        |> List.map pchar
        |> sequence
        |> mapP charListToStr
        <?> str

    let whitespaceChar =
        satisfy Char.IsWhiteSpace "whitespace"

    let spaces = many whitespaceChar

    let spaces1 = many1 whitespaceChar

    let digitChar =
        satisfy Char.IsDigit "digit"

    let pint =
        let inline resultToInt (sign, digits) =
            let i = int digits
            match sign with
            | Some ch -> -i
            | None -> i

        let digits = manyChars1 digitChar

        opt (pchar '-') .>>. digits
        |> mapP resultToInt
        <?> "integer"

    let pfloat =
        let inline resultToFloat (((sign, digits1), point), digits2) =
            let fl = float $"{digits1}.{digits2}"
            match sign with
            | Some ch -> -fl
            | None -> fl

        let digits = manyChars1 digitChar

        opt (pchar '-') .>>. digits .>>. pchar '.' .>>. digits
        |> mapP resultToFloat
        <?> "float"

    let inline createParserForwardedToRef<'a> () =
        let mutable parserRef : Parser<'a> =
            {
                label = "unknown"
                parseFn = fun _ -> failwith "unfixed forwarded parser"
            }

        let wrapperParser =
            { parserRef with
                parseFn = fun input -> runOnInput parserRef input
            }

        wrapperParser, (fun v -> parserRef <- v)

    let inline (>>%) p x =
        p
        |>> fun _ -> x
