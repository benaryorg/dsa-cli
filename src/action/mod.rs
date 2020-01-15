use crate::error::*;
use super::Hero;
use super::{App, ArgMatches, SubCommand};

pub use commands::*;

pub trait Action
{
	fn call(&self,hero: &Hero, matches: &ArgMatches) -> Result<Option<String>>;
	fn usage(&self) -> App;
}

mod commands
{
	use super::*;

	mod cli;
	pub use cli::Cli;

	pub struct Dump;

	impl Action for Dump
	{
		fn usage(&self) -> App
		{
			SubCommand::with_name("dump")
				.about("dump hero information")
		}

		fn call(&self,hero: &Hero,_: &ArgMatches) -> Result<Option<String>>
		{
			Ok(Some(format!("{:#?}",hero)))
		}
	}
}

