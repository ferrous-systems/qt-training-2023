import QtCore
import Qt.labs.platform // Prior to Qt 6.2, StandardPaths was still in labs
import QtQuick
import QtQuick.Controls
import QtQuick.Window
import QtQuick.Dialogs
import QtQuick.Layouts

// This must match the uri and version
// specified in the qml_module in the build.rs script.
import com.kdab.cxx_qt.demo 1.0

ApplicationWindow {
    height: 480
    title: qsTr("Rustagram Image Converter")
    visible: true
    width: 640

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        RowLayout {
            Button {
                text: qsTr("Browse...")
                enabled: !imagePainter.running

                onClicked: fileDialog.open()
            }

            Label {
                text: imagePainter.status
                elide: Text.ElideRight

                Layout.fillWidth: true
            }

            ComboBox {
                enabled: !imagePainter.running

                model: [
                    "1977",
                    "nineteenseventyseven",
                    "aden",
                    "brannan",
                    "brooklyn",
                    "clarendon",
                    "earlybird",
                    "gingham",
                    "hudson",
                    "inkwell",
                    "kelvin",
                    "lark",
                    "lofi",
                    "maven",
                    "mayfair",
                    "moon",
                    "nashville",
                    "reyes",
                    "rise",
                    "slumber",
                    "stinson",
                    "toaster",
                    "valencia",
                    "walden",
                ]

                onActivated: {
                    imagePainter.filter = currentText
                }
            }
        }

        ImagePainter {
            id: imagePainter
            Layout.fillWidth: true
            Layout.fillHeight: true
        }
    }

    FileDialog {
        id: fileDialog
        currentFolder: StandardPaths.standardLocations(StandardPaths.PicturesLocation)[0]
        onAccepted: {
            imagePainter.fileUrl = selectedFile
        }
    }
}
