# Rust Tutorials

- To start a new project, initiialize one with the command

```sh
cargo new <name_of_your_project>
```

- After this is done, `cd` into the new directory and the files for the project should be kept in the `src/`

## Rustfmt

- _The command `rustfmt .rs` is run inside the cargo folder and formats the code inside it._
- _Rust programmes cannot run without main function._

## Commands

- _Another cargo command is;_

```sh
cargo check
```

- _The command checks if your program is executable, without necessaily compiling it._
- Another command;

```sh
cargo run
```

- _The command builds/compile the project and runs it._

- _The conventional way to name constants is using all upper cases and  using underscores_

```rs
fn main() {
    const SECONDS_TO_MINUTE: u32 = 60;
    println!(" {}", SECONDS_TO_MINUTE)
}
```

- _Rust primitive data types are written out as formated strings_
- _Rust has tuples, which have various data types and cannot be mutable and cannot be updated._
- _Arrays, howevwer, unlike tupples can be updated, but only if stated as muttable. They however have a trade off. They contain only one data type_
- _The default data type of floats is `f64` whereas the default type of integers is `i32`. The compiler assigns the data types to the variables that were implicitly decalred while compiling_
- _Rust, like most compiled languges, does not have strings. It has characters, which together make up a customized string_
