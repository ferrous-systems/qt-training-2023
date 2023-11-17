use cxx_qt_lib::QColor;
use std::pin::Pin;

#[cxx_qt::bridge(cxx_file_stem = "image_painter")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;

        include!(<QtQuick/QQuickPaintedItem>);
    }

    unsafe extern "C++" {
        include!(<QtGui/QPainter>);
        type QPainter;
    }

    unsafe extern "RustQt" {
        #[qml_element]
        #[base = "QQuickPaintedItem"]
        #[qobject]
        type ImagePainter = super::ImagePainterRust;

        #[inherit]
        #[rust_name = "set_fill_color"]
        fn setFillColor(self: Pin<&mut ImagePainter>, color: &QColor);

        #[cxx_override]
        unsafe fn paint(self: Pin<&mut ImagePainter>, painter: *mut QPainter);
    }

    impl cxx_qt::Constructor<()> for ImagePainter {}
}

#[derive(Default)]
pub struct ImagePainterRust {}

impl cxx_qt::Initialize for qobject::ImagePainter {
    fn initialize(mut self: Pin<&mut Self>) {
        self.as_mut().set_fill_color(&QColor::from_rgb(0, 119, 200));
    }
}

impl qobject::ImagePainter {
    fn paint(self: Pin<&mut Self>, _painter: *mut qobject::QPainter) {
        // Do nothing (yet)
    }
}
