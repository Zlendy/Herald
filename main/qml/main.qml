import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.2
import QtQuick.Window 2.12

import org.kde.kirigami 2.20 as Kirigami

import Rust.Test 0.1

Kirigami.ApplicationWindow {
    id: appWindow
    height: 480
    title: qsTr("Gotify Rustop")
    visible: true
    width: 640

    GotifyRustop {
        id: rust

        // username: ""
        // password: ""
    }

    Binding {
        target: rust
        property: "username"
        value: username.text
    }

    Binding {
        target: rust
        property: "password"
        value: password.text
    }

    ColumnLayout {
        width: appWindow.width

        Kirigami.FormLayout {
            id: formLayout

            Layout.fillWidth: true

            Kirigami.Separator {
                Kirigami.FormData.isSection: true
                Kirigami.FormData.label: "Login to Gotify"
            }

            TextField {
                id: username
                Kirigami.FormData.label: "Username:"
                text: rust.username
            }

            TextField {
                id: password
                Kirigami.FormData.label: "Password:"
                text: rust.password
            }

            Button {
                text: qsTr("Login")
                Layout.fillWidth: true
                onClicked: rust.login()
            }

        }

    }

}
// ANCHOR_END: book_main_qml
