use crate::error::*;
use crate::hero::*;
use crate::output::Output;
use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};

pub use commands::*;

pub trait Action
{
	fn usage<'a,'b>(&'a self) -> App<'b,'b>;
	fn call(&mut self,hero: &Hero, matches: &ArgMatches) -> Result<Output>;
}

mod commands
{
	use super::*;

	mod cli;
	pub use cli::Cli;
	mod roll;
	pub use roll::Roll;
	mod tracker;
	pub use tracker::Tracker;

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
		fn usage<'a,'b>(&'a self) -> App<'b,'b>
		{
			SubCommand::with_name("dump")
				.about("dump hero information")
		}

		fn call(&mut self,hero: &Hero,_: &ArgMatches) -> Result<Output>
		{
			Ok(Output::Dump(hero.clone()))
		}
	}
}

