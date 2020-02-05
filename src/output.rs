//! Output formatting.
//!
//! Usually chosen via a command line parameter, the available output formats aim for human or machine readability.
//! Every possible output value does have its own enum kind, usually returned by an *Action*s *call* method.
//!
//! # Examples
//!
//! ```
//! # use dsa::output::*;
//! let formatter: Box<dyn Formatter> = Format::HumanReadable.into();
//! let output = formatter.format(&Output::Gauge {name: "health".to_string(), current: 1, max: 10});
//! assert_eq!("current health: 1/10 (10%)",output);
//! ```

use crate::hero::*;

use std::collections::HashMap;

use clap::arg_enum;

arg_enum!
{
	/// Enum of possible formats, can be converted to a *Formatter*.
	///
	/// A formatter can be constructed right from the enum kind.
	/// HumanReadable aims to be readable by humans by indenting, while Json is exclusively machine parsable with one line per object.
	#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Hash,Clone,Copy)]
	pub enum Format
	{
		HumanReadable,
		Json,
	}
}

impl Into<Box<dyn Formatter>> for Format
{
	/// Returns a formatter for the respective enum kind.
	fn into(self) -> Box<dyn Formatter>
	{
		match self
		{
			Format::HumanReadable => Box::new(formats::HumanReadable),
			Format::Json => Box::new(formats::Json),
		}
	}
}

mod formats
{
	#[derive(Copy,Clone,Hash,Debug)]
	pub struct HumanReadable;
	#[derive(Copy,Clone,Hash,Debug)]
	pub struct Json;
}

/// Converts an *Output* to a *String* based on its own rules.
///
/// This trait must be implemented by all *Format*s.
/// Each *Format* may choose how to represent each *Output*.
pub trait Formatter
{
	/// Convert the *Output* to a *String* for further presentation to the user.
	fn format(&self, data: &Output) -> String;
}

impl Formatter for formats::HumanReadable
{
	fn format(&self, data: &Output) -> String
	{
		match data
		{
			Output::Dump(hero) => format!("{:#?}", hero),
			Output::Roll {success,critical,remainder,checks,stat,dice,mods,mut base} =>
			{
				use std::io::Write;
				use std::cmp::Ordering;

				let mut output = vec![];

				let stat_mod = 0.max(mods - base);
				writeln!(output,"base: {} (= {}, {:+} mod)", 0.max(base - mods), base, -mods).unwrap();
				base = 0.max(base - mods);
				if stat_mod > 0
				{
					writeln!(output,"modifier larger than base, reducing stats by {}", stat_mod).unwrap();
				}
				for ((stat,die),check) in stat.iter().zip(dice).zip(checks)
				{
					let stat = stat-stat_mod;
					let sym = match die.cmp(&stat)
					{
						Ordering::Less => '<',
						Ordering::Equal => '=',
						Ordering::Greater => '>',
					};
					writeln!(output,"{:16} | {:2} {} {:2} = {:3} | {:3} => {:3}",
						format!("{:?}", check),
						die, sym, stat, 0.max(die - stat),
						base, base - 0.max(die - stat),
					).unwrap();
					base -= 0.max(die - stat);
				}
				write!(output,"{}{} ({})",
					if *critical { "critical " } else { "" },
					if *success { "success" } else { "failure" },
					remainder,
				).unwrap();
				String::from_utf8_lossy(&output).to_string()
			},
			Output::Gauge {name,current,max} => format!("current {}: {}/{} ({}%)",name,current,max,((100 * *current) as f64 / *max as f64).round()),
		}
	}
}

impl Formatter for formats::Json
{
	fn format(&self, data: &Output) -> String
	{
		use ::json::object;

		match data
		{
			Output::Dump(hero) => object!
			{
				"name" => hero.name.to_string(),
				"health" => hero.health,
				"stamina" => hero.stamina,
				"astral" => hero.astral,
				"basevalues" => hero.basevalues.iter()
					.map(|(key,value)| (format!("{:?}",key),*value))
					.collect::<HashMap<_,_>>(),
				"skills" => hero.skills.iter()
					.map(|(name,(value,rolls))| (name,object!
					{
						"value" => *value,
						"rolls" => rolls.iter().map(|roll| format!("{:?}",roll)).collect::<Vec<_>>(),
					}))
					.collect::<HashMap<_,_>>(),
			}.dump(),
			Output::Roll {success,critical,remainder,checks,stat,dice,mods,base} => object!
			{
				"success" => *success,
				"critical" => *critical,
				"remainder" => *remainder,
				"checks" => checks.iter().map(|value| format!("{:?}",value)).collect::<Vec<_>>(),
				"stat" => &stat[..],
				"dice" => &dice[..],
				"mod" => *mods,
				"base" => *base,
			}.dump(),
			Output::Gauge {name,current,max} => object!
			{
				"name" => name.to_string(),
				"current" => *current,
				"max" => *max,
			}.dump(),
		}
	}
}

/// Types of output generated by an *Action*.
#[derive(Clone,Debug)]
pub enum Output
{
	/// The result of a dice roll for a certain skill.
	Roll
	{
		/// Whether or not it was successful.
		success: bool,
		/// Whether the success/fail is critical.
		critical: bool,
		/// How many points of the skill did remain.
		remainder: isize,
		/// The basevalue to be rolled against.
		base: isize,
		/// The sum of all modifiers placed on the roll.
		mods: isize,
		/// Which *BaseValue*s were rolled against.
		checks: [BaseValue;3],
		/// The corresponding stat values.
		stat: [isize;3],
		/// The raw dice rolls.
		dice: [isize;3],
	},
	/// Any kind of gauge used to keep track during a cli session, e.g. health or stamina.
	Gauge
	{
		/// The name of the tracked attribute.
		name: String,
		/// The current value.
		current: isize,
		/// The maximum value.
		max: isize,
	},
	/// Dump of the hero structure.
	Dump(Hero),
}

