Document
===

# Tutorial

## An example program

The following is a Fix program that calculates the first 30 numbers of Fibonacci sequence. 

```
module Main;

calc_fib : Int -> Array Int;
calc_fib = |n| (
    let arr = Array::fill(n, 0);
    let arr = arr.set!(0, 1);
    let arr = arr.set!(1, 1);
    let arr = loop((2, arr), |(idx, arr)|
        if idx == arr.get_length {
            break $ arr
        } else {
            let x = arr.get(idx-1);
            let y = arr.get(idx-2);
            let arr = arr.set!(idx, x+y);
            continue $ (idx+1, arr)
        }
    );
    arr
);

main : IOState -> ((), IOState);
main = (
    let fib = calc_fib(30);
    println! $ Iterator::from_array(fib).map(to_string).join(", ")
);
```

If you save the above program to a file "main.fix" and run `fix run main.fix`, it prints 

```
1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040
```

to the standard output.

In the followings, I explain language specifications which are necessary to understand the above program.

## Modules

The first line is the module definition:

```
module Main;
```

In Fix, values, functions, types and traits defined in a source file is collected to a module. Each source file has to declare the name of the module it defines by `module {module_name};`. The first letter of the module name must be capitalized.

When Fix program runs, it calls `main` function defined in the `Main` module.

The usefulness of modules is hard to see in this example. They are useful when you construct a program from multiple source files.

## Global values

The following parts are definitions of two global values `calc_fib` and `main`.

```
calc_fib : Int -> Array Int;
calc_fib = ...; // call this expression A.

main : IOState -> ((), IOState);
main = ...; // call this expression B.
```

These lines means that:

- `calc_fib` global value has type `Int -> Array Int` and it's value is defined by expression A.
- `main` global value has type `IOState -> ((), IOState)` and it's value is defined by expression B.

In Fix, you have to specify the type of a global value explicitly. 

## Namespaces

The `Array` in `Array::fill` or `Iterator` in `Iterator::from_array` are namespaces. Namespace is the "address" of a name and used to distinguish two values (or types or traits, anything you define globally) with the same name.

Namespaces of a name can be omitted if the value specified by the name is unique, or can be inferred from the context. In fact, you can write simply `fill(n, 0)` instead of `Array::fill(n, 0)` because there is only one function named `fill` at the current version of standard library. The reasons I wrote `Array::fill(n, 0)` here are:

- `Array::fill(n, 0)` is more readable than `fill(n, 0)`, because it expresses that `fill` function is related to `Array` type. A reader may be able to infer that `Array::fill` will generate an array of specified length filled by a specified initial value.
- In the future, another function named `fill` may be added to a namespace other than `Array`. After that, the name `fill` may become ambiguous and the compile of the example program may start to fail.

Actually, the full name of `fill` is not `Array::fill` but `Std::Array::fill`. `Std` is a module to put values standard library provides. Module is nothing but a top-level namespace. The namespace `Array` is defined as the sub-namespace of `Std` and used to put functions related to arrays. Similarly, full name of `calc_fib` function is `Main::calc_fib`. You can omit (possibly full) prefix of namespaces of a name as long as the value referred to is uniquely inferred by compiler from the context.

## Types

Each value in Fix has it's type. You can consider that a type is a set in mathematics, and value in Fix is an element of it's type. 

The followings are examples of types:

- `Int`: the type of 64-bit signed integers.
- `Bool`: the type of boolean values (i.e., `true` and `false`).
- `Array a`: the type of arrays whose elements have type `a`. `Array` is called a type constructor, because it generates types `Array Int` or `Array Bool` when applied to a type. `a` is called a type parameter.
- `String`: the type of strings.
- `Int -> Array Int`: the type of functions that takes an integer and returns an array of integers.
- `()`: the unit type. This type has a single value which is also written as `()`. 
- `(a, b)`: the type of pairs of values of `a` and `b`, where `a` and `b` are type parameters.
- `IOState`: the type whose value corresponds to a state of the world outside the Fix program. For example, printing a string to the standard output can be thought as an operation that changes the external state, and Fix expresses such an operation by a function that takes an `IOState` value and returns updated `IOState` value.
- `IOState -> ((), IOState)`: the type of functions that update the external state and receive no data. This type is isomorphic to `IOState -> IOState`, but we put a redundant `()` for monadic composition (you don't need to understand this terminology).
- `Int -> Bool -> Array Bool`: this is equivalent to `Int -> (Bool -> Array Bool)`, that is, the type of functions that receives an integer and returns a function that converts a boolean value into a boolean array. As an example, a function that produces a boolean array from it's length and initial value has this type. In Fix, there is no concept of "two-variable functions". A function in Fix is a (partial) function in mathematical sense: it converts an element of a set into an element of another set (or fails). The type of something like "two-variable functions" can be represented as `a -> b -> c` or `(a, b) -> c`.

In Fix, the first letter of the name of a specific type (such as `Int` or `Bool`) or a type constructor (such as `Array`) has to be 
capitalized. A type that starts with a lowercase letter is interpreted as a type parameter. Each type parameter will be instanciated to a specific type when the program is compiled.

## Expressions

Expression is a sentence which describes how to calculate a value. The followings are examples of expressions:

- `42`: a literal expression which means the number 42 represented as a signed 64-bit integer.
- `false`, `true`: literal expressions which means boolean value (represented as a 8-bit integer `0` and `1` internally).
- `[1, 2, 3]`: a literal expression which means an integer array with elements `1`, `2` and `3`.
- `"Hello World!"`: a string literal.
- `()`: the unit literal, whose type is also written as `()` and called "the unit type".
- `(1, true)`: a tuple literal, which produces a value of the type `(Int, Bool)`.
- `3 + 5`: an expression which means "the integer obtained by adding `3` and `5`".
- `let x = 3 + 5 in x * x`: an expression which means "Compute `3 + 5` and call the result `x`. Then compute `x * x`."
- `if c { x + y } else { x - y }`: an expression which means "If a boolean value `c` is `true`, then the value of this expression is `x + y`. Otherwise, the value of this expression is `x - y`".
- `f(x)`: an expression which means "the value obtained by applying a function `f` to the value `x`".
- `|x| x + 3`: an expression which means "the function which converts `x` to `x + 3`".

## Let-expressions

To define a local name by a value, use `let`-expression. The syntax is `let {name} = {expression_0} in {expression_1}` or `let {name} = {expression_0}; {expression_1}`.

If you write the whole let-expression in one line, it is preferred to use `in`: For example, `let x = 5 in 2 + x`. Of course, you can also write it as `let x = 5; 2 + x`.

On the other hand, if you want to put `{epxression_0}` and `{expression_1}` in other lines, it is better to use semicolon:
```
let x = 3;
let y = 5;
x + y
```

If `{expression_0}` ranges several lines, it is preferred to indent `{expression_0}` with parenthes. For example, the following program is more readable
```
let twice_of_three_plus_five = (
    let n = 3 + 5;
    n * n
);
```
than 
```
let twice_of_three_plus_five = 
let n = 3 + 5;
n * n;
```

Fix's `let`-expression doesn't allow recursive definition. For example, a program

```
use_rec_defn : Int;
use_rec_defn = let x = x + 3 in x * x;
```

cannot be compiled. A program

```
use_rec_defn : Int;
use_rec_defn = (
    let x = 5;
    let x = x + 3;
    x * x
);
```

will be compiled, but the name `x` in the right hand side of `let x = x + 3` is considered as the name `x` defined in the previous line (i.e., it's value is `5`), not as the new one.

This means that you cannot define a local recursive function by let-expression naively. To do this, use `fix` built-in function.

## If-expressions

The syntax of `if` is the following: `if cond { expr_0 } (else|;) { expr_1 }` where curly braces around `expr_1` is optional.
The type of `cond` has to be `Bool`, and types of `expr_0` and `expr_1` must coincide.

For usual case, use `if cond { expr_0 } else { expr_1 }`:
```
if cond { 
    "cond is true!"
} else {
    "cond is false!"
}
```

To write "early return" pattern, it is useful to omit curly braces around `{expr_1}`:
```
if cache_is_available { "the cached value" };
"a long program which calculates a value, store it into cache, and returns the value."
```

## Function application

To apply a function `f` to a value `x`, write `f(x)`.

```
neg(3) // -3 -- `neg` is a built-in function that takes a Int value and returns negative of it.
```

As I wrote before, there is no type of "two-variable functions" or "three-variable functions" in Fix. Instead, treat the value of type `a -> b -> c` (which is equal to `a -> (b -> c)`) as a thing like "two-variable function that takes a value of `a` and a value of `b`".　

Let's consider a "two-variable function" `multiply : Int -> Int -> Int` that multiplies two integers. Then `multiply(3) : Int -> Int` is a function that multiplies 3 to the given integer. So `multiply(3)(5)` results in 15. Now, the last expression can be written as `multiply(3, 5)`, because we have a syntax sugar that `f(x, y)` is equivalent to `f(x)(y)`. 

In the program of Fibonacci sequence, the expression `Array::fill(n, 0)` is an example of calling two-variable function `Array::fill` on two values `n` and `0`.

As a special syntax, writing `f()` implies `f(())`, i.e., application of function `f` to the unit value `()`.

## Functions

You can make a function value (which is similar to things called "lambda" or "closure" in other languages) by `|{arg}| {body}`. To define a two-variable function, you can simply write `|{arg0}, {arg1}| {body}` which is a syntax sugar of `|{arg0}| |{arg1}| {body}`.

Functions in fix can "capture" a value defined outside the function definition. As an example, consider the following program.

```
fifteen : Int;
fifteen = (
    let x = 3;
    let add_x = |n| n + x;
    add_x(4) + add_x(5) // (4 + 3) + (5 + 3) = 15
);
```

In the expression `|n| n + x`, `n` is the argument of the function and `x` refers to the name defined in the previous line. The function `add_x` memorises the value `3` and uses it when called.

Since all values (including functions) in Fix are immutable, the behavior of the function `add_x` will never change after you have defined it. For example, 

```
fifteen : Int;
fifteen = (
    let x = 3;
    let add_x = |n| n + x;
    let x = 0;
    add_x(4) + add_x(5) // (4 + 3) + (5 + 3) = 15
);
```

still evaluates to 15, because `add_x` is not affected by the change of the value that the name `x` refers to.

If the `{body}` part of your function ranges multiple lines, it is preferred to indent `{body}` with parenthes. For example, the program

```
calc_fib = |n| (
    let arr = Array::fill(n, 0);
    let arr = arr.set!(0, 1);
    let arr = arr.set!(1, 1);
    let arr = loop((2, arr), |(idx, arr)|
        if idx == arr.get_length {
            break $ arr
        } else {
            let x = arr.get(idx-1);
            let y = arr.get(idx-2);
            let arr = arr.set!(idx, x+y);
            continue $ (idx+1, arr)
        }
    );
    arr
);
```

is more readable than the following: 

```
calc_fib = |n| 
let arr = Array::fill(n, 0);
let arr = arr.set!(0, 1);
let arr = arr.set!(1, 1);
let arr = loop((2, arr), |(idx, arr)|
    if idx == arr.get_length {
        break $ arr
    } else {
        let x = arr.get(idx-1);
        let y = arr.get(idx-2);
        let arr = arr.set!(idx, x+y);
        continue $ (idx+1, arr)
    }
);
arr;
```

## Operator `.` and `$`

The operator `.` is another way of applying function to a value. It is defined as `x.f == f(x)`.

The precedence of the operator `.` is lower than function application by parenthes. So, if a function `method` has a type `Param -> Obj -> Result`, then `obj.method(arg)` is interpreted as `obj.(method(arg)) == method(arg)(obj) == method(arg, obj)`, not as `(obj.method)(arg)`.

In the program of Fibonacci sequence, the followings are examples of use of operator `.`:

- `arr.get_length`: `get_length` is a function of type `Array a -> Int`, which returns the length of an array. Note that you should not write `arr.get_length()` as if you call a method of a class on an instance in other languages. Remembering syntax sugars `f() == f(())` and `x.f == f(x)`, you can desugar the expression `arr.get_length()` to `get_length((), arr)`, which raises an error because `get_length` takes only one argument.
- `arr.set!(0, 1)`: `set!` is a function of type `Int -> a -> Array a -> Array a`, which updates an element of an array to the specified value. 
- `arr.get(idx-1)`: `get` is a function of type `Int -> Array a -> a`, which returns the element at the specified index.

We sometimes call a function of type `Param0 -> ... -> ParamN -> Obj -> Result` as a "method" on the type `Obj` that has N+1 parameters and returns a value of type `Result`. A method can be called by `obj.method(arg0, ..., argN)` as if writing OOP languages.

Another way of function application is operator `$`: `f $ x = f(x)`. This operator is right associative: `f $ g $ x = f(g(x))`. This operator is useful for reducing parenthes. In the program of Fibonacci sequence, the followings are examples of use of operator `$`:

- `continue $ (idx+1, arr)`: the application of the `continue` function to the tuple value `(idx+1, arr)`. In Fix, `continue` and `break` are usual functions, not syntaxes. So you can write this expression as `continue((idx+1, arr))` or `(idx+1, arr).continue`, but I prefer to write `continue $ (idx+1, arr)`, because it looks special. More explanation of `continue` and `break` functions will be given later. 
- `println! $ Iterator::from_array(fib).map(to_string).join(", ")`: the application of the `println!` function to the string value expressed by `Iterator::from_array(fib).map(to_string).join(", ")`. The `println!` function has type `String -> IOState -> ((), IOState)`, so applying to `println!` to a string produces a value of `IOState -> ((), IOState)`, which is equal to the type of `main` function. This expression can also be written as `println!(Iterator::from_array(fib).map(to_string).join(", "))`, but using operator `$` you can reduce parenthes around the long string expression.

For more examples, consider following programs:

```
main : IOState -> ((), IOState);
main = println! $ "Hello World!";
```

```
main : IOState -> ((), IOState);
main = |io| io.println!("Hello World!");
```

These two programs are equivalent. The first uses operator `$` to obtain the value of type `IOState -> ((), IOState)` by applying `println! : String -> IOState -> ((), IOState)` to the string `"Hello World!"`. On the other hand, `println!` can be considered as a method of `IOState` that takes a string as an argument. In the second program, the implementation of `main` takes an arugment `io` of `IOState` explicitly, and call `println!` method on the `io` value.

The precedence between three ways of function application is `f(x)` > `x.f` > `f $ x`. By this, it is illegal to write `io.println! $ "Hello World!"`. It is equivalent to `println!(io) $ "Hello World!" == println!(io, "Hello World!")`, which is trying to apply `println!` on two arguments in the wrong ordering. It is ok to write `println!("Hello World!") $ io`, which can be read as "apply `println!` to a string to obtain a function of type `IOState -> ((), IOState)`, and apply it to `io: IOState`".

## Patterns

Both of let-expression and function expression introduces local names. If the type of the local name is tuple (or, more generally, structs), you can use patterns to destructure the passed value.

For example, let's define a function that takes a value of tuple type `(Int, Bool)`, and returns a value of `(Bool, Int)` by swapping two components. Using built-in functions `@0 : (a, b) -> a` and `@1 : (a, b) -> b` to extract the component from a tuple, you can write:

```
swap : (Int, Bool) -> (Bool, Int);
swap = |tuple| (
    let fst = tuple.@0;
    let snd = tuple.@1;
    (snd, fst)
);
```

Using pattern, this program can be written as:

```
swap : (Int, Bool) -> (Bool, Int);
swap = |tuple| (
    let (fst, snd) = tuple;
    (snd, fst)
);
```

or more shortly, 

```
swap : (Int, Bool) -> (Bool, Int);
swap = |(fst, snd)| (snd, fst);
```

Don't confuse `|(x, y)| ...` with `|x, y| ...`. The former defines a function that receives a tuple, and the latter defines a two-variable function.

## `loop`, `continue` and `break` function

The `loop` built-in function has type `s -> (s -> LoopResult s b) -> b`. The value of `LoopResult` type can be constructed from `continue` or `break` function.

- `continue : s -> LoopResult s b`
- `break : b -> LoopResult s b`

The `loop` function takes two arguments: the initial state of the loop `s0` and the loop body function `body`. It first calls `body` on `s0`. If `body` returns a value `break(r)`, then the `loop` function ends and returns `r` as the result. If `body` returns `continue(s)`, then the `loop` function calls again `body` on `s`.

In the program of Fibonacci sequence, the `loop` function is used in the following expression:

```
loop((2, arr), |(idx, arr)|
    if idx == arr.get_length {
        break $ arr
    } else {
        let x = arr.get(idx-1);
        let y = arr.get(idx-2);
        let arr = arr.set!(idx, x+y);
        continue $ (idx+1, arr)
    }
);
```

The initial value of this loop is `(2, arr)`. The loop body takes a tuple `(idx, arr)`, that is, the index of an array to be updated next, and an array to store the Fibonacci sequence whose values are already right at indices 0, ..., idx-1. If `idx` is less than `arr.get_length`, it calculates the value of Fibonacci sequence at `idx`, stores it to `arr`, and returns `continue $ (idx+1, arr)` to proceed to the next step. If `idx` has reached to `arr.get_length`, it returns `break $ arr` to end the loop. The return value of the `loop` function is an array.

## Unions

Then what is the type `LoopResult s b`? It is defined as an union with two type parameters `s` and `b`. It can be defined as follows:

```
type LoopResult s b = union { continue : s, break : b };
```

The above definition indicates that a `LoopResult s b` value contains either of a value of type `s` or a value of type `b`. If you write the set of values of a type as `|type|`, then `|LoopResult s b| = |s| ⨆ |b|`, where the symbol `⨆` is represents the disjoint union of sets.

For each union type, some basic methods are automatically defined. For example, for `LoopResult` as above, the following functions are defined in the namespace `LoopResult`.

- `continue : s -> LoopResult s b`: converts an value of type `s` into a `LoopResult` value.
- `break : b -> LoopResult s b`: converts an value of type `b` into a `LoopResult` value.
- `is_continue : LoopResult s b -> Bool`: checks if the `LoopResult` value was created by `continue`.
- `is_break : LoopResult s b -> Bool`: checks if the `LoopResult` value was created by `break`.
- `as_continue : LoopResult s b -> s`: extracts a value of type `s` from a `LoopResult` value if it is created by `continue`. If not, this function panics (i.e., prints an error message and stops the execution of the program).
- `as_break : LoopResult s b -> s`: extracts a value of type `b` from a `LoopResult` value if it is created by `break`. If not, this function panics (i.e., prints an error message and stops the execution of the program).

Another example of union is `Option` which is used to represent a value "which may not contain a value". It can be defined as follows: 

```
type Option a = union { none : (), some : s };
```

Note that, if you want to create a none value of `Option`, you need to write `none()`, because `none` is a function of type `() -> Option a`. (Remember that the syntax sugar `f() == f(())`.)

## Structs

Although it does not appear in the example Fibonacci program, here I explain how to define your own struct.

For example, you can define a struct called `Product` with two fields `price`  of type `Int` and `sold` of type `Bool` as follows.

```
type Product = struct { price: Int, sold: Bool };
```

You can construct a struct value by the syntax `{struct_name} { ({field_name}: {field_value}) } `:

```
let product = Product { price: 100, sold: false };
```

As in the case of unions, there are methods that are automatically defined for structs. For `Price` as above, the following methods are defined in the namespace `Price`.

- `@price : Product -> Int` and `@sold : Product -> Bool`
    - Extracts the value of a field from a `Product` value.
- `=price : Int -> Product -> Product` and `=sold : Bool -> Product -> Product`
    - Modify a `Product` value by setting a field.
- `mod_price : (Int -> Int) -> Product -> Product` and `mod_sold : (Bool -> Bool) -> Product -> Product`
    - Modify a `Product` value by a function acting on a field.

I already explained that we can use patterns to destructure tuples. You can also use patterns to destructure a struct value. For example, field accessor function `@price : Product -> Int` can be re-defined as follows: 

```
get_price : Product -> Int;
get_price = |product| (
    let Product { price: price, sold: sold } = product;
    price
);
```

or 

```
get_price : Product -> Int;
get_price = |Product { price: price, sold: sold }| price;
```

## Iterators

Now I explain about the expression `Iterator::from_array(fib).map(to_string).join(", ")`, where `fib : Array Int` is the array of Fibonacci sequence. This expression 
- converts a Fibonacci array into an iterator of integers, 
- apply `to_string : Int -> String` to each element to obtain the iterator of strings, and
- concatenates these strings separated by `", "`,
- results in a string "1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040".

Like array, iterator (a.k.a. "lazy list") is a way to represent sequences. Whereas an array stores the values of all elements in memory at the same time, an iterator only has a function to compute the next element and the next iterator. In fact, iterator in Fix is defined as follows:

```
type Iterator a = unbox struct { next: () -> Option (a, Iterator a) };
```

(You don't need to understand `unbox` specifier at now.)

The above definition indicates that the `Iterator` is a struct with only one field `next` of type `() -> Option (a, Iterator a)`.

The fundamental API (method) of `Iterator` is `advance` function, which just extract the `next` field from an iterator and calls it on `()`:
```
// Get next value and next iterator.
advance : Iterator a -> Option (a, Iterator a);
advance = |iter| (iter.@next)();
```

You can define an iterator that produces infinite sequence of zeros (0, 0, 0, ...) as follows: 
```
zeros : Iterator Int;
zeros = Iterator { next: |_| some $ (0, zeros) };
```

That is, if `advance` is called on `zeros`, it always returns `some` value (because it is an infinite sequence). If the programmer unwraps the `some` value, he obtains `0` as the value and `zeros` again as the next iterator.

```
let iter = zeros;
let (x, iter) = iter.advance.as_some; // x == 0
let (y, iter) = iter.advance.as_some; // y == 0
let (z, iter) = iter.advance.as_some; // z == 0
...
```

Since an iterator only has a function as a data, it consumes only a small memory. If we want to apply a function `f : a -> b` to each element of an array `arr : Array a` producing a new array of type `Array b`, we need to allocate an memory for the resulting array, which may be large. On the other hand, applying `f` to an iterator of `Iterator a` to produce an iterator of type `Iterator b` is faster and only needs small memory allocation, because any element of an iterator is not calculated until `advance` will be called. This operation is provided as `map` method of `Iterator`:

- `map : (a -> b) -> Iterator a -> Iterator b`

This can be defined as follows:

```
map : (a -> b) -> Iterator a -> Iterator b;
map = |f, iter| (
    let next = |_| (
        let adv = iter.advance;
        if adv.is_none { none() };
        let (val, iter_next) = adv.as_some;
        some $ (f(val), iter_next.map(f))
    );
    Iterator { next: next }
);
```

Going back to the Fibonacci program, there are more two functions related to `Iterator` used:

- `from_array : Array a -> Iterator a`: converts an array into an iterator.
- `join : String -> Iterator String -> String`: concatenates strings in an iterator separated by a specified string. NOTE: this is defined in `Std::String` namespace, not in `Std::Iterator`.

For example, `Iterator::from_array(["Hello", "World!"]).join(" ") == "Hello World!"`.

In the last, `to_string : Int -> String` is a function that converts an integer to a decimal string.

## Boxed type and reference counting

In the last of this tutorial, I explain the meaning of the exclamation mark of `set!` or `println!` function.

In Fix, all types are boxed or unboxed. Roughly speaking, types which may contain much data are boxed. For example, `Array` or `String` is boxed. Structs are boxed by defalut, because there may be many fields. On the other hand, `Int`, `Bool` or tuples are unboxed.

What is the difference between boxed type and unboxed type? In your program, you often give a "name" to an existing value. For example, in the following program,

```
let x = 42;
let y = x;
```

you make a value `42`, name it as `x`, and again name it as `y`. If you define a function 

```
multiply : Int -> Int -> Int;
multiply = |x, y| x*y;
```

and write `multiply(3, 5)`, then two integers `3` and `5` are named as `x` and `y`, and passed to `multiply` function. In the next example,

```
type Price = struct { value: Int };
...
let price_of_book = Price { value: 100 };
```

the value `100` is given a name "`value` of `price_of_book`". So, the "name" of a value in this explanation is not only a variable name, but should be understood as a way to reach the value.

The difference between boxed and unboxed types is the behavior when its value is named. For unboxed types, the value is simply cloned when a new name is created, and the new name refers to the new cloned value. In other words, all values of unboxed type has a unique name. 

On the other hand, a value of boxed type is not cloned, and therefore there may be many names that refer to a value. For example, consider

```
let x = Array::fill(100, 42);
let y = x;
```

In the above example, first an `Int` array of length `100` is created, and a name `x` is assigned to it. In the next line, a second name `y` to the same array value is created without cloning the array value (i.e., one-hundred integers). This is good because cloning a large array is waste of time and memory. 

As is the case with all languages, Fix stores all values on memory (or register). Since memory space is a limited resource of a computer, Fix should release a memory region for a value if it will not be used later. Then, how can Fix judge that a value will no longer be used?

For unboxed types, the answer is simple: when THE name of a value disappears, Fix should release its memory region. Every local name introduced by `let` or function argument has a limited life. When the name of a value ends it's life, the value is no longer needed.

For boxed types, the strategy Fix uses is called referencing counting. Since a value of boxed type may have multiple names, Fix is counting the number of names of a boxed value. For each boxed value an integer called "reference counter" is associated. When a name of a value is created, Fix increments the reference counter. When a name disappears, Fix decrements the reference counter. If refernce counter reached to zero, Fix releases the memory region of that value. 

Managing reference counter (i.e., incrementing, decrementing and checking if the counter is zero) has no small negative impact on the performance of a program. This is one reson that I didn't make all values boxed. Since cloning cost of `Int` or `Bool` is so low, they are suited to be unboxed.

Summary upto here:
- Types in Fix are classified into two kinds: boxed and unboxed.
- Unboxed value has unique name, and Fix simply releases the memory region for a boxed value when it's name disappears.
- Boxed value may have multiple names, so Fix is counting the number of names using reference counting method.

(TBA)

# Other topics on syntax

## Module and imports 

In Fix, values, functions, types and traits defined in a source file is collected to a module. Each source file has to declare the name of the module it defines by `module {module_name};`. The first letter of the module name must be capitalized.

As in other languages, a single program can be constructed from multiple source files. As an example, consider a program consists of two source files:

`lib.fix`:
```
module Lib;

module_name : String;
module_name = "Lib";
```

`main.fix`:
```
module Main;

import lib.fix;

module_name : String;
module_name = "Main";

main : IOState -> ((), IOState);
main = (
    println! $ "This program consists of two modules, `" + Lib::module_name + "` and `" + Main::module_name + "`."
);
```

If you put these two files in a same directory and execute `fix run main.fix`, it prints: 

```
This program consists of two modules, `Lib` and `Main`.
```

Note that here two strings named `module_name` are defined and you can use these strings separately by writing `{module_name}::module_name`. Like this, module name is used as the top-level namespace of values, types and traits defined in a source file.

You can import modules defined in other source files by writing `import {path_to_source_file};`. If `{path_to_source_file}` starts by `./` or `../`, then it is treated as a relative path to the source file in which the import statement is written. In other cases, `{path_to_source_file}` is treated as a relative path to the root source file, that is, the file passed to the `fix run` or `fix build` command.

## Recursion

You can make recursive global function as usual.

```
module Main;

fib : Int -> Int;
fib = |n| (
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
);

main : IOState -> ((), IOState);
main = print! $ fib(30).to_string; // 832040
```

On the other hand, Fix's `let`-binding doesn't allow to make recursive definition. To define a recursive function locally, use `fix` built-in function.

## Overloading

(TBA)

## Traits

(TBA)

## Type annotation

(TBA)

## Boxed and unboxed types

Types in Fix are divided into boxed types and unboxed types. Boxed types and unboxed types are similar to things called as "reference types" and "value types" in other languages, respectively.

* Value of boxed types are allocated in heap memory. Local names and struct / union fields whose types are boxed are compiled as pointers to the values. 
* Values of unboxed types are directly embedded into the stack memory, structs and unions. 

In general, types that contain a lot of data (such as `Array`) are suited to be boxed because boxed types have lower copying costs. On the other hand, types containing small data (such as `Int`) can be unboxed to reduce the cost of increasing or decreasing the reference counter.

### Functions

Functions are unboxed, but captured values are stored to an unnamed boxed struct.

### Tuples

Tuple types are unboxed, because tuple is intended to have only a few fields. If you want to use many fields, you should define a new struct.
Tuples are special forms of structs whose field names are `0`, `1`, `2`, etc. 

### Unit

The unit type `()` is unboxed.

### Array

`Std::Array` is a boxed type.

### Structs

Structs are boxed by default because they are assumed to have many fields. To define unboxed struct type, write `unbox` specifier before `struct`.

Example:
```
type Product = unbox struct { price: Int, sold: Bool };
```

### Unions

Unions are unboxed by default because they only contains a single value at a time. To define boxed union type, write `box` specifier before `struct`.

```
type Weight = box union (pound: Int, kilograms: Int);
```

### Type parameters

## Traits

### Trait bound

## Higher-kinded types

# Built-in / library features

## Types

### Structs

If you define a struct named `{struct}` with a field `{field_name}` of type `{field_type}`, the following methods are defined in the namespace named `{struct}`.

- `@{field_name} : {struct} -> {field_type}`
    - Extract the value of a field from a struct value.
- `={field_name} : {field_type} -> {struct} -> {struct}`
    - Modify a struct value by setting a field.
    - This function clones the struct value if it is shared between multiple references.
- `={field_name}! : {field_type} -> {struct} -> {struct}`
    - Modify a struct value by setting a field.
    - This function always updates the struct value. If the struct value is shared between multiple references, this function panics.
- `mod_{field_name} : ({field_type} -> {field_type}) -> {struct} -> {struct}`
    - Modify a struct value by a function acting on a field.
    - This function clones the struct value if it is shared between multiple references.
- `mod_{field_name}! : ({field_type} -> {field_type}) -> {struct} -> {struct}`
    - Modify a struct value by a function acting on a field.
    - This function always updates the struct value. If the struct value is shared between multiple references, this function panics.

NOTE: In a future, we will add lens functions such as `act_{field_name} : [f: Functor] ({field_type} -> f {field_type}) -> {struct} -> f {struct} `, which are generalization of `mod` functions.

### Unions

If you define a union named `{union}` with a variant `{variant_name}` of type `{variant_type}`, the following methods are defined in the namespace named `{union}`.

- `{variant_name} : {variant_type} -> {union}`
    - Constructs a union value from a variant value.
- `is_{variant_name} : {union} -> Bool`
    - Check if a union value is created as the specified variant.
- `as_{variant_name} : {union} -> {variant_type}`
    - Converts a union value into a variant value if it is created as the variant. If not so, this function panics.

### Std::Array

`Std::Array` is the type of variable-length arrays.

Methods:

- `__unsafe_set_length : Int -> Array a -> Array a`
    - Updates the length of an array, without uniqueness checking or validation of the given length value.
- `__unsafe_get : Int -> Array a -> a`
    - Gets a value from an array, without bounds checking and retaining the returned value.
- `__unsafe_set : Int -> a -> Array a -> Array a`
    - Sets a value into an array, without uniqueness checking, bounds checking and releasing the old value.
- `append : Array a -> Array a -> Array a`
    - Append an array to an array.
    - Note: Since `a1.append(a2)` puts `a2` after `a1`, `append(lhs, rhs)` puts `lhs` after `rhs`.    
- `fill : Int -> a -> Array a`
    - Creates an array filled with the initial value.
    - The capacity is set to the same value as the length.
    - `fill(n, x) == [x, x, x, ..., x]` (of length `n`).
- `force_unique : Array a -> Array a`
    - Force the uniqueness of an array.
    - If the given array is shared, this function returns the cloned array.
- `force_unique! : Array a -> Array a`
    - Force the uniqueness of an array.
    - If the given array is shared, this function panics.
- `from_map : Int -> (Int -> a) -> Array a`
    - Creates an array by a mapping function.
    - `from_map(n, f) = [f(0), f(1), f(2), ..., f(n-1)]`.
- `get : Int -> Array a -> a`
    - Returns an element of an array at an index.
- `get_length : Array a -> Int`
    - Returns the length of an array.
- `get_capacity : Array a -> Int`
    - Returns the capacity of an array.
- `make_empty : Int -> Array a`
    - Creates an empty array with specified capacity.
- `mod : Int -> (a -> a) -> Array a -> Array a`
    - Modifies a value of an element at the specified index of an array by a function.
    - This function clones the array if it is shared between multiple references.
- `mod! : Int -> (a -> a) -> Array a -> Array a`
    - This function clones the array if it is shared between multiple references.
    - This function always update the array. If the array is shared between multiple references, this function panics.  
- `pop_back : Array a -> Array a`
    - Pop an element at the back of an array.
    - If the array is empty, this function does nothing.
- `push_back : a -> Array a -> Array a`
    - Push an element to the back of an array.
- `reduce_length : Int -> Array a -> Array a`
    - Reduce the length of an array.
- `reserve : Int -> Array a -> Array a`
    - Reserves the memory region for an array.
- `set : Int -> a -> Array a -> Array a`
    - Updates a value of an element at an index of an array.
    - This function clones the given array if it is shared between multiple references.
- `set! : Int -> a -> Array a -> Array a`
    - Updates a value of an element at an index of an array.
    - This function always update the given array. If the given array is shared between multiple references, this function panics.
- `sort_by : ((a, a) -> Bool) -> Array a -> Array a`
    - Sort elements in an array by "less than" comparator.
- `_sort_range_by_using_buffer : Array a -> Int -> Int -> ((a, a) -> Bool) -> Array a -> (Array a, Array a)`
    - Sort elements in a range of an array by "less than" comparator.
    - This function receives a working buffer as the first argument to reduce memory allocation, and returns it as second element.

You can create array by the array literal syntax `[a0, a1, ..., an]`.

NOTE: In a future, we will add lens functions such as `act : [f: Functor] Int -> (a -> f a) -> Array a -> f (Array a)`, which are generalization of `mod` functions.

Implementing Traits:

- `[a : Eq] Array a : Eq`

### Std::Bool

`Std::Bool` is the type of boolean values, represented by 8-bit integer `1` (`true`) and `0` (`false`). 

### Std::Byte

`Std::Byte` is the type of 8-bit unsigned integers.

### Std::IOState

The virtual type that represents the state of world (=the outside of the Fix program). 

For example, `Std::IOState.print!(msg) : Std::IOState -> ((), Std::IOState)` function can be considered that it changes the state of the world by printing the message to the display. So it should receive `Std::IOState` and return the updated `Std::IOState` value paired with the result of the action (in this case, it is `()`, because printing message returns no result).

All functions that perform I/O action by `IOState` assert that the given state is unique.

Methods:

- `pure : () -> IOState -> ((), IOState)`
    - Makes a "do nothing" I/O action.
- `print! : String -> IOState -> ((), IOState)`
    - Prints a string to standard output.
- `println! : String -> IOState -> ((), IOState)`
    - Prints a string and a newline to standard output.

### Std::Int

`Std::Int` is the type of 64-bit signed integers.

Methods:

- `Std::Int._int_to_string : Int -> String`
    - Convert an integer to a decimal number string.
    - Implementation of trait method `Std::ToString.to_string`.

Implementing traits:

- `Std::ToString`

### Std::Iterator

Iterators (a.k.a. lazy lists) are generators of sequenced values.

Methods:

- `advance : Iterator a -> Option (a, Iterator a)`
    - Get next value and next iterator.
- `append : Iterator a -> Iterator a -> Iterator a`
    - Append an iterator to a iterator.
    - Note: Since `iter1.append(iter2)` puts `iter2` after `iter1`, `append(lhs, rhs)` puts `lhs` after `rhs`.    
- `count_up : Int -> Iterator Int`
    - Create an iterator that counts up from a number.
    - `count_up(n) = [n, n+1, n+2, ...]` (continues infinitely)
- `get_length : Iterator a -> Int`
    - Counts the length of an iterator.
- `intersperse : a -> Iterator a -> Iterator a`
    - Intersperse an elemnt between elements of an iterator.
    - Example: `Iterator::from_array([1,2,3]).intersperse(0) == Iterator::from_array([1,0,2,0,3])`
- `make_empty : Iterator a`
    - Creates an empty iterator.
- `filter : (a -> Bool) -> Iterator a -> Iterator a`
    - Filter elements by a condition function.
- `flatten : Iterator (Iterator a) -> Iterator a`
    - Flatten an iterator of iterators.
- `fold : b -> (b -> a -> b) -> Iterator a -> b`
    - Folds iterator from left.
    - `fold(init, op, [a0, a1, a2, ...]) = ...op(op(op(init, a0), a1), a2)...`
- `from_array : Array a -> Iterator a`
    - Create iterator from an array.
- `from_map : (Int -> a) -> Iterator a`
    - Create iterator from mapping function.
    - `from_map(f) = [f(0), f(1), f(2), ...]`
- `map : map : (a -> b) -> Iterator a -> Iterator b`
    - Apply a function to each value of iterator.
    - `map(f, [a0, a1, a2, ...]) = [f(a0), f(a1), f(a2), ...]`
- `push_front : a -> Iterator a -> Iterator a`
    - Append an element to an iterator.
- `reverse : Iterator a -> Iterator a`
    - Reverse an iterator.
- `take : Int -> Iterator a -> Iterator a`
    - Take at most n elements from an iterator.
- `zip : Iterator a -> Iterator b -> Iterator (a, b)`
    - Zip two iterators.

Implementing Traits:

- `Iterator a : Add`
    - Adds two iterators by `Iterator::append`.
- `[a : Eq] Iterator a : Eq`

### Std::Option

`Option a` contains a value of type `a`, or contains nothing.

```
type Option a = union { none: (), some: a };
```

Methods:

- `map : (a -> b) -> Option a -> Option b`
    - Apply a function to the contained value. If the option is `none()`, do nothing.

### Std::String

The type of strings.

Methods:

- `concat : String -> String -> String`
    - Concatenate two strings.
    - Note: Since `s1.concat(s2)` puts `s2` after `s1`, `concat(lhs, rhs)` puts `lhs` after `rhs`.
- `join : String -> Iterator String -> String`
    - Join strings by a separator.
    - Example: `Iterator::from_array(["a", "b", "c"]).join(", ") == "a, b, c"`
- `concat_iter : Iterator String -> String`
    - Concatenate an iterator of strings.
- `get_length : String -> Int`
    - Returns the length of the string.

Implementing Traits:

- `String : Add`
    - Add two strings by `String.concat`.
- `String : Eq`

## Functions

### Std::fix : ((a -> b) -> a -> b) -> a -> b

`fix` enables you to make a recursive function locally. The idiom is: `fix $ |loop, var| -> (expression calls loop)`.

```
module Main;

main : IOState -> ((), IOState);
main = (
    let fact = fix $ |loop, n| if n == 0 then 1 else n * loop (n-1);
    print! $ fact(5).to_string // evaluates to 5 * 4 * 3 * 2 * 1 = 120
);
```

### Std::loop : s -> (s -> LoopResult s r) -> r

`loop` enables you to make a loop. `LoopResult` is a union type defined as follows: 

```
type LoopResult s r = union (s: continue, r: break);
```

`loop` takes two arguments: the initial state of the loop `s0` and the loop body function `body`. It first calls `body` on `s0`. If `body` returns `break r`, then the loop ends and returns `r` as the result. If `body` returns `continue s`, then the loop calls again `body` on `s`.

```
module Main;
    
main : IOState -> ((), IOState);
main = (
    let sum = (
        loop((0, 0), |(i, sum)|
            if i == 100 then 
                break $ sum 
            else
                continue $ (i+1, sum+i)
        )
    );
    print! $ sum.to_string
); // evaluates to 0 + 1 + ... + 99 
```

### Std::Debug.debug_print : String -> ()

### Std::Debug.debug_println : String -> ()

### Std::Debug.abort : () -> a

### Std::Debug.assert : String -> Bool -> ()

### Std::Debug.assert_eq : [a: Eq] String -> a -> a -> ()

## Traits

### Std::ToString

- `to_string : [a: ToString] a -> String`

## Operators

The following is the table of operators sorted by it's precedence (operator of higher precedence appears earlier).

| Operator       | Associativity | Trait / method                     | Explanation                                                 | 
| -------------- | ------------- | ---------------------------------- | ----------------------------------------------------------- | 
| f(x)           | left          | -                                  | function application                                        | 
| .              | left          | -                                  | right-to-left function application: x.f = f(x)              | 
| - (minus sign) | -             | Std::Neg / neg                      | negative of number                                          | 
| !              | -             | Std::Not / not                      | logical NOT                                                 | 
| *              | left          | Std::Mul / mul                      | multiplication of numbers                                   | 
| /              | left          | Std::Div / div                      | division of numbers                                         | 
| %              | left          | Std::Rem / rem                      | reminder of division                                        | 
| +              | left          | Std::Add / add                      | addition of numbers                                         | 
| - (minus sign) | left          | Std::Sub / sub                      | subtraction of numbers                                      | 
| ==             | left          | Std::Eq / eq                        | equality comparison                                         | 
| !=             | left          | -                                  | `x != y` is interpreted as `!(x == y)`                      | 
| <=             | left          | Std::LessThanOrEq / less_than_or_eq | less-than-or-equal-to comparison                            | 
| >=             | left          | -                                  | `x >= y` is interpreted as `y <= x`                         | 
| <              | left          | Std::LessThan / less_than           | less-than comparison                                        | 
| >              | left          | -                                  | `x > y` is interpreted as `y < x`                           | 
| &&             | left          | Std::And / and                      | logical AND                                                 | 
| &#124;&#124;   | left          | Std::Or / or                        | logical OR                                                  | 
| $              | right         | -                                  | right associative function application: f $ g $ x = f(g(x)) | 

# Features of "fix" command