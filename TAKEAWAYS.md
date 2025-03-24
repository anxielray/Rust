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
