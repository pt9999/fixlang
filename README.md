Fix-lang
====

## Overview

Fix is a programming language with the following features: 
- Functional: All functions have no side effect and all values are immutable. This reduces bugs caused by state management failures.
- O(1) update of arrays and structures: Despite the 1st feature, Fix mutates a value if the mutation cannot be observed. For example, `let array1 = array0.set(10, 42);` defines a new array `array1` that is almost identical to `array0` but with the 10th element replaced by 42. If `array0` will not be referenced later, Fix will update the 10th element of `array0` and rename it as `array1`. On the other hand, if `array0` may be used later, Fix creates `array1` by cloning `array0` and setting the 10th element to 42, keeping immutability.
- Familier syntax: The syntax of Fix is more similar to languages such as C++ or Rust than to other functional languages such as Haskell. Even if you have never learned a functional language, you will be able to learn Fix quickly.

In another perspective, Fix is a language which uses reference counting to provide garbage collection and interior mutability. To avoid circular reference, all values are semantically immutable and it restricts dynamic recursive definition and forces to use fixed-point combinator instead. To reduce copy cost on "modify" operation of a value, Fix mutates it if the reference counter is one.

You can try Fix in [fixlang playground](https://tttmmmyyyy.github.io/fixlang-playground/).

(This project is still a WIP and has no practical use yet.)

## Examples

- [How to use loop function](https://tttmmmyyyy.github.io/fixlang-playground/index.html?src=module+Main%3B%0D%0A%0D%0A%2F%2F+Prints+30th+value+of+Fibonacci+sequence.%0D%0Amain+%3A+IO+%28%29%3B%0D%0Amain+%3D+%28%0D%0A++++let+arr+%3D+Array%3A%3Afill%2831%2C+0%29%3B%0D%0A++++let+arr+%3D+arr.set%21%280%2C+0%29%3B%0D%0A++++let+arr+%3D+arr.set%21%281%2C+1%29%3B%0D%0A++++%2F%2F+A+way+for+loop+is+to+use+%60loop%60%2C+%60continue%60+and+%60break%60.%0D%0A++++%2F%2F+loop+%3A+s+-%3E+LoopResult+s+r+-%3E+r+--+Takes+the+initial+state+and+loop+body%2C+and+performs+loop.%0D%0A++++%2F%2F+continue+%3A+s+-%3E+LoopResult+s+r+--+Takes+the+next+state+and+continues+the+loop.%0D%0A++++%2F%2F+break+%3A+r+-%3E+LoopResult+s+r+--+Breaks+the+loop+and+returns+the+given+value+as+a+result+of+loop.%0D%0A++++let+arr+%3D+loop%28%282%2C+arr%29%2C+%7C%28idx%2C+arr%29%7C%0D%0A++++++++if+idx+%3D%3D+arr.get_size+%7B%0D%0A++++++++++++break+%24+arr%0D%0A++++++++%7D+else+%7B%0D%0A++++++++++++let+x+%3D+arr.%40%28idx-1%29%3B%0D%0A++++++++++++let+y+%3D+arr.%40%28idx-2%29%3B%0D%0A++++++++++++let+arr+%3D+arr.set%21%28idx%2C+x%2By%29%3B%0D%0A++++++++++++continue+%24+%28idx%2B1%2C+arr%29%0D%0A++++++++%7D%0D%0A++++%29%3B%0D%0A++++println+%24+arr.%40%2830%29.to_string+%2F%2F+832040%0D%0A%29%3B%0D%0A)
- [How to define and use trait](https://tttmmmyyyy.github.io/fixlang-playground/index.html?src=module+Main%3B%0D%0A%0D%0A%2F%2A%0D%0AEq+trait+is+defined+in+standard+library+as+follows%3A+%0D%0A%0D%0Atrait+a+%3A+Eq+%7B%0D%0A++++eq+%3A+a+-%3E+a+-%3E+Bool%0D%0A%7D%0D%0A%0D%0AExpression+%60x+%3D%3D+y%60+is+interpreted+as+%60Eq%3A%3Aeq%28x%2C+y%29%60.%0D%0A%2A%2F%0D%0A%0D%0Atype+Pair+a+b+%3D+struct+%7B+fst%3A+a%2C+snd%3A+b+%7D%3B%0D%0A%0D%0A%2F%2F+In+the+trait+implementation%2C+you+can+specify+preconditions+on+type+variables+in+%60%5B%5D%60+bracket+after+%60impl%60.%0D%0Aimpl+%5Ba+%3A+Eq%2C+b+%3A+Eq%5D+Pair+a+b+%3A+Eq+%7B%0D%0A++++eq+%3D+%7Clhs%2C+rhs%7C+%28%0D%0A++++++++lhs.%40fst+%3D%3D+rhs.%40fst+%26%26+lhs.%40snd+%3D%3D+rhs.%40snd%0D%0A++++%29%3B%0D%0A%7D%0D%0A%0D%0A%2F%2F+You+can+specify+preconditions+of+type+variables+in+the+%60%5B%5D%60+bracket+before+type+signature.%0D%0Asearch+%3A+%5Ba+%3A+Eq%5D+a+-%3E+Array+a+-%3E+I64%3B%0D%0Asearch+%3D+%7Celem%2C+arr%7C+loop%280%2C+%7Cidx%7C%0D%0A++++if+idx+%3D%3D+arr.get_size+%7B+break+%24+-1+%7D%3B%0D%0A++++if+arr.%40%28idx%29+%3D%3D+elem+%7B+break+%24+idx+%7D%3B%0D%0A++++continue+%24+%28idx+%2B+1%29%0D%0A%29%3B%0D%0A%0D%0A%2F%2F+An+example+of+defining+higher-kinded+trait.%0D%0A%2F%2F+All+type+variable+has+kind+%60%2A%60+by+default%2C+and+any+kind+of+higher-kinded+type+variable+need+to+be+annoted+explicitly.%0D%0Atrait+%5Bf+%3A+%2A-%3E%2A%5D+f+%3A+MyFunctor+%7B%0D%0A++++mymap+%3A+%28a+-%3E+b%29+-%3E+f+a+-%3E+f+b%3B%0D%0A%7D%0D%0A%0D%0A%2F%2F+An+example+of+implementing+higher-kinded+trait.%0D%0A%2F%2F+%60Array%60+is+a+type+of+kind+%60%2A+-%3E+%2A%60%2C+so+matches+to+the+kind+of+trait+%60MyFunctor%60.%0D%0Aimpl+Array+%3A+MyFunctor+%7B%0D%0A++++mymap+%3D+%7Cf%2C+arr%7C+%28%0D%0A++++++++Array%3A%3Afrom_map%28arr.get_size%2C+%7Cidx%7C+f%28arr.%40%28idx%29%29%29%0D%0A++++%29%3B%0D%0A%7D%0D%0A%0D%0Amain+%3A+IO+%28%29%3B%0D%0Amain+%3D+%28%0D%0A++++let+arr+%3D+Array%3A%3Afrom_map%286%2C+%7Cx%7C+x%29%3B+%2F%2F+arr+%3D+%5B0%2C1%2C2%2C...%2C9%5D.%0D%0A++++let+arr+%3D+arr.mymap%28%7Cx%7C+Pair+%7B+fst%3A+x+%25+2%2C+snd%3A+x+%25+3+%7D%29%3B+%2F%2F+arr+%3D+%5B%280%2C+0%29%2C+%281%2C+1%29%2C+%280%2C+2%29%2C+...%5D.%0D%0A++++println+%24+arr.search%28Pair+%7B+fst%3A+1%2C+snd%3A+2%7D%29.to_string+%2F%2F+5%2C+the+first+number+x+such+that+x+%25+2+%3D%3D+1+and+x+%25+3+%3D%3D+2.%0D%0A%29%3B)

## Install (macOS / WSL)

- Install [Rust](https://www.rust-lang.org/tools/install).
- Install llvm12.0.1. It is recommended to use [llvmemv](https://crates.io/crates/llvmenv).
    - In macOS, llvmenv installs llvm to "~/Library/Application Support/llvmenv/12.0.1", but llvm-sys currently doesn't understand path with a whitespace correctly, so you need to copy/move "12.0.1" directory to another path.
- Set LLVM_SYS_120_PREFIX variable to the directory to which llvm installed.
- `git clone https://github.com/tttmmmyyyy/fixlang.git && cd fixlang`.
- `cargo install --path .`. Then the compiler command `fix` will be installed to `~/.cargo/bin`.

## Usage

- You can run the source file (with extension ".fix") by `fix run -f {source-file}`.
- If you want to build executable binary, run `fix build -f {source-file}.`.

## Tutorial / references

See [document](/Document.md).

## Discord

https://discord.gg/ad4GakEA7R