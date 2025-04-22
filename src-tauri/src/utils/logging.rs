use std::fmt;

#[derive(Debug)]
pub enum Type {
    Window,
    Core,
    Engine,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Window => write!(f, "[Window]"),
            Type::Core => write!(f, "[Core]"),
            Type::Engine => write!(f, "[Engine]"),
        }
    }
}

/// wrap the anyhow error
/// transform the error to String
/// code from clash verge
#[macro_export]
macro_rules! wrap_err {
    ($stat: expr) => {
        match $stat {
            Ok(a) => Ok(a),
            Err(err) => {
                log::error!(target: "app", "{}", err.to_string());
                Err(format!("{}", err.to_string()))
            }
        }
    };
}

#[macro_export]
macro_rules! logging {
    // Version with println (supports formatted arguments)
    ($level:ident, $type:expr, true, $($arg:tt)*) => {
        println!("{} {}", $type, format_args!($($arg)*));
        log::$level!(target: "app", "{} {}", $type, format_args!($($arg)*));
    };

    // Version with println (use false explicitly to disable printing)
    ($level:ident, $type:expr, false, $($arg:tt)*) => {
        log::$level!(target: "app", "{} {}", $type, format_args!($($arg)*));
    };

    // Version without print parameter (default is no printing)
    ($level:ident, $type:expr, $($arg:tt)*) => {
        log::$level!(target: "app", "{} {}", $type, format_args!($($arg)*));
    };
}
