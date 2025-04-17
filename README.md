# Nyoom
A programming language, designed to have as few token types as possible.

Nyoom contains three tokens, `nyoom`, `nugget`, and `I`. These can easily be replaced in the lexer to create a more compact language.

NYOOM IS CURRENTLY IN ALPHA. THE INTERPRETER MAY NOT WORK AS INTENDED
## Design
Nyoom is an imperative procedural scripting language. It has variables and functions. It is dynamically typed, with operators and other builtin functions throwing type errors.
### Data types
Nyoom contains two possible data types, integers and arrays. Arrays can be heterogenous (contain multiple data types).
Integers are created with integer literals or the array lenght operator.
Arrays are create with concat syntax.
### Builtin functions
Some language features, like arithmetic, array indexing or I/O, are implemented with builtin functions.
The interpreter supports custom builtin functions, created using lookup maps.
A complete list is avaible in the language reference.
### Control flow
Nyoom contains two basic control flow operators, if and while.
Both take an expression as an argument, and check if it is zero.
If it is not, then the block is executed. The while loop is executed until the expression evaluates to zero.
### Functions
