use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QColor, QImage, QRectF, QString, QUrl};
use image_manipulation::apply_filter;
use rustagram::FilterType;
use std::{error::Error, pin::Pin};

#[cxx_qt::bridge(cxx_file_stem = "image_painter")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;

        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = cxx_qt_lib::QSizeF;

        include!("cxx-qt-lib/qrectf.h");
        type QRectF = cxx_qt_lib::QRectF;

        include!("cxx-qt-lib/qimage.h");
        type QImage = cxx_qt_lib::QImage;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!(<QtQuick/QQuickPaintedItem>);
    }

    unsafe extern "C++" {
        include!(<QtGui/QPainter>);
        type QPainter;

        #[rust_name = "draw_image"]
        fn drawImage(self: Pin<&mut QPainter>, rectangle: &QRectF, image: &QImage);
    }

    unsafe extern "RustQt" {
        #[qml_element]
        #[base = "QQuickPaintedItem"]
        #[qobject]
        #[qproperty(QString, status)]
        #[qproperty(QString, filter)]
        #[qproperty(QUrl, file_url)]
        #[qproperty(bool, running)]
        type ImagePainter = super::ImagePainterRust;

        #[inherit]
        #[rust_name = "set_fill_color"]
        fn setFillColor(self: Pin<&mut ImagePainter>, color: &QColor);

        #[inherit]
        fn size(self: &ImagePainter) -> QSizeF;

        #[inherit]
        fn update(self: Pin<&mut ImagePainter>);

        #[cxx_override]
        unsafe fn paint(self: Pin<&mut ImagePainter>, painter: *mut QPainter);
    }

    impl cxx_qt::Threading for ImagePainter {}
    impl cxx_qt::Constructor<()> for ImagePainter {}
}

pub struct ImagePainterRust {
    filter: QString,
    status: QString,
    running: bool,
    file_url: QUrl,

    image: Option<QImage>,
}

impl Default for ImagePainterRust {
    fn default() -> Self {
        Self {
            filter: QString::from("1977"),
            status: QString::from("No file selected"),
            image: None,
            file_url: QUrl::default(),
            running: false,
        }
    }
}

impl cxx_qt::Initialize for qobject::ImagePainter {
    fn initialize(mut self: Pin<&mut Self>) {
        self.as_mut().set_fill_color(&QColor::from_rgb(0, 119, 200));

        self.as_mut().on_filter_changed(Self::load_file).release();

        self.as_mut().on_file_url_changed(Self::load_file).release();
    }
}

impl qobject::ImagePainter {
    unsafe fn paint(self: Pin<&mut Self>, painter: *mut qobject::QPainter) {
        // We need to convert the *mut QPainter to a Pin<&mut QPainter> so that we can reach the methods
        if let Some(painter) = painter.as_mut() {
            let pinned_painter = Pin::new_unchecked(painter);

            // Now pinned painter can be used as normal
            if let Some(image) = self.image.as_ref() {
                let size = self.size();
                pinned_painter
                    .draw_image(&QRectF::new(0.0, 0.0, size.width(), size.height()), image);
            }
        }
    }

    fn do_load_file(path: &str, filter: &str) -> Result<QImage, Box<dyn Error>> {
        let filter = filter
            .to_string()
            .parse()
            .unwrap_or(FilterType::NineTeenSeventySeven);
        let image = std::fs::read(path)?;
        let image = apply_filter(&image, filter);

        QImage::from_data(image.as_slice(), Some("PNG"))
            .ok_or("Failed to convert to QImage!".into())
    }

    fn load_file(mut self: Pin<&mut Self>) {
        if self.running {
            return;
        }

        if let Some(local_file) = self
            .file_url
            .to_local_file()
            .as_ref()
            .map(ToString::to_string)
        {
            let file_name = self.file_url.file_name();
            self.as_mut().set_status(file_name);
            self.as_mut().set_running(true);

            let filter = self.filter.to_string();
            let qt_thread = self.as_mut().qt_thread();

            std::thread::spawn(move || match Self::do_load_file(&local_file, &filter) {
                Ok(image) => {
                    qt_thread
                        .queue(move |mut this| {
                            this.as_mut().set_running(false);
                            this.as_mut().rust_mut().image = Some(image);
                            this.as_mut().update();
                        })
                        .ok();
                }
                Err(err) => {
                    let error = QString::from(&format!("Error: {}", err));
                    qt_thread
                        .queue(|mut this| {
                            this.as_mut().set_running(false);
                            this.as_mut().rust_mut().image = None;
                            this.as_mut().set_status(error);
                            this.update();
                        })
                        .ok();
                }
            });
        } else {
            self.set_status(QString::from("Error: Not a local file!"));
        }
    }
}
