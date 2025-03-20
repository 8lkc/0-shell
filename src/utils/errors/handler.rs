use {
    super::Selector,
    std::io
};

pub struct Error;

impl Error {
    pub fn throw(kind: io::ErrorKind, arguments: &[&str]) -> io::Error {
        use io::ErrorKind as EK;
        match kind {
            EK::InvalidInput        => Selector::missing_command(arguments[0]),
            EK::NotSeekable         => Selector::missing_operand(arguments[0]),
            EK::NotFound            => Selector::missing_item(arguments),
            EK::ArgumentListTooLong => Selector::too_many_args(arguments[0]),
            EK::HostUnreachable     => Selector::missing_parent(arguments),
            EK::AlreadyExists       => Selector::avoid_clone(arguments),
            EK::UnexpectedEof       => Selector::unexpected_argument(arguments),
            _ => io::Error::new(kind, "Unknown error ⚠️")
        }
    }
}
