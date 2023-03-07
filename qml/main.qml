import QtQuick 2.6
import QtQuick.Window 2.0
import Rust 0.1

Window {
    visible: true
    // Instantiate the rust struct
    Test {
        id: test;
        // Set a property
        name: "world!"
    }
    TextInput {
        anchors.centerIn: parent
        // Call a method
        text: test.compute_greetings("Hello")
        Keys.onReturnPressed: {
            text = test.compute_greetings(text)
        }

    }
}