// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_main_qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

import Rust.Test 0.1

Window {
    height: 480
    title: qsTr("Gotify Rustop")
    visible: true
    width: 640

    GotifyRustop {
        id: gotifyRustop
        number: 1
        string: "number = " + gotifyRustop.number
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: "Number: " + gotifyRustop.number
        }

        Label {
            text: "String: " + gotifyRustop.string
        }

        Button {
            text: "number++"

            onClicked: gotifyRustop.incrementNumber()
        }

        Button {
            text: "println!()"

            onClicked: gotifyRustop.print(gotifyRustop.string, gotifyRustop.number)
        }
    }
}
// ANCHOR_END: book_main_qml
