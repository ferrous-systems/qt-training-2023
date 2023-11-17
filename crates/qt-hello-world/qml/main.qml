import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15

ApplicationWindow {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640

    Label {
        anchors.fill: parent

        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter

        text: qsTr("Welcome to CXX-Qt - Rust ♥ Qt")
    }
}
