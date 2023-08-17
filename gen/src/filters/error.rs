use liquid_core::Error;

pub(super) fn invalid_input<S>(cause: S) -> Error
where
    S: Into<liquid_core::model::KString>,
{
    Error::with_msg("Invalid input").context("cause", cause)
}

pub(super) fn invalid_argument<S>(argument: S, cause: S) -> Error
where
    S: Into<liquid_core::model::KString>,
{
    Error::with_msg("Invalid argument")
        .context("argument", argument)
        .context("cause", cause)
}

pub(super) fn invalid_fmt(e: std::fmt::Error) -> Error {
    Error::with_msg(e.to_string())
}
