use crate::{
    cmd,
    resp::{BulkString, FromValue, Value},
    Command, CommandSend, Future, SingleArgOrCollection,
};

/// A group of Redis commands related to Scripting and Functions
/// # See Also
/// [Redis Scripting and Functions Commands](https://redis.io/commands/?group=scripting)
/// [Scripting with LUA](https://redis.io/docs/manual/programmability/eval-intro/)
/// [Functions](https://redis.io/docs/manual/programmability/functions-intro/)
pub trait ScriptingCommands: CommandSend {
    /// This command copies the value stored at the source key to the destination key.
    ///
    /// # See Also
    /// [https://redis.io/commands/eval/](https://redis.io/commands/eval/)
    fn eval<S>(&self, script: S) -> Eval<Self>
    where
        S: Into<BulkString>,
    {
        Eval {
            scripting_commands: &self,
            cmd: cmd("EVAL").arg(script),
            keys_added: false,
        }
    }

    /// Evaluate a script from the server's cache by its SHA1 digest.
    ///
    /// # See Also
    /// [https://redis.io/commands/eval/](https://redis.io/commands/eval/)
    fn evalsha<S>(&self, sha1: S) -> Eval<Self>
    where
        S: Into<BulkString>,
    {
        Eval {
            scripting_commands: &self,
            cmd: cmd("EVALSHA").arg(sha1),
            keys_added: false,
        }
    }

    /// Load a script into the scripts cache, without executing it.
    ///
    /// # Return
    /// The SHA1 digest of the script added into the script cache.
    ///
    /// # See Also
    /// [https://redis.io/commands/script-load/](https://redis.io/commands/script-load/)
    fn script_load<S, V>(&self, script: S) -> Future<'_, V>
    where
        S: Into<BulkString>,
        V: FromValue,
    {
        self.send_into(cmd("SCRIPT").arg("LOAD").arg(script))
    }
}

/// Builder for the [eval](crate::ScriptingCommands::eval) command
pub struct Eval<'a, T: ScriptingCommands + ?Sized> {
    scripting_commands: &'a T,
    cmd: Command,
    keys_added: bool,
}

impl<'a, T: ScriptingCommands> Eval<'a, T> {
    pub fn new(scripting_commands: &'a T, cmd: Command) -> Self {
        Self {
            scripting_commands,
            cmd,
            keys_added: false,
        }
    }

    /// All the keys accessed by the script.
    pub fn keys<K, C>(self, keys: C) -> Self
    where
        K: Into<BulkString>,
        C: SingleArgOrCollection<K>,
    {
        Self {
            scripting_commands: self.scripting_commands,
            cmd: self.cmd.arg(keys.num_args()).arg(keys),
            keys_added: true,
        }
    }

    /// Additional input arguments that should not represent names of keys.
    pub fn args<A, C>(self, args: C) -> Self
    where
        A: Into<BulkString>,
        C: SingleArgOrCollection<A>,
    {
        let cmd = if !self.keys_added {
            // numkeys = 0
            self.cmd.arg(0).arg(args)
        } else {
            self.cmd.arg(args)
        };

        Self {
            scripting_commands: self.scripting_commands,
            cmd: cmd,
            keys_added: true,
        }
    }

    /// execute with no option
    pub fn execute(self) -> Future<'a, Value> {
        self.scripting_commands.send_into(self.cmd)
    }
}
