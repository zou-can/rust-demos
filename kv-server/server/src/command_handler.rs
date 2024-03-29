use kv_core::command::{CommandRequest, CommandResponse, Del, Entry, Get, Mget, Mset, Set, Value};
use kv_core::command::command_request::Command;
use kv_core::command::value::Val;
use kv_core::error::KvError;

use crate::storage::Storage;

/// main process for commands
pub fn handle(request: CommandRequest, storage: &impl Storage) -> CommandResponse {
    let execution_result = match request.command {
        Some(command) => {
            match command {
                Command::Get(exe) => exe.execute(storage),
                Command::Mget(exe) => exe.execute(storage),
                Command::Set(exe) => exe.execute(storage),
                Command::Mset(exe) => exe.execute(storage),
                Command::Del(exe) => exe.execute(storage),
            }
        }
        None => CommandResponse::from(KvError::InvalidCommand),
    };

    // TODO: log
    // TODO: publish event

    execution_result
}

/// Command execution process
trait ExecutableCommand {
    fn execute(self, storage: &impl Storage) -> CommandResponse;
}

impl ExecutableCommand for Get {
    fn execute(self, storage: &impl Storage) -> CommandResponse {
        CommandResponse::from(storage.get(&self.key))
    }
}

impl ExecutableCommand for Mget {
    fn execute(self, storage: &impl Storage) -> CommandResponse {
        CommandResponse::from(storage.mget(&self.keys))
    }
}

impl ExecutableCommand for Set {
    fn execute(self, storage: &impl Storage) -> CommandResponse {
        match self.entry {
            Some(
                Entry {
                    key: k,
                    value: Some(
                        Value {
                            val: Some(Val::String(v))
                        }
                    )
                }
            ) => CommandResponse::from(storage.set(k, v)),
            _ => CommandResponse::from(KvError::InvalidCommand),
        }
    }
}

impl ExecutableCommand for Mset {
    fn execute(self, storage: &impl Storage) -> CommandResponse {
        CommandResponse::from(storage.mset(self.entries))
    }
}

impl ExecutableCommand for Del {
    fn execute(self, storage: &impl Storage) -> CommandResponse {
        CommandResponse::from(storage.del(&self.keys))
    }
}


