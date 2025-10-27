// #![crate_name = "test"]

mod test;

use proptest::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Weapon { name: String, damage: u32 },
    Potion { name: String, healing: u32 },
    Armor { name: String, defense: u32 },
}

impl proptest::prelude::Arbitrary for Item {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        let weapon_strategy = proptest::prelude::Strategy::prop_map(
            (
                proptest::arbitrary::any::<String>(),
                proptest::arbitrary::any::<u32>(),
            ),
            |(name, damage)| Item::Weapon { name, damage },
        );

        let potion_strategy = proptest::prelude::Strategy::prop_map(
            (
                proptest::arbitrary::any::<String>(),
                proptest::arbitrary::any::<u32>(),
            ),
            |(name, healing)| Item::Potion { name, healing },
        );

        let armor_strategy = proptest::prelude::Strategy::prop_map(
            (
                proptest::arbitrary::any::<String>(),
                proptest::arbitrary::any::<u32>(),
            ),
            |(name, defense)| Item::Armor { name, defense },
        );

        proptest::prop_oneof![weapon_strategy, potion_strategy, armor_strategy].boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct _Cart {
    items: Vec<Item>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SpiralToken {
    Identifier(String),
    Integer(i64),
    StringLiteral(String),
    Operator(String),
    Comment(String),
}

impl _Cart {
    fn _new() -> _Cart {
        _Cart { items: vec![] }
    }

    fn _add_item(&mut self, item: Item) {
        if !self.items.contains(&item) {
            self.items.push(item);
        }
    }

    fn _remove_item(&mut self, item: &Item) {
        self.items.retain(|i| i != item);
    }
}

proptest! {
    #[test]
    fn adding_and_then_removing_an_item_from_the_cart_leaves_the_cart_unchanged(
        ref item in any::<Item>(),
        ref items in prop::collection::vec(any::<Item>(), 0..30)) {

        let mut cart = _Cart::_new();
        for i in items {
            cart._add_item(i.clone());
        }
        let original_cart = cart.clone();
        cart._add_item(item.clone());
        cart._remove_item(item);
        prop_assert_eq!(cart, original_cart);
    }
}

impl proptest::prelude::Arbitrary for SpiralToken {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        let identifier_strategy = prop::string::string_regex("[a-zA-Z][a-zA-Z0-9]*")
            .unwrap()
            .prop_map(|s| SpiralToken::Identifier(s));
        let integer_strategy = any::<i64>().prop_map(|n| SpiralToken::Integer(n));
        let string_strategy = prop::string::string_regex("[ -~]*")
            .unwrap()
            .prop_map(|s| SpiralToken::StringLiteral(s.replace("\"", "").replace("\\", "")));
        let operator_strategy = prop_oneof![
            Just("+"),
            Just("-"),
            Just("*"),
            Just("/"),
            Just("="),
            Just("("),
            Just(")")
        ]
        .prop_map(|s| SpiralToken::Operator(s.to_string()));
        let comment_strategy = prop::string::string_regex("[ -~]*")
            .unwrap()
            .prop_map(|s| SpiralToken::Comment(s));

        prop_oneof![
            identifier_strategy,
            integer_strategy,
            string_strategy,
            operator_strategy,
            comment_strategy
        ]
        .boxed()
    }
}
fn _parse_comment(input: &str) -> nom::IResult<&str, SpiralToken> {
    let (input, _) = nom::bytes::complete::tag("//")(input)?;
    let (input, comment) = nom::bytes::complete::take_till(|c| c == '\n')(input)?;
    Ok((input, SpiralToken::Comment(comment.to_string())))
}

fn _parse_string(input: &str) -> nom::IResult<&str, SpiralToken> {
    let (input, _) = nom::character::complete::char('\"')(input)?;
    let (input, str_lit) = nom::combinator::opt(nom::bytes::complete::escaped(
        nom::bytes::complete::take_while1(|c: char| c != '\\' && c != '\"'),
        '\\',
        nom::character::complete::one_of("\"\\"),
    ))(input)?;
    let str_lit = match str_lit {
        Some(s) => s.replace("\\\"", "\"").replace("\\\\", "\\"),
        None => "".to_string(),
    };
    let (input, _) = nom::character::complete::char('\"')(input)?;
    Ok((input, SpiralToken::StringLiteral(str_lit)))
}

fn _parse_identifier(input: &str) -> nom::IResult<&str, SpiralToken> {
    let (input, id) = nom::character::complete::alphanumeric1(input)?;
    if id.chars().all(|c| c.is_digit(10)) {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::AlphaNumeric,
        )))
    } else {
        Ok((input, SpiralToken::Identifier(id.to_string())))
    }
}

fn _parse_integer(input: &str) -> nom::IResult<&str, SpiralToken> {
    let (input, negative) = nom::combinator::opt(nom::character::complete::char('-'))(input)?;
    let (input, digits) = nom::character::complete::digit1(input)?;
    let number = digits.parse::<i64>().unwrap();
    let number = if negative.is_some() { -number } else { number };
    Ok((input, SpiralToken::Integer(number)))
}

fn _parse_operator(input: &str) -> nom::IResult<&str, SpiralToken> {
    let (input, op) = nom::character::complete::one_of("=+-*/()")(input)?;
    Ok((input, SpiralToken::Operator(op.to_string())))
}

fn _parse_token(input: &str) -> nom::IResult<&str, SpiralToken> {
    nom::branch::alt((
        _parse_comment,
        _parse_string,
        _parse_identifier,
        _parse_integer,
        _parse_operator,
    ))(input)
}

fn _format_token(token: &SpiralToken) -> String {
    match token {
        SpiralToken::Identifier(s) => s.clone(),
        SpiralToken::Integer(n) => format!("{}", n),
        SpiralToken::StringLiteral(s) => format!("\"{}\"", s),
        SpiralToken::Operator(s) => s.clone(),
        SpiralToken::Comment(s) => format!("//{}", s),
    }
}

proptest! {
    #[test]
    fn prop_parse_format_idempotent(s in any::<SpiralToken>()) {
        println!("input={:?}", s);

        let formatted = _format_token(&s);
        let (_, parsed) = _parse_token(&formatted).unwrap();
        prop_assert_eq!(s, parsed);
    }
}

fn _parse_expression(input: &str) -> nom::IResult<&str, SpiralToken> {
    let (input, number) = nom::character::complete::digit1(input)?;

    match number.parse::<i64>() {
        Ok(n) => Ok((input, SpiralToken::Integer(n))),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        ))),
    }
}

#[test]
fn test_parse_number() {
    assert_eq!(_parse_expression("42"), Ok(("", SpiralToken::Integer(42))));
    assert_eq!(
        _parse_expression("1 + 2"),
        Ok((" + 2", SpiralToken::Integer(1)))
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let app = cli::Cli::v0;
    // let app = app()();
    let app = "test";
    println!("app={}", app);
    Ok(())
}
