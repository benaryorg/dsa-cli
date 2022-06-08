use crate::error::*;
use crate::hero::*;
use crate::output::Output;
use clap::{Arg, ArgEnum, ArgGroup, ArgMatches, Command};

/// A command line action providing usage and call functionality.
///
/// The `usage()` method returns the [*clap*](https://crates.io/crates/clap) definition of a command line subcommand.
/// The call function is then to be called with the corresponding *ArgMatches* and the *Hero* to act on, returning a *Result* of an arbitrary list of *Outputs*.
pub trait Action
{
	/// Command line argument definition of the subcommand of the action.
	///
	/// This *App* determines determines the *ArgMatches* of the `call()` method.
	fn usage<'a,'b>(&'a self) -> Command<'b>;
	/// A method which maps self, as well as the current *Hero* and the result of the `usage()`-invocation into zero or more *Output*s.
	fn call(&mut self,hero: &Hero, matches: &ArgMatches) -> Result<Vec<Output>>;
}

/// The commands which can be plugged into the command line *App*.
///
/// Each command is an *Action* and provides a `new_action()` method to safe the user some casting.
/// The commands can therefore be used to generate an *App* which has all of these commands as subcommands.
/// The call to the subcommand can then further be processed by the very same command via its `call()` method.
/// This enables each command to modularly contain both its usage definition and the actual processing.
pub mod commands
{
	use super::*;

	mod cli;
	pub use cli::Cli;
	mod roll;
	pub use roll::Roll;
	mod tracker;
	pub use tracker::Tracker;

	/// Simply dumps the hero, ready to be displayed by the used *Formatter*.
	///
	/// Really nothing more to it than that.
	pub struct Dump;

	impl Dump
	{
		fn new_action() -> Box<dyn Action>
		{
			Box::new(Dump)
		}
	}

	impl Action for Dump
	{
		fn usage<'a,'b>(&'a self) -> Command<'b>
		{
			Command::new("dump")
				.about("dump hero information")
		}

		fn call(&mut self,hero: &Hero,_: &ArgMatches) -> Result<Vec<Output>>
		{
			Ok(vec![Output::Dump(hero.clone())])
		}
	}
}

