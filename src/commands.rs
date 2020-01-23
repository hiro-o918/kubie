use structopt::clap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(global_setting(clap::AppSettings::VersionlessSubcommands))]
pub enum Kubie {
    /// Spawn a shell in the given context. The shell is isolated from other shells.
    /// Kubie shells can be spawned recursively without any issue.
    #[structopt(name = "ctx")]
    Context {
        /// Specify in which namespace of the context the shell is spawned.
        #[structopt(short = "n", long = "--namespace")]
        namespace_name: Option<String>,
        /// Name of the context in which to spawn the shell.
        context_name: Option<String>,
    },

    /// Change the namespace in which the current shell operates. The namespace change does
    /// not affect other shells.
    #[structopt(name = "ns")]
    Namespace {
        /// Name of the namespace to change to.
        namespace_name: Option<String>,
    },

    /// View info about the current kubie shell, such as the context name and the
    /// current namespace.
    #[structopt(name = "info")]
    Info(KubieInfo),

    /// Execute a command inside of the given context and namespace.
    #[structopt(name = "exec", setting(clap::AppSettings::TrailingVarArg))]
    Exec {
        /// Name of the context in which to run the command.
        context_name: String,
        /// Namespace in which to run the command. This is mandatory to avoid potential errors.
        namespace_name: String,
        /// Command to run as well as its arguments.
        args: Vec<String>,
    },
}

#[derive(Debug, StructOpt)]
pub struct KubieInfo {
    #[structopt(subcommand)]
    pub kind: KubieInfoKind,
}

#[derive(Debug, StructOpt)]
/// Type of info the user is requesting.
pub enum KubieInfoKind {
    /// Get the current shell's context name.
    #[structopt(name = "ctx")]
    Context,
    /// Get the current shell's namespace name.
    #[structopt(name = "ns")]
    Namespace,
    /// Get the current depth of contexts.
    #[structopt(name = "depth")]
    Depth,
}
