use {
    crate::*,
    argh::FromArgs,
};

#[derive(Debug, FromArgs)]
/// Backups Fediverse accounts - Source at https://github.com/Canop/fediback
pub struct Args {
    /// print the version
    #[argh(switch, short = 'v')]
    pub version: bool,

    #[argh(subcommand)]
    pub command: Option<ArgsCommand>,

    /// tell what files are modified
    #[argh(switch)]
    pub verbose: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum ArgsCommand {
    Add(AddCommand),
    Remove(RemoveCommand),
    Update(UpdateCommand),
    Check(CheckCommand),
    List(ListCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// check the existence of a user
#[argh(subcommand, name = "check")]
pub struct CheckCommand {
    #[argh(positional)]
    pub user: UserRef,
}

#[derive(FromArgs, PartialEq, Debug)]
/// add a user to the backup list
#[argh(subcommand, name = "add")]
pub struct AddCommand {
    #[argh(positional)]
    pub user: UserRef,
}

#[derive(FromArgs, PartialEq, Debug)]
/// remove a user from the backup list (doesn't remove saved data)
#[argh(subcommand, name = "remove")]
pub struct RemoveCommand {
    #[argh(positional)]
    pub user: UserRef,
}

#[derive(FromArgs, PartialEq, Debug)]
/// fetch data of all users
#[argh(subcommand, name = "update")]
pub struct UpdateCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// list users of the backup list
#[argh(subcommand, name = "list")]
pub struct ListCommand {}

/// An optional boolean for use in Argh
#[derive(Debug, Clone, Copy, Default)]
pub struct BoolArg(Option<bool>);

impl BoolArg {
    pub fn value(self) -> Option<bool> {
        self.0
    }
}

impl argh::FromArgValue for BoolArg {
    fn from_arg_value(value: &str) -> std::result::Result<Self, String> {
        match value.to_lowercase().as_ref() {
            "auto" => Ok(BoolArg(None)),
            "yes" => Ok(BoolArg(Some(true))),
            "no" => Ok(BoolArg(Some(false))),
            _ => Err(format!("Illegal value: {:?}", value)),
        }
    }
}
