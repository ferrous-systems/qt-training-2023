# Recreate as workspace

Let's get started by creating a so-called "cargo workspace". In
a growing project, this is a common refactoring, however, given
the simplicity of our CLI app, it's easier to just start from
the beginning again.

✅ Create yourself an working directory

```console
$ mkdir image-workspace
$ cd image-workspace
```

✅ create a library crate for image manipulation

```console
$ cargo new --lib image-manipulation
```

✅ create a binary crate for the CLI app

```console
$ cargo new --bin cli
```

You folder structure should currently look like this:

```console
$ tree
.
└── cli
└── image-manipulation
```

✅ create a file called `Cargo.toml` in the main folder:

```console
$ tree
.
└── Cargo.toml
└── cli
└── image-manipulation
```

✅ fill that file with the following info:

```toml
[workspace]
members = ["cli", "image-manipulation"]
resolver = "2"
```

✅ build the whole workspace once to check everything works

```console
$ cargo build
```

Move on to [creating the library](create-library).