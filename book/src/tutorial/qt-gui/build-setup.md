# Building a simple Qt application with Rust

CXX-Qt provides helper libraries that make it easy for the Rust build system to link to Qt.

The goal for now is to create a simple program that just launches an Qt/QML application.

✅ Create a new Rust application
```console
cargo new qt-gui
cd qt-gui
```

✅ Add a simple QML main file in `qml/main.qml`
```qml
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15

ApplicationWindow {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640
}
```

✅ Add dependencies to the `Cargo.toml` file
```toml
[dependencies]
rustagram2="2"
cxx="1.0.95"
cxx-qt = { git="https://github.com/LeonMatthesKDAB/cxx-qt", branch="qimage" }
cxx-qt-lib = { git="https://github.com/LeonMatthesKDAB/cxx-qt", branch="qimage" }

[build-dependencies]
# The "link_qt_object_files" is required when using Cargo to link to Qt statically.
cxx-qt-build = { git="https://github.com/LeonMatthesKDAB/cxx-qt", branch="qimage", features = [ "qt_qml", "link_qt_object_files" ] } 
```

```diff
- At the time of writing (15.11.2023), CXX-Qt 0.6 hasn't been released yet.
- To keep the training material up-to-date with the new API introduced in version 0.6,
- we'll use the version from Github for now.
- The training material on Github will be updated as soon as version 0.6 is released!

- Do note that the public documentation for CXX-Qt is still for version 0.5.
- You should wait before following this tutorial until version 0.6 is
- released in the coming weeks.
```

<details>
<summary> Future imports once CXX-Qt 0.6 is released </summary>

```toml
[dependencies]
rustagram2="2"
cxx="1.0.95"
cxx-qt="0.6"
cxx-qt-lib="0.6"

[build-dependencies]
# The "link_qt_object_files" is required when using Cargo to link to Qt statically.
cxx-qt-build = { version="0.6", features = [ "qt_qml", "link_qt_object_files" ] } 
```
</details>

CXX-Qt is split up into multiple crates.
* CXX-Qt - Allows you to create QObjects from Rust and to interact with existing QObjects
* CXX-Qt-lib - Library of bindings to Qt types like `QString`, `QApplication`, etc.
* CXX-Qt-build - A build-system library to link Rust with Qt, build QML modules, etc.

We will also need CXX itself later on, so include that as well.

✅ Next to the `Cargo.toml` file, add a `build.rs` file with the following contents:
```rust
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        // - Qt Core is always linked
        // - Qt Qml is linked by enabling the qt_qml Cargo feature.
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qt_module("Quick")
        .qml_module(QmlModule::<&str, _> {
            uri: "com.kdab.cxx_qt.demo",
            rust_files: &[],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .build();
}
```


It uses the `cxx-qt-build` library to:
1. Check that the QtQuick module is available
2. Create a QML module out of the `main.qml` file.
3. Link the QML module and Qt itself to our Rust application.

> Troubleshooting: It is important that the `build.rs` file is
> next to the `Cargo.toml` file at the root of the crate.
>
> It can **NOT** be located in the `src/` directory.

✅ Add the startup code to `src/main.rs`
```rust
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
```
> Take a minute to thoroughly read this code and compare it to a typical C++ main
> function that launches a Qt application.
>
> ❓ What is different? What is similar?\
> ❓ Why are the `if let Some(...)` expressions necessary?

✅ Run the application
```console
cargo run
```

> If `qmake` is not in your path, remember to tell Cargo where to find it
> with the `QMAKE` environment variable.
> ```console
> QMAKE=.../qmake cargo run
> ```

✅ An empty window appears
![An empty window with the title "Hello World"](./qt-gui-empty-window.png)
