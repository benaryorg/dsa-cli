use crate::error::*;
use crate::hero::*;
use crate::output::Output;
use super::{App, Arg, ArgMatches, SubCommand};

pub use commands::*;

pub trait Action
{
	fn call(&self,hero: &Hero, matches: &ArgMatches) -> Result<Output>;
	fn usage(&self) -> App;
}

mod commands
{
	use super::*;

	mod cli;
	pub use cli::Cli;
	mod roll;
	pub use roll::Roll;

	pub struct Dump;

	impl Action for Dump
	{
		fn usage(&self) -> App
		{
			SubCommand::with_name("dump")
				.about("dump hero information")
		}

		fn call(&self,hero: &Hero,_: &ArgMatches) -> Result<Output>
		{
			Ok(Output::Dump(hero.clone()))
		}
	}
}

