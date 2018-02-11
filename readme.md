This repository shows how to design a CLI interface with subcommands, using the
`structopt` crate. As an example, imagine you want to write a command-line program
to remove and rename files. While not very realistic, this scenario is good enough
to showcase `structopt`. This means you have two subcommands: `remove` and `rename`.

You may try the commands below to see how it works. Note that
the arguments at the right side of the `--` are passed directly to our Rust
program.

```
cargo run -- --help
cargo run -- remove --help
cargo run -- remove foo
cargo run -- rename --help
cargo run -- rename foo bar
```

The code needed to generate this CLI is surprisingly simple:

```rust
#[derive(StructOpt, Debug)]
enum Options {
    #[structopt(name = "remove")]
    RemoveFile(Remove),
    #[structopt(name = "rename")]
    RenameFile(Rename)
}

#[derive(StructOpt, Debug)]
struct Remove {
    #[structopt(name = "FILE")]
    path: String
}

#[derive(StructOpt, Debug)]
struct Rename {
    #[structopt(name = "FROM")]
    from: String,
    #[structopt(name = "TO")]
    to: String
}
```
