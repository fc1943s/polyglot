# Run as: iex --dot-iex path/to/notebook.exs

# Title: Exercism

# ── Exercism ──

# ### Elixir / Hello World

# ### Instructions

# The classical introductory exercise. Just say "Hello, World!".

# ["Hello, World!"](http://en.wikipedia.org/wiki/%22Hello,_world!%22_program) is the traditional first program for beginning programming in a new language or environment.

# The objectives are simple:

# * Modify the provided code so that it produces the string "Hello, World!".
# * Run the test suite and make sure that it succeeds.
# * Submit your solution and check it at the website.

# If everything goes well, you will be ready to fetch your first real exercise.

defmodule HelloWorld do
  def hello(), do: "Hello, World!"
end

ExUnit.start(autorun: false)

defmodule HelloWorldTest do
  use ExUnit.Case, async: true

  test "says 'Hello, World!'" do
    assert HelloWorld.hello() == "Hello, World!"
  end
end

ExUnit.run()

# ### Elixir / Lasagna {{Basics}}

# ### Introduction

# ### Basics

# #### Variables

# Elixir is a dynamically-typed language, meaning that the type of a variable is only checked at runtime. Using the match `=` operator, we can bind a value of any type to a variable name:

# ```elixir
# count = 1 # Bound an integer value of 1
# count = 2 # You may re-bind variables

# count = false # You may re-bind any type to a variable

# message = "Success!" # Strings can be created by enclosing characters within double quotes
# ```

# #### Modules

# Elixir is a [functional-programming language](https://en.wikipedia.org/wiki/Functional_programming) and requires all named functions to be defined in a _module_. The `defmodule` keyword is used to define a module. All modules are available to all other modules at runtime and do not require an _access modifier_ to make them visible to other parts of the program. A _module_ is analogous to a _class_ in other programming languages.

# ```elixir
# defmodule Calculator do
#   # ...
# end
# ```

# #### Named functions

# _Named Functions_ must be defined in a module. The `def` keyword is used to define a _public_ named function.

# Each function can have zero or more arguments. The value of the last expression in a function is always _implicitly returned_.

# ```elixir
# defmodule Calculator do
#   def add(x, y) do
#     x + y
#   end
# end
# ```

# Invoking a function is done by specifying its module and function name and passing arguments for each of the function's arguments.

# ```elixir
# sum = Calculator.add(1, 2)
# # => 3
# ```

# The `defp` keyword can be used instead of `def` to define a _private_ function. Private functions can only be used from within the same module that defined them.

# When invoking a function inside the same module where it's defined, the module name can be omitted.

# You may also write short functions using a one-line syntax (note the comma `,` and the colon `:` around the keyword `do`).

# ```elixir
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
# ```

# #### Arity of functions

# It is common to refer to functions with their _arity_. The _arity_ of a function is the number of arguments it accepts.

# ```elixir
# # add/3 because this function has three arguments, thus an arity of 3
# def add(x, y, z) do
#   x + y + z
# end
# ```

# #### Standard library

# Elixir has a very rich and well-documented standard library. The documentation is available online at [hexdocs.pm/elixir](https://hexdocs.pm/elixir/Kernel.html#content). Save this link somewhere - you will use it a lot!

# Most built-in data types have a corresponding module that offers functions for working with that data type, e.g. there's the `Integer` module for integers, `String` module for strings, `List` module for lists and so on.

# A notable module is the `Kernel` module. It provides the basic capabilities on top of which the rest of the standard library is built, like arithmetic operators, control-flow macros, and much more. Functions for the `Kernel` module are automatically imported, so you can use them without the `Kernel.` prefix.

# #### Code comments

# Comments can be used to leave notes for other developers reading the source code. Single line comments in Elixir are preceded by `#`.

# ### Instructions

# In this exercise you're going to write some code to help you cook a brilliant lasagna from your favorite cooking book.

# You have five tasks, all related to the time spent cooking the lasagna.

# **Task 1 - Define the expected oven time in minutes!**

# Define the `Lasagna.expected_minutes_in_oven/0` function that does not take any arguments and returns how many minutes the lasagna should be in the oven. According to the cooking book, the expected oven time in minutes is 40:

# ```elixir
# Lasagna.expected_minutes_in_oven()
# # => 40
# ```

# **Task 2 - Calculate the remaining oven time in minutes!**

# Define the `Lasagna.remaining_minutes_in_oven/1` function that takes the actual minutes the lasagna has been in the oven as an argument and returns how many minutes the lasagna still has to remain in the oven, based on the expected oven time in minutes from the previous task.

# ```elixir
# Lasagna.remaining_minutes_in_oven(30)
# # => 10
# ```

# **Task 3 - Calculate the preparation time in minutes!**

# Define the `Lasagna.preparation_time_in_minutes/1` function that takes the number of layers you added to the lasagna as an argument and returns how many minutes you spent preparing the lasagna, assuming each layer takes you 2 minutes to prepare.

# ```elixir
# Lasagna.preparation_time_in_minutes(2)
# # => 4
# ```

# **Task 4 - Calculate the total working time in minutes!**

# Define the `Lasagna.total_time_in_minutes/2` function that takes two arguments: the first argument is the number of layers you added to the lasagna, and the second argument is the number of minutes the lasagna has been in the oven. The function should return how many minutes in total you've worked on cooking the lasagna, which is the sum of the preparation time in minutes, and the time in minutes the lasagna has spent in the oven at the moment.

# ```elixir
# Lasagna.total_time_in_minutes(3, 20)
# # => 26
# ```

# **Task 5 - Create a notification that the lasagna is ready!**

# Define the `Lasagna.alarm/0` function that does not take any arguments and returns a message indicating that the lasagna is ready to eat.

# ```elixir
# Lasagna.alarm()
# # => "Ding!"
# ```

defmodule Lasagna do
  def expected_minutes_in_oven, do: 40
  def remaining_minutes_in_oven(elapsed), do: expected_minutes_in_oven() - elapsed
  def preparation_time_in_minutes(layers), do: layers * 2
  def total_time_in_minutes(layers, elapsed), do: elapsed + preparation_time_in_minutes(layers)
  def alarm, do: "Ding!"
end

ExUnit.start(autorun: false)

defmodule LasagnaTest do
  use ExUnit.Case, async: true

  test "Test" do
    assert Lasagna.expected_minutes_in_oven() === 40
    assert Lasagna.remaining_minutes_in_oven(25) === 15
    assert Lasagna.preparation_time_in_minutes(1) === 2
    assert Lasagna.preparation_time_in_minutes(4) === 8
    assert Lasagna.total_time_in_minutes(1, 30) === 32
    assert Lasagna.total_time_in_minutes(4, 8) === 16
    assert Lasagna.alarm() === "Ding!"
  end
end

ExUnit.run()

# ### Elixir / Pacman Rules {{Booleans}}

# ### Introduction

# #### Booleans

# Elixir represents true and false values with the boolean type. There are only two values: `true` and `false`. These values can be bound to a variable:

# ```elixir
# true_variable = true
# false_variable = false
# ```

# We can evaluate strict boolean expressions using the `and/2`, `or/2`, and `not/1` operators.

# ```elixir
# true_variable = true and true
# false_variable = true and false

# true_variable = false or true
# false_variable = false or false

# true_variable = not false
# false_variable = not true
# ```

# When writing a function that returns a boolean value, it is idiomatic to end the function name with a `?`.

# ```elixir
# def either_true?(a, b) do
#   a or b
# end
# ```

# ### Instructions

# In this exercise, you need to translate some rules from the classic game Pac-Man into Elixir functions.

# You have four rules to translate, all related to the game states.

# > Don't worry about how the arguments are derived, just focus on combining the arguments to return the intended result.

# **Task 1 - Define if Pac-Man eats a ghost!**

# Define the `Rules.eat_ghost?/2` function that takes two arguments (_if Pac-Man has a power pellet active_ and _if Pac-Man is touching a ghost_) and returns a boolean value if Pac-Man is able to eat the ghost. The function should return true only if Pac-Man has a power pellet active and is touching a ghost.

# ```elixir
# Rules.eat_ghost?(false, true)
# # => false
# ```

# **Task 2 - Define if Pac-Man scores!**

# Define the `Rules.score?/2` function that takes two arguments (_if Pac-Man is touching a power pellet_ and _if Pac-Man is touching a dot_) and returns a boolean value if Pac-Man scored. The function should return true if Pac-Man is touching a power pellet or a dot.

# ```elixir
# Rules.score?(true, true)
# # => true
# ```

# **Task 3 - Define if Pac-Man loses!**

# Define the `Rules.lose?/2` function that takes two arguments (_if Pac-Man has a power pellet active_ and _if Pac-Man is touching a ghost_) and returns a boolean value if Pac-Man loses. The function should return true if Pac-Man is touching a ghost and does not have a power pellet active.

# ```elixir
# Rules.lose?(false, true)
# # => true
# ```

# **Task 4 - Define if Pac-Man wins!**

# Define the `Rules.win?/3` function that takes three arguments (_if Pac-Man has eaten all of the dots_, _if Pac-Man has a power pellet active_, and _if Pac-Man is touching a ghost_) and returns a boolean value if Pac-Man wins. The function should return true if Pac-Man has eaten all of the dots and has not lost based on the arguments defined in part 3.

# ```elixir
# Rules.win?(false, true, false)
# # => false
# ```

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

ExUnit.start(autorun: false)

defmodule RulesTest do
  use ExUnit.Case, async: true

  test "Test" do
    assert Rules.eat_ghost?(true, true)
    refute Rules.eat_ghost?(false, true)
    refute Rules.eat_ghost?(true, false)
    assert Rules.score?(false, true)
    assert Rules.score?(true, false)
    refute Rules.score?(false, false)
    assert Rules.lose?(false, true)
    refute Rules.lose?(true, true)
    refute Rules.lose?(true, false)
    assert Rules.win?(true, false, false)
    refute Rules.win?(true, false, true)
    assert Rules.win?(true, true, true)
  end
end

ExUnit.run()

# ### Elixir / Freelancer Rates {{Floating Point Numbers}} {{Integers}}

# ### Introduction

# #### Integers

# There are two different kinds of numbers in Elixir - integers and floats.

# Integers are whole numbers.

# ```elixir
# integer = 3
# # => 3
# ```

# #### Floating Point Numbers

# Floats are numbers with one or more digits behind the decimal separator. They use the 64-bit double precision floating-point format.

# ```elixir
# float = 3.45
# # => 3.45
# ```

# ##### Working with numbers

# In the [`Integer`](https://hexdocs.pm/elixir/Integer.html#functions) and [`Float`](https://hexdocs.pm/elixir/Float.html#functions) modules you can find some useful functions for working with those types. Basic arithmetic operators are defined in the [`Kernel`](https://hexdocs.pm/elixir/Kernel.html#*/2) module.

# ##### Conversion

# Integers and floats can be mixed together in a single arithmetic expression. Using a float in an expression ensures the result will be a float too.

# ```elixir
# 2 * 3
# # => 6

# 2 * 3.0
# # => 6.0
# ```

# However, when doing division, the result will always be a float, even if only integers are used.

# ```elixir
# 6 / 2
# # => 3.0
# ```

# To convert a float to an integer, you can discard the decimal part with [`trunc/1`](https://hexdocs.pm/elixir/Kernel.html#trunc/1).

# ### Instructions

# In this exercise you'll be writing code to help a freelancer communicate with a project manager by providing a few utilities to quickly calculate daily and monthly rates, optionally with a given discount.

# We first establish a few rules between the freelancer and the project manager:

# * The daily rate is 8 times the hourly rate.
# * A month has 22 billable days.

# The freelancer is offering to apply a discount if the project manager chooses to let the freelancer bill per month, which can come in handy if there is a certain budget the project manager has to work with.

# Discounts are modeled as fractional numbers representing percentage, for example `25.0` (25%).

# **Task 1 - Calculate the daily rate given an hourly rate!**

# Implement a function to calculate the daily rate given an hourly rate:

# ```elixir
# FreelancerRates.daily_rate(60)
# # => 480.0
# ```

# The returned daily rate should be a float.

# **Task 2 - Calculate a discounted price!**

# Implement a function to calculate the price after a discount.

# ```elixir
# FreelancerRates.apply_discount(150, 10)
# # => 135.0
# ```

# The returned value should always be a float, not rounded in any way.

# **Task 3 - Calculate the monthly rate, given an hourly rate and a discount!**

# Implement a function to calculate the monthly rate, and apply a discount:

# ```elixir
# FreelancerRates.monthly_rate(77, 10.5)
# # => 12130
# ```

# The returned monthly rate should be rounded up (take the ceiling) to the nearest integer.

# **Task 4 - Calculate the number of workdays given a budget, hourly rate and discount!**

# Implement a function that takes a budget, an hourly rate, and a discount, and calculates how many days of work that covers.

# ```elixir
# FreelancerRates.days_in_budget(20000, 80, 11.0)
# # => 35.1
# ```

# The returned number of days should be rounded down (take the floor) to one decimal place.

defmodule FreelancerRates do
  @daily_rate 8.0
  @monthly_billable_days 22

  def daily_rate(hourly_rate), do: hourly_rate * @daily_rate
  def apply_discount(before_discount, discount), do: before_discount * (1 - discount / 100)

  def monthly_rate(hourly_rate, discount) do
    (daily_rate(hourly_rate) * @monthly_billable_days)
    |> apply_discount(discount)
    |> Float.ceil()
    |> round()
  end

  def days_in_budget(budget, hourly_rate, discount) do
    (budget / apply_discount(daily_rate(hourly_rate), discount))
    |> Float.floor(1)
  end
end

ExUnit.start(autorun: false)

defmodule FreelancerRatesTest do
  use ExUnit.Case, async: true

  test "Test" do
    assert FreelancerRates.daily_rate(50) == 400.0
    assert FreelancerRates.daily_rate(60) === 480.0
    assert FreelancerRates.daily_rate(55.1) == 440.8
    assert FreelancerRates.apply_discount(140.0, 10) == 126.0
    assert FreelancerRates.apply_discount(100, 10) == 90.0
    assert_in_delta FreelancerRates.apply_discount(111.11, 13.5), 96.11015, 1.0e-6
    assert FreelancerRates.monthly_rate(62, 0.0) == 10912
    assert FreelancerRates.monthly_rate(70, 0.0) === 12320
    assert FreelancerRates.monthly_rate(62.8, 0.0) == 11053
    assert FreelancerRates.monthly_rate(65.2, 0.0) == 11476
    assert FreelancerRates.monthly_rate(67, 12.0) == 10377
    assert FreelancerRates.days_in_budget(1600, 50, 0.0) == 4
    assert FreelancerRates.days_in_budget(520, 65, 0.0) === 1.0
    assert FreelancerRates.days_in_budget(4410, 55, 0.0) == 10.0
    assert FreelancerRates.days_in_budget(4480, 55, 0.0) == 10.1
    assert FreelancerRates.days_in_budget(480, 60, 20) == 1.2
  end
end

ExUnit.run()
