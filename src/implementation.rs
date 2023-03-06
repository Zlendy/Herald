use qmetaobject::prelude::*;

// The `QObject` custom derive macro allows to expose a class to Qt and QML
#[derive(QObject, Default)]
pub struct Test {
    // Specify the base class with the qt_base_class macro
    base: qt_base_class!(trait QObject),
    // Declare `name` as a property usable from Qt
    name: qt_property!(QString; NOTIFY name_changed),
    // Declare a signal
    name_changed: qt_signal!(),
    // And even a slot
    compute_greetings: qt_method!(fn compute_greetings(&self, verb: String) -> QString {
        format!("{}, {}", verb, self.name.to_string()).into()
    })
}