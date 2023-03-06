use cstr::cstr;
use qmetaobject::prelude::*;

mod implementation;

qrc!(my_resource,
    "qml" as "" {
        "main.qml",
    }
);

fn main() {
    my_resource();
    qml_register_type::<implementation::Test>(cstr!("Rust"), 0, 1, cstr!("Test"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/main.qml".into());
    engine.exec();
}
