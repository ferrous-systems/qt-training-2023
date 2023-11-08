use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        // - Qt Core is always linked
        // - Qt Qml is linked by enabling the qt_qml Cargo feature.
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qt_module("Quick")
        .qml_module(QmlModule {
            uri: "com.kdab.cxx_qt.demo",
            rust_files: &["src/image_painter.rs"],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .build();
}
