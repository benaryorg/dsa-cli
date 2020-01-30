use crate::hero::*;

mod formats
{
	#[derive(Copy,Clone,Hash,Debug)]
	pub struct HumanReadable;
}

pub fn humanreadable() -> Box<dyn Formatter>
{
	Box::new(formats::HumanReadable)
}

pub trait Formatter
{
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
				for ((stat,die),check) in stat.iter().zip(dice.iter()).zip(checks.iter())
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

#[derive(Clone,Debug)]
pub enum Output
{
	Roll
	{
		success: bool,
		critical: bool,
		remainder: isize,
		base: isize,
		mods: isize,
		checks: [BaseValue;3],
		stat: [isize;3],
		dice: [isize;3],
	},
	Gauge
	{
		name: String,
		current: isize,
		max: isize,
	},
	Dump(Hero),
}

