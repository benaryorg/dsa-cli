use super::*;

pub struct Tracker<'a>
{
	name: &'a str,
	max: isize,
	current: isize,
}

impl<'a> Tracker<'a>
{
	pub fn new_action(name: &'static str,current: isize,max: isize) -> Box<dyn Action>
	{
		Box::new(Tracker
		{
			name,
			max,
			current,
		})
	}
}

impl Action for Tracker<'_>
{
	fn usage<'a,'b>(&'a self) -> App<'b,'b>
	{
		SubCommand::with_name(self.name)
			.about("track the current value")
			.arg
				( Arg::with_name("get")
				.long("get")
				.short("g")
				.help("get current value (default)")
				)
			.arg
				( Arg::with_name("set")
				.long("set")
				.short("s")
				.help("set current value")
				.takes_value(true)
				)
			.arg
				( Arg::with_name("add")
				.long("add")
				.help("add to current value")
				.takes_value(true)
				)
			.arg
				( Arg::with_name("sub")
				.long("sub")
				.help("subtract of current value")
				.takes_value(true)
				)
			.arg
				( Arg::with_name("max")
				.long("max")
				.short("m")
				.help("change max value instead of current")
				)
			.group
				( ArgGroup::with_name("action")
				. args(&["get","set","sub","add"])
				)
	}

	fn call(&mut self, _: &Hero, matches: &ArgMatches) -> Result<Output>
	{
		if matches.is_present("action") && !matches.is_present("get")
		{
			let target = if matches.is_present("max") { &mut self.max } else { &mut self.current };
			*target = match [matches.value_of("set"),matches.value_of("add"),matches.value_of("sub")]
			{
				[Some(value),None,None] => value.parse::<isize>()?,
				[None,Some(value),None] => *target+value.parse::<isize>()?,
				[None,None,Some(value)] => *target-value.parse::<isize>()?,
				_ => unreachable!(),
			}
		}

		// keep it within bounds 0 <= current <= max
		self.max = self.max.max(0);
		self.current = self.current.max(0).min(self.max);

		Ok(Output::Gauge
		{
			name: self.name.to_string(),
			current: self.current,
			max: self.max,
		})
	}
}

