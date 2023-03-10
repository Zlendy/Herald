// Basic example with CXX-Qt

#[cxx_qt::bridge]
mod gotify_rustop {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "Rust.Test", qml_version = "0.1")]
    pub struct GotifyRustop {
        #[qproperty]
        number: i32,
        #[qproperty]
        string: QString,
    }

    impl Default for GotifyRustop {
        fn default() -> Self {
            Self {
                number: 0,
                string: QString::from(""),
            }
        }
    }

    impl qobject::GotifyRustop {
        #[qinvokable]
        pub fn increment_number(self: Pin<&mut Self>) {
            let previous = *self.as_ref().number();
            self.set_number(previous + 1);
        }

        #[qinvokable]
        pub fn print(&self, string: &QString, number: i32) {
            println!("Rust number = {number}");
            println!("QML string = '{string}'");
        }
    }
}
