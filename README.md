# Instant

JVM and LLVM compiler for Instant - a tiny, expression-based programming language (basically a calculator).


## Basic Usage

Complete toolchain for Rust programming language is required to compile the project.
The easiest way to install one is to run:
```shell script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To build the project executables (llvm and jvm compilers for instant):
```shell script
make
```

### Compiling Instant code to JVM

To execute the code compiled to JVM, you're going to need Java Runtime Environment
that supports Java bytecode version 47.0.

Compile file `e2e_test/test01.ins` to JVM and execute it:
```shell script
./insc_jvm e2e_test/test01.ins  # outputs: e2e_test/test01.j e2e_test/test01.class
cd e2e_test && java test01.class
```

Additionally created file, `e2e_test/test01.j` is a jvm bytecode represented as human-readable Jasmin commands.


### Compiling Instant code to LLVM

To execute the compiled code, you're going to need an LLVM interpreter (can be installed with entire llvm toolchain).

Compile file `e2e_test/test01.ins` to LLVM and execute it using LLVM interpreter:
```shell script
./insc_llvm e2e_test/test01.ins  # outputs: e2e_test/test01.ll e2e_test/test01.bc
cd e2e_test && lli test01.bc
```

Additionally created file, `e2e_test/test01.ll` contains human-redable version of compiled LLVM IR.


## Project Structure

The project is separated into `parser` and `compiler` Rust crates, and the main `instant` crate 
(located in project root directory, with source code in `src`) which generates both executables.

`Makefile`, `insc_jvm` and `insc_llvm` are added to bind appropriate `cargo` commands (required for the assignment).


### Parser

```
parser
├── Cargo.lock
├── Cargo.toml
├── build.rs
└── src
    ├── ast.rs
    ├── instant.lalrpop
    ├── instant.rs
    └── lib.rs
```

I decided to use [larlpop, a popular LR(1) parser generator for Rust](https://github.com/lalrpop/lalrpop).
While it requires re-writing the grammar from LBNF to another format, it seemed as a solid choice used,
as it's already for some of the largest Rust projects, including RustPython.

Structures forming abstract syntax tree of instant programs are defined in `parser/src/ast.rs`.
The grammar and parsing rules are defined in `parser/src/instant.larlpop`, 
which is compiled into `parser/src/instant.rs` during build.

Generally, I really like the flexibility that lalrpop allows in comparison to BNFC - by defining AST structure
and parsing rules, I managed to avoid generating a ton of boilerplate expressions. The missing features from BNFC
(coercions, separators) can be easily implemented by using macros in lalrpop grammar.


### Compiler

Compiler crate is much more complex, and contains all logic for LLVM and JVM compilers:

```
compiler
├── Cargo.toml
└── src
    ├── common.rs
    ├── jasmin.rs
    ├── lib.rs
    ├── llvm.rs
    └── stack.rs
```

#### LLVM Compiler

Parts of abstract syntax tree that are compiled implement trait `CompileLLVM`, in `compiler/src/llvm.rs`.

Compilation of expression results in a vector of LLVM instructions and a result: either register or constant.
This way, Instant constants are never translated into single instruction storing them in LLVM register.

All variables are allocated exactly once, to track this we pass mutable HashMap (`variables`) argument,
which contains names of all allocated variables and allows to prevent accessing undefined variable at compile time.

In order to track which registers are free, we also keep passing a mutable id of next free register number.
Integer register names are formatted using "%r{register_id}", while registers containing pointers to variables
are formatted using "%{variable_name}ptr" to prevent name collisions.

Result of compiling syntax tree to llvm is a vector of strings, which is then saved to `.ll` file - that action
is performed in the `insc_llvm.rs` executable. After that, the executable calls `llvm-as` to translate the text
file into binary one, and `llvm-link` to include `dist/runtime.bc` which contains `printInt` function.


#### JVM Compiler

I started learning to use Rust and Lalrpop before this assignment, by writing a simple calculator which translated
expressions into a set of stack-based vm commands. The result only needed few changes to be compatible with Instant
language, so I decided to build them on top of it - this is the reason why `compiler/src/stack.rs` exists.

Parts of abstract syntax tree that are compiled implement trait `CompileStack`. During compilation,
necessary stack depth is returned together with the vector of compiled instructions. This allows for optimization
of evaluation order in the case of binary expressions. Stack limit is also necessary for the final jasmin output.

Program statements are evaluated one by one, mapping between instant variables and jvm variable ids is stored
in a mutable HashMap (`env` argument). Compiler also prevents access to undefined variables.

The JVM compilation process, first translates the parsed abstract syntax tree into abstract stack representation
(implemented in `stack.rs`), which is later translated to Jasmin representation (implemented in `jasmin.rs`).

The executable `insc_jvm.rs` saves this representation, and runs `jasmin.jar` distributed in `dist` folder on the
created jasmin file in order to translate it to JVM bytecode, that is also saved. 


### Executables

The root-directory executables (`isnc_jvm` and `insc_llvm`) are just bash scripts wrapping compiled rust programs
(also setting environment variables, which I added to make local development easier).

Source code for the executables is contained within `src` directory:

```
src
├── bin
│   ├── insc_jvm.rs
│   └── insc_llvm.rs
└── lib.rs
```

Common utility methods for these executables are grouped in `src/lib.rs`.


### External resources

I also packaged `e2e_tests` for testing and demonstration purposes, as well as
the `dist` folder containing utilities necessary to translate compiled code to final executable representation.

None of these files are my work:
* `dist` files were supplied for the purposes of earlier labs
* `e2e_tests` were supplied together with assignment description

