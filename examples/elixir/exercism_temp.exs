# Elixir / Hello World

# Instructions
# The classical introductory exercise. Just say "Hello, World!".

# "Hello, World!" is the traditional first program for beginning programming in a new language or environment.

# The objectives are simple:

# Modify the provided code so that it produces the string "Hello, World!".
# Run the test suite and make sure that it succeeds.
# Submit your solution and check it at the website.
# If everything goes well, you will be ready to fetch your first real exercise.

# How to debug
# To help with debugging, you can use the fact that all console output will be captured and shown in the test results window. You can output any value by passing it to IO.inspect.

# IO.inspect returns the value unchanged, which makes it especially useful for inspecting intermediate values in function call chains.

# "hello"
# |> String.upcase()
# |> IO.inspect()
# |> String.split("", trim: true)
# Note that for security reasons, the output for each test will get truncated after 500 characters.


defmodule HelloWorld do
  def hello(), do: "Hello, World!"
end



# Elixir / Lasagna {{Basics}}

# Introduction
# Basics
# Variables
# Elixir is a dynamically-typed language, meaning that the type of a variable is only checked at runtime. Using the match = operator, we can bind a value of any type to a variable name:

# count = 1 # Bound an integer value of 1
# count = 2 # You may re-bind variables

# count = false # You may re-bind any type to a variable

# message = "Success!" # Strings can be created by enclosing characters within double quotes
# Modules
# Elixir is a functional-programming language and requires all named functions to be defined in a module. The defmodule keyword is used to define a module. All modules are available to all other modules at runtime and do not require an access modifier to make them visible to other parts of the program. A module is analogous to a class in other programming languages.

# defmodule Calculator do
#   # ...
# end
# Named functions
# Named Functions must be defined in a module. The def keyword is used to define a public named function.

# Each function can have zero or more arguments. The value of the last expression in a function is always implicitly returned.

# defmodule Calculator do
#   def add(x, y) do
#     x + y
#   end
# end
# Invoking a function is done by specifying its module and function name and passing arguments for each of the function's arguments.

# sum = Calculator.add(1, 2)
# # => 3
# The defp keyword can be used instead of def to define a private function. Private functions can only be used from within the same module that defined them.

# When invoking a function inside the same module where it's defined, the module name can be omitted.

# You may also write short functions using a one-line syntax (note the comma , and the colon : around the keyword do).

# defmodule Calculator do
#   def subtract(x, y) do
#     private_subtract(x, y)
#   end

#   defp private_subtract(x, y), do: x - y
# end

# difference = Calculator.subtract(7, 2)
# # => 5

# difference = Calculator.private_subtract(7, 2)
# # => ** (UndefinedFunctionError) function Calculator.private_subtract/2 is undefined or private
# #       Calculator.private_subtract(7, 2)
# Arity of functions
# It is common to refer to functions with their arity. The arity of a function is the number of arguments it accepts.

# # add/3 because this function has three arguments, thus an arity of 3
# def add(x, y, z) do
#   x + y + z
# end
# Standard library
# Elixir has a very rich and well-documented standard library. The documentation is available online at hexdocs.pm/elixir. Save this link somewhere - you will use it a lot!

# Most built-in data types have a corresponding module that offers functions for working with that data type, e.g. there's the Integer module for integers, String module for strings, List module for lists and so on.

# A notable module is the Kernel module. It provides the basic capabilities on top of which the rest of the standard library is built, like arithmetic operators, control-flow macros, and much more. Functions for the Kernel module are automatically imported, so you can use them without the Kernel. prefix.

# Code comments
# Comments can be used to leave notes for other developers reading the source code. Single line comments in Elixir are preceded by #.

# Instructions
# In this exercise you're going to write some code to help you cook a brilliant lasagna from your favorite cooking book.

# You have five tasks, all related to the time spent cooking the lasagna.

# Task 1
# Define the expected oven time in minutes

# Define the Lasagna.expected_minutes_in_oven/0 function that does not take any arguments and returns how many minutes the lasagna should be in the oven. According to the cooking book, the expected oven time in minutes is 40:

# Lasagna.expected_minutes_in_oven()
# # => 40

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Calculate the remaining oven time in minutes

# Define the Lasagna.remaining_minutes_in_oven/1 function that takes the actual minutes the lasagna has been in the oven as an argument and returns how many minutes the lasagna still has to remain in the oven, based on the expected oven time in minutes from the previous task.

# Lasagna.remaining_minutes_in_oven(30)
# # => 10

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Calculate the preparation time in minutes

# Define the Lasagna.preparation_time_in_minutes/1 function that takes the number of layers you added to the lasagna as an argument and returns how many minutes you spent preparing the lasagna, assuming each layer takes you 2 minutes to prepare.

# Lasagna.preparation_time_in_minutes(2)
# # => 4

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Calculate the total working time in minutes

# Define the Lasagna.total_time_in_minutes/2 function that takes two arguments: the first argument is the number of layers you added to the lasagna, and the second argument is the number of minutes the lasagna has been in the oven. The function should return how many minutes in total you've worked on cooking the lasagna, which is the sum of the preparation time in minutes, and the time in minutes the lasagna has spent in the oven at the moment.

# Lasagna.total_time_in_minutes(3, 20)
# # => 26

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Create a notification that the lasagna is ready

# Define the Lasagna.alarm/0 function that does not take any arguments and returns a message indicating that the lasagna is ready to eat.

# Lasagna.alarm()
# # => "Ding!"


defmodule Lasagna do
  def expected_minutes_in_oven, do: 40
  def remaining_minutes_in_oven(elapsed), do: expected_minutes_in_oven - elapsed
  def preparation_time_in_minutes(layers), do: layers * 2
  def total_time_in_minutes(layers, elapsed), do: elapsed + preparation_time_in_minutes(layers)
  def alarm, do: "Ding!"
end



# Elixir / Pacman Rules {{Booleans}}

# Introduction
# Booleans
# Elixir represents true and false values with the boolean type. There are only two values: true and false. These values can be bound to a variable:

# true_variable = true
# false_variable = false
# We can evaluate strict boolean expressions using the and/2, or/2, and not/1 operators.

# true_variable = true and true
# false_variable = true and false

# true_variable = false or true
# false_variable = false or false

# true_variable = not false
# false_variable = not true
# When writing a function that returns a boolean value, it is idiomatic to end the function name with a ?.

# def either_true?(a, b) do
#   a or b
# end
# Instructions
# In this exercise, you need to translate some rules from the classic game Pac-Man into Elixir functions.

# You have four rules to translate, all related to the game states.

# Don't worry about how the arguments are derived, just focus on combining the arguments to return the intended result.

# Task 1
# Define if Pac-Man eats a ghost

# Define the Rules.eat_ghost?/2 function that takes two arguments (if Pac-Man has a power pellet active and if Pac-Man is touching a ghost) and returns a boolean value if Pac-Man is able to eat the ghost. The function should return true only if Pac-Man has a power pellet active and is touching a ghost.

# Rules.eat_ghost?(false, true)
# # => false

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Define if Pac-Man scores

# Define the Rules.score?/2 function that takes two arguments (if Pac-Man is touching a power pellet and if Pac-Man is touching a dot) and returns a boolean value if Pac-Man scored. The function should return true if Pac-Man is touching a power pellet or a dot.

# Rules.score?(true, true)
# # => true

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Define if Pac-Man loses

# Define the Rules.lose?/2 function that takes two arguments (if Pac-Man has a power pellet active and if Pac-Man is touching a ghost) and returns a boolean value if Pac-Man loses. The function should return true if Pac-Man is touching a ghost and does not have a power pellet active.

# Rules.lose?(false, true)
# # => true

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Define if Pac-Man wins

# Define the Rules.win?/3 function that takes three arguments (if Pac-Man has eaten all of the dots, if Pac-Man has a power pellet active, and if Pac-Man is touching a ghost) and returns a boolean value if Pac-Man wins. The function should return true if Pac-Man has eaten all of the dots and has not lost based on the arguments defined in part 3.

# Rules.win?(false, true, false)
# # => false


defmodule Rules do
  def eat_ghost?(true, true), do: true
  def eat_ghost?(_power_pellet_active, _touching_ghost), do: false

  def score?(false, false), do: false
  def score?(_touching_power_pellet, _touching_dot), do: true

  def lose?(false, true), do: true
  def lose?(_power_pellet_active, _touching_ghost), do: false

  def win?(true, false, true), do: false
  def win?(has_eaten_all_dots, _power_pellet_active, _touching_ghost), do: has_eaten_all_dots
end



# Elixir / Freelancer Rates {{Floating Point Numbers}} {{Integers}}

# Introduction
# Integers
# There are two different kinds of numbers in Elixir - integers and floats.

# Integers are whole numbers.

# integer = 3
# # => 3
# Floating Point Numbers
# Floats are numbers with one or more digits behind the decimal separator. They use the 64-bit double precision floating-point format.

# float = 3.45
# # => 3.45
# Working with numbers
# In the Integer and Float modules you can find some useful functions for working with those types. Basic arithmetic operators are defined in the Kernel module.

# Conversion
# Integers and floats can be mixed together in a single arithmetic expression. Using a float in an expression ensures the result will be a float too.

# 2 * 3
# # => 6

# 2 * 3.0
# # => 6.0
# However, when doing division, the result will always be a float, even if only integers are used.

# 6 / 2
# # => 3.0
# To convert a float to an integer, you can discard the decimal part with trunc/1.

# Instructions
# In this exercise you'll be writing code to help a freelancer communicate with a project manager by providing a few utilities to quickly calculate daily and monthly rates, optionally with a given discount.

# We first establish a few rules between the freelancer and the project manager:

# The daily rate is 8 times the hourly rate.
# A month has 22 billable days.
# The freelancer is offering to apply a discount if the project manager chooses to let the freelancer bill per month, which can come in handy if there is a certain budget the project manager has to work with.

# Discounts are modeled as fractional numbers representing percentage, for example 25.0 (25%).

# Task 1
# Calculate the daily rate given an hourly rate

# Implement a function to calculate the daily rate given an hourly rate:

# FreelancerRates.daily_rate(60)
# # => 480.0
# The returned daily rate should be a float.


# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Calculate a discounted price

# Implement a function to calculate the price after a discount.

# FreelancerRates.apply_discount(150, 10)
# # => 135.0
# The returned value should always be a float, not rounded in any way.


# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Calculate the monthly rate, given an hourly rate and a discount

# Implement a function to calculate the monthly rate, and apply a discount:

# FreelancerRates.monthly_rate(77, 10.5)
# # => 12130
# The returned monthly rate should be rounded up (take the ceiling) to the nearest integer.


# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Calculate the number of workdays given a budget, hourly rate and discount

# Implement a function that takes a budget, an hourly rate, and a discount, and calculates how many days of work that covers.

# FreelancerRates.days_in_budget(20000, 80, 11.0)
# # => 35.1
# The returned number of days should be rounded down (take the floor) to one decimal place.


defmodule FreelancerRates do
  @daily_rate 8.0
  @monthly_billable_days 22

  def daily_rate(hourly_rate), do: hourly_rate * @daily_rate
  def apply_discount(before_discount, discount), do: before_discount * (1 - discount / 100)

  def monthly_rate(hourly_rate, discount) do
    daily_rate(hourly_rate) * @monthly_billable_days
    |> apply_discount(discount)
    |> Float.ceil()
    |> round()
  end

  def days_in_budget(budget, hourly_rate, discount) do
    budget / apply_discount(daily_rate(hourly_rate), discount)
    |> Float.floor(1)
  end
end



# Elixir / Secrets {{Anonymous Functions}} {{Bit Manipulation}}

# Introduction
# Anonymous Functions
# Functions are treated as first class citizens in Elixir. This means that:

# Named and anonymous functions can be assigned to variables.
# Named and anonymous functions can be passed around like data as arguments and return values.
# Anonymous functions can be created dynamically.
# Anonymous functions, in contrast to named functions, don't have a static reference available to them, they are only available if they are assigned to a variable or immediately invoked.

# We might use anonymous functions to:

# Hide data using lexical scope (also known as a closure).
# Dynamically create functions at run-time.
# Anonymous functions start with the reserved word fn, the arguments are separated from the body of the function with the -> token, and they are finished with an end. As with named functions, the last expression in the function is implicitly returned to the calling function.

# To invoke a function reference, you must use a . between the reference variable and the list of arguments:

# function_variable = fn param ->
#   param + 1
# end

# function_variable.(1)
# # => 2
# You can even use short hand capture notation to make this more concise:

# variable = &(&1 + 1)

# variable.(1)
# # => 2
# Bit Manipulation
# Elixir supports many functions for working with bits found in the Bitwise module.

# band/2: bitwise AND
# bsl/2: bitwise SHIFT LEFT
# bsr/2: bitwise SHIFT RIGHT
# bxor/2: bitwise XOR
# bor/2: bitwise OR
# bnot/1: bitwise NOT
# Here is an example how to use a bitwise function:

# Bitwise.bsl(1, 3)
# # => 8
# All bitwise functions only work on integers.

# If you are running Elixir version 1.9 or lower, you will need to call require Bitwise at the beginning of the module definition to be able to use the Bitwise module.

# Instructions
# In this exercise, you've been tasked with writing the software for an encryption device that works by performing transformations on data. You need a way to flexibly create complicated functions by combining simpler functions together.

# For each task, return an anonymous function that can be invoked from the calling scope.

# All functions should expect integer arguments. Integers are also suitable for performing bitwise operations in Elixir.

# Task 1
# Create an adder

# Implement Secrets.secret_add/1. It should return a function which takes one argument and adds to it the argument passed in to secret_add.

# adder = Secrets.secret_add(2)
# adder.(2)
# # => 4

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Create a subtractor

# Implement Secrets.secret_subtract/1. It should return a function which takes one argument and subtracts the secret passed in to secret_subtract from that argument.

# subtractor = Secrets.secret_subtract(2)
# subtractor.(3)
# # => 1

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Create a multiplier

# Implement Secrets.secret_multiply/1. It should return a function which takes one argument and multiplies it by the secret passed in to secret_multiply.

# multiplier = Secrets.secret_multiply(7)
# multiplier.(3)
# # => 21

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Create a divider

# Implement Secrets.secret_divide/1. It should return a function which takes one argument and divides it by the secret passed in to secret_divide.

# divider = Secrets.secret_divide(3)
# divider.(32)
# # => 10
# Make use of integer division so the output is compatible with the other functions' expected input.


# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Create an "and"-er

# Implement Secrets.secret_and/1. It should return a function which takes one argument and performs a bitwise and operation on it and the secret passed in to secret_and.

# ander = Secrets.secret_and(1)
# ander.(2)
# # => 0

# Stuck? Reveal Hints
# Opens in a modal
# Task 6
# Create an "xor"-er

# Implement Secrets.secret_xor/1. It should return a function which takes one argument and performs a bitwise xor operation on it and the secret passed in to secret_xor.

# xorer = Secrets.secret_xor(1)
# xorer.(3)
# # => 2

# Stuck? Reveal Hints
# Opens in a modal
# Task 7
# Create a function combiner

# Implement Secrets.secret_combine/2. It should return a function which takes one argument and applies to it the two functions passed in to secret_combine in order.

# multiply = Secrets.secret_multiply(7)
# divide = Secrets.secret_divide(3)
# combined = Secrets.secret_combine(multiply, divide)

# combined.(6)
# # => 14


defmodule Secrets do
  use Bitwise, only_operators: true
  def secret_add(secret), do: & &1 + secret
  def secret_subtract(secret), do: & &1 - secret
  def secret_multiply(secret), do: & &1 * secret
  def secret_divide(secret) when secret != 0, do: & div &1, secret
  def secret_and(secret), do: & &1 &&& secret
  def secret_xor(secret), do: & &1 ^^^ secret
  def secret_combine(f, g), do: & &1 |> f.() |> g.()
end



# Elixir / Log Level {{Atoms}} {{Cond}}

# Introduction
# Atoms
# Elixir's atom type represents a fixed constant. An atom's value is simply its own name. This gives us a type-safe way to interact with data. Atoms can be defined as follows:

# # All atoms are preceded with a ':' then follow with alphanumeric snake-cased characters
# variable = :an_atom
# Atoms are internally represented by an integer in a lookup table, which are set automatically. It is not possible to change this internal value.

# Cond
# Often, we want to write code that can branch based on a condition. While there are many ways to do this in Elixir, one of the simplest ways is using cond/1.

# At its simplest, cond follows the first path that evaluates to true with one or more branches:

# cond do
#   x > 10 -> :this_might_be_the_way
#   y < 7 -> :or_that_might_be_the_way
#   true -> :this_is_the_default_way
# end
# If no path evaluates to true, an error is raised by the runtime.

# Instructions
# You are running a system that consists of a few applications producing many logs. You want to write a small program that will aggregate those logs and give them labels according to their severity level. All applications in your system use the same log codes, but some of the legacy applications don't support all the codes.

# Log code	Log label	Supported in legacy apps?
# 0	trace	no
# 1	debug	yes
# 2	info	yes
# 3	warning	yes
# 4	error	yes
# 5	fatal	no
# ?	unknown	-
# Task 1
# Return the logging code label

# Implement the LogLevel.to_label/2 function. It should take an integer code and a boolean flag telling you if the log comes from a legacy app, and return the label of a log line as an atom. Unknown log codes and codes unsupported in a legacy app should return an unknown label.

# LogLevel.to_label(0, false)
# # => :trace

# LogLevel.to_label(0, true)
# # => :unknown

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Send an alert

# Somebody has to be notified when unexpected things happen.

# Implement the LogLevel.alert_recipient/2 function to determine to whom the alert needs to be sent. The function should take an integer code and a boolean flag telling you if the log comes from a legacy app, and return the name of the recipient as an atom.

# If the log label is error or fatal, send the alert to the ops team. If you receive a log with an unknown label from a legacy system, send the alert to the dev1 team, other unknown labels should be sent to the dev2 team. All other log labels can be safely ignored.

# LogLevel.alert_recipient(-1, true)
# # => :dev1

# LogLevel.alert_recipient(0, false)
# # => false


defmodule LogLevel do
  def to_label(0, false), do: :trace
  def to_label(1, _), do: :debug
  def to_label(2, _), do: :info
  def to_label(3, _), do: :warning
  def to_label(4, _), do: :error
  def to_label(5, false), do: :fatal
  def to_label(_level, _legacy?), do: :unknown

  def alert_recipient(level, legacy?) when is_number(level), do: alert_recipient(to_label(level, legacy?), legacy?)
  def alert_recipient(:error, _), do: :ops
  def alert_recipient(:fatal, _), do: :ops
  def alert_recipient(:unknown, true), do: :dev1
  def alert_recipient(:unknown, false), do: :dev2
  def alert_recipient(_, _), do: false
end



# Elixir / Language List {{Lists}}

# Introduction
# Lists
# Lists are built-in to the Elixir language. They are considered a basic type, denoted by square brackets. Lists may be empty or hold any number of items of any type. For example:

# empty_list = []
# one_item_list = [1]
# two_item_list = [1, 2]
# multiple_type_list = [1, :pi, 3.14, "four"]
# Elixir implements lists as a linked list, where each node stores the reference to the next list. The first item in the list is referred to as the head and the remaining list of items is called the tail. We can use this notation in code:

# # [1] represented in [head | tail] notation
# [1 | []]

# # [1, 2, 3] represented in [head | tail] notation
# [1 | [2 | [3 | []]]]
# We can use [head | tail] notation to prepend elements to a list:

# # Suppose
# list = [2, 1]

# [3, 2, 1] == [3 | list]
# # => true
# There are several functions in the Kernel module for working with lists, as well as the whole List module.

# # Check if 1 is a member of the list
# 1 in [1, 2, 3, 4]
# # => true
# Instructions
# In this exercise you need to implement some functions to manipulate a list of programming languages.

# Task 1
# Define a function to return an empty language list

# Define the new/0 function that takes no arguments and returns an empty list.

# LanguageList.new()
# # => []

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Define a function to add a language to the list

# Define the add/2 function that takes 2 arguments (a language list and a string literal of a language). It should return the resulting list with the new language prepended to the given list.

# language_list = LanguageList.new()
# # => []
# language_list = LanguageList.add(language_list, "Clojure")
# # => ["Clojure"]
# language_list = LanguageList.add(language_list, "Haskell")
# # => ["Haskell", "Clojure"]

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Define a function to remove a language from the list

# Define the remove/1 function that takes 1 argument (a language list). It should return the list without the first item. Assume the list will always have at least one item.

# language_list = LanguageList.new()
# # => []
# language_list = LanguageList.add(language_list, "Clojure")
# # => ["Clojure"]
# language_list = LanguageList.add(language_list, "Haskell")
# # => ["Haskell", "Clojure"]
# language_list = LanguageList.remove(language_list)
# # => ["Clojure"]

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Define a function to return the first item in the list

# Define the first/1 function that takes 1 argument (a language list). It should return the first language in the list. Assume the list will always have at least one item.

# language_list = LanguageList.new()
# # => []
# language_list = LanguageList.add(language_list, "Elm")
# # => ["Elm"]
# language_list = LanguageList.add(language_list, "Prolog")
# # => ["Prolog", "Elm"]
# LanguageList.first(language_list)
# # => "Prolog"

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Define a function to return how many languages are in the list

# Define the count/1 function that takes 1 argument (a language list). It should return the number of languages in the list.

# language_list = LanguageList.new()
# # => []
# language_list = LanguageList.add(language_list, "Elm")
# # => ["Elm"]
# language_list = LanguageList.add(language_list, "Prolog")
# # => ["Prolog", "Elm"]
# LanguageList.count(language_list)
# # => 2

# Stuck? Reveal Hints
# Opens in a modal
# Task 6
# Define a function to determine if the list includes a functional language

# Define the functional_list?/1 function which takes 1 argument (a language list). It should return a boolean value. It should return true if "Elixir" is one of the languages in the list.

# language_list = LanguageList.new()
# # => []
# language_list = LanguageList.add(language_list, "Elixir")
# # => ["Elixir"]
# LanguageList.functional_list?(language_list)
# # => true


defmodule LanguageList do
  def new(), do: []
  def add(list, language), do: [language | list]
  def remove([_ | tail]), do: tail
  def first([head | _]), do: head
  def count(list), do: length(list)
  def functional_list?(list), do: "Elixir" in list
end



# Elixir / Guessing Game {{Default Arguments}} {{Guards}} {{Multiple Clause Functions}}

# Introduction
# Multiple Clause Functions
# Elixir facilitates Open-Close Principle practices by allowing functions to have multiple clauses, so instead of sprawling and hard-coded control-logic, pointed functions can be written to add/remove behavior easily.

# Elixir offers multiple function clauses and guards to write:

# def number(n) when n == 7 do
#   "Awesome, that's my favorite"
# end
# def number(_n) do
#   "That's not my favorite"
# end
# At run-time, Elixir will test, from top to bottom of the source file, which function clause to invoke.

# Variables that are unused should be prefixed with an underscore.

# Guards
# Guards are used to prevent Elixir from invoking functions based on evaluation of the arguments by guard functions. Guards begin with the when keyword, followed by a boolean expression. Guard functions are special functions which:

# Must be pure and not mutate any global states.
# Must return strict true or false values.
# A list of common guards can be found in the Elixir documentation. It includes type checks, basic arithmetic, comparisons, and strictly boolean operators.

# Default Arguments
# Functions may declare default values for one or more arguments. Let's consider this function:

# def number(n \\ 13), do: "That's not my favorite"
# When compiled, Elixir creates a function definition for number/0 (no arguments), and number/1 (one argument).

# If more than one argument has default values, the default values will be applied to the function from left to right to fill in for missing arguments.

# If the function has more than one clause, the default arguments should be defined in a function header (a function without a body) before the function clauses:

# def number(n \\ 13)
# def number(n) when n < 10, do: "Dream bigger!"
# def number(n) when n > 100, do: "Not that big..."
# Instructions
# You are creating a trivial online game where a friend can guess a secret number. You want to give some feedback, but not give away the answer with a guess. You need to devise a function to provide different responses depending on how the guess relates to the secret number.

# Condition	Response
# When the guess matches the secret number	"Correct"
# When the guess is one more or one less than the secret number	"So close"
# When the guess is greater than the secret number	"Too high"
# When the guess is less than the secret number	"Too low"
# When a guess isn't made	"Make a guess"
# All guesses and secret numbers are integer numbers.

# Task 1
# Make the response when the guess matches the secret number

# Implement the compare/2 function which takes two arguments, secret_number and guess, which are both integers.

# GuessingGame.compare(5, 5)
# # => "Correct"

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Make the response when the guess is greater than the secret number

# Modify the compare function to respond to guesses that are higher than the secret number.

# GuessingGame.compare(5, 8)
# # => "Too high"

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Make the response when the guess is less than the secret number

# Modify the compare function to respond to guesses that are lower than the secret number.

# GuessingGame.compare(5, 2)
# # => "Too low"

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Make the responses when the guess is one more or one less than the secret number

# Modify the compare function to respond to guesses that are close to the secret number.

# GuessingGame.compare(5, 6)
# # => "So close"
# GuessingGame.compare(5, 4)
# # => "So close"

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Make the response when there is no guess

# Modify the compare function to respond to a lack of guess.

# GuessingGame.compare(5)
# # => "Make a guess"

# GuessingGame.compare(5, :no_guess)
# # => "Make a guess"


defmodule GuessingGame do
  def compare(secret_number, guess) when guess == secret_number, do: "Correct"
  def compare(secret_number, guess \\ :no_guess) when guess == :no_guess, do: "Make a guess"
  def compare(secret_number, guess) when guess == secret_number - 1, do: "So close"
  def compare(secret_number, guess) when guess == secret_number + 1, do: "So close"
  def compare(secret_number, guess) when guess > secret_number, do: "Too high"
  def compare(secret_number, guess) when guess < secret_number, do: "Too low"
end



# Elixir / Kitchen Calculator {{Pattern Matching}} {{Tuples}}

# Introduction
# Tuples
# In Elixir, a tuple is a data structure which organizes data, holding a fixed number of items of any type, but without explicit names for each element. Tuples are often used in Elixir for memory read-intensive operations, since read-access of an element is a constant-time operation. They are not usually used when elements may need to be added/removed dynamically because rather than modifying the existing tuple, a new tuple is created which requires memory to be allocated upfront.

# In practice, tuples are created in Elixir using curly braces. Elements in a tuple can be individually accessed using the elem/2 function using 0-based indexing:

# empty_tuple = {}
# one_element_tuple = {1}
# multiple_element_tuple = {1, :a, "hello"}

# elem(multiple_element_tuple, 2)
# # => "hello"
# Tuples as grouped information
# Tuples are often used in practice to represent grouped information.

# Float.ratio(0.25)
# # => {1, 4} indicating the numerator and denominator of the fraction ¼
# Pattern Matching
# The use of pattern matching is dominant in assertive, idiomatic Elixir code. You might recall that =/2 is described as a match operator rather than as an assignment operator. When using the match operator, if the pattern on the left matches the right, any variables on the left are bound, and the value of the right side is returned. A MatchError is raised if there is no match.

# 2 = 2
# # => 2
# # Literals can be matched if they are the same

# 2 = 3
# # => ** (MatchError) no match of right hand side value: 3

# {_, denominator} = Float.ratio(0.25)
# # => {1, 4}
# # the variable `denominator` is bound to the value 4
# Remember, matches occur from the right side to the left side.

# In the last example if we don't need a variable in a pattern match, we can discard it by referencing _. Any variable starting with an _ is not tracked by the runtime.

# Pattern matching in named functions
# Pattern matching is also a useful tool when creating multiple function clauses. Pattern matching can be used on the functions' arguments which then determines which function clause to invoke -- starting from the top of the file down until the first match. Variables may be bound in a function head and used in the function body.

# defmodule Example do
#   def named_function(:a = variable_a) do
#     {variable_a, 1}
#   end

#   def named_function(:b = variable_b) do
#     {variable_b, 2}
#   end
# end

# Example.named_function(:a)
# # => {:a, 1}

# Example.named_function(:b)
# # => {:b, 2}

# Example.named_function(:c)
# # => ** (FunctionClauseError) no function clause matching in Example.named_function/1
# Instructions
# While preparing to bake cookies for your friends, you have found that you have to convert some of the measurements used in the recipe. Being only familiar with the metric system, you need to come up with a way to convert common US baking measurements to milliliters (mL) for your own ease.

# Use this conversion chart for your solution:

# Unit to convert	volume	in milliliters (mL)
# mL	1	1
# US cup	1	240
# US fluid ounce	1	30
# US teaspoon	1	5
# US tablespoon	1	15
# Being a talented programmer in training, you decide to use milliliters as a transition unit to facilitate the conversion from any unit listed to any other (even itself).

# Task 1
# Get the numeric component from a volume-pair

# Implement the KitchenCalculator.get_volume/1 function. Given a volume-pair tuple, it should return just the numeric component.

# KitchenCalculator.get_volume({:cup, 2.0})
# # => 2.0

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Convert the volume-pair to milliliters

# Implement the KitchenCalculator.to_milliliter/1 function. Given a volume-pair tuple, it should convert the volume to milliliters using the conversion chart.

# Use multiple function clauses and pattern matching to create the functions for each unit. The atoms used to denote each unit are: :cup, :fluid_ounce, :teaspoon, :tablespoon, :milliliter. Return the result of the conversion wrapped in a tuple.

# KitchenCalculator.to_milliliter({:cup, 2.5})
# # => {:milliliter, 600.0}

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Convert the milliliter volume-pair to another unit

# Implement the KitchenCalculator.from_milliliter/2 function. Given a volume-pair tuple and the desired unit, it should convert the volume to the desired unit using the conversion chart.

# Use multiple function clauses and pattern matching to create the functions for each unit. The atoms used to denote each unit are: :cup, :fluid_ounce, :teaspoon, :tablespoon, :milliliter

# KitchenCalculator.from_milliliter({:milliliter, 1320.0}, :cup)
# # => {:cup, 5.5}

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Convert from any unit to any unit

# Implement the KitchenCalculator.convert/2 function. Given a volume-pair tuple and the desired unit, it should convert the given volume to the desired unit.

# KitchenCalculator.convert({:teaspoon, 9.0}, :tablespoon)
# # => {:tablespoon, 3.0}


defmodule KitchenCalculator do
  def get_volume({_unit, volume}), do: volume
  def to_milliliter({unit, volume}), do: {:milliliter, volume * factor(unit)}
  def from_milliliter({:milliliter, volume}, unit), do: {unit, volume / factor(unit)}
  def convert(volume_pair, unit), do: volume_pair |> to_milliliter() |> from_milliliter(unit)
  defp factor(:milliliter), do: 1
  defp factor(:teaspoon), do: 5
  defp factor(:tablespoon), do: 15
  defp factor(:fluid_ounce), do: 30
  defp factor(:cup), do: 240
end






# Elixir / High School Sweetheart {{Pipe Operator}} {{Strings}}

# Introduction
# Strings
# Strings in Elixir are delimited by double quotes, and they are encoded in UTF-8:

# "Hi!"
# Strings can be concatenated using the <>/2 operator:

# "Welcome to" <> " " <> "New York"
# # => "Welcome to New York"
# Strings in Elixir support interpolation using the #{} syntax:

# "6 * 7 = #{6 * 7}"
# # => "6 * 7 = 42"
# To put a newline character in a string, use the \n escape code:

# "1\n2\n3\n"
# To comfortably work with texts with a lot of newlines, use the triple-double-quote heredoc syntax instead:

# """
# 1
# 2
# 3
# """
# Elixir provides many functions for working with strings in the String module.

# Pipe Operator
# The |> operator is called the pipe operator. It can be used to chain function calls together in such a way that the value returned by the previous function call is passed as the first argument to the next function call.

# "hello"
# |> String.upcase()
# |> Kernel.<>("?!")
# # => "HELLO?!"
# Instructions
# In this exercise, you are going to help high school sweethearts profess their love on social media by generating an ASCII heart with their initials:

#      ******       ******
#    **      **   **      **
#  **         ** **         **
# **            *            **
# **                         **
# **     J. K.  +  M. B.     **
#  **                       **
#    **                   **
#      **               **
#        **           **
#          **       **
#            **   **
#              ***
#               *
# Task 1
# Get the name's first letter

# Implement the HighSchoolSweetheart.first_letter/1 function. It should take a name and return its first letter. It should clean up any unnecessary whitespace from the name.

# HighSchoolSweetheart.first_letter("Jane")
# # => "J"

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Format the first letter as an initial

# Implement the HighSchoolSweetheart.initial/1 function. It should take a name and return its first letter, uppercase, followed by a dot. Make sure to reuse HighSchoolSweetheart.first_letter/1 that you defined in the previous step.

# HighSchoolSweetheart.initial("Robert")
# # => "R."

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Split the full name into the first name and the last name

# Implement the HighSchoolSweetheart.initials/1 function. It should take a full name, consisting of a first name and a last name separated by a space, and return the initials. Make sure to reuse HighSchoolSweetheart.initial/1 that you defined in the previous step.

# HighSchoolSweetheart.initials("Lance Green")
# # => "L. G."

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Put the initials inside of the heart

# Implement the HighSchoolSweetheart.pair/2 function. It should take two full names and return the initials. Make sure to reuse HighSchoolSweetheart.initials/1 that you defined in the previous step.

# HighSchoolSweetheart.pair("Blake Miller", "Riley Lewis")
# # => """
# #      ******       ******
# #    **      **   **      **
# #  **         ** **         **
# # **            *            **
# # **                         **
# # **     B. M.  +  R. L.     **
# #  **                       **
# #    **                   **
# #      **               **
# #        **           **
# #          **       **
# #            **   **
# #              ***
# #               *
# # """


defmodule HighSchoolSweetheart do
  def first_letter(name), do: name |> String.trim() |> String.first()
  def initial(name), do: first_letter(name) <> "." |> String.upcase()
  def initials(full_name), do: full_name |> String.split(" ") |> Enum.map(&initial/1) |> Enum.join(" ")
  def pair(full_name1, full_name2) do
    i1 = initials(full_name1)
    i2 = initials(full_name2)
    """
       ******       ******
     **      **   **      **
   **         ** **         **
  **            *            **
  **                         **
  **     #{i1}  +  #{i2}     **
   **                       **
     **                   **
       **               **
         **           **
           **       **
             **   **
               ***
                *
  """
    end
end



# Elixir / Bird Count {{Recursion}}

# Introduction
# Recursion
# Recursive functions are functions that call themselves.

# A recursive function needs to have at least one base case and at least one recursive case.

# A base case returns a value without calling the function again. A recursive case calls the function again, modifying the input so that it will at some point match the base case.

# Very often, each case is written in its own function clause.

# # base case
# def count([]), do: 0

# # recursive case
# def count([_head | tail]), do: 1 + count(tail)
# Instructions
# You're an avid bird watcher that keeps track of how many birds have visited your garden on any given day.

# You decided to bring your bird watching to a new level and implement a few tools that will help you track and process the data.

# You have chosen to store the data as a list of integers. The first number in the list is the number of birds that visited your garden today, the second yesterday, and so on.

# Task 1
# Check how many birds visited today

# Implement the BirdCount.today/1 function. It should take a list of daily bird counts and return today's count. If the list is empty, it should return nil.

# BirdCount.today([2, 5, 1])
# # => 2

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Increment today's count

# Implement the BirdCount.increment_day_count/1 function. It should take a list of daily bird counts and increment the today's count by 1. If the list is empty, return [1].

# BirdCount.increment_day_count([4, 0, 2])
# # => [5, 0, 2]

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Check if there was a day with no visiting birds

# Implement the BirdCount.has_day_without_birds?/1 function. It should take a list of daily bird counts. It should return true if there was at least one day when no birds visited the garden, and false otherwise.

# BirdCount.has_day_without_birds?([2, 0, 4])
# # => true

# BirdCount.has_day_without_birds?([3, 8, 1, 5])
# # => false

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Calculate the total number of visiting birds

# Implement the BirdCount.total/1 function. It should take a list of daily bird counts and return the total number that visited your garden since you started collecting the data.

# BirdCount.total([4, 0, 9, 0, 5])
# # => 18

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Calculate the number of busy days

# Some days are busier than others. A busy day is one where five or more birds have visited your garden.

# Implement the BirdCount.busy_days/1 function. It should take a list of daily bird counts and return the number of busy days.

# BirdCount.busy_days([4, 5, 0, 0, 6])
# # => 2


defmodule BirdCount do
  @busy_day_count 5
  def today(list), do: list |> List.first()
  def increment_day_count([]), do: [1]
  def increment_day_count([head | tail]), do: [head + 1 | tail]
  def has_day_without_birds?(list), do: list |> Enum.member?(0)
  def total(list), do: list |> Enum.sum()
  def busy_days(list), do: list |> Enum.count(& &1 >= @busy_day_count)
end



# Elixir / High Score {{Maps}} {{Module Attributes As Constants}}

# Introduction
# Maps
# Maps in Elixir are the data structure for storing information in key-value pairs. In other languages, these might also be known as associative arrays (PHP), hashes (Perl 5, Raku), or dictionaries (Python).

# Keys and values can be of any data type, but if the key is an atom we can use a shorthand syntax. Maps do not guarantee the order of their entries when accessed or returned.

# Literal forms
# An empty map is simply declared with %{}. If we want to add items to a map literal, we can use two forms:

# # If the key is an atom:
# %{atom_key: 1}

# # If the key is a different type:
# %{1 => :atom_value}

# # You can even mix these if the atom form comes second:
# %{"first_form" => :a, atom_form: :b}
# While there is no canonical format, choose a consistent way to represent the key-value literal pairs.

# Map module functions
# Elixir provides many functions for working with maps in the Map module. Some Map module functions require an anonymous function to be passed into the function to assist with the map transformation.

# Module Attributes As Constants
# In Elixir, we can define module attributes which can be used as constants in our functions.

# defmodule Example do

#   # Defines the attribute as the value 1
#   @constant_number 1

#   def example_value() do
#     # Returns the value 1
#     @constant_number
#   end
# end
# When used to define module constants, attributes can be any expression which can be evaluated at compilation time. After compilation, module attributes are not accessible since they are expanded during compilation, similar to defined macros in languages like C.

# Instructions
# In this exercise, you're implementing a way to keep track of the high scores for the most popular game in your local arcade hall.

# Task 1
# Define a new high score map

# To make a new high score map, define the HighScore.new/0 function which doesn't take any arguments and returns a new, empty map of high scores.

# HighScore.new()
# # => %{}

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Add players to the high score map

# To add a player to the high score map, define HighScore.add_player/3, which is a function which takes 3 arguments:

# The first argument is the map of scores.
# The second argument is the name of a player as a string.
# The third argument is the score as an integer. The argument is optional, implement the third argument with a default value of 0.
# Store the default initial score in a module attribute. It will be needed again.

# score_map = HighScore.new()
# # => %{}
# score_map = HighScore.add_player(score_map, "Dave Thomas")
# # => %{"Dave Thomas" => 0}
# score_map = HighScore.add_player(score_map, "José Valim", 486_373)
# # => %{"Dave Thomas" => 0, "José Valim"=> 486_373}

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Remove players from the score map

# To remove a player from the high score map, define HighScore.remove_player/2, which takes 2 arguments:

# The first argument is the map of scores.
# The second argument is the name of the player as a string.
# score_map = HighScore.new()
# # => %{}
# score_map = HighScore.add_player(score_map, "Dave Thomas")
# # => %{"Dave Thomas" => 0}
# score_map = HighScore.remove_player(score_map, "Dave Thomas")
# # => %{}

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Reset a player's score

# To reset a player's score, define HighScore.reset_score/2, which takes 2 arguments:

# The first argument is the map of scores.
# The second argument is the name of the player as a string, whose score you wish to reset.
# The function should also work if the player doesn't have a score.

# score_map = HighScore.new()
# # => %{}
# score_map = HighScore.add_player(score_map, "José Valim", 486_373)
# # => %{"José Valim"=> 486_373}
# score_map = HighScore.reset_score(score_map, "José Valim")
# # => %{"José Valim"=> 0}

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Update a player's score

# To update a player's score by adding to the previous score, define HighScore.update_score/3, which takes 3 arguments:

# The first argument is the map of scores.
# The second argument is the name of the player as a string, whose score you wish to update.
# The third argument is the score that you wish to add to the stored high score.
# The function should also work if the player doesn't have a previous score - assume the previous score is 0.

# score_map = HighScore.new()
# # => %{}
# score_map = HighScore.add_player(score_map, "José Valim", 486_373)
# # => %{"José Valim"=> 486_373}
# score_map = HighScore.update_score(score_map, "José Valim", 5)
# # => %{"José Valim"=> 486_378}

# Stuck? Reveal Hints
# Opens in a modal
# Task 6
# Get a list of players

# To get a list of players, define HighScore.get_players/1, which takes 1 argument:

# The first argument is the map of scores.
# score_map = HighScore.new()
# # => %{}
# score_map = HighScore.add_player(score_map, "Dave Thomas", 2_374)
# # => %{"Dave Thomas" => 2_374}
# score_map = HighScore.add_player(score_map, "José Valim", 486_373)
# # => %{"Dave Thomas" => 2_374, "José Valim"=> 486_373}
# HighScore.get_players(score_map)
# # => ["Dave Thomas", "José Valim"]


defmodule HighScore do
  @initial_score 0
  def new(), do: %{}
  def add_player(scores, name, score \\ @initial_score), do: scores |> Map.put(name, score)
  def remove_player(scores, name), do: scores |> Map.delete(name)
  def reset_score(scores, name), do: add_player(scores, name)
  def update_score(scores, name, score), do: scores |> Map.update(name, score, & &1 + score)
  def get_players(scores), do: scores |> Map.keys()
end



# Elixir / City Office {{Docs}} {{Typespecs}}

# Introduction
# Docs
# Documentation in Elixir is a first-class citizen.

# There are two module attributes commonly used to document your code - @moduledoc for documenting a module and @doc for documenting a function that follows the attribute. The @moduledoc attribute usually appears on the first line of the module, and the @doc attribute usually appears right before a function definition, or the function's typespec if it has one. The documentation is commonly written in a multiline string using the heredoc syntax.

# Elixir documentation is written in Markdown.

# defmodule String do
#   @moduledoc """
#   Strings in Elixir are UTF-8 encoded binaries.
#   """

#   @doc """
#   Converts all characters in the given string to uppercase according to `mode`.

#   ## Examples

#       iex> String.upcase("abcd")
#       "ABCD"

#       iex> String.upcase("olá")
#       "OLÁ"
#   """
#   def upcase(string, mode \\ :default)
# end
# Typespecs
# Elixir is a dynamically typed language, which means it doesn't provide compile-time type checks. Still, type specifications can be used as a form of documentation.

# A type specification can be added to a function using the @spec module attribute right before the function definition. @spec is followed by the function name and a list of all of its arguments' types, in parentheses, separated by commas. The type of the return value is separated from the function's arguments with a double colon ::.

# @spec longer_than?(String.t(), non_neg_integer()) :: boolean()
# def longer_than?(string, length), do: String.length(string) > length
# Types
# Most commonly used types include:

# booleans: boolean()
# strings: String.t()
# numbers: integer(), non_neg_integer(), pos_integer(), float()
# lists: list()
# a value of any type: any()
# Some types can also be parameterized, for example list(integer) is a list of integers.

# Literal values can also be used as types.

# A union of types can be written using the pipe |. For example, integer() | :error means either an integer or the atom literal :error.

# A full list of all types can be found in the "Typespecs" section in the official documentation.

# Naming arguments
# Arguments in the typespec could also be named which is useful for distinguishing multiple arguments of the same type. The argument name, followed by a double colon, goes before the argument's type.

# @spec to_hex({hue :: integer, saturation :: integer, lightness :: integer}) :: String.t()
# Custom types
# Typespecs aren't limited to just the built-in types. Custom types can be defined using the @type module attribute. A custom type definition starts with the type's name, followed by a double colon and then the type itself.

# @type color :: {hue :: integer, saturation :: integer, lightness :: integer}

# @spec to_hex(color()) :: String.t()
# A custom type can be used from the same module where it's defined, or from another module.

# Instructions
# You have been working in the city office for a while, and you have developed a set of tools that speed up your day-to-day work, for example with filling out forms.

# Now, a new colleague is joining you, and you realized your tools might not be self-explanatory. There are a lot of weird conventions in your office, like always filling out forms with uppercase letters and avoiding leaving fields empty.

# You decide to write some documentation so that it's easier for your new colleague to hop right in and start using your tools.

# Task 1
# Document the purpose of the form tools

# Add documentation to the Form module that describes its purpose. It should read:

# A collection of loosely related functions helpful for filling out various forms at the city office.

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Document filling out fields with blank values

# Add documentation and a typespec to the Form.blanks/1 function. The documentation should read:

# Generates a string of a given length.

# This string can be used to fill out a form field that is supposed to have no value.
# Such fields cannot be left empty because a malicious third party could fill them out with false data.
# The typespec should explain that the function accepts a single argument, a non-negative integer, and returns a string.


# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Document splitting values into lists of uppercase letters

# Add documentation and a typespec to the Form.letters/1 function. The documentation should read:

# Splits the string into a list of uppercase letters.

# This is needed for form fields that don't offer a single input for the whole string,
# but instead require splitting the string into a predefined number of single-letter inputs.
# The typespec should explain that the function accepts a single argument, a string, and returns a list of strings.


# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Document checking if a value fits a field with a max length

# Add documentation and a typespec to the Form.check_length/2 function. The documentation should read:

# Checks if the value has no more than the maximum allowed number of letters.

# This is needed to check that the values of fields do not exceed the maximum allowed length.
# It also tells you by how much the value exceeds the maximum.
# The typespec should explain that the function accepts two arguments, a string and a non-negative integer, and returns one of two possible values. It returns either the :ok atom or a 2-tuple with the first element being the :error atom, and the second a positive integer.


# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Document different address formats

# For some unknown to you reason, the city office's internal system uses two different ways of representing addresses - either as a map or as a tuple.

# Document this fact by defining three custom public types:

# address_map - a map with the keys :street, :postal_code, and :city. Each key holds a value of type string.
# address_tuple - a tuple with three values - street, postal_code, and city. Each value is of type string. Differentiate the values by giving them names in the typespec.
# address - can be either an address_map or an address_tuple.

# Stuck? Reveal Hints
# Opens in a modal
# Task 6
# Document formatting the address

# Add documentation and a typespec to the Form.format_address/1 function. The documentation should read:

# Formats the address as an uppercase multiline string.
# The typespec should explain that the function accepts one argument, an address, and returns a string.


defmodule Form do
  @moduledoc """
  A collection of loosely related functions helpful for filling out various forms at the city office.
  """

  @type address_map :: %{street: String.t(), postal_code: String.t(), city: String.t()}
  @type address_tuple :: {street :: String.t(), postal_code :: String.t(), city :: String.t()}
  @type address :: address_map | address_tuple

  @doc """
  Generates a string of a given length.

  This string can be used to fill out a form field that is supposed to have no value.
  Such fields cannot be left empty because a malicious third party could fill them out with false data.
  """
  @spec blanks(non_neg_integer()) :: String.t()
  def blanks(n), do: String.duplicate("X", n)

  @doc """
  Splits the string into a list of uppercase letters.

  This is needed for form fields that don't offer a single input for the whole string,
  but instead require splitting the string into a predefined number of single-letter inputs.
  """
  @spec letters(String.t()) :: [String.t()]
  def letters(word), do: word |> String.upcase() |> String.split("", trim: true)

  @doc """
  Checks if the value has no more than the maximum allowed number of letters.

  This is needed to check that the values of fields do not exceed the maximum allowed length.
  It also tells you by how much the value exceeds the maximum.
  """
  @spec check_length(String.t(), non_neg_integer()) :: :ok | {:error, pos_integer()}
  def check_length(word, length) do
    diff = String.length(word) - length

    if diff <= 0 do
      :ok
    else
      {:error, diff}
    end
  end

  @doc """
  Formats the address as an uppercase multiline string.
  """
  @spec format_address(address()) :: String.t()
  def format_address(%{street: street, postal_code: postal_code, city: city}) do
    format_address({street, postal_code, city})
  end

  def format_address({street, postal_code, city}) do
    """
    #{String.upcase(street)}
    #{String.upcase(postal_code)} #{String.upcase(city)}
    """
  end
end



# Elixir / German Sysadmin {{Case}} {{Charlists}}

# Introduction
# Charlists
# Charlists are created using single quotes.

# 'hello'
# Although they look similar to strings, the two data types are quite different from one another. A charlist is a list of integers. The integers represent the Unicode values of a given character — also known as code points.

# [65, 66, 67]
# # => 'ABC'
# You can prepend a character with ? to get its code point.

# ?A
# # => 65

# [?:, ?)]
# # => ':)'
# Because charlist are lists, you can work with them just like with any other list - using recursion and pattern matching.

# [first_letter | _] = 'cat'
# first_letter
# # => 99
# You can concatenate two lists using ++.

# 'hi' ++ '!'
# # => 'hi!'
# The longer the first list is, the slower the concatenation, so avoid repeatedly appending to lists of arbitrary length.

# Case
# case is a control flow structure that allows us to compare a given value against many patterns. Clauses in a case expression are evaluated from top to bottom, until a match is found.

# age = 15

# case age do
#   0 -> 'infant'
#   age when age < 4 -> 'baby'
#   age when age < 13 -> 'child'
#   age when age < 18 -> 'teenager'
#   _ -> 'adult'
# end

# # => 'teenager'
# Instructions
# You are working as a system administrator for a big company in Munich, Germany. One of your responsibilities is managing email accounts.

# You have been hearing complaints from people saying they are unable to write emails to Mr. Müller. You quickly realize that most of the company uses an old email client that doesn't recognize müller@firma.de as a valid email address because of the non-Latin character.

# Telling people to give up their favorite old email client is a lost battle, so you decide to create sanitized aliases for all email accounts.

# Task 1
# Sanitize existing usernames by removing everything but lowercase letters

# Implement the sanitize/1 function. It should accept a username as a charlist and return the username with all characters but lowercase letters removed.

# Username.sanitize('schmidt1985')
# # => 'schmidt'

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Allow underscores

# Extend the sanitize/1 function. It should not remove underscores from the username.

# Username.sanitize('mark_fischer$$$')
# # => 'mark_fischer'

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Substitute German characters

# There are 4 non-Latin characters in the German alphabet, and all of them have commonly-recognized Latin substitutes.

# German character	Latin substitute
# ä	ae
# ö	oe
# ü	ue
# ß	ss
# Extend the sanitize/1 function. It should substitute German characters according to the table. You can safely assume all usernames are already downcase.

# Username.sanitize('cäcilie_weiß')
# # => 'caecilie_weiss'


defmodule Username do
  def sanitize([]), do: []
  def sanitize([?ä | t]), do: 'ae' ++ sanitize(t)
  def sanitize([?ö | t]), do: 'oe' ++ sanitize(t)
  def sanitize([?ü | t]), do: 'ue' ++ sanitize(t)
  def sanitize([?ß | t]), do: 'ss' ++ sanitize(t)
  def sanitize([h | t]) when h in ?a..?z or h == ?_, do: [h | sanitize(t)]
  def sanitize([h | t]), do: sanitize(t)
end



# Elixir / RPG Character Sheet {{IO}}

# Introduction
# IO
# Functions for handling input and output are provided by the IO module.

# Output
# To write a string to the standard output, use IO.puts. IO.puts always adds a new line at the end of the string. If you don't want that behavior, use IO.write instead. Both functions return the atom :ok if they succeed.

# IO.puts("Hi!")
# # > Hi!
# # => :ok
# IO.puts is useful for writing strings, but not much else. If you need a tool for debugging that will allow you to write any value to standard output, use IO.inspect instead. IO.inspect returns the value it was passed unchanged, so it can be inserted in any point in your code. It also accepts many options, for example :label, that will allow you to distinguish it from other IO.inspect calls.

# Input
# To read a line from the standard input, use IO.gets. IO.gets accepts one argument - a string that it will print as a prompt for the input. IO.gets doesn't add a new line after the prompt, include it yourself if you need it.

# IO.gets("What's your name?\n")
# # > What's your name?
# # < Mary
# # => "Mary\n"
# Instructions
# You and your friends love to play pen-and-paper role-playing games, but you noticed that it's difficult to get new people to join your group. They often struggle with character creation. They don't know where to start. To help new players out, you decided to write a small program that will guide them through the process.

# Task 1
# Welcome the new player

# Implement the RPG.CharacterSheet.welcome/0 function. It should print a welcome message, and return :ok.

# RPG.CharacterSheet.welcome()
# # > Welcome! Let's fill out your character sheet together.
# # => :ok

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Ask for the character's name

# Implement the RPG.CharacterSheet.ask_name/0 function. It should print a question, wait for an answer, and return the answer without leading and trailing whitespace.

# RPG.CharacterSheet.ask_name()
# # > What is your character's name?
# # < Mathilde
# # => "Mathilde"

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Ask for the character's class

# Implement the RPG.CharacterSheet.ask_class/0 function. It should print a question, wait for an answer, and return the answer without leading and trailing whitespace.

# RPG.CharacterSheet.ask_class()
# # > What is your character's class?
# # < healer
# # => "healer"

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Ask for the character's level

# Implement the RPG.CharacterSheet.ask_level/0 function. It should print a question, wait for an answer, and return the answer as an integer.

# RPG.CharacterSheet.ask_level()
# # > What is your character's level?
# # < 2
# # => 2

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Combine previous steps into one

# Implement the RPG.CharacterSheet.run/0 function. It should welcome the new player, ask for the character's name, class, and level, and return the character sheet as a map. It should also print the map with the label "Your character".

# RPG.CharacterSheet.run()
# # > Welcome! Let's fill out your character sheet together.
# # > What is your character's name?
# # < Mathilde
# # > What is your character's class?
# # < healer
# # > What is your character's level?
# # < 2
# # > Your character: %{class: "healer", level: 2, name: "Mathilde"}
# # => %{class: "healer", level: 2, name: "Mathilde"}


defmodule RPG.CharacterSheet do
  def welcome(), do: IO.puts("Welcome! Let's fill out your character sheet together.")
  def ask_name(), do: IO.gets("What is your character's name?\n") |> String.trim()
  def ask_class(), do: IO.gets("What is your character's class?\n") |> String.trim()
  def ask_level(), do: IO.gets("What is your character's level?\n") |> String.trim() |> String.to_integer()
  def run() do
    welcome()
    name = ask_name()
    class = ask_class()
    level = ask_level()

    %{name: name, class: class, level: level}
    |> IO.inspect(label: "Your character")
  end
end



# Elixir / Name Badge {{If}} {{Nil}}

# Introduction
# Nil
# Nil is an English word meaning "nothing" or "zero". In Elixir, nil is a special value that means an absence of a value.

# # I do not have a favorite color
# favorite_color = nil
# In other programming languages, null or none values might play a similar role.

# If
# Besides cond, Elixir also provides the macro if/2 which is useful when you need to check for only one condition.

# if/2 accepts a condition and two options. It returns the first option if the condition is truthy, and the second option if the condition is falsy.

# age = 15

# if age >= 16 do
#   "You are allowed to drink beer in Germany."
# else
#   "No beer for you!"
# end

# # => "No beer for you!"
# It is also possible to write an if expression on a single line. Note the comma after the condition.

# if age > 16, do: "beer", else: "no beer"
# This syntax is helpful for very short expressions, but should be avoided if the expression won't fit on a single line.

# Truthy and falsy
# In Elixir, all datatypes evaluate to a truthy or falsy value when they are encountered in a boolean context (like an if expression). All data is considered truthy except for false and nil. In particular, empty strings, the integer 0, and empty lists are all considered truthy in Elixir.

# Instructions
# In this exercise you'll be writing code to print name badges for factory employees. Employees have an ID, name, and department name. Employee badge labels are formatted as follows: "[id] - name - DEPARTMENT".

# Task 1
# Print a badge for an employee

# Implement the NameBadge.print/3 function. It should take an id, name, and a department. It should return the badge label, with the department name in uppercase.

# NameBadge.print(67, "Katherine Williams", "Strategic Communication")
# # => "[67] - Katherine Williams - STRATEGIC COMMUNICATION"

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Print a badge for a new employee

# Due to a quirk in the computer system, new employees occasionally don't yet have an ID when they start working at the factory. As badges are required, they will receive a temporary badge without the ID prefix.

# Extend the NameBadge.print/3 function. When the id is missing, it should print a badge without it.

# NameBadge.print(nil, "Robert Johnson", "Procurement")
# # => "Robert Johnson - PROCUREMENT"

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Print a badge for the owner

# Even the factory's owner has to wear a badge at all times. However, an owner does not have a department. In this case, the label should print "OWNER" instead of the department name.

# Extend the NameBadge.print/3 function. When the department is missing, assume the badge belongs to the company owner.

# NameBadge.print(204, "Rachel Miller", nil)
# # => "[204] - Rachel Miller - OWNER"
# Note that it is possible for the owner to also be a new employee.

# NameBadge.print(nil, "Rachel Miller", nil)
# # => "Rachel Miller - OWNER"


defmodule NameBadge do
  def print(id, name, nil), do: print(id, name, "Owner")
  def print(nil, name, department), do: "#{name} - #{department |> String.upcase()}"
  def print(id, name, department), do: "[#{id}] - #{print(nil, name, department)}"
end



# Elixir / Take-A-Number {{PIDs}} {{Processes}}

# Introduction
# Processes
# In Elixir, all code runs inside processes.

# By default, a function will execute in the same process from which it was called. When you need to explicitly run a certain function in a new process, use spawn/1:

# spawn(fn -> 2 + 2 end)
# # => #PID<0.125.0>
# spawn/1 creates a new process that executes the given function and returns a process identifier (PID). The new process will stay alive as long as the function executes, and then silently exit.

# Elixir's processes should not be confused with operating system processes. Elixir's processes use much less memory and CPU. It's perfectly fine to have Elixir applications that run hundreds of Elixir processes.

# Messages
# Processes do not directly share information with one another. Processes send and receive messages to share data.

# You can send a message to any process with send/2. The first argument to send/2 is the PID of the recipient, the second argument is the message.

# A message can be of any type. Often it consists of atoms and tuples. If you want to get a response, you should include the PID of the sender somewhere in the message. You can get the PID of the current process with self().

# send/2 does not check if the message was received by the recipient, nor if the recipient is still alive. The message ends up in the recipient's mailbox and it will only be read if and when the recipient explicitly asks to receive messages.

# A message can be read from a mailbox using the receive/1 macro. It accepts a do block that can pattern match on the messages.

# receive do
#   {:ping, sender_pid} -> send(sender_pid, :pong)
#   :do_nothing -> nil
# end
# receive/1 will take one message from the mailbox that matches any of the given patterns and execute the expression given for that pattern. If there are no messages in the mailbox, or none of messages in the mailbox match any of the patterns, receive/1 is going to wait for one.

# Receive loop
# If you want to receive more than one message, you need to call receive/1 recursively. It is a common pattern to implement a recursive function, for example named loop, that calls receive/1, does something with the message, and then calls itself to wait for more messages. If you need to carry some state from one receive/1 call to another, you can do it by passing an argument to that loop function.

# PIDs
# Process identifiers are their own data type. They function as mailbox addresses - if you have a process's PID, you can send a message to that process. PIDs are usually created indirectly, as a return value of functions that create new processes, like spawn.

# Instructions
# You are writing an embedded system for a Take-A-Number machine. It is a very simple model. It can give out consecutive numbers and report what was the last number given out.

# Task 1
# Start the machine

# Implement the start/0 function. It should spawn a new process that has an initial state of 0 and is ready to receive messages. It should return the process's PID.

# TakeANumber.start()
# # => #PID<0.138.0>
# Note that each time you run this code, the PID may be different.


# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Report the machine state

# Modify the machine so that it can receive {:report_state, sender_pid} messages. It should send its current state (the last given out ticket number) to sender_pid and then wait for more messages.

# machine_pid = TakeANumber.start()
# send(machine_pid, {:report_state, self()})

# receive do
#   msg -> msg
# end
# # => 0

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Give out numbers

# Modify the machine so that it can receive {:take_a_number, sender_pid} messages. It should increase its state by 1, send the new state to sender_pid, and then wait for more messages.

# machine_pid = TakeANumber.start()
# send(machine_pid, {:take_a_number, self()})

# receive do
#   msg -> msg
# end
# # => 1

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Stop the machine

# Modify the machine so that it can receive a :stop message. It should stop waiting for more messages.


# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Ignore unexpected messages

# Modify the machine so that when it receives an unexpected message, it ignores it and continues waiting for more messages.


defmodule TakeANumber do
  def start(), do: spawn(&loop/0)

  defp loop(state \\ 0) do
    receive do
      {:report_state, sender_pid} -> send(sender_pid, state) |> loop()
      {:take_a_number, sender_pid} -> send(sender_pid, state + 1) |> loop()
      :stop -> nil
      _ -> loop(state)
    end
  end
end



# Elixir / Wine Cellar {{Keyword Lists}}

# Introduction
# Keyword Lists
# Keyword lists are a key-value data structure.

# [month: "April", year: 2018]
# Keyword lists are lists of {key, value} tuples, and can also be written as such, but the shorter syntax is more widely used.

# [month: "April"] == [{:month, "April"}]
# # => true
# Keys in a keyword list must be atoms, but the values can be anything. Each key can be used more than once. The key-value pairs in a keyword list are ordered.

# You can work with keyword lists using the same approaches as for lists, or you can use the Keyword module.

# Instructions
# You are the manager of a fancy restaurant that has a sizable wine cellar. A lot of your customers are demanding wine enthusiasts. Finding the right bottle of wine for a particular customer is not an easy task.

# As a tech-savvy restaurant owner, you decided to speed up the wine selection process by writing an app that will let guests filter your wines by their preferences.

# Task 1
# Explain wine colors

# On the welcome screen of your app, you want to display a short explanation of each wine color.

# Implement the WineCellar.explain_colors/0 function. It takes no arguments and returns a keyword list with wine colors as keys and explanations as values.

# Color	Explanation
# :white	Fermented without skin contact.
# :red	Fermented with skin contact using dark-colored grapes.
# :rose	Fermented with some skin contact, but not enough to qualify as a red wine.

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Get all wines of a given color

# A bottle of wine is represented as a 3-tuple of grape variety, year, and country of origin. The wines are stored by wine color in a keyword list.

# [
#   white: {"Chardonnay", 2015, "Italy"},
#   white: {"Pinot grigio", 2017, "Germany"},
#   red: {"Pinot noir", 2016, "France"},
#   rose: {"Dornfelder", 2018, "Germany"}
# ]
# Implement the WineCellar.filter/3 function. It should take a keyword list of wines, a color atom and a keyword list of options, with a default value of []. The function should return a list of wines of a given color.

# WineCellar.filter(
#   [
#     white: {"Chardonnay", 2015, "Italy"},
#     white: {"Pinot grigio", 2017, "Germany"},
#     red: {"Pinot noir", 2016, "France"},
#     rose: {"Dornfelder", 2018, "Germany"}
#   ],
#   :white
# )
# # => [
# #      {"Chardonnay", 2015, "Italy"},
# #      {"Pinot grigio", 2017, "Germany"}
# #    ]

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Get all wines of a given color bottled in a given year

# Extend the WineCellar.filter/3 function. When given a :year option, the function should return a list of wines of a given color from a given year.

# Use the already-implemented WineCellar.filter_by_year/2 function. It takes a list of wines and a year as arguments and returns a list of wines from a given year.

# WineCellar.filter(
#   [
#     white: {"Chardonnay", 2015, "Italy"},
#     white: {"Pinot grigio", 2017, "Germany"},
#     red: {"Pinot noir", 2016, "France"},
#     rose: {"Dornfelder", 2018, "Germany"}
#   ],
#   :white,
#   year: 2015
# )
# # => [
# #      {"Chardonnay", 2015, "Italy"}
# #    ]

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Get all wines of a given color bottled in a given country

# Extend the WineCellar.filter/3 function. When given a :country option, the function should return a list of wines of a given color from a given country.

# Use the already-implemented WineCellar.filter_by_country/2 function. It takes a list of wines and a country as arguments and returns a list of wines from a given country.

# Make sure that the function works when given both the :year and the :country option, in any order.

# WineCellar.filter(
#   [
#     white: {"Chardonnay", 2015, "Italy"},
#     white: {"Pinot grigio", 2017, "Germany"},
#     red: {"Pinot noir", 2016, "France"},
#     rose: {"Dornfelder", 2018, "Germany"}
#   ],
#   :white,
#   year: 2015,
#   country: "Germany"
# )
# # => []


defmodule WineCellar do
  def explain_colors, do:
    [
      white: "Fermented without skin contact.",
      red: "Fermented with skin contact using dark-colored grapes.",
      rose: "Fermented with some skin contact, but not enough to qualify as a red wine."
    ]

  def filter(cellar, color, opts \\ []) do
    Keyword.get_values(cellar, color)
    |> filter_by_year(opts[:year])
    |> filter_by_country(opts[:country])
  end

  defp filter_by_year(wines, nil), do: wines
  defp filter_by_year([], _year), do: []
  defp filter_by_year([{_, year, _} = wine | tail], year), do: [wine | filter_by_year(tail, year)]
  defp filter_by_year([{_, _, _} | tail], year), do: filter_by_year(tail, year)
  defp filter_by_country(wines, nil), do: wines
  defp filter_by_country([], _country), do: []
  defp filter_by_country([{_, _, country} = wine | tail], country), do: [wine | filter_by_country(tail, country)]
  defp filter_by_country([{_, _, _} | tail], country), do: filter_by_country(tail, country)
end



# Elixir / DNA Encoding {{Bitstrings}} {{Tail Call Recursion}}

# Introduction
# Bitstrings
# Working with binary data is an important concept in any language, and Elixir provides an elegant syntax to write, match, and construct binary data.

# In Elixir, binary data is referred to as the bitstring type. The binary data type (not to be confused with binary data in general) is a specific form of a bitstring, which we will discuss in a later exercise.

# Bitstring literals are defined using the bitstring special form <<>>. When defining a bitstring literal, it is defined in segments. Each segment has a value and type, separated by the :: operator. The type specifies how many bits will be used to encode the value. If the value of the segment overflows the capacity of the segment's type, it will be truncated from the left.

# # This defines a bitstring with three segments of a single bit each
# <<0::1, 1::1, 0::1>> == <<0::size(1), 1::size(1), 0::size(1)>>
# # => true
# <<0::1, 1::1, 0::1>> == <<2::size(3)>>
# # => true
# <<2::1>> == <<0::1>>
# # => true because of value overflow
# When writing binary integer literals, we can write them directly in base-2 notation by prefixing the literal with 0b.

# value = 0b11111011011 = 2011
# By default, bitstrings are displayed in chunks of 8 bits (a byte).

# <<value::11>>
# # => <<251, 3::size(3)>>
# Constructing
# We can combine bitstrings stored in variables using the special form:

# first = <<0b110::3>>
# second = <<0b001::3>>
# combined = <<first::bitstring, second::bitstring>>
# # => <<49::size(6)>>
# Pattern matching
# Pattern matching can also be done to obtain the value from within the special form:

# <<value::4, rest::bitstring>> = <<0b01101001::8>>
# value == 0b0110
# # => true
# Tail Call Recursion
# When recursing through enumerables (lists, bitstrings, strings), there are often two concerns:

# how much memory is required to store the trail of recursive function calls
# how to build the solution efficiently
# To deal with these concerns an accumulator may be used.

# An accumulator is a variable that is passed along in addition to the data. It is used to pass the current state of the function's execution, from function call to function call, until the base case is reached. In the base case, the accumulator is used to return the final value of the recursive function call.

# Accumulators should be initialized by the function's author, not the function's user. To achieve this, declare two functions - a public function that takes just the necessary data as arguments and initializes the accumulator, and a private function that also takes an accumulator. In Elixir, it is a common pattern to prefix the private function's name with do_.

# # Count the length of a list without an accumulator
# def count([]), do: 0
# def count([_head | tail]), do: 1 + count(tail)

# # Count the length of a list with an accumulator
# def count(list), do: do_count(list, 0)

# defp do_count([], count), do: count
# defp do_count([_head | tail], count), do: do_count(tail, count + 1)
# The usage of an accumulator allows us to turn recursive functions into tail-recursive functions. A function is tail-recursive if the last thing executed by the function is a call to itself.

# Instructions
# In your DNA research lab, you have been working through various ways to compress your research data to save storage space. One teammate suggests converting the DNA data to a binary representation:

# Nucleic Acid	Code
# a space	0000
# A	0001
# C	0010
# G	0100
# T	1000
# You ponder this, as it will potentially halve the required data storage costs, but at the expense of human readability. You decide to write a module to encode and decode your data to benchmark your savings.

# Task 1
# Encode nucleic acid to binary value

# Implement encode_nucleotide/1 to accept the code point for the nucleic acid and return the integer value of the encoded code.

# DNA.encode_nucleotide(?A)
# # => 0b0001

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Decode the binary value to the nucleic acid

# Implement decode_nucleotide/1 to accept the integer value of the encoded code and return the code point for the nucleic acid.

# DNA.decode_nucleotide(0b0001)
# # => ?A

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Encode a DNA charlist

# Implement encode/1 to accept a charlist representing nucleic acids and gaps and return a bitstring of the encoded data.

# DNA.encode('AC GT')
# # => <<18, 4, 8::size(4)>>

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Decode a DNA bitstring

# Implement decode/1 to accept a bitstring representing nucleic acids and gaps and return the decoded data as a charlist.

# DNA.decode(<<132, 2, 1::size(4)>>)
# # => 'TG CA'


defmodule DNA do
  @code_point_map %{
    ?\s => 0b0000,
    ?A => 0b0001,
    ?C => 0b0010,
    ?G => 0b0100,
    ?T => 0b1000
  }
  @encoded_code_point_map @code_point_map |> Map.new(fn {key, val} -> {val, key} end)

  def encode_nucleotide(code_point), do: @code_point_map[code_point]
  def decode_nucleotide(encoded_code), do: @encoded_code_point_map[encoded_code]

  def encode([]), do: <<>>
  def encode([head | tail]), do: <<encode_nucleotide(head)::4, encode(tail)::bitstring>>

  def decode(<<>>), do: []
  def decode(<<value::4, rest::bitstring>>), do: [decode_nucleotide(value) | decode(rest)]
end



# Elixir / Library Fees {{Dates and Time}}

# Introduction
# Dates and Time
# Elixir's standard library offers 4 different modules for working with dates and time, each with its own struct.

# The Date module. A Date struct can be created with the ~D sigil.

# ~D[2021-01-01]
# The Time module. A Time struct can be created with the ~T sigil.

# ~T[12:00:00]
# The NaiveDateTime module for datetimes without a timezone. A NaiveDateTime struct can be created with the ~N sigil.

# ~N[2021-01-01 12:00:00]
# The DateTime module for datetimes with a timezone. Using this module for timezones other than UTC requires an external dependency, a timezone database.

# Comparisons
# To compare dates or times to one another, look for a compare or diff function in the corresponding module. Comparison operators such as ==, >, and < seem to work, but they don't do a correct semantic comparison for those structs.

# Instructions
# Your librarian friend has asked you to extend her library software to automatically calculate late fees. Her current system stores the exact date and time of a book checkout as an ISO8601 datetime string. She runs a local library in a small town in Ghana, which uses the GMT timezone (UTC +0), doesn't use daylight saving time, and doesn't need to worry about other timezones.

# Task 1
# Parse the stored datetimes

# Implement the LibraryFees.datetime_from_string/1 function. It should take an ISO8601 datetime string as an argument, and return a NaiveDateTime struct.

# LibraryFees.datetime_from_string("2021-01-01T13:30:45Z")
# # => ~N[2021-01-01 13:30:45]

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Determine if a book was checked out before noon

# If a book was checked out before noon, the reader has 28 days to return it. If it was checked out at or after noon, it's 29 days.

# Implement the LibraryFees.before_noon?/1 function. It should take a NaiveDateTime struct and return a boolean.

# LibraryFees.before_noon?(~N[2021-01-12 08:23:03])
# # => true

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Calculate the return date

# Based on the checkout datetime, calculate the return date.

# Implement the LibraryFees.return_date/1 function. It should take a NaiveDateTime struct and return a Date struct, either 28 or 29 days later.

# LibraryFees.return_date(~N[2020-11-28 15:55:33])
# # => ~D[2020-12-27]

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Determine how late the return of the book was

# The library has a flat rate for late returns. To be able to calculate the fee, we need to know how many days after the return date the book was actually returned.

# Implement the LibraryFees.days_late/2 function. It should take a Date struct - the planned return datetime, and a NaiveDateTime struct - the actual return datetime.

# If the actual return date is on an earlier or the same day as the planned return datetime, the function should return 0. Otherwise, the function should return the difference between those two dates in days.

# The library tracks both the date and time of the actual return of the book for statistical purposes, but doesn't use the time when calculating late fees.

# LibraryFees.days_late(~D[2020-12-27], ~N[2021-01-03 09:23:36])
# # => 7

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Determine if the book was returned on a Monday

# The library has a special offer for returning books on Mondays.

# Implement the LibraryFees.monday?/1 function. It should take a NaiveDateTime struct and return a boolean.

# LibraryFees.monday?(~N[2021-01-03 13:30:45Z])
# # => false

# Stuck? Reveal Hints
# Opens in a modal
# Task 6
# Calculate the late fee

# Implement the LibraryFees.calculate_late_fee/3 function. It should take three arguments - two ISO8601 datetime strings, checkout datetime and actual return datetime, and the late fee for one day. It should return the total late fee according to how late the actual return of the book was.

# Include the special Monday offer. If you return the book on Monday, your late fee is 50% off, rounded down.

# # Sunday, 7 days late
# LibraryFees.calculate_late_fee("2020-11-28T15:55:33Z", "2021-01-03T13:30:45Z", 100)
# # => 700

# # one day later, Monday, 8 days late
# LibraryFees.calculate_late_fee("2020-11-28T15:55:33Z", "2021-01-04T09:02:11Z", 100)
# # => 400


defmodule LibraryFees do
  def datetime_from_string(string), do: NaiveDateTime.from_iso8601!(string)
  def before_noon?(datetime), do: datetime.hour < 12
  def return_date(checkout_datetime) do
    checkout_datetime |> Date.add(if before_noon?(checkout_datetime), do: 28, else: 29)
  end
  def days_late(planned_return_date, actual_return_datetime) do
    actual_return_datetime |> Date.diff(planned_return_date) |> max(0)
  end
  def monday?(datetime), do: datetime |> Date.day_of_week() == 1
  def calculate_late_fee(checkout_datetime, actual_return_datetime, late_fee_per_day) do
    checkout_datetime = datetime_from_string(checkout_datetime)
    actual_return_datetime = datetime_from_string(actual_return_datetime)
    days_late = days_late(return_date(checkout_datetime), actual_return_datetime)
    late_fee = days_late * late_fee_per_day
    if monday?(actual_return_datetime), do: div(late_fee, 2), else: late_fee
  end
end



# Elixir / Basketball Website {{Access Behaviour}}

# Introduction
# Access Behaviour
# Elixir uses code Behaviours to provide common generic interfaces while facilitating specific implementations for each module which implements it. One such common example is the Access Behaviour.

# The Access Behaviour provides a common interface for retrieving data from a key-based data structure. The Access Behaviour is implemented for maps and keyword lists, but let's look at its use for maps to get a feel for it. Access Behaviour specifies that when you have a map, you may follow it with square brackets and then use the key to retrieve the value associated with that key.

# # Suppose we have these two maps defined (note the difference in the key type)
# my_map = %{key: "my value"}
# your_map = %{"key" => "your value"}

# # Obtain the value using the Access Behaviour
# my_map[:key] == "my value"
# your_map[:key] == nil
# your_map["key"] == "your value"
# If the key does not exist in the data structure, then nil is returned. This can be a source of unintended behavior, because it does not raise an error. Note that nil itself implements the Access Behaviour and always returns nil for any key.

# Instructions
# You are working with a web development team to maintain a website for a local basketball team. The web development team is less familiar with Elixir and is asking for a function to be able to extract data from a series of nested maps to facilitate rapid development.

# Task 1
# Extract data from a nested map structure

# Implement the extract_from_path/2 function to take two arguments:

# data: a nested map structure with data about the basketball team.
# path: a string consisting of period-delimited keys to obtain the value associated with the last key.
# If the value or the key does not exist at any point in the path, nil should be returned

# data = %{
#   "team_mascot" => %{
#     "animal" => "bear",
#     "actor" => %{
#       "first_name" => "Noel"
#     }
#   }
# }
# BasketballWebsite.extract_from_path(data, "team_mascot.animal")
# # => "bear"
# BasketballWebsite.extract_from_path(data, "team_mascot.colors")
# # => nil
# Use the Access Behaviour when implementing this function.

# Do not use any Map or Kernel module functions for working with the nested map data structure.


# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Refactor using included functions

# Your coworker reviewing your code tells you about a Kernel module function which does something very similar to your implementation.

# Implement get_in_path/2 to use this Kernel module function.

# The arguments expected are the same as part 1.

# BasketballWebsite.get_in_path(data, "team_mascot.actor.first_name")
# # => "Noel"


defmodule BasketballWebsite do
  def extract_from_path(data, path), do: path |> String.split(".") |> Enum.reduce(data, & &2[&1])
  def get_in_path(data, path), do: data |> get_in(path |> String.split("."))
end



# Elixir / Boutique Inventory {{Enum}}

# Introduction
# Enum
# Enum is a very useful module that provides a set of algorithms for working with enumerables. It offers sorting, filtering, grouping, counting, searching, finding min/max values, and much more.

# In general, an enumerable is any data that can be iterated over, a collection. In Elixir, an enumerable is any data type that implements the Enumerable protocol. The most common of those are lists and maps.

# Many Enum functions accept a function as an argument.

# Enum.all?([1, 2, 3, 4, 5], fn x -> x > 3 end)
# # => false
# The most common Enum functions are map and reduce.

# map/2
# Enum.map/2 allows you to replace every element in an enumerable with another element. The second argument to Enum.map/2 is a function that accepts the original element and returns its replacement.

# reduce/3
# Enum.reduce/3 allows you to reduce the whole enumerable to a single value. To achieve this, a special variable called the accumulator is used. The accumulator carries the intermediate state of the reduction between iterations.

# The second argument to Enum.reduce/3 is the initial value of the accumulator. The third argument is a function that accepts an element and an accumulator, and returns the new value for the accumulator.

# Working with maps
# When using maps with Enum functions, the map gets automatically converted to a list of 2 {key, value} tuples.

# To transform it back to a map, use Enum.into/2. Enum.into/2 is a function that transforms an enumerable into a collectable - any data structure implementing the Collectable protocol. It can be thought of as the opposite of Enum.reduce/3.

# Enum also has Enum.into/3. Enum.into/3 is a variation of Enum.into/2 that accepts a transformation function to be applied while transforming the enumerable into a collectable.

# Mapping maps
# Instead of using Enum.into/3 or Enum.map/2 plus Enum.into/1 to apply a transformation (mapping) to a map, we can also use a dedicated Map.new/2 function. It works exactly like Enum.into/3 in that it accepts an enumerable and a transformation function, but it always returns a new map instead of letting us choose a collectible.

# Instructions
# You are running an online fashion boutique. Your big annual sale is coming up, so you need to take stock of your inventory to make sure you're ready.

# A single item in the inventory is represented by a map, and the whole inventory is a list of such maps.

# %{
#   name: "White Shirt",
#   price: 40,
#   quantity_by_size: %{s: 3, m: 7, l: 8, xl: 4}
# }
# Task 1
# Sort items by price

# Implement the sort_by_price/1 function. It should take the inventory and return it sorted by item price, ascending.

# BoutiqueInventory.sort_by_price([
#   %{price: 65, name: "Maxi Brown Dress", quantity_by_size: %{}},
#   %{price: 50, name: "Red Short Skirt", quantity_by_size: %{}},
#   %{price: 50, name: "Black Short Skirt", quantity_by_size: %{}},
#   %{price: 20, name: "Bamboo Socks Cats", quantity_by_size: %{}}
# ])
# # => [
# #      %{price: 20, name: "Bamboo Socks Cats", quantity_by_size: %{}},
# #      %{price: 50, name: "Red Short Skirt", quantity_by_size: %{}},
# #      %{price: 50, name: "Black Short Skirt", quantity_by_size: %{}},
# #      %{price: 65, name: "Maxi Brown Dress", price: 65, quantity_by_size: %{}}
# #    ]

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Find all items with missing prices

# After sorting your inventory by price, you noticed that you must have made a mistake when you were taking stock and forgot to fill out prices for a few items.

# Implement the with_missing_price/1 function. It should take the inventory and return a list of items that do not have prices.

# BoutiqueInventory.with_missing_price([
#   %{price: 40, name: "Black T-shirt", quantity_by_size: %{}},
#   %{price: nil, name: "Denim Pants", quantity_by_size: %{}},
#   %{price: nil, name: "Denim Skirt", quantity_by_size: %{}},
#   %{price: 40, name: "Orange T-shirt", quantity_by_size: %{}}
# ])
# # => [
# #      %{price: nil, name: "Denim Pants", quantity_by_size: %{}},
# #      %{price: nil, name: "Denim Skirt", quantity_by_size: %{}}
# #    ]

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Update item names

# You noticed that some item names have a word that you don't like to use anymore. Now you need to update all the item names with that word.

# Implement the update_names/3 function. It should take the inventory, the old word that you want to remove, and a new word that you want to use instead. It should return a list of items with updated names.

# BoutiqueInventory.update_names(
#   [
#     %{price: 40, name: "Black T-shirt", quantity_by_size: %{}},
#     %{price: 70, name: "Denim Pants", quantity_by_size: %{}},
#     %{price: 65, name: "Denim Skirt", quantity_by_size: %{}},
#     %{price: 40, name: "Orange T-shirt", quantity_by_size: %{}}
#   ],
#   "T-shirt",
#   "Tee"
# )
# # => [
# #      %{price: 40, name: "Black Tee", quantity_by_size: %{}},
# #      %{price: 70, name: "Denim Pants", quantity_by_size: %{}},
# #      %{price: 65, name: "Denim Skirt", quantity_by_size: %{}},
# #      %{price: 40, name: "Orange Tee", quantity_by_size: %{}}
# #    ]

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Increment the item's quantity

# Some items were selling especially well, so you ordered more, in all sizes.

# Implement the increase_quantity/2 function. It should take a single item and a number n, and return that item with the quantity for each size increased by n.

# BoutiqueInventory.increase_quantity(
#  %{
#    name: "Polka Dot Skirt",
#    price: 68,
#    quantity_by_size: %{s: 3, m: 5, l: 3, xl: 4}
#  },
#  6
# )
# # => %{
# #      name: "Polka Dot Skirt",
# #      price: 68,
# #      quantity_by_size: %{l: 9, m: 11, s: 9, xl: 10}
# #    }


# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Calculate the item's total quantity

# To know how much space you need in your storage, you need to know how many of each item you have in total.

# Implement the total_quantity/1 function. It should take a single item and return how many pieces you have in total, in any size.

# BoutiqueInventory.total_quantity(%{
#   name: "Red Shirt",
#   price: 62,
#   quantity_by_size: %{s: 3, m: 6, l: 5, xl: 2}
# })
# # => 16


defmodule BoutiqueInventory do
  def sort_by_price(inventory), do: inventory |> Enum.sort_by(& &1.price)
  def with_missing_price(inventory), do: inventory |> Enum.filter(& is_nil(&1.price))

  def update_names(inventory, old_word, new_word) do
    inventory
    |> Enum.map(fn item ->
      %{item | name: String.replace(item.name, old_word, new_word)}
    end)
  end

  def increase_quantity(item, count) do
    %{item | quantity_by_size: item.quantity_by_size |> Map.new(fn {k, v} -> {k, v + count} end)}
  end

  def total_quantity(item) do
    item.quantity_by_size
    |> Enum.reduce(0, fn {_size, quantity}, acc -> acc + quantity end)
  end
end



# Elixir / File Sniffer {{Binaries}}

# Introduction
# Binaries
# Elixir provides an elegant syntax for working with binary data as we have seen with the <<>> special form provided for working with bitstrings.

# The binary type is a specialization on the bitstring type. Where bitstrings could be of any length (any number of bits), binaries are where the number of bits can be evenly divided by 8. That is, when working with binaries, we often think of things in terms of bytes (8 bits). A byte can represent integer numbers from 0 to 255. It is common to work with byte values in hexadecimal, 0x00 - 0xFF.

# Binary literals are defined using the bitstring special form <<>>. When defining a binary literal, we can use integer and string literals. Integer values greater than 255 will overflow and only the last 8 bits of the integer will be used. By default, the ::binary modifier is applied to the value. We can concatenate binaries with the <>/2 operator.

# <<255>> == <<0xFF>>
# <<256>> == <<0>> # Overflowing bits are truncated
# <<2, 4, 6, 8, 10, 12, 14, 16>> == <<0x02, 0x04, 0x06, 0x08, 0x0A, 0x0C, 0x0E, 0x10>>
# A null-byte is another name for <<0>>.

# Pattern matching on binary data
# Pattern matching is even extended to binaries, and we can pattern match on a portion of binary data much like we could for a list.

# # Ignore the first 8 bytes, match and bind the remaining to `body`
# <<_::binary-size(8), body::binary>>
# Like with other types of pattern matching, we can use this in function signatures to match when selecting from multiple function clauses.

# Instructions
# You have been working on a project which allows users to upload files to the server to be shared with other users. You have been tasked with writing a function to verify that an upload matches its media type. You do some research and discover that the first few bytes of a file are generally unique to that file type, giving it a sort of signature.

# Use the following table for reference:

# File type	Common extension	Media type	binary 'signature'
# ELF	"exe"	"application/octet-stream"	0x7F, 0x45, 0x4C, 0x46
# BMP	"bmp"	"image/bmp"	0x42, 0x4D
# PNG	"png"	"image/png"	0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A
# JPG	"jpg"	"image/jpg"	0xFF, 0xD8, 0xFF
# GIF	"gif"	"image/gif"	0x47, 0x49, 0x46
# Task 1
# Given an extension, return the expected media type

# Implement the type_from_extension/1 function. It should take a file extension (string) and return the media type (string) or nil if the extension does not match with the expected ones.

# FileSniffer.type_from_extension("exe")
# # => "application/octet-stream"

# FileSniffer.type_from_extension("txt")
# # => nil

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Given a binary file, return the expected media type

# Implement the type_from_binary/1 function. It should take a file (binary) and return the media type (string) or nil if the extension does not match with the expected ones.

# file = File.read!("application.exe")
# FileSniffer.type_from_binary(file)
# # => "application/octet-stream"

# file = File.read!("example.txt")
# FileSniffer.type_from_binary(file)
# # => nil
# Don't worry about reading the file as a binary. Assume that has been done for you and is provided by the tests as an argument.


# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Given an extension and a binary file, verify that the file matches the expected type

# Implement the verify/2 function. It should take a file (binary) and extension (string) and return an :ok or :error tuple.

# file = File.read!("application.exe")

# FileSniffer.verify(file, "exe")
# # => {:ok, "application/octet-stream"}

# FileSniffer.verify(file, "png")
# # => {:error, "Warning, file format and file extension do not match."}


defmodule FileSniffer do
  @types %{
    "exe" => {"application/octet-stream", <<0x7F, 0x45, 0x4C, 0x46>>},
    "png" => {"image/png", <<0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A>>},
    "bmp" => {"image/bmp", <<0x42, 0x4D>>},
    "gif" => {"image/gif", <<0x47, 0x49, 0x46>>},
    "jpg" => {"image/jpg", <<0xFF, 0xD8, 0xFF>>}
  }

  def type_from_extension(extension), do: case @types[extension], do: ({type, _} -> type; _ -> nil)
  def type_from_binary(binary) do
    @types
    |> Enum.find_value(fn {_extension, {type, signature}} ->
      if String.starts_with?(binary, signature), do: type
    end)
  end
  def verify(file, extension) do
    case {type_from_extension(extension), type_from_binary(file)} do
      {expected, expected} -> {:ok, expected}
      {expected, actual} -> {:error, "Warning, file format and file extension do not match."}
    end
  end
end



# Elixir / Newsletter {{File}}

# Introduction
# File
# Functions for working with files are provided by the File module.

# To read a whole file, use File.read/1. To write to a file, use File.write/2.

# Every time a file is written to with File.write/2, a file descriptor is opened and a new Elixir process is spawned. For this reason, writing to a file in a loop using File.write/2 should be avoided.

# Instead, a file can be opened using File.open/2. The second argument to File.open/2 is a list of modes, which allows you to specify if you want to open the file for reading or for writing.

# File.open/2 returns a PID of a process that handles the file. To read and write to the file, use functions from the IO module and pass this PID as the IO device.

# When you're finished working with the file, close it with File.close/1.

# All the mentioned functions from the File module also have a ! variant that raises an error instead of returning an error tuple (e.g. File.read!/1). Use that variant if you don't intend to handle errors such as missing files or lack of permissions.

# Instructions
# You're a big model train enthusiast and have decided to share your passion with the world by starting a newsletter. You'll start by sending the first issue of your newsletter to your friends and acquaintances that share your hobby. You have a text file with a list of their email addresses.

# Task 1
# Read email addresses from a file

# Implement the Newsletter.read_emails/1 function. It should take a file path. The file is a text file that contains email addresses separated by newlines. The function should return a list of the email addresses from the file.

# Newsletter.read_emails("/home/my_user/documents/model_train_friends_emails.txt")
# # => ["rick@example.com", "choochoo42@example.com", "anna@example.com"]

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Open a log file for writing

# Sending an email is a task that might fail for many unpredictable reasons, like a typo in the email address or temporary network issues. To ensure that you can retry sending the emails to all your friends without sending duplicates, you need to log the email addresses that already received the email. For this, you'll need a log file.

# Implement the Newsletter.open_log/1 function. It should take a file path, open the file for writing, and return the PID of the process that handles the file.

# Newsletter.open_log("/home/my_user/documents/newsletter_issue1_log.txt")
# # => #PID<0.145.0>

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Log a sent email

# Implement the Newsletter.log_sent_email/2 function. It should take a PID of the process that handles the file and a string with the email address. It should write the email address to the file, followed by a newline.

# Newsletter.log_sent_email(pid, "joe@example.com")
# # => :ok

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Close the log file

# Implement the Newsletter.close_log/1 function. It should take a PID of the process that handles the file and close the file.

# Newsletter.close_log(pid)
# # => :ok

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Send the newsletter

# Now that you have all of the building blocks of the email sending procedure, you need to combine them together in a single function.

# Implement the Newsletter.send_newsletter/3 function. It should take a path of the file with email addresses, a path of a log file, and an anonymous function that sends an email to a given email address. It should read all the email addresses from the given file and attempt to send an email to every one of them. If the anonymous function that sends the email returns :ok, write the email address to the log file, followed by a new line. Make sure to do it as soon as the email is sent. Afterwards, close the log file.

# Newsletter.send_newsletter(
#   "model_train_friends_emails.txt",
#   "newsletter_issue1_log.txt",
#   fn email -> :ok end
# )
# # => :ok


defmodule Newsletter do
  def read_emails(path), do: path |> File.read!() |> String.split()
  def open_log(path), do: File.open!(path, [:write])
  def log_sent_email(pid, email), do: IO.puts(pid, email)
  def close_log(pid), do: File.close(pid)
  def send_newsletter(emails_path, log_path, send_fun) do
    log = open_log(log_path)

    read_emails(emails_path)
    |> Enum.each(&(send_fun.(&1) == :ok and log_sent_email(log, &1)))

    close_log(log)
  end
end



# Elixir / Chessboard {{Ranges}}

# Introduction
# Ranges
# Ranges represent a sequence of one or many consecutive integers. They are created by connecting two integers with ...

# 1..5
# Ranges can be ascending or descending. They are always inclusive of the first and last values.

# A range implements the Enumerable protocol, which means functions in the Enum module can be used to work with ranges.

# Instructions
# As a chess enthusiast, you would like to write your own version of the game. Yes, there maybe plenty of implementations of chess available online already, but yours will be unique!

# But before you can let your imagination run wild, you need to take care of the basics. Let's start by generating the board.

# Each square of the chessboard is identified by a letter-number pair. The vertical columns of squares, called files, are labeled A through H. The horizontal rows of squares, called ranks, are numbered 1 to 8.

# Task 1
# Define the rank range

# Implement the rank_range/0 function. It should return a range of integers, from 1 to 8.

# Chessboard.rank_range()

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Define the file range

# Implement the file_range/0 function. It should return a range of integers, from the code point of the uppercase letter A, to the code point of the uppercase letter H.

# Chessboard.file_range()

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Transform the rank range into a list of ranks

# Implement the ranks/0 function. It should return a list of integers, from 1 to 8. Do not write the list by hand, generate it from the range returned by the rank_range/0 function.

# Chessboard.ranks()
# # => [1, 2, 3, 4, 5, 6, 7, 8]

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Transform the file range into a list of files

# Implement the files/0 function. It should return a list of letters (strings), from "A" to "H". Do not write the list by hand, generate it from the range returned by the file_range/0 function.

# Chessboard.files()
# # => ["A", "B", "C", "D", "E", "F", "G", "H"]


defmodule Chessboard do
  def rank_range(), do: 1..8
  def file_range(), do: ?A..?H
  def ranks(), do: rank_range() |> Enum.to_list()
  def files(), do: file_range() |> Enum.map(& <<&1>>)
end



# Elixir / Remote Control Car {{Structs}}

# Introduction
# Structs
# Structs are an extension built on top of maps which provide compile-time checks and default values. A struct is named after the module it is defined in. To define a struct use the defstruct construct. The construct usually immediately follows after the module definition. defstruct accepts either a list of atoms (for nil default values) or a keyword list (for specified default values). The fields without defaults must precede the fields with default values.

# defmodule Plane do
#   defstruct [:engine, wings: 2]
# end

# plane = %Plane{}
# # => %Plane{engine: nil, wings: 2}
# Accessing fields and updating
# Since structs are built on maps, we can use most map functions to get and manipulate values. The Access Behaviour is not implemented for structs. It is recommended to use the static access operator . to access struct fields instead.

# get/fetch field values:

# plane = %Plane{}
# plane.engine
# # => nil
# Map.fetch(plane, :wings)
# # => 2
# update field values

# plane = %Plane{}
# %{plane | wings: 4}
# # => %Plane{engine: nil, wings: 4}
# Enforcing field value initialization
# We can use the @enforce_keys module attribute with a list of the field keys to ensure that the values are initialized when the struct is created. If a key is not listed, its value will be nil as seen in the above example. If an enforced key is not initialized, an error is raised.

# defmodule User do
#   @enforce_keys [:username]
#   defstruct [:username]
# end

# %User{}
# # => (ArgumentError) the following keys must also be given when building struct User: [:username]
# Instructions
# In this exercise you'll be playing around with a remote controlled car, which you've finally saved enough money for to buy.

# Cars start with full (100%) batteries. Each time you drive the car using the remote control, it covers 20 meters and drains one percent of the battery. The car's nickname is not known until it is created.

# The remote controlled car has a fancy LED display that shows two bits of information:

# The total distance it has driven, displayed as: "<METERS> meters".
# The remaining battery charge, displayed as: "Battery at <PERCENTAGE>%".
# If the battery is at 0%, you can't drive the car anymore and the battery display will show "Battery empty".

# Task 1
# Create a brand-new remote controlled car

# Implement the RemoteControlCar.new/0 function to return a brand-new remote controlled car struct:

# RemoteControlCar.new()
# # => %RemoteControlCar{
# #      battery_percentage: 100,
# #      distance_driven_in_meters: 0,
# #      nickname: "none"
# #    }
# The nickname is required by the struct, make sure that a value is initialized in the new function, but not in the struct.


# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Create a brand-new remote controlled car with a nickname

# Implement the RemoteControlCar.new/1 function to return a brand-new remote controlled car struct with a provided nickname:

# RemoteControlCar.new("Blue")
# # => %RemoteControlCar{
# #      battery_percentage: 100,
# #      distance_driven_in_meters: 0,
# #      nickname: "Blue"
# #    }

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Display the distance

# Implement the RemoteControlCar.display_distance/1 function to return the distance as displayed on the LED display:

# car = RemoteControlCar.new()
# RemoteControlCar.display_distance(car)
# # => "0 meters"
# Make sure the function only accepts a RemoteControlCar struct as the argument.


# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Display the battery percentage

# Implement the RemoteControlCar.display_battery/1 function to return the battery percentage as displayed on the LED display:

# car = RemoteControlCar.new()
# RemoteControlCar.display_battery(car)
# # => "Battery at 100%"
# Make sure the function only accepts a RemoteControlCar struct as the argument. If the battery is at 0%, the battery display will show "Battery empty".


# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Driving changes the battery and distance driven

# Implement the RemoteControlCar.drive/1 function that:

# updates the number of meters driven by 20
# drains 1% of the battery
# RemoteControlCar.new("Red")
# |> RemoteControlCar.drive()
# # => %RemoteControlCar{
# #      battery_percentage: 99,
# #      distance_driven_in_meters: 20,
# #      nickname: "Red"
# #    }

# Make sure the function only accepts a RemoteControlCar struct as the argument.


# Stuck? Reveal Hints
# Opens in a modal
# Task 6
# Account for driving with a dead battery

# Update the RemoteControlCar.drive/1 function to not increase the distance driven nor decrease the battery percentage when the battery is drained (at 0%):

# %RemoteControlCar{
#   battery_percentage: 0,
#   distance_driven_in_meters: 1980,
#   nickname: "Red"
# }
# |> RemoteControlCar.drive()
# # => %RemoteControlCar{
# #      battery_percentage: 0,
# #      distance_driven_in_meters: 1980,
# #      nickname: "Red"
# #    }


defmodule RemoteControlCar do
  @enforce_keys [:nickname]
  defstruct [
    :nickname,
    battery_percentage: 100,
    distance_driven_in_meters: 0
  ]

  @meters_per_drive_unit 20
  @battery_percent_per_drive_unit 1

  def new(nickname \\ "none"), do: %__MODULE__{nickname: nickname}
  def display_distance(%__MODULE__{} = module), do: "#{module.distance_driven_in_meters} meters"
  def display_battery(%__MODULE__{battery_percentage: 0}), do: "Battery empty"
  def display_battery(%__MODULE__{} = module), do: "Battery at #{module.battery_percentage}%"
  def drive(%__MODULE__{battery_percentage: 0} = module), do: module
  def drive(%__MODULE__{} = module), do: %__MODULE__{ module |
      battery_percentage: module.battery_percentage - @battery_percent_per_drive_unit,
      distance_driven_in_meters: module.distance_driven_in_meters + @meters_per_drive_unit
    }
end



# Elixir / Boutique Suggestions {{List Comprehensions}}

# Introduction
# List Comprehensions
# Comprehensions provide a facility for transforming Enumerables easily and declaratively.

# To declare a very simple comprehension, we can use the for keyword followed by a generator and a do-block which creates the new values from the enumerated values.

# for n <- [0, 1, 2, 3], do: n + 1
# # => [1, 2, 3, 4]
# Comprehensions can also have filters. Values that do not pass the filter are removed from the final list:

# for n <- [0, 1, 2, 3], n > 1, do: n + 1
# # => [3, 4]
# We can declare more complicated comprehensions over several lines:

# for {atom, number} <- [a: 1, b: 2, c: 3, d: 4],
#     rem(number, 2) == 0 do
#   atom
# end
# # => [:b, :d]
# A cartesian product can be created using multiple generators. That means that each value generated by the first generator will be paired once with each value generated by the second generator:

# for x <- [0, 1],
#     y <- [0, 1] do
#   {x, y}
# end
# # => [{0, 0}, {0, 1}, {1, 0}, {1, 1}]
# Instructions
# Your work at the online fashion boutique store continues. You come up with the idea for a website feature where an outfit is suggested to the user. While you want to give lots of suggestions, you don't want to give bad suggestions, so you decide to use a list comprehension since you can easily generate outfit combinations, then filter them by some criteria.

# Clothing items are stored as a map:

# %{
#   item_name: "Descriptive Name",
#   price: 99.00,
#   base_color: "red"
# }
# Task 1
# Suggest a combination

# Implement get_combinations/3 to take a list of tops, a list of bottoms, and keyword list of options. For now, set options to default to an empty keyword list. The function should return the cartesian product of the lists.

# tops = [
#   %{item_name: "Dress shirt"},
#   %{item_name: "Casual shirt"}
# ]
# bottoms = [
#   %{item_name: "Jeans"},
#   %{item_name: "Dress trousers"}
# ]
# BoutiqueSuggestions.get_combinations(tops, bottoms)
# # => [
# #      {%{item_name: "Dress shirt"}, %{item_name: "Jeans"}},
# #      {%{item_name: "Dress shirt"}, %{item_name: "Dress trousers"}},
# #      {%{item_name: "Casual shirt"}, %{item_name: "Jeans"}},
# #      {%{item_name: "Casual shirt"}, %{item_name: "Dress trousers"}}
# #    ]

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Filter out clashing outfits

# Each piece of clothing has a :base_color field, use this field to filter out all combinations where the top and the bottom have the same base color.

# tops = [
#   %{item_name: "Dress shirt", base_color: "blue"},
#   %{item_name: "Casual shirt", base_color: "black"}
# ]
# bottoms = [
#   %{item_name: "Jeans", base_color: "blue"},
#   %{item_name: "Dress trousers", base_color: "black"}
# ]
# BoutiqueSuggestions.get_combinations(tops, bottoms)
# # => [
# #      {%{item_name: "Dress shirt", base_color: "blue"},
# #       %{item_name: "Dress trousers", base_color: "black"}},
# #      {%{item_name: "Casual shirt", base_color: "black"},
# #       %{item_name: "Jeans", base_color: "blue"}}
# #    ]

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Filter by combination price

# Each piece of clothing has a :price field associated with it. While you want to give lots of suggestions, you want to be able to provide users an opportunity to select a price within their budget. From the keyword list of options, use :maximum_price to filter out combinations where the price of the top and bottom exceed the maximum price.

# If no maximum_price is specified, the default should be 100.00

# tops = [
#   %{item_name: "Dress shirt", base_color: "blue", price: 35},
#   %{item_name: "Casual shirt", base_color: "black", price: 20}
# ]
# bottoms = [
#   %{item_name: "Jeans", base_color: "blue", price: 30},
#   %{item_name: "Dress trousers", base_color: "black", price: 75}
# ]
# BoutiqueSuggestions.get_combinations(tops, bottoms, maximum_price: 50)
# # => [
# #      {%{item_name: "Casual shirt", base_color: "black", price: 20},
# #       %{item_name: "Jeans", base_color: "blue", price: 30}}
# #    ]


defmodule BoutiqueSuggestions do
  def get_combinations(tops, bottoms, options \\ [maximum_price: 100]) do
    for top <- tops,
        bottom <- bottoms,
        top.base_color != bottom.base_color
        && options[:maximum_price]
        && top.price + bottom.price <= options[:maximum_price] do
      {top, bottom}
    end
  end
end



# Elixir / Community Garden {{Agent}}

# Introduction
# Agent
# The Agent module facilitates an abstraction for spawning processes and the receive-send loop. From here, we will call processes started using the Agent module 'agent processes'. An agent process might be chosen to represent a central shared state.

# To start an agent process, Agent provides the start/2 function. The supplied function when start-ing the agent process returns the initial state of the process:

# # Start an agent process with an initial value of an empty list
# {:ok, agent_pid} = Agent.start(fn -> [] end)
# Just like Map or List, Agent provides many functions for working with agent processes.

# It is customary to organize and encapsulate all Agent-related functionality into a module for the domain being modeled.

# Instructions
# Your community association has asked you to implement a simple registry application to manage the community garden registrations. The Plot struct has already been provided for you.

# Task 1
# Open the garden

# Implement the CommunityGarden.start/1 function, it should receive a optional keyword list of options to pass forward to the agent process. The garden's initial state should be initialized to represent an empty collection of plots. It should return an :ok tuple with the garden's pid.

# {:ok, pid} = CommunityGarden.start()
# # => {:ok, #PID<0.112.0>}

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# List the registrations

# Implement the CommunityGarden.list_registrations/1 function. It should receive the pid for the community garden. It should return a list of the stored plots that are registered.

# CommunityGarden.list_registrations(pid)
# # => []
# At this point, we haven't added the ability to register a plot, so this list should be empty


# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Register plots to a person

# Implement the CommunityGarden.register/2 function. It should receive the pid for the community garden and a name to register the plot. It should return the Plot struct with the plot's id and person registered to when it is successful.

# The ids should be incremental and unique. You can keep an id counter in the agent's state.

# CommunityGarden.register(pid, "Emma Balan")
# # => %Plot{plot_id: 1, registered_to: "Emma Balan"}
# CommunityGarden.list_registrations(pid)
# # => [%Plot{plot_id: 1, registered_to: "Emma Balan"}]

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Release plots

# Implement the CommunityGarden.release/2 function. It should receive the pid and id of the plot to be released. It should return :ok on success. When a plot is released, the id is not re-used, it is used as a unique identifier only.

# CommunityGarden.release(pid, 1)
# # => :ok
# CommunityGarden.list_registrations(pid)
# # => []

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Get a registered plot

# Implement the CommunityGarden.get_registration/2 function. It should receive the pid and id of the plot to be checked. It should return the plot if it is registered, and :not_found if it is unregistered.

# CommunityGarden.get_registration(pid, 1)
# # => %Plot{plot_id: 1, registered_to: "Emma Balan"}
# CommunityGarden.get_registration(pid, 7)
# # => {:not_found, "plot is unregistered"}


defmodule Plot do
  @enforce_keys [:plot_id, :registered_to]
  defstruct [:plot_id, :registered_to]
end

defmodule CommunityGarden do
  defstruct [plots: %{}, next_id: 1]

  def start(opts \\ []), do: Agent.start(fn -> %__MODULE__{} end)
  def list_registrations(pid), do: pid |> Agent.get(& &1.plots |> Map.values())

  def register(pid, register_to) do
    pid
    |> Agent.get_and_update(fn %__MODULE__{plots: plots, next_id: next_id} ->
      plot = %Plot{plot_id: next_id, registered_to: register_to}
      {plot, %__MODULE__{plots: plots |> Map.put(next_id, plot), next_id: next_id + 1}}
    end)
  end

  def release(pid, id), do:
    pid |> Agent.update(& %{&1 | plots: &1.plots |> Map.delete(id)})

  def get_registration(pid, id), do:
    pid
    |> Agent.get(& &1.plots)
    |> Map.get(id, {:not_found, "plot is unregistered"})
end



# Elixir / Bread And Potions {{Protocols}}

# Introduction
# Protocols
# Protocols are a mechanism to achieve polymorphism in Elixir when you want behavior to vary depending on the data type.

# Protocols are defined using defprotocol and contain one or more function headers.

# defprotocol Reversible do
#   def reverse(term)
# end
# Protocols can be implemented using defimpl.

# defimpl Reversible, for: List do
#   def reverse(term) do
#     Enum.reverse(term)
#   end
# end
# A protocol can be implemented for any existing Elixir data type or for a struct.

# When a protocol function is invoked, the appropriate implementation gets automatically chosen based on the type of the first argument.

# Instructions
# You're developing your own role-playing video game. In your game, there are characters and items. One of the many actions that you can do with an item is to make a character eat it.

# Not all items are edible, and not all edible items have the same effects on the character. Some items, when eaten, turn into a different item (e.g. if you eat an apple, you are left with an apple core).

# To allow for all that flexibility, you decided to create an Edible protocol that some of the items can implement.

# Task 1
# Define edibility

# Create the RPG.Edible protocol. The protocol has one function - eat. The eat function accepts an item and a character and returns a by-product and a character.


# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Make loaves of bread edible

# Implement the RPG.Edible protocol for the RPG.LoafOfBread item. When eaten, a loaf of bread gives the character 5 health points and has no by-product.

# RPG.Edible.eat(%RPG.LoafOfBread{}, %RPG.Character{health: 31})
# # => {nil, %RPG.Character{health: 36, mana: 0}}

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Make mana potions edible

# Implement the RPG.Edible protocol for the RPG.ManaPotion item. When eaten, a mana potion gives the character as many mana points as the potion's strength, and produces an empty bottle.

# RPG.Edible.eat(%RPG.ManaPotion{strength: 13}, %RPG.Character{mana: 50})
# # => {%RPG.EmptyBottle{}, %RPG.Character{health: 100, mana: 63}}

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Make poisons edible

# Implement the RPG.Edible protocol for the RPG.Poison item. When eaten, a poison takes away all the health points from the character, and produces an empty bottle.

# RPG.Edible.eat(%RPG.Poison{}, %RPG.Character{health: 3000})
# # => {%RPG.EmptyBottle{}, %RPG.Character{health: 0, mana: 0}}


defmodule RPG do
  defmodule Character, do: defstruct health: 100, mana: 0
  defmodule LoafOfBread, do: defstruct []
  defmodule ManaPotion, do: defstruct strength: 10
  defmodule Poison, do: defstruct []
  defmodule EmptyBottle, do: defstruct []
  defprotocol Edible, do: def eat(item, character)
  defimpl Edible, for: LoafOfBread do
    def eat(_item, %Character{} = character), do: {nil, %{character | health: character.health + 5}}
  end
  defimpl Edible, for: ManaPotion do
    def eat(item, %Character{} = character), do: {%EmptyBottle{}, %{character | mana: character.mana + item.strength}}
  end
  defimpl Edible, for: Poison do
    def eat(_item, %Character{} = character), do: {%EmptyBottle{}, %{character | health: 0}}
  end
end



# Elixir / Captain's Log {{Erlang Libraries}} {{Randomness}}

# Introduction
# Randomness
# In Elixir, to choose a random element from an enumerable data structure (e.g. list, range), we use Enum.random. This function will pick a single element, with every element having equal probability of being picked.

# Elixir does not have its own functions for picking a random float. To do that, we have to use Erlang directly.

# Erlang Libraries
# Elixir code runs in the BEAM virtual machine. BEAM is part of the Erlang Run-Time System. Being inspired by Erlang, and sharing its run environment, Elixir provides great interoperability with Erlang libraries. This means that Elixir developers can use Erlang libraries from within their Elixir code. In fact, writing Elixir libraries for functionality already provided by Erlang libraries is discouraged in the Elixir community.

# As a result, certain functionality, like mathematical operations or timer functions, is only available in Elixir via Erlang.

# Erlang's standard library is available for use in our Elixir code without any extra steps necessary.

# Erlang functions can be called in the same way we call Elixir functions, with one small difference. Erlang module names are snake_case atoms. For example, to call the Erlang pi/0 function from the math module, one would write:

# :math.pi()
# # => 3.141592653589793
# The most commonly used Erlang functions that do not have an Elixir equivalent are:

# :timer.sleep/1 which suspends a process for the given amount of milliseconds.
# :rand.uniform/0 which generates a random float x, where 0.0 <= x < 1.0.
# :io_lib.format/2 which provides C-style string formatting (using control sequences). Using this function, we could for example print an integer in any base between 2 and 36 or format a float with desired precision. Note that this function, like many Erlang functions, returns a charlist.
# The math module that provides mathematical functions such as sin/1, cos/1, log2/1, log10/1, pow/2, and more.
# To discover Erlang's standard library, explore the STDLIB Reference Manual.

# Instructions
# Mary is a big fan of the TV series Star Trek: The Next Generation. She often plays pen-and-paper role playing games, where she and her friends pretend to be the crew of the Starship Enterprise. Mary's character is Captain Picard, which means she has to keep the captain's log. She loves the creative part of the game, but doesn't like to generate random data on the spot.

# Help Mary by creating random generators for data commonly appearing in the captain's log.

# Task 1
# Generate a random planet

# The Starship Enterprise encounters many planets in its travels. Planets in the Star Trek universe are split into categories based on their properties. For example, Earth is a class M planet. All possible planetary classes are: D, H, J, K, L, M, N, R, T, and Y.

# Implement the random_planet_class/0 function. It should return one of the planetary classes at random.

# CaptainsLog.random_planet_class()
# # => "K"

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Generate a random starship registry number

# Enterprise (registry number NCC-1701) is not the only starship flying around! When it rendezvous with another starship, Mary needs to log the registry number of that starship.

# Registry numbers start with the prefix "NCC-" and then use a number from 1000 to 9999 (inclusive).

# Implement the random_ship_registry_number/0 function that returns a random starship registry number.

# CaptainsLog.random_ship_registry_number()
# # => "NCC-1947"

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Generate a random stardate

# What's the use of a log if it doesn't include dates?

# A stardate is a floating point number. The adventures of the Starship Enterprise from the first season of The Next Generation take place between the stardates 41000.0 and 42000.0. The "4" stands for the 24th century, the "1" for the first season.

# Implement the function random_stardate/0 that returns a floating point number between 41000.0 (inclusive) and 42000.0 (exclusive).

# CaptainsLog.random_stardate()
# # => 41458.15721310934

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Format the stardate

# In the captain's log, stardates are usually rounded to a single decimal place.

# Implement the format_stardate/1 function that will take a floating point number and return a string with the number rounded to a single decimal place.

# CaptainsLog.format_stardate(41458.15721310934)
# # => "41458.2"


defmodule CaptainsLog do
  @planetary_classes ["D", "H", "J", "K", "L", "M", "N", "R", "T", "Y"]
  @ship_range 1_000..9_999
  @start 41_000.0
  @finish 42_000.0
  def random_planet_class(), do: @planetary_classes |> Enum.random()
  def random_ship_registry_number(), do: "NCC-#{@ship_range |> Enum.random()}"
  def random_stardate(), do: :random.uniform() * (@finish - @start) + @start
  def format_stardate(stardate), do: :io_lib.format("~.1f", [stardate]) |> to_string()
end



# Elixir / Need For Speed {{Alias}} {{Import}}

# Introduction
# Alias
# To share code between different Elixir modules within the same project, you need to reference the outside module by its full name. But what if that name is too long or confusing?

# The special form alias allows you to shorten or change the name by which you reference an outside module. When used without any arguments, it trims down the module name to its last segment, e.g. MyApp.Logger.Settings becomes Settings. A custom name can be specified with the :as option.

# Usually aliases are added at the beginning of the module definition.

# defmodule Square do
#   alias Integer, as: I

#   def area(a), do: I.pow(a, 2)
# end
# Import
# The special form import allows you to use functions from an outside module without using the module's name.

# Importing a whole outside module might create conflicts with existing local functions. To avoid this, two options are available: :except and :only. Both expect a keyword list, where the key is the function name, and the value is the function's arity.

# Usually imports are added at the beginning of the module definition.

# defmodule Square do
#   import Integer, only: [pow: 2]

#   def area(a), do: pow(a, 2)
# end
# Instructions
# That remote controlled car that you bought recently has turned into a whole new hobby. You have been organizing remote control car races.

# You were almost finished writing a program that would allow to run race simulations when your cat jumped at your keyboard and deleted a few lines of code. Now your program doesn't compile anymore...

# Task 1
# Fix compilation error Race.__struct__/0 is undefined

# Add an alias so that the module NeedForSpeed.Race can be referenced by the shorter name Race.


# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Fix compilation error Car.__struct__/0 is undefined

# Add an alias so that the module NeedForSpeed.RemoteControlCar can be referenced by the shorter name Car.


# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Fix compilation error undefined function puts/1

# The function puts/1 comes from the module IO. Import it to be able to use it without referencing the module.


# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Fix compilation error undefined function default_color/0

# The functions default_color/0, red/0, cyan/0, and green/0 all come from the module IO.ANSI. You're planning to add support for other car colors, so you want to import the whole module. Unfortunately, the function color/1 from the module IO.ANSI conflicts with one of your local functions. Import the whole IO.ANSI module except that one function.


defmodule NeedForSpeed.RemoteControlCar do
  defstruct [
    :nickname,
    :color,
    battery_percentage: 100,
    distance_driven_in_meters: 0
  ]

  def new(color, nickname) when color in [:red, :blue, :green], do: %__MODULE__{nickname: nickname, color: color}
  def display_distance(%__MODULE__{distance_driven_in_meters: d}), do: "#{d} meters"
  def display_battery(%__MODULE__{battery_percentage: 0}), do: "Battery empty"
  def display_battery(%__MODULE__{battery_percentage: b}), do: "Battery at #{b}%"
end

defmodule NeedForSpeed.Race do
  defstruct [
    :title,
    :total_distance_in_meters,
    cars: []
  ]

  def display_status(%__MODULE__{} = race) do
    cond do
      race.cars |> Enum.any?(& &1.distance_driven_in_meters >= race.total_distance_in_meters) -> "Finished"
      race.cars |> Enum.any?(& &1.distance_driven_in_meters > 0) -> "In Progress"
      true -> "Not Started"
    end
  end

  def display_distance(%__MODULE__{total_distance_in_meters: d}), do: "#{d} meters"
end

defmodule NeedForSpeed do
  alias NeedForSpeed.Race
  alias NeedForSpeed.RemoteControlCar, as: Car
  import IO
  import IO.ANSI, except: [color: 1]

  def print_race(%Race{} = race) do
    puts("""
    🏁 #{race.title} 🏁
    Status: #{Race.display_status(race)}
    Distance: #{Race.display_distance(race)}

    Contestants:
    """)

    race.cars
    |> Enum.sort_by(& -1 * &1.distance_driven_in_meters)
    |> Enum.with_index()
    |> Enum.each(fn {car, index} -> print_car(car, index + 1) end)
  end

  defp print_car(%Car{} = car, index) do
    puts("""
      #{index}. #{color(car)}#{car.nickname}#{default_color()}
      Distance: #{Car.display_distance(car)}
      Battery: #{Car.display_battery(car)}
    """)
  end

  defp color(%Car{} = car) do
    case car.color do
      :red -> red()
      :blue -> cyan()
      :green -> green()
    end
  end
end



# Elixir / RPN Calculator {{Errors}} {{Try/Rescue}}

# Introduction
# Errors
# Errors happen. In Elixir, while people often say to "let it crash", there are times when we need to rescue the function call to a known good state to fulfill a software contract. In some languages, errors are used as a method of control flow, but in Elixir, this pattern is discouraged. We can often recognize functions that may raise an error just by their name: functions that raise errors are to have ! at the end of their name. This is in comparison with functions that return {:ok, value} or :error. Look at these library examples:

# Map.fetch(%{a: 1}, :b)
# # => :error
# Map.fetch!(%{a: 1}, :b)
# # => raises KeyError
# Try/Rescue
# Elixir provides a construct for rescuing from errors using try .. rescue

# try do                             #1
#   raise RuntimeError, "error"      #2
# rescue
#   e in RuntimeError -> :error      #3
# end
# Let's examine this construct:

# Line 1, the block is declared with try.
# Line 2, the function call which may error is placed here, in this case we are calling raise/2.
# Line 3, in the rescue section, we pattern match on the Module name of the error raised
# on the left side of ->:
# e is matched to the error struct.
# in is a keyword.
# RuntimeError is the error that we want to rescue.
# If we wanted to rescue from all errors, we could use _ instead of the module name or omit the in keyword entirely.
# on the right side:
# the instructions to be executed if the error matches.
# Error structs
# Errors (sometimes also called "exceptions") that you rescue this way are structs. Rescuing errors in Elixir is done very rarely. Usually the rescued error is logged or sent to an external monitoring service, and then reraised. This means we usually don't care about the internal structure of the specific error struct.

# In the Exceptions concept you will learn more about error structs, including how to define your own custom error.

# Instructions
# While working at Instruments of Texas, you are tasked to work on an experimental Reverse Polish Notation [RPN] calculator written in Elixir. Your team is having a problem with some operations raising errors and crashing the process. You have been tasked to write a function which wraps the operation function so that the errors can be handled more elegantly with idiomatic Elixir code.

# Task 1
# Warn the team

# Implement the function calculate!/2 to call the operation function with the stack as the only argument. The operation function is defined elsewhere, but you know that it can either complete successfully or raise an error.

# stack = []
# operation = fn _ -> :ok end
# RPNCalculator.calculate!(stack, operation)
# # => :ok

# stack = []
# operation = fn _ -> raise ArgumentError, "An error occurred" end
# RPNCalculator.calculate!(stack, operation)
# # => ** (ArgumentError) An error occurred
# Function names that end in ! are a warning to programmers that this function may raise an error


# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Wrap the error

# When doing more research you notice that many functions use atoms and tuples to indicate their success/failure. Implement calculate/2 using this strategy.

# stack = []
# operation = fn _ -> "operation completed" end
# RPNCalculator.calculate(stack, operation)
# # => {:ok, "operation completed"}

# stack = []
# operation = fn _ -> raise ArgumentError, "An error occurred" end
# RPNCalculator.calculate(stack, operation)
# # => :error

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Pass on the message

# Some of the errors contain important information that your coworkers need to have to ensure the correct operation of the system. Implement calculate_verbose/2 to pass on the error message. The error is a struct that has a :message field.

# stack = []
# operation = fn _ -> "operation completed" end
# RPNCalculator.calculate_verbose(stack, operation)
# # => {:ok, "operation completed"}

# stack = []
# operation = fn _ -> raise ArgumentError, "An error occurred" end
# RPNCalculator.calculate_verbose(stack, operation)
# # => {:error, "An error occurred"}

defmodule RPNCalculator do
  def calculate!(stack, operation), do: operation.(stack)
  def calculate(stack, operation) do
    {:ok, calculate!(stack, operation)}
  rescue
    _ -> :error
  end
  def calculate_verbose(stack, operation) do
    {:ok, calculate!(stack, operation)}
  rescue
    e -> {:error, e.message}
  end
end



# Elixir / Stack Underflow {{Exceptions}}

# Introduction
# Exceptions
# All errors in Elixir implement the Exception Behaviour. Just like the Access Behaviour, the Exception Behaviour defines callback functions that a module must implement to fulfill the software contract of the behaviour. Once an error is defined, it has the following properties:

# The module's name defines the error's name.
# The module defines an error-struct.
# The struct will have a :message field.
# The module can be be used with raise/1 and raise/2 to raise the intended error
# The Exception Behaviour also specifies two callbacks: message/1 and exception/1. If unimplemented, default implementations will be used. message/1 transforms the error-struct to a readable message when called with raise. exception/1 allows additional context to be added to the message when it is called with raise/2

# Defining an exception
# To define an exception from an error module, we use the defexception macro:

# # Defines a minimal error, with the name `MyError`
# defmodule MyError do
#   defexception message: "error"
# end

# # Defines an error with a customized exception/1 function
# defmodule MyCustomizedError do
#   defexception message: "custom error"

#   @impl true
#   def exception(value) do
#     case value do
#       [] ->
#         %MyCustomizedError{}
#       _ ->
#         %MyCustomizedError{message: "Alert: " <> value}
#     end
#   end
# end
# Using exceptions
# Defined errors may be used like a built in error using either raise/1 or raise/2.

# raise/1 raises a specific error by its module name, or, if the argument is a string, it will raise a RuntimeError with the string as the message.
# raise/2 raises a specific error by its module name, and accepts an attributes argument which is used to obtain the error with the appropriate message.
# Instructions
# While continuing your work at Instruments of Texas, there is progress being made on the Elixir implementation of the RPN calculator. Your team would like to be able to raise errors that are more specific than the generic errors provided by the standard library. You are doing some research, but you have decided to implement two new errors which implement the Exception Behaviour.

# Task 1
# Error for Division by Zero

# Dividing a number by zero produces an undefined result, which the team decides is best represented by an error.

# Implement the DivisionByZeroError module to have the error message: "division by zero occurred"

# raise DivisionByZeroError
# # => ** (DivisionByZeroError) division by zero occurred

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Error when encountering stack underflow

# RPN calculators use a stack to keep track of numbers before they are added. The team represents this stack with a list of numbers (integer and floating-point), e.g.: [3, 4.0]. Each operation needs a specific number of numbers on the stack in order to perform its calculation. When there are not enough numbers on the stack, this is called a stack underflow error. Implement the StackUnderflowError exception which provides a default message, and optional extra context

# raise StackUnderflowError
# # => ** (StackUnderflowError) stack underflow occurred

# raise StackUnderflowError, "when dividing"
# # => ** (StackUnderflowError) stack underflow occurred, context: when dividing

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Write a dividing function

# Implement the divide/1 function which takes a stack (list of numbers) and:

# raises stack underflow when the stack does not contain enough numbers
# raises division by zero when the divisor is 0 (note the stack of numbers is stored in the reverse order)
# performs the division when no errors are raised
# RPNCalculator.Exception.divide([])
# # => ** (StackUnderflowError) stack underflow occurred, context: when dividing

# RPNCalculator.Exception.divide([0, 100])
# # => ** (DivisionByZeroError) division by zero occurred

# RPNCalculator.Exception.divide([4, 16])
# # => 4
# Note the order of the list is reversed!


defmodule RPNCalculator.Exception do
  defmodule DivisionByZeroError, do: defexception message: "division by zero occurred"

  defmodule StackUnderflowError do
    @message "stack underflow occurred"
    defexception message: @message

    def exception([]), do: %__MODULE__{}
    def exception(term), do: %__MODULE__{message: "#{@message}, context: #{term}"}
  end

  def divide([]), do: raise StackUnderflowError, "when dividing"
  def divide([_]), do: raise StackUnderflowError, "when dividing"
  def divide([0 | _]), do: raise DivisionByZeroError
  def divide([divisor, dividend]), do: dividend / divisor
end



# Elixir / RPN Calculator Inspection {{Links}} {{Tasks}}

# Introduction
# Links
# Elixir processes are isolated and don't share anything by default. When an unlinked child process crashes, its parent process is not affected.

# This behavior can be changed by linking processes to one another. If two processes are linked, a failure in one process will be propagated to the other process. Links are bidirectional.

# Processes can be spawned already linked to the calling process using spawn_link/1 which is an atomic operation, or they can be linked later with Process.link/1.

# Linking processes can be useful when doing parallelized work when each chunk of work shouldn't be continued in case another chunk fails to finish.

# Trapping exits
# Linking can also be used for supervising processes. If a process traps exits, it will not crash when a process to which it's linked crashes. It will instead receive a message about the crash. This allows it to deal with the crash gracefully, for example by restarting the crashed process.

# A process can be configured to trap exits by calling Process.flag(:trap_exit, true). Note that Process.flag/2 returns the old value of the flag, not the new one.

# The message that will be sent to the process in case a linked process crashes will match the pattern {:EXIT, from, reason}, where from is a PID. If reason is anything other than the atom :normal, that means that the process crashed or was forcefully killed.

# Tasks
# Tasks are processes meant to execute one specific operation. They usually don't communicate with other processes, but they can return a result to the process that started the task.

# Tasks are commonly used to parallelize work.

# async/await
# To start a task, use Task.async/1. It takes an anonymous function as an argument and executes it in a new process that is linked to the caller process. It returns a %Task{} struct.

# To get the result of the execution, pass the %Task{} struct to Task.await/2. It will wait for the task to finish and return its result. The second argument is a timeout in milliseconds, defaulting to 5000.

# Note that between starting the task and awaiting the task, the process that started the task is not blocked and might do other operations.

# Any task started with Task.async/1 should be awaited because it will send a message to the calling process. Task.await/2 can be called for each task only once.

# start/start_link
# If you want to start a task for side-effects only, use Task.start/1 or Task.start_link/1. Task.start/1 will start a task that is not linked to the calling process, and Task.start_link/1 will start a task that is linked to the calling process. Both functions return a {:ok, pid} tuple.

# Instructions
# Your work at Instruments of Texas on an experimental RPN calculator continues. Your team has built a few prototypes that need to undergo a thorough inspection, to choose the best one that can be mass-produced.

# You want to conduct two types of checks.

# Firstly, a reliability check that will detect inputs for which the calculator under inspection either crashes or doesn't respond fast enough. To isolate failures, the calculations for each input need to be run in a separate process. Linking and trapping exits in the caller process can be used to detect if the calculation finished or crashed.

# Secondly, a correctness check that will check if for a given input, the result returned by the calculator is as expected. Only calculators that already passed the reliability check will undergo a correctness check, so crashes are not a concern. However, the operations should be run concurrently to speed up the process, which makes it the perfect use case for asynchronous tasks.

# Task 1
# Start a reliability check for a single input

# Implement the RPNCalculatorInspection.start_reliability_check/2 function. It should take 2 arguments, a function (the calculator), and an input for the calculator. It should return a map that contains the input and the PID of the spawned process.

# The spawned process should call the given calculator function with the given input. The process should be linked to the caller process.

# RPNCalculatorInspection.start_reliability_check(fn _ -> 0 end, "2 3 +")
# # => %{input: "2 3 +", pid: #PID<0.169.0>}

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Interpret the results of a reliability check

# Implement the RPNCalculatorInspection.await_reliability_check_result/2 function. It should take two arguments. The first argument is a map with the input of the reliability check and the PID of the process running the reliability check for this input, as returned by RPNCalculatorInspection.start_reliability_check/2. The second argument is a map that serves as an accumulator for the results of reliability checks with different inputs.

# The function should wait for an exit message.

# If it receives an exit message ({:EXIT, from, reason}) with the reason :normal from the same process that runs the reliability check, it should return the results map with the value :ok added under the key input.

# If it receives an exit message with a different reason from the same process that runs the reliability check, it should return the results map with the value :error added under the key input.

# If it doesn't receive any messages matching those criteria in 100ms, it should return the results map with the value :timeout added under the key input.

# # when an exit message is waiting for the process in its inbox
# send(self(), {:EXIT, pid, :normal})

# RPNCalculatorInspection.await_reliability_check_result(
#   %{input: "5 7 -", pid: pid},
#   %{}
# )
# # => %{"5 7 -" => :ok}

# # when there are no messages in the process inbox
# RPNCalculatorInspection.await_reliability_check_result(
#   %{input: "3 2 *", pid: pid},
#   %{"5 7 -" => :ok}
# )
# # => %{"5 7 -" => :ok, "3 2 *" => :timeout}

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Run a concurrent reliability check for many inputs

# Implement the RPNCalculatorInspection.reliability_check/2 function. It should take 2 arguments, a function (the calculator), and a list of inputs for the calculator.

# For every input on the list, it should start the reliability check in a new linked process by using start_reliability_check/2. Then, for every process started this way, it should await its results by using await_reliability_check_result/2.

# Before starting any processes, the function needs to flag the current process to trap exits, to be able to receive exit messages. Afterwards, it should reset this flag to its original value.

# The function should return a map with the results of reliability checks of all the inputs.

# fake_broken_calculator = fn input ->
#   if String.ends_with?(input, "*"), do: raise "oops"
# end

# inputs = ["2 3 +", "10 3 *", "20 2 /"]

# RPNCalculatorInspection.reliability_check(fake_broken_calculator, inputs)
# # => %{
# #       "2 3 +" => :ok,
# #       "10 3 *" => :error,
# #       "20 2 /" => :ok
# #     }

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Run a concurrent correctness check for many inputs

# Implement the RPNCalculatorInspection.correctness_check/2 function. It should take 2 arguments, a function (the calculator), and a list of inputs for the calculator.

# For every input on the list, it should start an asynchronous task that will call the calculator with the given input. Then, for every task started this way, it should await its results for 100ms.

# fast_cheating_calculator = fn input -> 14 end
# inputs = ["13 1 +", "50 2 *", "1000 2 /"]
# RPNCalculatorInspection.correctness_check(fast_cheating_calculator, inputs)
# # => [14, 14, 14]


defmodule RPNCalculatorInspection do
  def start_reliability_check(calculator, input), do:
    %{input: input, pid: fn -> input |> calculator.() end |> spawn_link()}

  def await_reliability_check_result(%{pid: pid, input: input}, results) do
    receive do
      {:EXIT, ^pid, :normal} -> results |> Map.put(input, :ok)
      {:EXIT, ^pid, reason} -> results |> Map.put(input, :error)
    after
      100 -> results |> Map.put(input, :timeout)
    end
  end

  def reliability_check(calculator, inputs) do
    trap_exit = :trap_exit |> Process.flag(true)
    results =
      inputs
      |> Enum.map(& start_reliability_check(calculator, &1))
      |> Enum.reduce(%{}, &await_reliability_check_result(&1, &2))
    :trap_exit |> Process.flag(trap_exit)
    results
  end

  def correctness_check(calculator, inputs) do
    inputs
    |> Enum.map(& fn -> &1 |> calculator.() end |> Task.async())
    |> Enum.map(& &1 |> Task.await(100))
  end
end



# Elixir / Lucas Numbers {{Streams}}

# Introduction
# Streams
# All functions in the Enum module are eager. When performing multiple operations on enumerables with the Enum module, each operation is going to generate an intermediate result.

# The Stream module is a lazy alternative to the eager Enum module. It offers many of the same functions as Enum, but instead of generating intermediate results, it builds a series of computations that are only executed once the stream is passed to a function from the Enum module.

# Streams implement the Enumerable protocol and are composable -- you can chain them together to create more complex functionality.

# Instructions
# You are a huge fan of the Numberphile Youtube channel and you just saw a cool video about the Lucas Number Sequence. You want to create this sequence using Elixir.

# While designing your function, you want to make use of lazy evaluation, so that you can generate as many numbers as you want, but only if you need to -- So you decide to use a stream:

# Task 1
# Generate the base cases

# You know that the sequence has two starting numbers which don't follow the same rule. Write two base case clauses to return these numbers:

# LucasNumbers.generate(1)
# # => [2]

# LucasNumbers.generate(2)
# # => [2, 1]

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Create the generalized case

# For any sequence longer than 2, you know that you need to add the previous two numbers to get the next number and so on. Write the generalized case.

# LucasNumbers.generate(3)
# # => [2, 1, 3]

# LucasNumbers.generate(4)
# # => [2, 1, 3, 4]

# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Catch bad arguments

# Later, you find someone is using your function and having problems because they are using incorrect arguments. Add a guard clause to raise an error if a non-integer or an integer less than 1 is used to generate the sequence:

# LucasNumbers.generate("Hello World")
# # => ** (ArgumentError) count must be specified as an integer >= 1


defmodule LucasNumbers do
  @moduledoc """
  Lucas numbers are an infinite sequence of numbers which build progressively
  which hold a strong correlation to the golden ratio (φ or ϕ)

  E.g.: 2, 1, 3, 4, 7, 11, 18, 29, ...
  """
  def generate(count) when is_integer(count) and count >= 1, do: lucas() |> Enum.take(count)
  def generate(_), do: raise ArgumentError, "count must be specified as an integer >= 1"
  defp lucas(), do: {2, 1} |> Stream.unfold(fn {x, y} -> {x, {y, x + y}} end)
end



# Elixir / New Passport {{With}}

# Introduction
# With
# The special form with provides a way to focus on the "happy path" of a series of potentially failing steps and deal with the failures later.

# with {:ok, id} <- get_id(username),
#      {:ok, avatar} <- fetch_avatar(id),
#      {:ok, image_type} <- check_valid_image_type(avatar) do
#   {:ok, image_type, avatar}
# else
#   :not_found ->
#     {:error, "invalid username"}

#   {:error, "not an image"} ->
#     {:error, "avatar associated to #{username} is not an image"}

#   err ->
#     err
# end
# At each step, if a clause matches, the chain will continue until the do block is executed. If one match fails, the chain stops and the non-matching clause is returned. You have the option of using an else block to catch failed matches and modify the return value.

# Instructions
# Your passport is about to expire, so you need to drop by the city office to renew it. You know from previous experience that your city office is not necessarily the easiest to deal with, so you decide to do your best to always "focus on the happy path".

# You print out the form you need to get your new passport, fill it out, jump into your car, drive around the block, park and head to the office.

# All the following tasks will require implementing and extending get_new_passport/3.

# Task 1
# Get into the building

# It turns out that the building is only open in the afternoon, and not at the same time everyday.

# Call the function enter_building/1 with the current time (given to you as first argument of get_new_passport/3). If the building is open, the function will return a tuple with :ok and a timestamp that you will need later, otherwise a tuple with :error and a message. For now, the happy path can return the :ok tuple.

# If you get an :error tuple, use the else block to return it.


# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Go to the information desk and find which counter you should go to

# The information desk is notorious for taking long coffee breaks. If you are lucky enough to find someone there, they will give you an instruction manual which will explain which counter you need to go to depending on your birth date.

# Call the function find_counter_information/1 with the current time. You will get either a tuple with :ok and a manual, represented by an anonymous function, or a tuple with :coffee_break and more instructions. In your happy path where you receive the manual, apply it to your birthday (second argument of get_new_passport/3). It will return the number of the counter where you need to go. Return an :ok tuple with that counter number.

# If you get a :coffee_break message, return a tuple with :retry and a NaiveDateTime pointing to 15 minutes after the current time. As before, if you get an :error tuple, return it.


# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Go to the counter and get your form stamped

# For some reason, different counters require forms of different colors. Of course, you printed the first one you found on the website, so you focus on your happy path and hope for the best.

# Call the function stamp_form/3 with the timestamp you received at the entrance, the counter and the form you brought (last argument of get_new_passport/3). You will get either a tuple with :ok and a checksum that will be used to verify your passport number or a tuple with :error and a message. Have your happy path return an :ok tuple with the checksum. If you get an :error tuple, return it.


# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Receive your new passport

# Finally, you have all the documents you need.

# Call get_new_passport_number/3 with the timestamp, the counter and the checksum you received earlier. You will receive a string with your final passport number, all that is left to do is to return that string in a tuple with :ok and go home.


defmodule NewPassport do
  @eighteen_years 18 * 365

  defp enter_building(%NaiveDateTime{} = datetime) do
    day = Date.day_of_week(datetime)
    time = NaiveDateTime.to_time(datetime)

    cond do
      day <= 4 and time_between(time, ~T[13:00:00], ~T[15:30:00]) ->
        {:ok, datetime |> DateTime.from_naive!("Etc/UTC") |> DateTime.to_unix()}

      day == 5 and time_between(time, ~T[13:00:00], ~T[14:30:00]) ->
        {:ok, datetime |> DateTime.from_naive!("Etc/UTC") |> DateTime.to_unix()}

      true ->
        {:error, "city office is closed"}
    end
  end

  defp find_counter_information(%NaiveDateTime{} = datetime) do
    if datetime |> NaiveDateTime.to_time() |> time_between(~T[14:00:00], ~T[14:20:00]) do
      {:coffee_break, "information counter staff on coffee break, come back in 15 minutes"}
    else
      {:ok, fn %Date{} = birthday -> 1 + div(Date.diff(datetime, birthday), @eighteen_years) end}
    end
  end

  defp stamp_form(timestamp, counter, :blue) when rem(counter, 2) == 1, do: {:ok, 3 * (timestamp + counter) + 1}
  defp stamp_form(timestamp, counter, :red) when rem(counter, 2) == 0, do: {:ok, div(timestamp + counter, 2)}
  defp stamp_form(_, _, _), do: {:error, "wrong form color"}
  defp get_new_passport_number(timestamp, counter, checksum), do: "#{timestamp}-#{counter}-#{checksum}"
  defp time_between(time, from, to), do: Time.compare(from, time) != :gt and Time.compare(to, time) == :gt

  def get_new_passport(current_time, birthday, form) do
    with {:ok, timestamp} <- enter_building(current_time),
         {:ok, manual} <- find_counter_information(current_time),
         counter = manual.(birthday),
         {:ok, checksum} <- stamp_form(timestamp, counter, form),
         new_passport_number <- get_new_passport_number(timestamp, counter, checksum) do
      {:ok, new_passport_number}
    else
      {:coffee_break, _message} -> {:retry, current_time |> NaiveDateTime.add(15, :minute)}
      error -> error
    end
  end
end



# Elixir / Top Secret {{AST}}

# Introduction
# AST
# The Abstract Syntax Tree (AST), also called a quoted expression, is a way to represent code as data.

# Each node in the AST is a three-element tuple.

# # AST representation of:
# # 2 + 3
# {:+, [], [2, 3]}
# The first element, an atom, is the operation. The second element, a keyword list, is the metadata. The third element is a list of arguments, which contains other nodes. Literal values such as integers, atoms, and strings are represented in the AST as themselves instead of three-element tuples.

# Turning code into ASTs
# Changing Elixir code to ASTs and ASTs back to code is part of the standard library. You can find functions for working with ASTs in the modules Code (e.g. to change a string with code to an AST) and Macro (e.g. to traverse the AST or change it to a string).

# Note that all of the functions in the standard library use the name "quoted" to mean the AST (short for quoted expression).

# The special form for turning code into an AST is called quote. It accepts a code block and returns its AST.

# quote do
#   2 + 3 - 1
# end

# # => {:-, [], [
# #      {:+, [], [2, 3]},
# #      1
# #    ]}
# Use cases
# The ability to represent code as an AST is at the heart of metaprogramming in Elixir. Macros, which is a way to write Elixir code that produces Elixir code, work by returning ASTs as output.

# Another use case for ASTs is static code analysis, like Exercism's own tool, the Analyzer, which you might already know as the little bot that leaves comments on your solutions.

# Instructions
# You're part of a task force fighting against corporate espionage. You have a secret informer at Shady Company X, which you suspect of stealing secrets from its competitors.

# Your informer, Agent Ex, is an Elixir developer. She is encoding secret messages in her code.

# To decode her secret messages:

# Take all functions (public and private) in the order they're defined in.
# For each function, take the first n characters from its name, where n is the function's arity.
# Task 1
# Turn code into data

# Implement the TopSecret.to_ast/1 function. It should take a string with Elixir code and return its AST.

# TopSecret.to_ast("div(4, 3)")
# # => {:div, [line: 1], [4, 3]}

# Stuck? Reveal Hints
# Opens in a modal
# Task 2
# Parse a single AST node

# Implement the TopSecret.decode_secret_message_part/2 function. It should take an AST node and an accumulator for the secret message (a list). It should return a tuple with the AST node unchanged as the first element, and the accumulator as the second element.

# If the operation of the AST node is defining a function (def or defp), prepend the function name (changed to a string) to the accumulator. If the operation is something else, return the accumulator unchanged.

# ast_node = TopSecret.to_ast("defp cat(a, b, c), do: nil")
# TopSecret.decode_secret_message_part(ast_node, ["day"])
# # => {ast_node, ["cat", "day"]}

# ast_node = TopSecret.to_ast("10 + 3")
# TopSecret.decode_secret_message_part(ast_node, ["day"])
# # => {ast_node, ["day"]}
# This function doesn't need to do any recursive calls to check the whole AST, only the given node. We will traverse the whole AST with built-in tools in the last step.


# Stuck? Reveal Hints
# Opens in a modal
# Task 3
# Decode the secret message part from function definition

# Extend the TopSecret.decode_secret_message_part/2 function. If the operation in the AST node is defining a function, don't return the whole function name. Instead, check the function's arity. Then, return only first n character from the name, where n is the arity.

# ast_node = TopSecret.to_ast("defp cat(a, b), do: nil")
# TopSecret.decode_secret_message_part(ast_node, ["day"])
# # => {ast_node, ["ca", "day"]}

# ast_node = TopSecret.to_ast("defp cat(), do: nil")
# TopSecret.decode_secret_message_part(ast_node, ["day"])
# # => {ast_node, ["", "day"]}

# Stuck? Reveal Hints
# Opens in a modal
# Task 4
# Fix the decoding for functions with guards

# Extend the TopSecret.decode_secret_message_part/2 function. Make sure the function's name and arity is correctly detected for function definitions that use guards.

# ast_node = TopSecret.to_ast("defp cat(a, b) when is_nil(a), do: nil")
# TopSecret.decode_secret_message_part(ast_node, ["day"])
# # => {ast_node, ["ca", "day"]}

# Stuck? Reveal Hints
# Opens in a modal
# Task 5
# Decode the full secret message

# Implement the TopSecret.decode_secret_message/1 function. It should take a string with Elixir code and return the secret message as a string decoded from all function definitions found in the code. Make sure to reuse functions defined in previous steps.

# code = """
# defmodule MyCalendar do
#   def busy?(date, time) do
#     Date.day_of_week(date) != 7 and
#       time.hour in 10..16
#   end

#   def yesterday?(date) do
#     Date.diff(Date.utc_today, date)
#   end
# end
# """

# TopSecret.decode_secret_message(code)
# # => "buy"



# Decode the full secret message

# FAILED
# Test 13
# test decode_secret_message/1 decodes a secret message from a single function definition

# CODE RUN
# code =
#   "defmodule Notebook do\n  def note(notebook, text) do\n    add_to_notebook(notebook, text, append: true)\n  end\nend\n"

# secret_message = "no"
# assert TopSecret.decode_secret_message(code) == secret_message
# TEST FAILURE
#   1) test decode_secret_message/1 decodes a secret message from a single function definition (TopSecretTest)
#      test/top_secret_test.exs:212
#      Assertion with == failed
#      code:  assert TopSecret.decode_secret_message(code) == secret_message
#      left:  ""
#      right: "no"
#      stacktrace:
#        test/top_secret_test.exs:222: (test)
# %{ast: {:defmodule, [line: 1], [{:__aliases__, [line: 1], [:Notebook]}, [do: {:def, [line: 2], [{:note, [line: 2], [{:notebook, [line: 2], nil}, {:text, [line: 2], nil}]}, [do: {:add_to_notebook, [line: 3], [{:notebook, [line: 3], nil}, {:text, [line: 3], nil}, [append: true]]}]]}]]}}

defmodule TopSecret do
  def to_ast(string), do: Code.string_to_quoted!(string)
  def decode_function_name(name, nil), do: ""
  def decode_function_name(name, body), do: "#{name}" |> String.slice(0, length(body))
  def decode_secret_message_part({_, _, [{:when, _, [{name, _, body} | _]} | _]} = ast, acc), do:
    {ast, [decode_function_name(name, body) | acc]}
  def decode_secret_message_part({op, _, [{name, _, body} | _]} = ast, acc) when op in [:def, :defp], do:
    {ast, [decode_function_name(name, body) | acc]}
  def decode_secret_message_part(ast, acc), do: {ast, acc}
  def decode_secret_message(code), do:
    code
    |> to_ast()
    |> Macro.prewalk([], &decode_secret_message_part/2)
    |> elem(1)
    |> Enum.reverse()
    |> Enum.join()
end



# Elixir / Two Fer {{Default Arguments}} {{Guards}}

# Instructions
# Two-fer or 2-fer is short for two for one. One for you and one for me.

# Given a name, return a string with the message:

# One for name, one for me.
# Where "name" is the given name.

# However, if the name is missing, return the string:

# One for you, one for me.
# Here are some examples:

# Name	String to return
# Alice	One for Alice, one for me.
# Bob	One for Bob, one for me.
# One for you, one for me.
# Zaphod	One for Zaphod, one for me.


defmodule TwoFer do
  def two_fer(name \\ "you") when is_binary(name), do: "One for #{name}, one for me."
end



# Elixir / Collatz Conjecture {{Integers}} {{Guards}}

# Instructions
# The Collatz Conjecture or 3x+1 problem can be summarized as follows:

# Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.

# Given a number n, return the number of steps required to reach 1.

# Examples
# Starting with n = 12, the steps would be as follows:

# 12
# 6
# 3
# 10
# 5
# 16
# 8
# 4
# 2
# 1
# Resulting in 9 steps. So for input n = 12, the return value would be 9.


defmodule CollatzConjecture do
  import Integer

  @spec calc(input :: pos_integer()) :: non_neg_integer()
  def calc(1), do: 0
  def calc(input) when input > 0 and is_even(input), do: 1 + calc(div(input, 2))
  def calc(input) when input > 0 and is_odd(input), do: 1 + calc(input * 3 + 1)
end



# Elixir / Darts {{Cond}}

# Instructions
# Write a function that returns the earned points in a single toss of a Darts game.

# Darts is a game where players throw darts at a target.

# In our particular instance of the game, the target rewards 4 different amounts of points, depending on where the dart lands:

# If the dart lands outside the target, player earns no points (0 points).
# If the dart lands in the outer circle of the target, player earns 1 point.
# If the dart lands in the middle circle of the target, player earns 5 points.
# If the dart lands in the inner circle of the target, player earns 10 points.
# The outer circle has a radius of 10 units (this is equivalent to the total radius for the entire target), the middle circle a radius of 5 units, and the inner circle a radius of 1. Of course, they are all centered at the same point (that is, the circles are concentric) defined by the coordinates (0, 0).

# Write a function that given a point in the target (defined by its Cartesian coordinates x and y, where x and y are real), returns the correct amount earned by a dart landing at that point.


defmodule Darts do
  def score({x, y}) do
    r = :math.sqrt(x * x + y * y)
    cond do
      r > 10 -> 0
      r > 5 -> 1
      r > 1 -> 5
      true -> 10
    end
  end
end



# Elixir / Binary Search {{Cond}} {{Tuples}} {{Recursion}}

# Instructions
# Implement a binary search algorithm.

# Searching a sorted collection is a common task. A dictionary is a sorted list of word definitions. Given a word, one can find its definition. A telephone book is a sorted list of people's names, addresses, and telephone numbers. Knowing someone's name allows one to quickly find their telephone number and address.

# If the list to be searched contains more than a few items (a dozen, say) a binary search will require far fewer comparisons than a linear search, but it imposes the requirement that the list be sorted.

# In computer science, a binary search or half-interval search algorithm finds the position of a specified input value (the search "key") within an array sorted by key value.

# In each step, the algorithm compares the search key value with the key value of the middle element of the array.

# If the keys match, then a matching element has been found and its index, or position, is returned.

# Otherwise, if the search key is less than the middle element's key, then the algorithm repeats its action on the sub-array to the left of the middle element or, if the search key is greater, on the sub-array to the right.

# If the remaining array to be searched is empty, then the key cannot be found in the array and a special "not found" indication is returned.

# A binary search halves the number of items to check with each iteration, so locating an item (or determining its absence) takes logarithmic time. A binary search is a dichotomic divide and conquer search algorithm.


defmodule BinarySearch do
  @spec search(tuple, integer) :: {:ok, integer} | :not_found
  def search(tuple, key), do: search(tuple, key, 0, tuple_size(tuple) - 1)
  defp search(_tuple, _key, left, right) when left > right, do: :not_found
  defp search(tuple, key, left, right) do
    mid = div(left + right, 2)
    case elem(tuple, mid) do
      ^key -> {:ok, mid}
      value when value > key -> search(tuple, key, left, mid - 1)
      value when value < key -> search(tuple, key, mid + 1, right)
    end
  end
end



# Elixir / Leap {{Booleans}} {{Integers}} {{Guards}} {{Multiple Clause Functions}}

# Instructions
# Given a year, report if it is a leap year.

# The tricky thing here is that a leap year in the Gregorian calendar occurs:

# on every year that is evenly divisible by 4
#   except every year that is evenly divisible by 100
#     unless the year is also evenly divisible by 400
# For example, 1997 is not a leap year, but 1996 is. 1900 is not a leap year, but 2000 is.

# Notes
# Though our exercise adopts some very simple rules, there is more to learn!

# For a delightful, four minute explanation of the whole leap year phenomenon, go watch this youtube video.


defmodule Year do
  def leap_year?(year) when rem(year, 400) == 0, do: true
  def leap_year?(year) when rem(year, 100) == 0, do: false
  def leap_year?(year) when rem(year, 4) == 0, do: true
  def leap_year?(_), do: false
end



# Elixir  / Rational Numbers {{Basics}}

# Instructions
# A rational number is defined as the quotient of two integers a and b, called the numerator and denominator, respectively, where b != 0.

# Note
# Note that mathematically, the denominator can't be zero. However in many implementations of rational numbers, you will find that the denominator is allowed to be zero with behaviour similar to positive or negative infinity in floating point numbers. In those cases, the denominator and numerator generally still can't both be zero at once.

# The absolute value |r| of the rational number r = a/b is equal to |a|/|b|.

# The sum of two rational numbers r₁ = a₁/b₁ and r₂ = a₂/b₂ is r₁ + r₂ = a₁/b₁ + a₂/b₂ = (a₁ * b₂ + a₂ * b₁) / (b₁ * b₂).

# The difference of two rational numbers r₁ = a₁/b₁ and r₂ = a₂/b₂ is r₁ - r₂ = a₁/b₁ - a₂/b₂ = (a₁ * b₂ - a₂ * b₁) / (b₁ * b₂).

# The product (multiplication) of two rational numbers r₁ = a₁/b₁ and r₂ = a₂/b₂ is r₁ * r₂ = (a₁ * a₂) / (b₁ * b₂).

# Dividing a rational number r₁ = a₁/b₁ by another r₂ = a₂/b₂ is r₁ / r₂ = (a₁ * b₂) / (a₂ * b₁) if a₂ is not zero.

# Exponentiation of a rational number r = a/b to a non-negative integer power n is r^n = (a^n)/(b^n).

# Exponentiation of a rational number r = a/b to a negative integer power n is r^n = (b^m)/(a^m), where m = |n|.

# Exponentiation of a rational number r = a/b to a real (floating-point) number x is the quotient (a^x)/(b^x), which is a real number.

# Exponentiation of a real number x to a rational number r = a/b is x^(a/b) = root(x^a, b), where root(p, q) is the qth root of p.

# Implement the following operations:

# addition, subtraction, multiplication and division of two rational numbers,
# absolute value, exponentiation of a given rational number to an integer power, exponentiation of a given rational number to a real (floating-point) power, exponentiation of a real number to a rational number.
# Your implementation of rational numbers should always be reduced to lowest terms. For example, 4/4 should reduce to 1/1, 30/60 should reduce to 1/2, 12/8 should reduce to 3/2, etc. To reduce a rational number r = a/b, divide a and b by the greatest common divisor (gcd) of a and b. So, for example, gcd(12, 8) = 4, so r = 12/8 can be reduced to (12/4)/(8/4) = 3/2. The reduced form of a rational number should be in "standard form" (the denominator should always be a positive integer). If a denominator with a negative integer is present, multiply both numerator and denominator by -1 to ensure standard form is reached. For example, 3/-4 should be reduced to -3/4

# Assume that the programming language you are using does not have an implementation of rational numbers.


defmodule RationalNumbers do
  @type rational :: {integer, integer}

  @spec add(a :: rational, b :: rational) :: rational
  def add({a1, b1}, {a2, b2}), do: {(a1 * b2) + (a2 * b1), b1 * b2} |> reduce()

  @spec subtract(a :: rational, b :: rational) :: rational
  def subtract({a1, b1}, {a2, b2}), do: {(a1 * b2) - (a2 * b1), b1 * b2} |> reduce()

  @spec multiply(a :: rational, b :: rational) :: rational
  def multiply({a1, b1}, {a2, b2}), do: {a1 * a2, b1 * b2} |> reduce()

  @spec divide_by(num :: rational, den :: rational) :: rational
  def divide_by({a1, b1}, {a2, b2}), do: {a1 * b2, a2 * b1} |> reduce()

  @spec abs(a :: rational) :: rational
  def abs({a, b}), do: {Kernel.abs(a), Kernel.abs(b)} |> reduce()

  @spec pow_rational(a :: rational, n :: integer) :: rational
  def pow_rational({a, b}, n) when n < 0, do: {b ** -n, a ** -n} |> reduce()
  def pow_rational({a, b}, n), do: {a ** n, b ** n} |> reduce()

  @spec pow_real(x :: integer, n :: rational) :: float
  def pow_real(x, {a, b}), do: x ** (a / b)

  @spec reduce(a :: rational) :: rational
  def reduce({a, b}) when b < 0, do: reduce({-a, -b})
  def reduce({a, b}) do
    gcd = Integer.gcd(a, b)
    {a / gcd, b / gcd}
  end
end



# Elixir / Knapsack

# Instructions
# In this exercise, let's try to solve a classic problem.

# Bob is a thief. After months of careful planning, he finally manages to crack the security systems of a high-class apartment.

# In front of him are many items, each with a value (v) and weight (w). Bob, of course, wants to maximize the total value he can get; he would gladly take all of the items if he could. However, to his horror, he realizes that the knapsack he carries with him can only hold so much weight (W).

# Given a knapsack with a specific carrying capacity (W), help Bob determine the maximum value he can get from the items in the house. Note that Bob can take only one of each item.

# All values given will be strictly positive. Items will be represented as a list of pairs, wi and vi, where the first element wi is the weight of the ith item and vi is the value for that item.

# For example:

# Items: [ { "weight": 5, "value": 10 }, { "weight": 4, "value": 40 }, { "weight": 6, "value": 30 }, { "weight": 4, "value": 50 } ]

# Knapsack Limit: 10

# For the above, the first item has weight 5 and value 10, the second item has weight 4 and value 40, and so on.

# In this example, Bob should take the second and fourth item to maximize his value, which, in this case, is 90. He cannot get more than 90 as his knapsack has a weight limit of 10.


defmodule Knapsack do
  def maximum_value([], _), do: 0
  def maximum_value([item | items], max_weight) when item.weight > max_weight, do:
    maximum_value(items, max_weight)
  def maximum_value([item | items], max_weight), do:
    [
      maximum_value(items, max_weight),
      maximum_value(items, max_weight - item.weight) + item.value
    ]
    |> Enum.max()
end
