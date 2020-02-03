use super::*;

/// A generic tracker for a property during a cli session.
///
/// # Examples
///
/// ```
/// # use dsa::commands::Tracker;
/// # use dsa::output::Output;
/// let mut health = Tracker::new_action("health",10,10);
/// let matches = health.usage().get_matches_from_safe(&["health","-s","5"]).unwrap();
/// # let hero = Default::default();
/// let output = health.call(&hero,&matches).unwrap();
/// assert_eq!(output.len(), 1);
/// if let Output::Gauge { name, current, max } = &output[0] {
///     assert_eq!("health", name);
///     assert_eq!(5, *current);
///     assert_eq!(10, *max);
/// }
/// # else {
/// #     panic!("unexpected output");
/// # }
/// ```
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

	fn call(&mut self, _: &Hero, matches: &ArgMatches) -> Result<Vec<Output>>
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

		Ok(vec![Output::Gauge
		{
			name: self.name.to_string(),
			current: self.current,
			max: self.max,
		}])
	}
}

