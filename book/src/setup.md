# Setup

This section describes how to set up the toolchain
for compiling Rust programs to WebAssembly
and integrate them with the different environments we will look at.

## The Rust Toolchain

You will need the standard Rust toolchain, including `rustup`, `rustc`, and
`cargo`.

Follow the instructions provided at rust-lang.org to install the Rust toolchain[^rust-install].

This will install the Rust toolchain and the targets for the host platform.
Windows users need to choose between the `msvc` and `mingw` targets.
We recommend the `msvc` target, unless you already have `mingw` installed.

The course will only use the stable compiler, we don't require any experimental feature flags.
The latest Rust should work best, though older versions have no known issues.

[^rust-install]: https://www.rust-lang.org/tools/install

## Additional tooling

For those interested in following the Qt section of the course,
installation of Qt is needed.

The provided examples use Qt 6 and are tested against Qt 6.5 LTS.
If you can only install Qt 5, you will need to modify the QML code,
as the FileDialog API has changed between Qt 5 and 6.
Apart from that, The Rust-Qt integration ([CXX-Qt](https://github.com/KDAB/cxx-qt))
should work the same with Qt 5 and Qt 6.

How to install Qt depends a lot on the platform you're using.
Many platforms even have multiple options on how to install Qt.
The main goal is to ensure that `qmake` for Qt 6 is in your `PATH`.

You can check this by using `qmake --version`, which should return something like this:
```console
$ qmake --version
QMake version 3.1
Using Qt version 6.5.1 in /usr/lib64
```

<div style="break-after:page"></div>

### Linux
Many distributions have Qt already packaged, if your package manager doesn't offer development versions of Qt, you can always fall back to using the Qt online installer[^qt-installer].

No matter which installation method you choose, make sure to install the QML (aka. QtQuick/Declarative) and Network modules, as well as any packages needed for QtQuickControls2.

On Fedora for a full installation of Qt, use:
```bash
$ dnf install "qt6-*-devel"
```

### Windows

We have had the best experience with installing Qt on Windows using the Qt online installer[^qt-installer].
Make sure to select and install a version of Qt 6.

Then add the installation directory to your `PATH` environment variable and make sure `qmake` is in your `PATH` by running `qmake --version`.
You may have to restart your terminal for this to work correctly.

> Note that on Windows the Qt installation usually shouldn't be in your system path.
> That may cause issues with other programs that link to Qt dynamically.
>
> CXX-Qt still needs to be able to find the `qmake` executable however.
> A good compromise is to create a development environment which either temporarily
> adds the Qt installation in the PATH or assigns the `QMAKE` environment variable
> to point to the correct `qmake` executable.
>
> If this is too much trouble to set up you can always fall back to providing the
> `QMAKE` environment variable each time you execute `cargo build` or `cargo run`.

### MacOS

Preferrably install Qt using the Qt online installer[^qt-installer].
Make sure to select and install a version of Qt 6.

You can also try installing Qt using homebrew.
```zsh
$ brew install qt6
```

In any case, make sure the installation directory is added to your path and qmake can be found by your command line using `qmake --version`.
You may have to restart your terminal for this to work correctly.

[^qt-installer]:https://doc.qt.io/qt-6/get-and-install-qt.html
