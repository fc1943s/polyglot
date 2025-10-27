#if !INTERACTIVE
namespace Polyglot
#endif

module JsonParser =

    open Common
    open Parser

    /// ## JsonParser
    (*
    // --------------------------------
    JSON spec from http://www.json.org/
    // --------------------------------

    The JSON spec is available at [json.org](http://www.json.org/). I'll paraphase it here:

    * A `value` can be a `string` or a `number` or a `bool` or `null` or an `object` or an `array`.
      * These structures can be nested.
    * A `string` is a sequence of zero or more Unicode characters, wrapped in double quotes, using backslash escapes.
    * A `number` is very much like a C or Java number, except that the octal and hexadecimal formats are not used.
    * A `boolean` is the literal `true` or `false`
    * A `null` is the literal `null`
    * An `object` is an unordered set of name/value pairs.
      * An object begins with { (left brace) and ends with } (right brace).
      * Each name is followed by : (colon) and the name/value pairs are separated by , (comma).
    * An `array` is an ordered collection of values.
      * An array begins with [ (left bracket) and ends with ] (right bracket).
      * Values are separated by , (comma).
    * Whitespace can be inserted between any pair of tokens.
    *)

    /// ### JValue
    type JValue =
        | JString of string
        | JNumber of float
        | JBool   of bool
        | JNull
        | JObject of Map<string, JValue>
        | JArray  of JValue list

    let jValue, jValueRef = createParserForwardedToRef<JValue> ()

    /// ### jNull
    let jNull =
        pstring "null"
        >>% JNull
        <?> "null"

    /// ### jBool
    let jBool =
        let jtrue =
            pstring "true"
            >>% JBool true
        let jfalse =
            pstring "false"
            >>% JBool false

        jtrue <|> jfalse
        <?> "bool"

    /// ### jUnescapedChar
    let jUnescapedChar =
        satisfy (fun ch -> ch <> '\\' && ch <> '\"') "char"

    /// ### jEscapedChar
    let jEscapedChar =
        [
            ("\\\"",'\"')
            ("\\\\",'\\')
            ("\\/",'/')
            ("\\b",'\b')
            ("\\f",'\f')
            ("\\n",'\n')
            ("\\r",'\r')
            ("\\t",'\t')
        ]
        |> List.map (fun (toMatch, result) ->
            pstring toMatch >>% result
        )
        |> choice
        <?> "escaped char"

    /// ### jUnicodeChar
    let jUnicodeChar =
        let backslash = pchar '\\'
        let uChar = pchar 'u'
        let hexdigit = anyOf ([ '0' .. '9' ] @ [ 'A' .. 'F' ] @ [ 'a' .. 'f' ])
        let fourHexDigits = hexdigit .>>. hexdigit .>>. hexdigit .>>. hexdigit

        let inline convertToChar (((h1, h2), h3), h4) =
            let str = $"%c{h1}%c{h2}%c{h3}%c{h4}"
            Int32.Parse (str, Globalization.NumberStyles.HexNumber) |> char

        backslash >>. uChar >>. fourHexDigits
        |>> convertToChar

    /// ### jString
    let quotedString =
        let quote = pchar '\"' <?> "quote"
        let jchar = jUnescapedChar <|> jEscapedChar <|> jUnicodeChar

        quote >>. manyChars jchar .>> quote

    let jString =
        quotedString
        |>> JString
        <?> "quoted string"

    /// ### jNumber
    let jNumber =
        let optSign = opt (pchar '-')

        let zero = pstring "0"

        let digitOneNine =
            satisfy (fun ch -> Char.IsDigit ch && ch <> '0') "1-9"

        let digit =
            satisfy Char.IsDigit "digit"

        let point = pchar '.'

        let e = pchar 'e' <|> pchar 'E'

        let optPlusMinus = opt (pchar '-' <|> pchar '+')

        let nonZeroInt =
            digitOneNine .>>. manyChars digit
            |>> fun (first, rest) -> string first + rest

        let intPart = zero <|> nonZeroInt

        let fractionPart = point >>. manyChars1 digit

        let exponentPart = e >>. optPlusMinus .>>. manyChars1 digit

        let inline (|>?) opt f =
            match opt with
            | None -> ""
            | Some x -> f x

        let inline convertToJNumber (((optSign, intPart), fractionPart), expPart) =
            let signStr =
                optSign
                |>? string

            let fractionPartStr =
                fractionPart
                |>? (fun digits -> "." + digits)

            let expPartStr =
                expPart
                |>? fun (optSign, digits) ->
                    let sign = optSign |>? string
                    "e" + sign + digits

            (signStr + intPart + fractionPartStr + expPartStr)
            |> float
            |> JNumber

        optSign .>>. intPart .>>. opt fractionPart .>>. opt exponentPart
        |>> convertToJNumber
        <?> "number"

    /// ### jArray
    let jArray =
        let left = pchar '[' .>> spaces
        let right = pchar ']' .>> spaces
        let comma = pchar ',' .>> spaces
        let value = jValue .>> spaces

        let values = sepBy value comma

        between left values right
        |>> JArray
        <?> "array"

    /// ### jObject
    let jObject =
        let left = spaces >>. pchar '{' .>> spaces
        let right = pchar '}' .>> spaces
        let colon = pchar ':' .>> spaces
        let comma = pchar ',' .>> spaces
        let key = quotedString .>> spaces
        let value = jValue .>> spaces

        let keyValue = (key .>> colon) .>>. value
        let keyValues = sepBy keyValue comma

        between left keyValues right
        |>> Map.ofList
        |>> JObject
        <?> "object"

    jValueRef <|
        choice
            [
                jNull
                jBool
                jString
                jNumber
                jArray
                jObject
            ]

    /// ### jValue
