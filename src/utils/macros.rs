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
            error!("{}", $msg);
            anyhow::anyhow!($msg)
        })
    };
}

macro_rules! toast {
    ($e:expr, $msg:literal) => {
        $e.activate_action("win.show-toast", Some($msg.to_variant()))
            .map_or_else(|e| error!("Could not show toast: {e}."), |_| ());
    };
}

macro_rules! error_toast {
    ($self:expr, $e:expr) => {
        if let Err(err) = $e {
            toast!($self, &err.to_string());
        }
    };
}

pub(crate) use ok_or;
pub(crate) use toast;
