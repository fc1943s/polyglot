#![allow(unused)]

mod tests;

pub mod hello_world {
    pub fn hello() -> &'static str {
        "Hello, World!"
    }
    #[test]
    fn tests() {
        assert_eq!("Hello, World!", hello());
    }
}

pub mod lucians_luscious_lasagna {
    /*
    Introduction
    In Rust, assigning a value to a name is referred to as a binding. Bindings are immutable unless declared with the mut keyword. As Rust is a statically-typed language, each binding has a type known at compile-time.

    Bindings are most commonly defined using the let keyword. Specifying a binding's type is optional for most bindings, as Rust's type inference can usually infer the type based on their value. A binding looks like this:

    // Automatically inferred type
    let fingers = 10;
    Functions are items. Where bindings typically refer to a particular value, items refer to a unit of code organization, typically a function or a module, which is available throughout the lifetime of the program. A function automatically returns the result of its last expression. A function may have 0 or more parameters, which are bindings with a lifetime of the function call.

    Type inference is theoretically possible for functions, but is disabled as an intentional language design choice. While this means that you need to spend a little more time when writing code to specify precisely what a function's input and output types are, you save the time when you're reading the code, because all the input and output types are explicitly defined.

    fn add(x: i32, y: i32) -> i32 {
      x + y
    }
    Invoking a function is done by specifying its name followed by parentheses. If the function requires parameters, an argument must be specified for each within the parentheses.

    let five = add(2, 3);
    If a binding's type cannot be inferred, the compiler will report an error. To fix this, add an explicit type annotation to the binding.

    // Explicit type annotation
    let fingers: i32 = 10;
    Items in Rust can be used before or after they are defined, because they have a static lifetime. Bindings, on the other hand, can only be used after they have been defined. Using a binding before it has been defined results in a compile error.

    fn main() {
        // `fn add` hasn't yet been defined, but that's perfectly ok
        dbg!(add(3, 4));
    }

    fn add(x: i32, y: i32) -> i32 {
      x + y
    }
    // this won't compile; `a` is used before its binding is defined
    let b = a;
    let a = x + y;
    Rust uses curly braces ({}) to define a scope. A binding defined within a scope can't escape from it.

    let a = 1;
    dbg!(a); // 1
    {
        // Here, we re-bind `a` to a new value, which is still immutable.
        // This technique is called _shadowing_. The new binding is constrained to
        // this anonymous scope. Outside this scope, the previous binding still
        // applies.
        let a = 2;
        let b = 3;
        dbg!(a, b); // 2, 3
    }
    // can't use `b` anymore because it is out of scope
    // dbg!(b);

    // The shadowed `a` in the inner scope above has fallen out of scope,
    // leaving us with our original binding.
    dbg!(a); // 1
    Rust items are often organized in modules. Each crate is implicitly a module, but it can define inner sub-modules of arbitrary depth. A module groups related functionality and is defined using the mod keyword.

    mod calc_i32 {
        fn add(a: i32, b: i32) -> i32 { a + b }
        fn sub(a: i32, b: i32) -> i32 { a - b }
        fn mul(a: i32, b: i32) -> i32 { a * b }
        fn div(a: i32, b: i32) -> i32 { a / b }
    }
    Rust supports two types of comments. The keyword // indicates a single-line comment; everything following the keyword until the end of the line is ignored. The keywords /* and */ indicate a multi-line comment; everything within those two keywords is ignored. It is idiomatic and good practice to prefer single-line comments.

    Rust also supports doc-comments, which show up in the generated documentation produced by cargo doc. Outer doc comments are formed with the keyword ///, which acts identically to the // keyword. They apply to the item which follows them, such as a function:

    /// The `add` function produces the sum of its arguments.
    fn add(x: i32, y: i32) -> i32 { x + y }
    Inner doc comments are formed with the keyword //!, which acts identically to the // keyword. They apply to the item enclosing them, such as a module:

    mod my_cool_module {
        //! This module is the bee's knees.
    }
    Doc comments can be of arbitrary length and contain markdown, which is rendered into the generated documentation.

    Instructions
    In this exercise you're going to write some code to help you cook a brilliant lasagna from your favorite cooking book.

    You have four tasks, all related to the time spent cooking the lasagna.

    Task 1
    Define the expected oven time in minutes

    Define the expected_minutes_in_oven binding to check how many minutes the lasagna should be in the oven. According to the cooking book, the expected oven time in minutes is 40:

    expected_minutes_in_oven()
    // Returns: 40

    Stuck? Reveal Hints
    Opens in a modal
    Task 2
    Calculate the remaining oven time in minutes

    Define the remaining_minutes_in_oven function that takes the actual minutes the lasagna has been in the oven as a parameter and returns how many minutes the lasagna still has to remain in the oven, based on the expected oven time in minutes from the previous task.

    remaining_minutes_in_oven(30)
    // Returns: 10

    Stuck? Reveal Hints
    Opens in a modal
    Task 3
    Calculate the preparation time in minutes

    Define the preparation_time_in_minutes function that takes the number of layers you added to the lasagna as a parameter and returns how many minutes you spent preparing the lasagna, assuming each layer takes you 2 minutes to prepare.

    preparation_time_in_minutes(2)
    // Returns: 4

    Stuck? Reveal Hints
    Opens in a modal
    Task 4
    Calculate the elapsed time in minutes

    Define the elapsed_time_in_minutes function that takes two parameters: the first parameter is the number of layers you added to the lasagna, and the second parameter is the number of minutes the lasagna has been in the oven. The function should return how many minutes you've worked on cooking the lasagna, which is the sum of the preparation time in minutes, and the time in minutes the lasagna has spent in the oven at the moment.

    elapsed_time_in_minutes(3, 20)
    // Returns: 26
    */

    pub fn expected_minutes_in_oven() -> i32 {
        40
    }
    pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
        expected_minutes_in_oven() - actual_minutes_in_oven
    }
    pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
        number_of_layers * 2
    }
    pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
        preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
    }
    #[test]
    fn tests() {
        assert_eq!(expected_minutes_in_oven(), 40);
        assert_eq!(remaining_minutes_in_oven(30), 10);
        assert_eq!(preparation_time_in_minutes(2), 4);
        assert_eq!(elapsed_time_in_minutes(3, 20), 26);
        assert_eq!(elapsed_time_in_minutes(1, 60), 62);
    }
}

mod assembly_line {
    /*
    Introduction
    Numbers
    There are two different categories of numbers in Rust: integers (which can be signed or unsigned) and floating-point numbers.

    Integers
    Integers: numbers with no digits behind the decimal separator (whole numbers). Integer types can either store only positive numbers (unsigned) or store either positive and negative numbers (signed). Examples are -6, 0, 1, 25, 976 and 500000.
    Floating-Point Numbers
    Floating-point numbers: numbers with zero or more digits behind the decimal separator. Examples are -2.4, 0.1, 3.14, 16.984025 and 1024.0.
    Naming numeric types
    The name of a numeric type consists of two parts:

    A letter to specify whether it's an unsigned integer (u), signed integer (i), or floating-point number (f).
    A number to specify the type's size in bits. Larger types have a greater range between minimum and maximum values. For floating points it will also allow for more numbers behind the decimal separator.
    The following combinations are possible:

    8 bits: u8, i8
    16 bits: u16, i16
    32 bits: u32, i32, f32
    64 bits: u64, i64, f64
    128 bits: u128, i128
    Note that there are only 32-bits and 64-bits variants for floating-point numbers.

    Converting between number types
    Rust doesn't do any implicit type conversion. This means that if you need to turn one numeric type into another, you have to do so explicitly. When converting from a larger type to a smaller one (for instance u64 to u32) you could lose data. Converting from a floating point to an integer will lose everything behind the decimal point, effectively rounding down.

    Instructions
    In this exercise you'll be writing code to analyze the production of an assembly line in a car factory. The assembly line's speed can range from 0 (off) to 10 (maximum).

    At its lowest speed (1), 221 cars are produced each hour. The production increases linearly with the speed. So with the speed set to 4, it should produce 4 * 221 = 884 cars per hour. However, higher speeds increase the likelihood that faulty cars are produced, which then have to be discarded. The following table shows how speed influences the success rate:

    1 to 4: 100% success rate.
    5 to 8: 90% success rate.
    9 and 10: 77% success rate.
    You have two tasks.

    Task 1
    Calculate the production rate per hour

    Implement a method to calculate the assembly line's production rate per hour, taking into account its success rate:

    assembly_line::production_rate_per_hour(6)
    // Returns: 1193.4
    Note that the value returned is an f64.


    Stuck? Reveal Hints
    Opens in a modal
    Task 2
    Calculate the number of working items produced per minute

    Implement a method to calculate how many working cars are produced per minute:

    assembly_line::working_items_per_minute(6)
    // Returns: 19
    Note that the value returned is an u32.
    */

    pub fn production_rate_per_hour(speed: u8) -> f64 {
        221.0
            * speed as f64
            * match speed {
                1..=4 => 1.0,
                5..=8 => 0.9,
                _ => 0.77,
            }
    }
    pub fn working_items_per_minute(speed: u8) -> u32 {
        production_rate_per_hour(speed) as u32 / 60
    }
    #[test]
    fn tests() {
        assert_eq!(production_rate_per_hour(0), 0.0);
        assert_eq!(production_rate_per_hour(1), 221.0);
        assert_eq!(production_rate_per_hour(4), 884.0);
        assert_eq!(production_rate_per_hour(5), 994.5);
        assert_eq!(production_rate_per_hour(8), 1591.2);
        assert_eq!(production_rate_per_hour(9), 1531.53);
        assert_eq!(production_rate_per_hour(10), 1701.7);
        assert_eq!(working_items_per_minute(0), 0);
        assert_eq!(working_items_per_minute(1), 3);
        assert_eq!(working_items_per_minute(4), 14);
        assert_eq!(working_items_per_minute(5), 16);
        assert_eq!(working_items_per_minute(8), 26);
        assert_eq!(working_items_per_minute(9), 25);
        assert_eq!(working_items_per_minute(10), 28);
    }
}

pub mod semi_structured_logs {
    /*
    Introduction
    Enums, short for enumerations, are a type that limits all possible values of some data. The possible values of an enum are called variants. Enums also work well with match and other control flow operators to help you express intent in your Rust programs.

    Instructions
    In this exercise you'll be generating semi-structured log messages.

    Task 1
    Emit semi-structured messages

    You'll start with some stubbed functions and the following enum:

    #[derive(Clone, PartialEq, Debug)]
    pub enum LogLevel {
        Info,
        Warning,
        Error,
    }
    Your goal is to emit a log message as follows: "[<LEVEL>]: <MESSAGE>". You'll need to implement functions that correspond with log levels.

    For example, the below snippet demonstrates an expected output for the log function.

    log(LogLevel::Error, "Stack overflow")
    // Returns: "[ERROR]: Stack overflow"
    And for info:

    info("Timezone changed")
    // Returns: "[INFO]: Timezone changed"
    Have fun!


    Stuck? Reveal Hints
    Opens in a modal
    Task 2
    Optional further practice

    There is a feature-gated test in this suite. Feature gates disable compilation entirely for certain sections of your program. They will be covered later. For now just know that there is a test which is only built and run when you use a special testing invocation:

    cargo test --features add-a-variant
    This test is meant to show you how to add a variant to your enum.
    */

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub enum LogLevel {
        Info,
        Warning,
        Error,
    }
    pub fn log(level: LogLevel, message: &str) -> String {
        let level = format!("{:?}", level).to_uppercase();
        format!("[{level}]: {message}")
    }
    pub fn info(message: &str) -> String {
        log(LogLevel::Info, message)
    }
    pub fn warn(message: &str) -> String {
        log(LogLevel::Warning, message)
    }
    pub fn error(message: &str) -> String {
        log(LogLevel::Error, message)
    }
    #[test]
    fn tests() {
        assert_eq!(
            log(LogLevel::Info, "Timezone changed"),
            "[INFO]: Timezone changed"
        );
        assert_eq!(
            log(LogLevel::Warning, "Out of memory"),
            "[WARNING]: Out of memory"
        );
        assert_eq!(
            log(LogLevel::Error, "Stack overflow"),
            "[ERROR]: Stack overflow"
        );
        assert_eq!(info("Timezone changed"), "[INFO]: Timezone changed");
        assert_eq!(warn("Out of memory"), "[WARNING]: Out of memory");
        assert_eq!(error("Stack overflow"), "[ERROR]: Stack overflow");
    }
}

// mod tmp;

fn main() {
    // tmp::main();
}
