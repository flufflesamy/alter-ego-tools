#![allow(unused_macros)]
/// Wrapper around [`Option::ok_or_else`] with error message
///
/// Transforms `Option<T>` to `Result<T, E>`.
/// On `Some(T)`, returns `T`.
/// On None, returns an [`anyhow::Error`] and emits a [`tracing::error`] message.
///
/// Use only on user-facing code.
///
/// # Examples
///
/// ```
/// let result = ok_or!(Some(42), "Value is missing");
/// assert_eq!(result, Ok(42));
/// ```
macro_rules! ok_or {
    ($e:expr, $msg:literal) => {
        $e.ok_or_else(|| {
            tracing::error!("{}", $msg);
            anyhow::anyhow!($msg)
        })
    };
}

/// Shows a toast notification using the window's `win.show-toast` action.
///
/// # Parameters
///
/// - `obj`: The Widget object.
/// - `msg`: The toast message.
///
/// # Examples
///
/// ```
/// // Used in wrapper
/// toast!(self, "Hello, world!")
///
/// // Used inside imp()
/// toast!(self.obj(), "Hello, again!");
/// ```
macro_rules! toast {
    ($obj:expr, $msg:literal) => {
        $obj.activate_action("win.show-toast", Some(&$msg.to_variant()))
            .map_or_else(|e| error!("Could not show toast: {e}."), |_| ())
    };
}

/// Shows a toast notification using the window's `win.show-toast` action.
/// Also logs a [`tracing::error`] message.
///
/// Works with any type that implements [`std::error::Error`].
///
/// # Parameters
///
/// - `$self`: The Widget object.
/// - `$msg`: The error message literal.
/// - `$e`: The error.
///
/// # Examples
///
/// ```
/// toast_error!(self, "Something bad happened", err);
/// ```
macro_rules! toast_error {
    ($self:expr, $msg:literal, $e:expr) => {{
        let err = $e;
        toast!($self, $msg);
        tracing::error!("{}: {err}", $msg)
    }};
}

/// Shows a toast notification using the window's `win.show-toast` action.
/// Also logs a [`tracing::warn`] message.
///
/// # Parameters
///
/// - `self`: The Widget object.
/// - `msg`: The warning message.
///
/// # Examples
///
/// ```
/// toast_warn!(self, "Something is not right");
/// ```
macro_rules! toast_warn {
    ($self:expr, $msg:literal) => {
        toast!($self, $msg);
        tracing::warn!("$msg")
    };
}

pub(crate) use ok_or;
pub(crate) use toast;
pub(crate) use toast_error;
pub(crate) use toast_warn;
