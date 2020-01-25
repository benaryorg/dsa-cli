use super::*;

pub struct Health
{
	max: isize,
	current: isize,
}

impl Health
{
	pub fn new_action(health: isize) -> Box<dyn Action>
	{
		Box::new(Health
		{
			max: health,
			current: health,
		})
	}
}

impl Action for Health
{
	fn usage<'a,'b>(&'a self) -> App<'b,'b>
	{
		SubCommand::with_name("health")
			.about("track your current health")
			.arg
				( Arg::with_name("get")
				.long("get")
				.short("g")
				.help("get health status (default)")
				)
			.arg
				( Arg::with_name("set")
				.long("set")
				.short("s")
				.help("set current health")
				.takes_value(true)
				)
			.arg
				( Arg::with_name("add")
				.long("add")
				.help("add to current health")
				.takes_value(true)
				)
			.arg
				( Arg::with_name("sub")
				.long("sub")
				.help("subtract of current health")
				.takes_value(true)
				)
			.arg
				( Arg::with_name("max")
				.long("max")
				.short("m")
				.help("change max health instead of current")
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

		Ok(Output::Health
		{
			current: self.current,
			max: self.max,
		})
	}
}

