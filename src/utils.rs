use colored::Colorize;

pub fn success(message: impl std::fmt::Display) {
    println!("{}", format!("* {message}").green());
}

pub fn die(message: impl std::fmt::Display) -> ! {
    println!("{}", format!("* {message}").red());
    std::process::exit(1)
}

pub fn die_with_error(message: impl std::fmt::Display, error: impl std::error::Error) -> ! {
    println!("{}", format!("* {message}: {error}").red());
    std::process::exit(1)
}
