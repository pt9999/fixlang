# module `Std`

## namespace `Std`

### trait `a : Add`

#### method `add : a -> a -> a`

Addition.

An expression `x + y` is translated to `add(x, y)`.

### type `Array a = box { primitive }`

## namespace `Std::Array`

### value `@ : Std::I64 -> Std::Array a -> a`

Gets an element of an array at the specified index.

### value `_get_ptr : Std::Array a -> Std::Ptr`

Get the pointer to the memory region where elements are stored.

This function is dangerous because if the array is not used after call of this function, the array will be deallocated soon and the returned pointer will be dangling.
Try using `borrow_ptr` instead.

### value `_get_sub_size_asif : I64 -> I64 -> I64 -> I64 -> Array a -> Array a`

A function like `get_sub`, but behaves as if the size of the array is the specified value,
and has a parameter to specify additional capacity of the returned `Array`.

### value `_sort_range_using_buffer : Array a -> I64 -> I64 -> ((a, a) -> Bool) -> Array a -> (Array a, Array a)`

Sort elements in a range of a vector by "less than" comparator.
This function receives a working buffer as the first argument to reduce memory allocation, and returns it as second element.

### value `_unsafe_get : Std::I64 -> Std::Array a -> a`

Gets a value from an array, without bounds checking and retaining the returned value.

### value `_unsafe_set : Std::I64 -> a -> Std::Array a -> Std::Array a`

Sets a value into an array, without uniqueness checking, bounds checking and releasing the old value.

### value `_unsafe_set_size : Std::I64 -> Std::Array a -> Std::Array a`

Updates the length of an array, without uniqueness checking or validation of the given length value.

### value `act : [f : Functor] I64 -> (a -> f a) -> Array a -> f (Array a)`

Modifies an array by a functorial action.

Semantically, `arr.act(idx, fun)` is equivalent to `fun(arr.@(idx)).map(|elm| arr.set(idx, elm))`.

This function can be defined for any functor `f` in general, but it is easier to understand the behavior when `f` is a monad:
the monadic action `act(idx, fun, arr)` first performs `fun(arr.@(idx))` to get a value `elm`, and returns a pure value `arr.set(idx, elm)`.

If you call `arr.act(idx, fun)` when both of `arr` and `arr.@(idx)` are unique, it is assured that `fun` receives the unique value.

If you call `act` on an array which is shared, this function clones the given array when inserting the result of your action into the array.
This means that you don't need to pay cloning cost when your action failed, as expected.

### value `append : Array a -> Array a -> Array a`

Appends an array to an array.

Note: Since `a1.append(a2)` puts `a2` after `a1`, `append(lhs, rhs)` puts `lhs` after `rhs`.

### value `borrow_ptr : (Ptr -> b) -> Array a -> b`

Call a function with a pointer to the memory region where elements are stored.

### value `empty : Std::I64 -> Std::Array a`

Creates an empty array with specified capacity.

### value `fill : Std::I64 -> a -> Std::Array a`

Creates an array of the specified length filled with the initial value.

The capacity is set to the same value as the length.

Example: `fill(n, x) == [x, x, x, ..., x]` (of length `n`).

### value `find_by : (a -> Bool) -> Array a -> Option I64`

Find the first index at which the element satisfies a condition.

### value `force_unique : Std::Array a -> Std::Array a`

Force the uniqueness of an array.
If the given array is shared, this function returns the cloned array.

### value `from_iter : Iterator a -> Array a`

Create an array from an iterator.

### value `from_map : I64 -> (I64 -> a) -> Array a`

Creates an array by a mapping function.

### value `get_capacity : Std::Array a -> Std::I64`

Gets the capacity of an array.

### value `get_first : Array a -> Option a`

Get the first element of an array. Returns none if the array is empty.

### value `get_last : Array a -> Option a`

Get the last element of an array. Returns none if the array is empty.

### value `get_size : Std::Array a -> Std::I64`

Gets the length of an array.

### value `get_sub : I64 -> I64 -> Array a -> Array a`

`arr.get_sub(s, e)` returns an array `[ arr.@(i) | i ∈ [s, e) ]`,
More precisely, let `N` denote the the size of the `arr`.
Then `arr.get_sub(s, e)` returns `[ arr.@(s + i mod N) | i ∈ [0, n), n >= 0 is the minimum number such that s + n == e mod N ]`.

### value `is_empty : Array a -> Bool`

Returns if the array is empty

### value `mod : Std::I64 -> (a -> a) -> Std::Array a -> Std::Array a`

Updates an array by applying a function to the element at the specified index.

This function clones the given array if it is shared.

If you call `arr.mod(i, f)` when both of `arr` and `arr.@(i)` are unique, it is assured that `f` receives the element value which is unique.

### value `pop_back : Array a -> Array a`

Pop an element at the back of an array.
If the array is empty, this function does nothing.

### value `push_back : a -> Array a -> Array a`

Push an element to the back of an array.

### value `reserve : I64 -> Array a -> Array a`

Reserves the memory region for an array.
TODO: change to more optimized implementation.

### value `set : Std::I64 -> a -> Std::Array a -> Std::Array a`

Updates an array by setting a value as the element at the specified index.

This function clones the given array if it is shared.

### value `sort_by : ((a, a) -> Bool) -> Array a -> Array a`

Sort elements in a vector by "less than" comparator.

### value `to_iter : Array a -> Iterator a`

Convert an array to an iterator.

### value `truncate : I64 -> Array a -> Array a`

Truncate an array, keeping the given number of first elements.
`truncante(len, arr)` does nothing if `len >= arr.get_size`.

## namespace `Std`

### type `Bool = unbox { primitive }`

### type `Boxed a = box struct { ...fields... }`

Boxed wrapper for a type.

```
type Boxed a = box struct { value : a };
```

#### field `value : a`

### trait `a : Div`

#### method `div : a -> a -> a`

Division.

An expression `x / y` is translated to `div(x, y)`.

### trait `a : Eq`

#### method `eq : a -> a -> Bool`

Equality comparison.

An expression `x == y` is translated to `eq(x, y)`.

### type `ErrMsg = String`

A type (alias) for error message.

### type `F32 = unbox { primitive }`

## namespace `Std::F32`

### value `abs : F32 -> F32`

### value `infinity : Std::F32`

The infinity value for the given floating point type.

### value `quiet_nan : Std::F32`

A floating number represented by `01...1` in binary.

### value `to_CChar : Std::F32 -> Std::I8`

Casts a value of `F32` into a value of `CChar`.

### value `to_CDouble : Std::F32 -> Std::F64`

Casts a value of `F32` into a value of `CDouble`.

### value `to_CFloat : Std::F32 -> Std::F32`

Casts a value of `F32` into a value of `CFloat`.

### value `to_CInt : Std::F32 -> Std::I32`

Casts a value of `F32` into a value of `CInt`.

### value `to_CLong : Std::F32 -> Std::I64`

Casts a value of `F32` into a value of `CLong`.

### value `to_CLongLong : Std::F32 -> Std::I64`

Casts a value of `F32` into a value of `CLongLong`.

### value `to_CShort : Std::F32 -> Std::I16`

Casts a value of `F32` into a value of `CShort`.

### value `to_CSizeT : Std::F32 -> Std::U64`

Casts a value of `F32` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::F32 -> Std::U8`

Casts a value of `F32` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::F32 -> Std::U32`

Casts a value of `F32` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::F32 -> Std::U64`

Casts a value of `F32` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::F32 -> Std::U64`

Casts a value of `F32` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::F32 -> Std::U16`

Casts a value of `F32` into a value of `CUnsignedShort`.

### value `to_F32 : Std::F32 -> Std::F32`

Casts a value of `F32` into a value of `F32`.

### value `to_F64 : Std::F32 -> Std::F64`

Casts a value of `F32` into a value of `F64`.

### value `to_I16 : Std::F32 -> Std::I16`

Casts a value of `F32` into a value of `I16`.

### value `to_I32 : Std::F32 -> Std::I32`

Casts a value of `F32` into a value of `I32`.

### value `to_I64 : Std::F32 -> Std::I64`

Casts a value of `F32` into a value of `I64`.

### value `to_I8 : Std::F32 -> Std::I8`

Casts a value of `F32` into a value of `I8`.

### value `to_U16 : Std::F32 -> Std::U16`

Casts a value of `F32` into a value of `U16`.

### value `to_U32 : Std::F32 -> Std::U32`

Casts a value of `F32` into a value of `U32`.

### value `to_U64 : Std::F32 -> Std::U64`

Casts a value of `F32` into a value of `U64`.

### value `to_U8 : Std::F32 -> Std::U8`

Casts a value of `F32` into a value of `U8`.

### value `to_string_exp : F32 -> String`

Convert a floating number to a string of exponential form.

### value `to_string_exp_precision : U8 -> F32 -> String`

Convert a floating number to a string of exponential form with specified precision (i.e., number of digits after the decimal point).

### value `to_string_precision : U8 -> F32 -> String`

Convert a floating number to a string with specified precision (i.e., number of digits after the decimal point).

## namespace `Std`

### type `F64 = unbox { primitive }`

## namespace `Std::F64`

### value `abs : F64 -> F64`

### value `infinity : Std::F64`

The infinity value for the given floating point type.

### value `quiet_nan : Std::F64`

A floating number represented by `01...1` in binary.

### value `to_CChar : Std::F64 -> Std::I8`

Casts a value of `F64` into a value of `CChar`.

### value `to_CDouble : Std::F64 -> Std::F64`

Casts a value of `F64` into a value of `CDouble`.

### value `to_CFloat : Std::F64 -> Std::F32`

Casts a value of `F64` into a value of `CFloat`.

### value `to_CInt : Std::F64 -> Std::I32`

Casts a value of `F64` into a value of `CInt`.

### value `to_CLong : Std::F64 -> Std::I64`

Casts a value of `F64` into a value of `CLong`.

### value `to_CLongLong : Std::F64 -> Std::I64`

Casts a value of `F64` into a value of `CLongLong`.

### value `to_CShort : Std::F64 -> Std::I16`

Casts a value of `F64` into a value of `CShort`.

### value `to_CSizeT : Std::F64 -> Std::U64`

Casts a value of `F64` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::F64 -> Std::U8`

Casts a value of `F64` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::F64 -> Std::U32`

Casts a value of `F64` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::F64 -> Std::U64`

Casts a value of `F64` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::F64 -> Std::U64`

Casts a value of `F64` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::F64 -> Std::U16`

Casts a value of `F64` into a value of `CUnsignedShort`.

### value `to_F32 : Std::F64 -> Std::F32`

Casts a value of `F64` into a value of `F32`.

### value `to_F64 : Std::F64 -> Std::F64`

Casts a value of `F64` into a value of `F64`.

### value `to_I16 : Std::F64 -> Std::I16`

Casts a value of `F64` into a value of `I16`.

### value `to_I32 : Std::F64 -> Std::I32`

Casts a value of `F64` into a value of `I32`.

### value `to_I64 : Std::F64 -> Std::I64`

Casts a value of `F64` into a value of `I64`.

### value `to_I8 : Std::F64 -> Std::I8`

Casts a value of `F64` into a value of `I8`.

### value `to_U16 : Std::F64 -> Std::U16`

Casts a value of `F64` into a value of `U16`.

### value `to_U32 : Std::F64 -> Std::U32`

Casts a value of `F64` into a value of `U32`.

### value `to_U64 : Std::F64 -> Std::U64`

Casts a value of `F64` into a value of `U64`.

### value `to_U8 : Std::F64 -> Std::U8`

Casts a value of `F64` into a value of `U8`.

### value `to_string_exp : F64 -> String`

Convert a floating number to a string of exponential form.

### value `to_string_exp_precision : U8 -> F64 -> String`

Convert a floating number to a string of exponential form with specified precision (i.e., number of digits after the decimal point).

### value `to_string_precision : U8 -> F64 -> String`

Convert a floating number to a string with specified precision (i.e., number of digits after the decimal point).

## namespace `Std::FFI`

### type `CChar = Std::I8`

### type `CDouble = Std::F64`

### type `CFloat = Std::F32`

### type `CInt = Std::I32`

### type `CLong = Std::I64`

### type `CLongLong = Std::I64`

### type `CShort = Std::I16`

### type `CSizeT = Std::U64`

### type `CUnsignedChar = Std::U8`

### type `CUnsignedInt = Std::U32`

### type `CUnsignedLong = Std::U64`

### type `CUnsignedLongLong = Std::U64`

### type `CUnsignedShort = Std::U16`

### type `Destructor a = box struct { ...fields... }`

`Destructor a` is a boxed type which is containing a value of type `a` and a function `a -> ()` which is called destructor.
When a value of `Destructor a` is deallocated, the destructor function will be called on the contained value.
This type is useful to free a resouce allocated by a C function automatically when the resource is no longer needed in Fix code.

NOTE1: Accessing the contained value directly by the field accessor function is not recommended. Use `borrow` function to access the value.
NOTE2: If the contained value is captured by another Fix's object than `Destructor`, the contained value is still alive after the destructor function is called.

#### field `_value : a`

#### field `dtor : a -> ()`

## namespace `Std::FFI::Destructor`

### value `borrow : (a -> b) -> Destructor a -> b`

Borrow the contained value.
`borrow(worker, dtor)` calls `worker` on the contained value captured by `dtor`, and returns the value returned by `worker`.
It is guaranteed that the `dtor` is alive during the call of `worker`.
In other words, the `worker` receives the contained value on which the destructor is not called yet.

### value `make : a -> (a -> ()) -> Destructor a`

Make a destructor value.

## namespace `Std::FFI`

### value `_unsafe_get_boxed_data_ptr : a -> Std::Ptr`

Returns a pointer to the data of a boxed value.

The difference from `unsafe_get_retained_ptr_of_boxed_value` is that this function returns a pointer to region where the payload of a boxed value is stored;
on the other hand, `unsafe_get_retained_ptr_of_boxed_value` returns a pointer to the boxed value itself (i.e., the control block of the value).

Note that if the call `v._unsafe_get_boxed_data_ptr` is the last usage of `v`, then this function deallocates `v` and returns a dangling pointer.
To avoid issues caused by this, use `unsafe_borrow_boxed_data_ptr` instead.

### value `unsafe_borrow_boxed_data_ptr : (Ptr -> b) -> a -> b`

Borrows a pointer to the data of a boxed value.
For more details, see the document of `_unsafe_get_boxed_data_ptr`.

### value `unsafe_clear_errno : () -> ()`

Set errno to zero.

### value `unsafe_get_boxed_value_from_retained_ptr : Std::Ptr -> a`

Creates a boxed value from a retained pointer obtained by `unsafe_get_retained_ptr_of_boxed_value`.

### value `unsafe_get_errno : () -> CInt`

Get errno which is set by C functions.

### value `unsafe_get_release_function_of_boxed_value : Std::Lazy a -> Std::Ptr`

Returns a pointer to the function of type `void (*)(void*)` which releases a boxed value of type `a`.
This function is used to release a pointer obtained by `_unsafe_get_retained_ptr_of_boxed_value`.

Note that this function is requires a value of type `Lazy a`, not of `a`.
So you can get release function for a boxed type `T` even when you don't have a value of type `T` -- you can just use `|_| undefined() : T`:

```
module Main;

type VoidType = box struct {};
// No constructor for `VoidType` is provided.

main: IO ();
main = (
    let release = (|_| undefined() : VoidType).unsafe_get_release_function_of_boxed_value; // Release function of `VoidType`.
    pure()
);
```

In case the type is not a specific `T`, but a generic parameter `a` that appears in the type signature of a function you are implementing, you cannot use the above technique, because writing `|_| undefined() : a` is not allowed in Fix's syntax. Even in such a case, if you have some value related to `a`, you can make a `Lazy a` value in many cases. For example:
- If you have a function `f : b -> a`, then you can use `|_| f(undefined())` of type `Lazy a`. 
- If you have a function `f : a -> b`, then you can use `|_| let x = undefined(); let _ = f(x); x` of type `Lazy a`.

### value `unsafe_get_retain_function_of_boxed_value : Std::Lazy a -> Std::Ptr`

Returns a pointer to the function of type `void (*)(void*)` which retains a boxed value of type `a`.
This function is used to retain a pointer obtained by `_unsafe_get_retained_ptr_of_boxed_value`.

For the reason that this function requires a value of type `Lazy a`, not of `a`, see the document for `unsafe_get_release_function_of_boxed_value`.

### value `unsafe_get_retained_ptr_of_boxed_value : a -> Std::Ptr`

Returns a retained pointer to a boxed value.
This function is used to share ownership of Fix's boxed values with foreign languages.

To get back the boxed value from the retained pointer, use `unsafe_get_boxed_value_from_retained_ptr`.
To release / retain the value in a foreign language, call the function pointer obtained by `unsafe_get_release_function_of_boxed_value` or `unsafe_get_retain_function_of_boxed_value` on the pointer.

Note that the returned pointer points to the control block allocated by Fix, and does not necessary points to the data of the boxed value.
If you want to get a pointer to the data of the boxed value, use `unsafe_borrow_boxed_data_ptr`.

## namespace `Std`

### trait `a : FromBytes`

#### method `from_bytes : Array U8 -> Result ErrMsg a`

### trait `a : FromString`

#### method `from_string : String -> Result ErrMsg a`

### trait `[f : *->*] f : Functor`

#### method `map : (a -> b) -> f a -> f b`

## namespace `Std::Functor`

### value `forget : [f : Functor] f a -> f ()`

## namespace `Std`

### type `I16 = unbox { primitive }`

## namespace `Std::I16`

### value `abs : I16 -> I16`

### value `bit_and : Std::I16 -> Std::I16 -> Std::I16`

Calculates bitwise AND of two values.

### value `bit_or : Std::I16 -> Std::I16 -> Std::I16`

Calculates bitwise OR of two values.

### value `bit_xor : Std::I16 -> Std::I16 -> Std::I16`

Calculates bitwise XOR of two values.

### value `maximum : I16`

### value `minimum : I16`

### value `shift_left : Std::I16 -> Std::I16 -> Std::I16`

`v.shift_left(w)` shifts `v` to left by `w` bits.

### value `shift_right : Std::I16 -> Std::I16 -> Std::I16`

`v.shift_right(w)` shifts `v` to right by `w` bits.

### value `to_CChar : Std::I16 -> Std::I8`

Casts a value of `I16` into a value of `CChar`.

### value `to_CDouble : Std::I16 -> Std::F64`

Casts a value of `I16` into a value of `CDouble`.

### value `to_CFloat : Std::I16 -> Std::F32`

Casts a value of `I16` into a value of `CFloat`.

### value `to_CInt : Std::I16 -> Std::I32`

Casts a value of `I16` into a value of `CInt`.

### value `to_CLong : Std::I16 -> Std::I64`

Casts a value of `I16` into a value of `CLong`.

### value `to_CLongLong : Std::I16 -> Std::I64`

Casts a value of `I16` into a value of `CLongLong`.

### value `to_CShort : Std::I16 -> Std::I16`

Casts a value of `I16` into a value of `CShort`.

### value `to_CSizeT : Std::I16 -> Std::U64`

Casts a value of `I16` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::I16 -> Std::U8`

Casts a value of `I16` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::I16 -> Std::U32`

Casts a value of `I16` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::I16 -> Std::U64`

Casts a value of `I16` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::I16 -> Std::U64`

Casts a value of `I16` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::I16 -> Std::U16`

Casts a value of `I16` into a value of `CUnsignedShort`.

### value `to_F32 : Std::I16 -> Std::F32`

Casts a value of `I16` into a value of `F32`.

### value `to_F64 : Std::I16 -> Std::F64`

Casts a value of `I16` into a value of `F64`.

### value `to_I16 : Std::I16 -> Std::I16`

Casts a value of `I16` into a value of `I16`.

### value `to_I32 : Std::I16 -> Std::I32`

Casts a value of `I16` into a value of `I32`.

### value `to_I64 : Std::I16 -> Std::I64`

Casts a value of `I16` into a value of `I64`.

### value `to_I8 : Std::I16 -> Std::I8`

Casts a value of `I16` into a value of `I8`.

### value `to_U16 : Std::I16 -> Std::U16`

Casts a value of `I16` into a value of `U16`.

### value `to_U32 : Std::I16 -> Std::U32`

Casts a value of `I16` into a value of `U32`.

### value `to_U64 : Std::I16 -> Std::U64`

Casts a value of `I16` into a value of `U64`.

### value `to_U8 : Std::I16 -> Std::U8`

Casts a value of `I16` into a value of `U8`.

## namespace `Std`

### type `I32 = unbox { primitive }`

## namespace `Std::I32`

### value `abs : I32 -> I32`

### value `bit_and : Std::I32 -> Std::I32 -> Std::I32`

Calculates bitwise AND of two values.

### value `bit_or : Std::I32 -> Std::I32 -> Std::I32`

Calculates bitwise OR of two values.

### value `bit_xor : Std::I32 -> Std::I32 -> Std::I32`

Calculates bitwise XOR of two values.

### value `maximum : I32`

### value `minimum : I32`

### value `shift_left : Std::I32 -> Std::I32 -> Std::I32`

`v.shift_left(w)` shifts `v` to left by `w` bits.

### value `shift_right : Std::I32 -> Std::I32 -> Std::I32`

`v.shift_right(w)` shifts `v` to right by `w` bits.

### value `to_CChar : Std::I32 -> Std::I8`

Casts a value of `I32` into a value of `CChar`.

### value `to_CDouble : Std::I32 -> Std::F64`

Casts a value of `I32` into a value of `CDouble`.

### value `to_CFloat : Std::I32 -> Std::F32`

Casts a value of `I32` into a value of `CFloat`.

### value `to_CInt : Std::I32 -> Std::I32`

Casts a value of `I32` into a value of `CInt`.

### value `to_CLong : Std::I32 -> Std::I64`

Casts a value of `I32` into a value of `CLong`.

### value `to_CLongLong : Std::I32 -> Std::I64`

Casts a value of `I32` into a value of `CLongLong`.

### value `to_CShort : Std::I32 -> Std::I16`

Casts a value of `I32` into a value of `CShort`.

### value `to_CSizeT : Std::I32 -> Std::U64`

Casts a value of `I32` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::I32 -> Std::U8`

Casts a value of `I32` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::I32 -> Std::U32`

Casts a value of `I32` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::I32 -> Std::U64`

Casts a value of `I32` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::I32 -> Std::U64`

Casts a value of `I32` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::I32 -> Std::U16`

Casts a value of `I32` into a value of `CUnsignedShort`.

### value `to_F32 : Std::I32 -> Std::F32`

Casts a value of `I32` into a value of `F32`.

### value `to_F64 : Std::I32 -> Std::F64`

Casts a value of `I32` into a value of `F64`.

### value `to_I16 : Std::I32 -> Std::I16`

Casts a value of `I32` into a value of `I16`.

### value `to_I32 : Std::I32 -> Std::I32`

Casts a value of `I32` into a value of `I32`.

### value `to_I64 : Std::I32 -> Std::I64`

Casts a value of `I32` into a value of `I64`.

### value `to_I8 : Std::I32 -> Std::I8`

Casts a value of `I32` into a value of `I8`.

### value `to_U16 : Std::I32 -> Std::U16`

Casts a value of `I32` into a value of `U16`.

### value `to_U32 : Std::I32 -> Std::U32`

Casts a value of `I32` into a value of `U32`.

### value `to_U64 : Std::I32 -> Std::U64`

Casts a value of `I32` into a value of `U64`.

### value `to_U8 : Std::I32 -> Std::U8`

Casts a value of `I32` into a value of `U8`.

## namespace `Std`

### type `I64 = unbox { primitive }`

## namespace `Std::I64`

### value `abs : I64 -> I64`

### value `bit_and : Std::I64 -> Std::I64 -> Std::I64`

Calculates bitwise AND of two values.

### value `bit_or : Std::I64 -> Std::I64 -> Std::I64`

Calculates bitwise OR of two values.

### value `bit_xor : Std::I64 -> Std::I64 -> Std::I64`

Calculates bitwise XOR of two values.

### value `maximum : I64`

### value `minimum : I64`

### value `shift_left : Std::I64 -> Std::I64 -> Std::I64`

`v.shift_left(w)` shifts `v` to left by `w` bits.

### value `shift_right : Std::I64 -> Std::I64 -> Std::I64`

`v.shift_right(w)` shifts `v` to right by `w` bits.

### value `to_CChar : Std::I64 -> Std::I8`

Casts a value of `I64` into a value of `CChar`.

### value `to_CDouble : Std::I64 -> Std::F64`

Casts a value of `I64` into a value of `CDouble`.

### value `to_CFloat : Std::I64 -> Std::F32`

Casts a value of `I64` into a value of `CFloat`.

### value `to_CInt : Std::I64 -> Std::I32`

Casts a value of `I64` into a value of `CInt`.

### value `to_CLong : Std::I64 -> Std::I64`

Casts a value of `I64` into a value of `CLong`.

### value `to_CLongLong : Std::I64 -> Std::I64`

Casts a value of `I64` into a value of `CLongLong`.

### value `to_CShort : Std::I64 -> Std::I16`

Casts a value of `I64` into a value of `CShort`.

### value `to_CSizeT : Std::I64 -> Std::U64`

Casts a value of `I64` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::I64 -> Std::U8`

Casts a value of `I64` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::I64 -> Std::U32`

Casts a value of `I64` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::I64 -> Std::U64`

Casts a value of `I64` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::I64 -> Std::U64`

Casts a value of `I64` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::I64 -> Std::U16`

Casts a value of `I64` into a value of `CUnsignedShort`.

### value `to_F32 : Std::I64 -> Std::F32`

Casts a value of `I64` into a value of `F32`.

### value `to_F64 : Std::I64 -> Std::F64`

Casts a value of `I64` into a value of `F64`.

### value `to_I16 : Std::I64 -> Std::I16`

Casts a value of `I64` into a value of `I16`.

### value `to_I32 : Std::I64 -> Std::I32`

Casts a value of `I64` into a value of `I32`.

### value `to_I64 : Std::I64 -> Std::I64`

Casts a value of `I64` into a value of `I64`.

### value `to_I8 : Std::I64 -> Std::I8`

Casts a value of `I64` into a value of `I8`.

### value `to_U16 : Std::I64 -> Std::U16`

Casts a value of `I64` into a value of `U16`.

### value `to_U32 : Std::I64 -> Std::U32`

Casts a value of `I64` into a value of `U32`.

### value `to_U64 : Std::I64 -> Std::U64`

Casts a value of `I64` into a value of `U64`.

### value `to_U8 : Std::I64 -> Std::U8`

Casts a value of `I64` into a value of `U8`.

## namespace `Std`

### type `I8 = unbox { primitive }`

## namespace `Std::I8`

### value `abs : I8 -> I8`

### value `bit_and : Std::I8 -> Std::I8 -> Std::I8`

Calculates bitwise AND of two values.

### value `bit_or : Std::I8 -> Std::I8 -> Std::I8`

Calculates bitwise OR of two values.

### value `bit_xor : Std::I8 -> Std::I8 -> Std::I8`

Calculates bitwise XOR of two values.

### value `maximum : I8`

### value `minimum : I8`

### value `shift_left : Std::I8 -> Std::I8 -> Std::I8`

`v.shift_left(w)` shifts `v` to left by `w` bits.

### value `shift_right : Std::I8 -> Std::I8 -> Std::I8`

`v.shift_right(w)` shifts `v` to right by `w` bits.

### value `to_CChar : Std::I8 -> Std::I8`

Casts a value of `I8` into a value of `CChar`.

### value `to_CDouble : Std::I8 -> Std::F64`

Casts a value of `I8` into a value of `CDouble`.

### value `to_CFloat : Std::I8 -> Std::F32`

Casts a value of `I8` into a value of `CFloat`.

### value `to_CInt : Std::I8 -> Std::I32`

Casts a value of `I8` into a value of `CInt`.

### value `to_CLong : Std::I8 -> Std::I64`

Casts a value of `I8` into a value of `CLong`.

### value `to_CLongLong : Std::I8 -> Std::I64`

Casts a value of `I8` into a value of `CLongLong`.

### value `to_CShort : Std::I8 -> Std::I16`

Casts a value of `I8` into a value of `CShort`.

### value `to_CSizeT : Std::I8 -> Std::U64`

Casts a value of `I8` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::I8 -> Std::U8`

Casts a value of `I8` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::I8 -> Std::U32`

Casts a value of `I8` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::I8 -> Std::U64`

Casts a value of `I8` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::I8 -> Std::U64`

Casts a value of `I8` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::I8 -> Std::U16`

Casts a value of `I8` into a value of `CUnsignedShort`.

### value `to_F32 : Std::I8 -> Std::F32`

Casts a value of `I8` into a value of `F32`.

### value `to_F64 : Std::I8 -> Std::F64`

Casts a value of `I8` into a value of `F64`.

### value `to_I16 : Std::I8 -> Std::I16`

Casts a value of `I8` into a value of `I16`.

### value `to_I32 : Std::I8 -> Std::I32`

Casts a value of `I8` into a value of `I32`.

### value `to_I64 : Std::I8 -> Std::I64`

Casts a value of `I8` into a value of `I64`.

### value `to_I8 : Std::I8 -> Std::I8`

Casts a value of `I8` into a value of `I8`.

### value `to_U16 : Std::I8 -> Std::U16`

Casts a value of `I8` into a value of `U16`.

### value `to_U32 : Std::I8 -> Std::U32`

Casts a value of `I8` into a value of `U32`.

### value `to_U64 : Std::I8 -> Std::U64`

Casts a value of `I8` into a value of `U64`.

### value `to_U8 : Std::I8 -> Std::U8`

Casts a value of `I8` into a value of `U8`.

## namespace `Std`

### type `IO a = unbox struct { ...fields... }`

#### field `_data : () -> a`

## namespace `Std::IO`

### type `IOFail a = unbox struct { ...fields... }`

The type for I/O actions which may fail.

#### field `_data : IO (Result ErrMsg a)`

## namespace `Std::IO::IOFail`

### value `from_result : Result ErrMsg a -> IOFail a`

Create an pure `IOFail` value from a `Result` value.

### value `lift : IO a -> IOFail a`

Lift an `IO` action to a successful `IOFail` action.

### value `throw : ErrMsg -> IOFail a`

Create an error `IOFail` action.

### value `to_result : IOFail a -> IO (Result ErrMsg a)`

Convert an `IOFail` to an `Result` value (wrapped by `IO`).

### value `try : (ErrMsg -> IO a) -> IOFail a -> IO a`

Convert an `IOFail` value to an `IO` value by an error handler (i.e., a `catch`) function.

## namespace `Std::IO`

### type `IOHandle = unbox struct { ...fields... }`

A handle type for read / write operations on files, stdin, stdout, stderr.
You can create `IOHandle` value by `IO::open_file`, and close it by `IO::close_file`.
Also there are global `IO::IOHandle::stdin`, `IO::IOHandle::stdout`, `IO::IOHandle::stderr`.

#### field `_data : Destructor Ptr`

## namespace `Std::IO::IOHandle`

### value `_file_ptr : IOHandle -> Ptr`

Get pointer to C's `FILE` value from an `IOHandle`.
DO NOT call `fclose` on the pointer returned by this function.
To close an `IOHandle`, use `IO::close_file`.

### value `_unsafe_close : IOHandle -> ()`

Close an `IOHandle`.
This is an I/O action not wrapped by `IO`; use `IO::close_file` in the usual case.

### value `from_file_ptr : Ptr -> IOHandle`

Create an `IOHandle` from a file pointer (i.e., pointer to C's `FILE`).
DO NOT create two `IOHandle`s from a single file pointer.

## namespace `Std::IO`

### value `_read_line_inner : Bool -> IOHandle -> IOFail String`

Read characters from an IOHandle.
If the first argument `upto_newline` is true, this function reads a file upto newline or EOF.

### value `_unsafe_perform : IO a -> a`

Perform the I/O action. This may violate purity of Fix.

### value `close_file : IOHandle -> IO ()`

Close a file.
Unlike C's `fclose`, closing an already closed `IOHandle` is safe and does nothing.

### value `eprint : String -> IO ()`

Print a string to stderr.

### value `eprintln : String -> IO ()`

Print a string followed by a newline to stderr.

### value `exit : I64 -> IO a`

Exit the program with an error code.

### value `exit_with_msg : I64 -> String -> IO a`

Exit the program with an error message and an error code.
The error message is written to the standard error output.

### value `from_func : (() -> a) -> IO a`

Create an IO action from a function.

### value `get_arg : I64 -> IO (Option String)`

`get_arg(n)` returns the n-th (0-indexed) command line argument.
If n is greater than or equal to the number of command line arguments, this function returns none.

### value `get_arg_count : IO I64`

Get the number of command line arguments.

### value `get_args : IO (Array String)`

Get command line arguments.

### value `input_line : IO String`

Read a line from stdin. If some error occurr, this function aborts the program.
If you want to handle errors, use `read_line(stdin)` instead.

### value `is_eof : IOHandle -> IO Bool`

Check if an `IOHandle` reached to the EOF.

### value `loop_lines : IOHandle -> s -> (s -> String -> LoopResult s s) -> IOFail s`

Loop on lines read from an `IOHandle`.
`loop_lines(handle, initial_state, worker)` calls `worker` on the pair of current state and a line string read from `handle`.
The function `worker` should return an updated state as `LoopResult` value, i.e., a value created by `continue` or `break`.
When the `handle` reaches to the EOF or `worker` returns a `break` value, `loop_lines` returns the last state value.
Note that the line string passed to `worker` may contain a newline code at the end. To remove it, use `String::strip_last_spaces`.

### value `loop_lines_io : IOHandle -> s -> (s -> String -> IOFail (LoopResult s s)) -> IOFail s`

Loop on lines read from an `IOHandle`.
Similar to `loop_lines`, but the worker function can perform an IO action.

### value `open_file : Path -> String -> IOFail IOHandle`

Open a file. The second argument is a mode string for `fopen` C function.

### value `print : String -> IO ()`

Print a string to stdout.

### value `println : String -> IO ()`

Print a string followed by a newline to stdout.

### value `read_bytes : IOHandle -> IOFail (Array U8)`

Read all bytes from an IOHandle.

### value `read_file_bytes : Path -> IOFail (Array U8)`

Read all bytes from a file.

### value `read_file_string : Path -> IOFail String`

Raad all characters from a file.

### value `read_line : IOHandle -> IOFail String`

Read characters from a IOHandle upto newline or EOF.
The returned string may include newline at its end.

### value `read_n_bytes : IOHandle -> I64 -> IOFail (Array U8)`

Read at most n bytes from an IOHandle.

### value `read_string : IOHandle -> IOFail String`

Read all characters from an IOHandle.

### value `stderr : IOHandle`

The handle for standard error.

### value `stdin : IOHandle`

The handle for standard input.

### value `stdout : IOHandle`

The handle for standard output.

### value `with_file : Path -> String -> (IOHandle -> IOFail a) -> IOFail a`

Perform a function with a file handle. The second argument is a mode string for `fopen` C function.
The file handle will be closed automatically.

### value `write_bytes : IOHandle -> Array U8 -> IOFail ()`

Write a byte array into an IOHandle.

### value `write_file_bytes : Path -> Array U8 -> IOFail ()`

Write a byte array into a file.

### value `write_file_string : Path -> String -> IOFail ()`

Write a string into a file.

### value `write_string : IOHandle -> String -> IOFail ()`

Write a string into an IOHandle.

## namespace `Std`

### type `Iterator a = unbox struct { ...fields... }`

Iterator (a.k.a lazy list)

#### field `next : () -> Option (a, Iterator a)`

## namespace `Std::Iterator`

### value `_flatten : Iterator (Iterator a) -> Iterator a`

Flatten an iterator of iterators.
You should use `Monad::flatten` instead of this function.
This function is used in the implementation of `Monad::bind` for `Iterator`.

### value `_flatten_sub : Iterator a -> Iterator (Iterator a) -> Iterator a`

### value `advance : Iterator a -> Option (a, Iterator a)`

Get next value and next iterator.

### value `append : Iterator a -> Iterator a -> Iterator a`

Append an iterator to a iterator.
Note: Since `iter1.append(iter2)` puts `iter2` after `iter1`, `append(lhs, rhs)` puts `lhs` after `rhs`.

### value `bang : Iterator a -> Iterator a`

Evaluate all elements of iterator.
TODO: add test

### value `count_up : I64 -> Iterator I64`

Creates an iterator that counts up from a number.
count_up(n) = [n, n+1, n+2, ...]

### value `empty : Iterator a`

Create an empty iterator.

### value `filter : (a -> Bool) -> Iterator a -> Iterator a`

Filter elements by a condition function

### value `find_last : Iterator a -> Option a`

Find the last element of an iterator.

### value `fold : b -> (b -> a -> b) -> Iterator a -> b`

Folds iterator from left to right.
Example: `fold(init, op, [a0, a1, a2, ...]) = ...op(op(op(init, a0), a1), a2)...`

### value `fold_m : [m : Monad] b -> (b -> a -> m b) -> Iterator a -> m b`

Folds iterator from left to right by monadic action.

### value `from_array : Array a -> Iterator a`

Create iterator from an array.

### value `from_map : (I64 -> a) -> Iterator a`

Creates iterator from mapping function.
from_map(f) = [f(0), f(1), f(2), ...]

### value `generate : s -> (s -> Option (a, s)) -> Iterator a`

Generate an iterator from a state transition function.
- if `f(s)` is none, `generate(s, f)` is empty.
- if `f(s)` is some value `(e, s1)`, then `generate(s, f)` starts by `e` followed by `generate(s2, f)`.

### value `get_first : Iterator a -> Option a`

Get the first element of an iterator. If the iterator is empty, this function returns `none`.
TODO: add test

### value `get_size : Iterator a -> I64`

Count the number of elements of an iterator.

### value `get_tail : Iterator a -> Option (Iterator a)`

Remove the first element from an iterator. If the iterator is empty, this function returns `none`.
TODO: add test

### value `intersperse : a -> Iterator a -> Iterator a`

Intersperse an elemnt between elements of an iterator.
Example:
```
Iterator::from_array([1,2,3]).intersperse(0) == Iterator::from_array([1,0,2,0,3])
```

### value `is_empty : Iterator a -> Bool`

Check if the iterator is empty.

### value `loop_iter : b -> (b -> a -> LoopResult b b) -> Iterator a -> b`

Loop along an iterator. At each iteration step, you can choose to continue or to break.

### value `loop_iter_m : [m : Monad] b -> (b -> a -> m (LoopResult b b)) -> Iterator a -> m b`

Loop by monadic action along an iterator. At each iteration step, you can choose to continue or to break.

### value `product : Iterator a -> Iterator b -> Iterator (b, a)`

The cartesian product of two iterators.
Example: `[1, 2, 3].to_iter.product(['a', 'b'].to_iter).to_array == [(1, 'a'), (2, 'a'), (3, 'a'), (1, 'b'), (2, 'b'), (3, 'b')]`

### value `push_front : a -> Iterator a -> Iterator a`

Push an element to an iterator.

### value `range : I64 -> I64 -> Iterator I64`

Create a range iterator, i.e. an iterator of the form `[a, a+1, a+2, ..., b-1]`.

### value `reverse : Iterator a -> Iterator a`

Reverse an iterator.

### value `subsequences : Iterator a -> Iterator (Iterator a)`

Generated all subsequences of an iterator.
`[1,2,3].to_iter.subsequences` is `[[], [3], [2], [2, 3], [1], [1, 3], [1, 2], [1, 2, 3]].to_iter.map(to_iter)`.

### value `sum : [a : Additive] Iterator a -> a`

Calculate the sum of elements of an iterator.

### value `take : I64 -> Iterator a -> Iterator a`

Take at most n elements from an iterator.

### value `take_while : (a -> Bool) -> Iterator a -> Iterator a`

Take elements of an iterator while a condition is satisfied.
TODO: add test

### value `to_array : Iterator a -> Array a`

Convert an iterator to an array.

### value `zip : Iterator b -> Iterator a -> Iterator (a, b)`

Zip two iterators.

## namespace `Std`

### type `Lazy = () -> a`

The type of lazily generated values.
This is a type alias defined as `type Lazy a = () -> a;`
You can create a lazy value by `|_| (...an expression to generate the value...)`, and
you can evaluate a lazy value `v` by `v()`.

### trait `a : LessThan`

#### method `less_than : a -> a -> Bool`

Less than comparison.

An expression `x < y` is translated to `less_than(x, y)`.

## namespace `Std::LessThan`

### value `max : [a : LessThan] a -> a -> a`

### value `min : [a : LessThan] a -> a -> a`

## namespace `Std`

### trait `a : LessThanOrEq`

#### method `less_than_or_eq : a -> a -> Bool`

Less than or equal comparison.

An expression `x <= y` is translated to `less_than_or_eq(x, y)`.

### type `LoopResult s b = unbox union { ...variants... }`

#### variant `continue : s`

#### variant `break : b`

## namespace `Std::LoopResult`

### value `break_m : [m : Monad] r -> m (LoopResult s r)`

Make a break value wrapped in a monad.
This is used with `loop_m` function.

### value `continue_m : [m : Monad] s -> m (LoopResult s r)`

Make a continue value wrapped in a monad.
This is used with `loop_m` function.

## namespace `Std`

### trait `[m : *->*] m : Monad`

#### method `bind : (a -> m b) -> m a -> m b`

#### method `pure : a -> m a`

## namespace `Std::Monad`

### value `flatten : [m : Monad] m (m a) -> m a`

Flatten a nested monadic action.

### value `unless : [m : Monad] Bool -> m () -> m ()`

`unless(cond, act)` where `act` is a monadic value which returns `()` perfoms `act` only when `cond` is false.

### value `when : [m : Monad] Bool -> m () -> m ()`

`when(cond, act)` where `act` is a monadic value which returns `()` perfoms `act` only when `cond` is true.

## namespace `Std`

### trait `a : Mul`

#### method `mul : a -> a -> a`

Multiplication.

An expression `x * y` is translated to `mul(x, y)`.

### trait `a : Neg`

#### method `neg : a -> a`

Negates a value.

An expression `-x` is translated to `neg(x)`.

### trait `a : Not`

#### method `not : a -> a`

Logical NOT.

An expression `!x` is translated to `not(x)`.

### type `Option a = unbox union { ...variants... }`

#### variant `none : ()`

#### variant `some : a`

## namespace `Std::Option`

### value `as_some_or : a -> Option a -> a`

Unwrap an option value if it is `some`, or returns given default value if it is `none`.

### value `map_or : b -> (a -> b) -> Option a -> b`

Returns the provided default value if the option is none, or applies a function to the contained value if the option is some.

## namespace `Std`

### type `Path = unbox struct { ...fields... }`

The type for file path.
TODO: give better implementation.

#### field `_data : String`

## namespace `Std::Path`

### value `parse : String -> Option Path`

Parse a string.

## namespace `Std`

### type `Ptr = unbox { primitive }`

## namespace `Std::Ptr`

### value `add_offset : I64 -> Ptr -> Ptr`

Add an offset to a pointer.

### value `subtract_ptr : Ptr -> Ptr -> I64`

Subtract two pointers.
Note that `x.subtract_ptr(y)` calculates `x - y`, so `subtract_ptr(x, y)` calculates `y - x`.

## namespace `Std`

### type `PunchedArray a = unbox struct { ...fields... }`

The type of punched arrays.
A punched array is an array from which a certain element has been removed.
This is used in the implementation of `Array::act`.

#### field `_data : Destructor (Array a)`

#### field `idx : I64`

## namespace `Std::PunchedArray`

### value `plug_in : a -> PunchedArray a -> Array a`

Plug in an element to a punched array to get back an array.

### value `unsafe_punch : I64 -> Array a -> (PunchedArray a, a)`

Creates a punched array by moving out the element at the specified index.
NOTE: this function assumes that the given array is unique WITHOUT CHECKING.
The uniqueness of the array is ensured in the `Array::act` function.

## namespace `Std`

### trait `a : Rem`

#### method `rem : a -> a -> a`

Remainder.

An expression `x % y` is translated to `rem(x, y)`.

### type `Result e o = unbox union { ...variants... }`

A type of result value for a computation that may fail.

#### variant `ok : o`

#### variant `err : e`

## namespace `Std::Result`

### value `unwrap : Result e o -> o`

Returns the containing value if the value is ok, or otherwise aborts the program.

## namespace `Std`

### type `String = unbox struct { ...fields... }`

#### field `_data : Array U8`

## namespace `Std::String`

### value `_get_c_str : String -> Ptr`

Get the null-terminated C string.
Note that in case the string is not used after call of this function, the returned pointer will be already released.

### value `_unsafe_from_c_str : Array U8 -> String`

Create a string from C string (i.e., null-terminated byte array).
If the byte array doesn't include `\0`, this function causes undefined behavior.

### value `_unsafe_from_c_str_ptr : Ptr -> String`

Create a `String` from a pointer to null-terminated C string.
If `ptr` is not pointing to a valid null-terminated C string, this function cause undefined behavior.

### value `borrow_c_str : (Ptr -> a) -> String -> a`

Call a function with a null-terminated C string.

### value `concat : String -> String -> String`

Concatenate two strings.
Note: Since `s1.concat(s2)` puts `s2` after `s1`, `concat(lhs, rhs)` puts `lhs` after `rhs`.

### value `concat_iter : Iterator String -> String`

Concatenate an iterator of strings.

### value `empty : I64 -> String`

Create an empty string, which is reserved for a length.

### value `find : String -> I64 -> String -> Option I64`

`str.find(token, start_idx)` finds the index where `token` firstly appears in `str`, starting from `start_idx`.
Note that this function basically returns a number less than or equal to `start_idx`, but there is an exception:
`str.find("", start_idx)` with `start_idx >= str.get_size` returns `str.get_size`, not `start_idx`.

### value `get_bytes : String -> Array U8`

Get the byte array of a string, containing null-terminator.

### value `get_first_byte : String -> Option U8`

Get the first byte of a string. Returns none if the string is empty.

### value `get_last_byte : String -> Option U8`

Get the last byte of a string. Returns none if the string is empty.

### value `get_size : String -> I64`

Get the length of a string.

### value `get_sub : I64 -> I64 -> String -> String`

`String` version of `Array::get_sub`.

### value `is_empty : String -> Bool`

Returns if the string is empty or not.

### value `join : String -> Iterator String -> String`

Join strings by a separator.

### value `pop_back_byte : String -> String`

Removes the last byte.
If the string is empty, this function does nothing.

### value `split : String -> String -> Iterator String`

`str.split(sep)` splits `str` by `sep` into an iterator.
- If `sep` is empty, this function returns an infinite sequence of ""s.
- If `sep` is non-empty and `str` is empty, this function returns an iterator with a single element "".

### value `strip_first_bytes : (U8 -> Bool) -> String -> String`

Removes the first byte of a string while it satisifies the specified condition.

### value `strip_first_spaces : String -> String`

Removing leading whitespace characters.

### value `strip_last_bytes : (U8 -> Bool) -> String -> String`

Removes the last byte of a string while it satisifies the specified condition.

### value `strip_last_newlines : String -> String`

Removes newlines and carriage returns at the end of the string.

### value `strip_last_spaces : String -> String`

Removing trailing whitespace characters.

### value `strip_spaces : String -> String`

Strip leading and trailing whitespace characters.

## namespace `Std`

### trait `a : Sub`

#### method `sub : a -> a -> a`

Subtraction.

An expression `x - y` is translated to `sub(x, y)`.

### trait `a : ToBytes`

#### method `to_bytes : a -> Array U8`

### trait `a : ToString`

#### method `to_string : a -> String`

### type `U16 = unbox { primitive }`

## namespace `Std::U16`

### value `bit_and : Std::U16 -> Std::U16 -> Std::U16`

Calculates bitwise AND of two values.

### value `bit_or : Std::U16 -> Std::U16 -> Std::U16`

Calculates bitwise OR of two values.

### value `bit_xor : Std::U16 -> Std::U16 -> Std::U16`

Calculates bitwise XOR of two values.

### value `maximum : U16`

### value `minimum : U16`

### value `shift_left : Std::U16 -> Std::U16 -> Std::U16`

`v.shift_left(w)` shifts `v` to left by `w` bits.

### value `shift_right : Std::U16 -> Std::U16 -> Std::U16`

`v.shift_right(w)` shifts `v` to right by `w` bits.

### value `to_CChar : Std::U16 -> Std::I8`

Casts a value of `U16` into a value of `CChar`.

### value `to_CDouble : Std::U16 -> Std::F64`

Casts a value of `U16` into a value of `CDouble`.

### value `to_CFloat : Std::U16 -> Std::F32`

Casts a value of `U16` into a value of `CFloat`.

### value `to_CInt : Std::U16 -> Std::I32`

Casts a value of `U16` into a value of `CInt`.

### value `to_CLong : Std::U16 -> Std::I64`

Casts a value of `U16` into a value of `CLong`.

### value `to_CLongLong : Std::U16 -> Std::I64`

Casts a value of `U16` into a value of `CLongLong`.

### value `to_CShort : Std::U16 -> Std::I16`

Casts a value of `U16` into a value of `CShort`.

### value `to_CSizeT : Std::U16 -> Std::U64`

Casts a value of `U16` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::U16 -> Std::U8`

Casts a value of `U16` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::U16 -> Std::U32`

Casts a value of `U16` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::U16 -> Std::U64`

Casts a value of `U16` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::U16 -> Std::U64`

Casts a value of `U16` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::U16 -> Std::U16`

Casts a value of `U16` into a value of `CUnsignedShort`.

### value `to_F32 : Std::U16 -> Std::F32`

Casts a value of `U16` into a value of `F32`.

### value `to_F64 : Std::U16 -> Std::F64`

Casts a value of `U16` into a value of `F64`.

### value `to_I16 : Std::U16 -> Std::I16`

Casts a value of `U16` into a value of `I16`.

### value `to_I32 : Std::U16 -> Std::I32`

Casts a value of `U16` into a value of `I32`.

### value `to_I64 : Std::U16 -> Std::I64`

Casts a value of `U16` into a value of `I64`.

### value `to_I8 : Std::U16 -> Std::I8`

Casts a value of `U16` into a value of `I8`.

### value `to_U16 : Std::U16 -> Std::U16`

Casts a value of `U16` into a value of `U16`.

### value `to_U32 : Std::U16 -> Std::U32`

Casts a value of `U16` into a value of `U32`.

### value `to_U64 : Std::U16 -> Std::U64`

Casts a value of `U16` into a value of `U64`.

### value `to_U8 : Std::U16 -> Std::U8`

Casts a value of `U16` into a value of `U8`.

## namespace `Std`

### type `U32 = unbox { primitive }`

## namespace `Std::U32`

### value `bit_and : Std::U32 -> Std::U32 -> Std::U32`

Calculates bitwise AND of two values.

### value `bit_or : Std::U32 -> Std::U32 -> Std::U32`

Calculates bitwise OR of two values.

### value `bit_xor : Std::U32 -> Std::U32 -> Std::U32`

Calculates bitwise XOR of two values.

### value `maximum : U32`

### value `minimum : U32`

### value `shift_left : Std::U32 -> Std::U32 -> Std::U32`

`v.shift_left(w)` shifts `v` to left by `w` bits.

### value `shift_right : Std::U32 -> Std::U32 -> Std::U32`

`v.shift_right(w)` shifts `v` to right by `w` bits.

### value `to_CChar : Std::U32 -> Std::I8`

Casts a value of `U32` into a value of `CChar`.

### value `to_CDouble : Std::U32 -> Std::F64`

Casts a value of `U32` into a value of `CDouble`.

### value `to_CFloat : Std::U32 -> Std::F32`

Casts a value of `U32` into a value of `CFloat`.

### value `to_CInt : Std::U32 -> Std::I32`

Casts a value of `U32` into a value of `CInt`.

### value `to_CLong : Std::U32 -> Std::I64`

Casts a value of `U32` into a value of `CLong`.

### value `to_CLongLong : Std::U32 -> Std::I64`

Casts a value of `U32` into a value of `CLongLong`.

### value `to_CShort : Std::U32 -> Std::I16`

Casts a value of `U32` into a value of `CShort`.

### value `to_CSizeT : Std::U32 -> Std::U64`

Casts a value of `U32` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::U32 -> Std::U8`

Casts a value of `U32` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::U32 -> Std::U32`

Casts a value of `U32` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::U32 -> Std::U64`

Casts a value of `U32` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::U32 -> Std::U64`

Casts a value of `U32` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::U32 -> Std::U16`

Casts a value of `U32` into a value of `CUnsignedShort`.

### value `to_F32 : Std::U32 -> Std::F32`

Casts a value of `U32` into a value of `F32`.

### value `to_F64 : Std::U32 -> Std::F64`

Casts a value of `U32` into a value of `F64`.

### value `to_I16 : Std::U32 -> Std::I16`

Casts a value of `U32` into a value of `I16`.

### value `to_I32 : Std::U32 -> Std::I32`

Casts a value of `U32` into a value of `I32`.

### value `to_I64 : Std::U32 -> Std::I64`

Casts a value of `U32` into a value of `I64`.

### value `to_I8 : Std::U32 -> Std::I8`

Casts a value of `U32` into a value of `I8`.

### value `to_U16 : Std::U32 -> Std::U16`

Casts a value of `U32` into a value of `U16`.

### value `to_U32 : Std::U32 -> Std::U32`

Casts a value of `U32` into a value of `U32`.

### value `to_U64 : Std::U32 -> Std::U64`

Casts a value of `U32` into a value of `U64`.

### value `to_U8 : Std::U32 -> Std::U8`

Casts a value of `U32` into a value of `U8`.

## namespace `Std`

### type `U64 = unbox { primitive }`

## namespace `Std::U64`

### value `bit_and : Std::U64 -> Std::U64 -> Std::U64`

Calculates bitwise AND of two values.

### value `bit_or : Std::U64 -> Std::U64 -> Std::U64`

Calculates bitwise OR of two values.

### value `bit_xor : Std::U64 -> Std::U64 -> Std::U64`

Calculates bitwise XOR of two values.

### value `maximum : U64`

### value `minimum : U64`

### value `shift_left : Std::U64 -> Std::U64 -> Std::U64`

`v.shift_left(w)` shifts `v` to left by `w` bits.

### value `shift_right : Std::U64 -> Std::U64 -> Std::U64`

`v.shift_right(w)` shifts `v` to right by `w` bits.

### value `to_CChar : Std::U64 -> Std::I8`

Casts a value of `U64` into a value of `CChar`.

### value `to_CDouble : Std::U64 -> Std::F64`

Casts a value of `U64` into a value of `CDouble`.

### value `to_CFloat : Std::U64 -> Std::F32`

Casts a value of `U64` into a value of `CFloat`.

### value `to_CInt : Std::U64 -> Std::I32`

Casts a value of `U64` into a value of `CInt`.

### value `to_CLong : Std::U64 -> Std::I64`

Casts a value of `U64` into a value of `CLong`.

### value `to_CLongLong : Std::U64 -> Std::I64`

Casts a value of `U64` into a value of `CLongLong`.

### value `to_CShort : Std::U64 -> Std::I16`

Casts a value of `U64` into a value of `CShort`.

### value `to_CSizeT : Std::U64 -> Std::U64`

Casts a value of `U64` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::U64 -> Std::U8`

Casts a value of `U64` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::U64 -> Std::U32`

Casts a value of `U64` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::U64 -> Std::U64`

Casts a value of `U64` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::U64 -> Std::U64`

Casts a value of `U64` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::U64 -> Std::U16`

Casts a value of `U64` into a value of `CUnsignedShort`.

### value `to_F32 : Std::U64 -> Std::F32`

Casts a value of `U64` into a value of `F32`.

### value `to_F64 : Std::U64 -> Std::F64`

Casts a value of `U64` into a value of `F64`.

### value `to_I16 : Std::U64 -> Std::I16`

Casts a value of `U64` into a value of `I16`.

### value `to_I32 : Std::U64 -> Std::I32`

Casts a value of `U64` into a value of `I32`.

### value `to_I64 : Std::U64 -> Std::I64`

Casts a value of `U64` into a value of `I64`.

### value `to_I8 : Std::U64 -> Std::I8`

Casts a value of `U64` into a value of `I8`.

### value `to_U16 : Std::U64 -> Std::U16`

Casts a value of `U64` into a value of `U16`.

### value `to_U32 : Std::U64 -> Std::U32`

Casts a value of `U64` into a value of `U32`.

### value `to_U64 : Std::U64 -> Std::U64`

Casts a value of `U64` into a value of `U64`.

### value `to_U8 : Std::U64 -> Std::U8`

Casts a value of `U64` into a value of `U8`.

## namespace `Std`

### type `U8 = unbox { primitive }`

## namespace `Std::U8`

### value `bit_and : Std::U8 -> Std::U8 -> Std::U8`

Calculates bitwise AND of two values.

### value `bit_or : Std::U8 -> Std::U8 -> Std::U8`

Calculates bitwise OR of two values.

### value `bit_xor : Std::U8 -> Std::U8 -> Std::U8`

Calculates bitwise XOR of two values.

### value `maximum : U8`

### value `minimum : U8`

### value `shift_left : Std::U8 -> Std::U8 -> Std::U8`

`v.shift_left(w)` shifts `v` to left by `w` bits.

### value `shift_right : Std::U8 -> Std::U8 -> Std::U8`

`v.shift_right(w)` shifts `v` to right by `w` bits.

### value `to_CChar : Std::U8 -> Std::I8`

Casts a value of `U8` into a value of `CChar`.

### value `to_CDouble : Std::U8 -> Std::F64`

Casts a value of `U8` into a value of `CDouble`.

### value `to_CFloat : Std::U8 -> Std::F32`

Casts a value of `U8` into a value of `CFloat`.

### value `to_CInt : Std::U8 -> Std::I32`

Casts a value of `U8` into a value of `CInt`.

### value `to_CLong : Std::U8 -> Std::I64`

Casts a value of `U8` into a value of `CLong`.

### value `to_CLongLong : Std::U8 -> Std::I64`

Casts a value of `U8` into a value of `CLongLong`.

### value `to_CShort : Std::U8 -> Std::I16`

Casts a value of `U8` into a value of `CShort`.

### value `to_CSizeT : Std::U8 -> Std::U64`

Casts a value of `U8` into a value of `CSizeT`.

### value `to_CUnsignedChar : Std::U8 -> Std::U8`

Casts a value of `U8` into a value of `CUnsignedChar`.

### value `to_CUnsignedInt : Std::U8 -> Std::U32`

Casts a value of `U8` into a value of `CUnsignedInt`.

### value `to_CUnsignedLong : Std::U8 -> Std::U64`

Casts a value of `U8` into a value of `CUnsignedLong`.

### value `to_CUnsignedLongLong : Std::U8 -> Std::U64`

Casts a value of `U8` into a value of `CUnsignedLongLong`.

### value `to_CUnsignedShort : Std::U8 -> Std::U16`

Casts a value of `U8` into a value of `CUnsignedShort`.

### value `to_F32 : Std::U8 -> Std::F32`

Casts a value of `U8` into a value of `F32`.

### value `to_F64 : Std::U8 -> Std::F64`

Casts a value of `U8` into a value of `F64`.

### value `to_I16 : Std::U8 -> Std::I16`

Casts a value of `U8` into a value of `I16`.

### value `to_I32 : Std::U8 -> Std::I32`

Casts a value of `U8` into a value of `I32`.

### value `to_I64 : Std::U8 -> Std::I64`

Casts a value of `U8` into a value of `I64`.

### value `to_I8 : Std::U8 -> Std::I8`

Casts a value of `U8` into a value of `I8`.

### value `to_U16 : Std::U8 -> Std::U16`

Casts a value of `U8` into a value of `U16`.

### value `to_U32 : Std::U8 -> Std::U32`

Casts a value of `U8` into a value of `U32`.

### value `to_U64 : Std::U8 -> Std::U64`

Casts a value of `U8` into a value of `U64`.

### value `to_U8 : Std::U8 -> Std::U8`

Casts a value of `U8` into a value of `U8`.

## namespace `Std`

### trait `a : Zero`

#### method `zero : a`

### value `compose : (a -> b) -> (b -> c) -> a -> c`

Compose two functions. Composition operators `<<` and `>>` is translated to use of `compose`.

### value `fix : ((a -> b) -> a -> b) -> a -> b`

`fix` enables you to make a recursive function locally.

The idiom is `fix $ |loop, arg| -> {loop_body}`. In `{loop_body}`, you can call `loop` to make a recursion.

Example:
```
module Main;

main : IO ();
main = (
    let fact = fix $ |loop, n| if n == 0 { 1 } else { n * loop (n-1) };
    println $ fact(5).to_string // evaluates to 5 * 4 * 3 * 2 * 1 = 120
);
```

### value `loop : s -> (s -> Std::LoopResult s b) -> b`

`loop` enables you to make a loop. `LoopResult` is a union type defined as follows: 

```
type LoopResult s r = unbox union { continue : s, break : r };
```

`loop` takes two arguments: the initial state of the loop `s0` and the loop body function `body`. 
It first calls `body` on `s0`. 
If `body` returns `break(r)`, then the loop ends and returns `r` as the result. 
If `body` returns `continue(s)`, then the loop calls again `body` on `s`.

Example:
```
module Main;
    
main : IO ();
main = (
    let sum = loop((0, 0), |(i, sum)|
        if i == 100 { break $ sum };
        continue $ (i + 1, sum + i)
    );
    println $ sum.to_string
); // evaluates to 0 + 1 + ... + 99 
```

### value `loop_m : [m : Monad] s -> (s -> m (LoopResult s r)) -> m r`

Monadic loop function. This is similar to `loop` but can be used to perform monadic action at each loop.

It is convenient to use `continue_m` and `break_m` to create monadic loop body function.

The following program prints "Hello World! (i)" for i = 0, 1, 2.

```
module Main;

main : IO ();
main = (
    loop_m(0, |i| (
        if i == 3 { break_m $ () };
        eval *println("Hello World! (" + i.to_string + ")");
        continue_m $ i + 1
    ))
);
```

### value `mark_threaded : a -> a`

Traverses all values reachable from the given value, and changes the reference counters of them into multi-threaded mode.

### value `undefined : Std::Lazy a`

An undefined value.

Since `undefined()` has generic type `a`, you can put it anywhere and it will be type-checked.
This is useful as a placeholder value that you haven't implemented yet.

Calling this value aborts the execution of the program (calls `abort` in libc).

### value `unsafe_is_unique : a -> (Std::Bool, a)`

This function checks if a value is uniquely referenced by a name, and returns the result paired with the given value itself. An unboxed value is always considered unique.

NOTE: Changing outputs of your function depending on uniqueness breaks the referential transparency of the function. If you want to assert that a value is unique, consider using `Debug::assert_unique` instead.

Example: 

```
module Main;

import Debug;

main : IO ();
main = (
    // For unboxed value, it returns true even if the value is used later.
    let int_val = 42;
    let (unique, _) = int_val.unsafe_is_unique;
    let use = int_val + 1;
    eval assert_eq(|_|"fail: int_val is shared", unique, true);

    // For boxed value, it returns true if the value isn't used later.
    let arr = Array::fill(10, 10);
    let (unique, arr) = arr.unsafe_is_unique;
    let use = arr.@(0); // This `arr` is not the one passed to `is_unique`, but the one returned by `is_unique`.
    eval assert_eq(|_|"fail: arr is shared", unique, true);

    // Fox boxed value, it returns false if the value will be used later.
    let arr = Array::fill(10, 10);
    let (unique, _) = arr.unsafe_is_unique;
    let use = arr.@(0);
    eval assert_eq(|_|"fail: arr is unique", unique, false);

    pure()
);
```