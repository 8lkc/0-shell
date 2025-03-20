use {
    crate::Tool,
    std::io
};

pub(super) struct Selector;

impl Selector {
    pub(super) fn avoid_clone(arguments: &[&str]) -> io::Error {
        let item_nature = match arguments[0] {
            "mkdir:" => "directory",
            "touch:" => "file",
            _ => "Unknown item"
        };
        io::Error::new(
            io::ErrorKind::AlreadyExists, 
            format!(
                "{} cannot create {} '{}': {}",
                Tool::boldify(&Tool::italicify(arguments[0])), item_nature,
                Tool::boldify(&Tool::colorize(arguments[1], (251, 177, 60))),
                Tool::colorize(&format!("{} exists", item_nature), (220, 45, 34))
            )
        )
    }

    pub(super) fn missing_command(command: &str) -> io::Error {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Command '{}' not found", Tool::boldify(&Tool::italicify(&Tool::colorize(command, (220, 45, 34)))))
        )
    }

    pub(super) fn missing_item(arguments: &[&str]) -> io::Error {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "{} cannot access '{}': {}",
                Tool::boldify(&Tool::italicify(arguments[0])),
                Tool::boldify(&Tool::colorize(arguments[1], (251, 177, 60))),
                Tool::colorize("No such file or directory", (220, 45, 34))
            )
        )
    }

    pub(super) fn missing_operand(command: &str) -> io::Error {
        io::Error::new(
            io::ErrorKind::NotSeekable, 
            format!(
                "{} {}",
                Tool::boldify(&Tool::italicify(command)),
                Tool::colorize("missing operand", (220, 45, 34))
            )
        )
    }

    pub(super) fn missing_parent(arguments: &[&str]) -> io::Error {
        let item_nature = match arguments[0] {
            "mkdir:" => "directory",
            "touch:" => "file",
            _ => "Unknown item"
        };
        io::Error::new(
            io::ErrorKind::HostUnreachable, 
            format!(
                "{} cannot create {} '{}': {}",
                Tool::boldify(&Tool::italicify(arguments[0])), item_nature,
                Tool::boldify(&Tool::colorize(arguments[1], (251, 177, 60))),
                Tool::colorize("No such file or directory", (220, 45, 34))
            )
        )
    }

    pub(super) fn too_many_args(command: &str) -> io::Error {
        io::Error::new(
            io::ErrorKind::ArgumentListTooLong, 
            format!(
                "{} {}",
                Tool::boldify(&Tool::italicify(command)),
                Tool::colorize("too many arguments", (220, 45, 34))
            )
        )
    }

    pub(super) fn unexpected_argument(arguments: &[&str]) -> io::Error {
        io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!(
                "{} unexpected argument '{}' found",
                Tool::boldify(&Tool::italicify(arguments[0])),
                Tool::boldify(&Tool::colorize(&format!("-{}", arguments[1]), (251, 177, 60)))
            )
        )
    }
}
