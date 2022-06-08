use super::*;

use rand::distributions::{Distribution, Uniform};

pub struct Roll;

impl Roll
{
	pub fn new_action() -> Box<dyn Action>
	{
		Box::new(Roll)
	}
}

/// Rolls the dice for a certain skill of the *Hero*, supports modifiers.
///
/// # Examples
///
/// ```
/// # use dsa::Quality::*;
/// # use dsa::Hero;
/// # use dsa::output::Output;
/// # use dsa::commands::Roll;
/// let mut roll = Roll::new_action();
/// # let mut hero = Hero::default();
/// hero.qualities.extend(
///     vec![
///         (Agility,1),
///         (Dexterity,2),
///         (Strength,3),
///     ]
/// );
/// hero.skills.insert("bogen".to_string(),(4,[Agility,Dexterity,Strength]));
/// let matches = roll.usage().get_matches_from(&["roll","-m","-5","--mod","3","bogen"]);
/// let output = roll.call(&hero,&matches).unwrap();
/// assert_eq!(1,output.len());
/// 
/// if let Output::Roll { base, stat, mods, .. } = &output[0] {
///     assert_eq!(4, *base);
///     assert_eq!(&[1,2,3], stat);
///     assert_eq!(-2, *mods);
/// }
/// # else {
/// #     panic!("unexpected output");
/// # }
/// ```
impl Action for Roll
{
	fn usage<'a,'b>(&'a self) -> Command<'b>
	{
		Command::new("roll")
			.about("roll for a skill")
			.arg
				( Arg::new("modifier")
				.short('m')
				.long("modifier")
				.alias("mod")
				.help("modification as positive (bad) or negative (good) integer")
				.allow_hyphen_values(true)
				.takes_value(true)
				.multiple_occurrences(true)
				.number_of_values(1)
				)
			.arg
				( Arg::new("skill")
				.value_name("SKILL")
				.help("the skills to test")
				.takes_value(true)
				.multiple_occurrences(true)
				.required(true)
				)
	}

	fn call(&mut self, hero: &Hero, matches: &ArgMatches) -> Result<Vec<Output>>
	{
		let d20: Uniform<_> = (1..21).into();
		let mut rng = rand::thread_rng();

		matches.values_of("skill")
			.unwrap()
			.map(|skill|
			{
				let mods = matches.values_of("modifier")
					.map(|mods| mods
						.map(|modi| Ok(modi.parse()?))
						.collect::<Result<Vec<isize>>>()
						.map(|mods| mods.into_iter().sum())
					)
					.unwrap_or(Ok(0))?;

				// TODO: use custom error
				let (base,values_enum) = hero.skills.get(&skill.to_lowercase()).ok_or_else(|| format!("unknown skill '{}'", skill))?;

				let values =
				{
					let mut iter = values_enum.iter()
						.map(|value| -> Result<isize>
						{
							Ok(*hero.qualities.get(value).ok_or("cannot roll unknown quality")?)
						});
					[
						iter.next().ok_or("quality retrieval is broken")??,
						iter.next().ok_or("quality retrieval is broken")??,
						iter.next().ok_or("quality retrieval is broken")??,
					]
				};
				let rolls =
				{
					let mut iter = d20.sample_iter(&mut rng);
					[
						iter.next().ok_or("rng sampling is broken")?,
						iter.next().ok_or("rng sampling is broken")?,
						iter.next().ok_or("rng sampling is broken")?,
					]
				};
				let num_20 = rolls.iter().filter(|i| **i == 20).count();
				let num_1 = rolls.iter().filter(|i| **i == 1).count();
				let result = 0.max(base - mods) + values.iter()
					.map(|stat| stat + 0.min(base - mods))
					.zip(&rolls)
					.map(|(stat,die)| (stat-die).min(0))
					.sum::<isize>();

				Ok(Output::Roll
				{
					success: (num_20 < 2) && (result >= 0 || num_1 > 1),
					critical: (num_20 > 1) || (num_1 > 1),
					dice: rolls,
					checks: *values_enum,
					stat: values,
					remainder: result,
					base: *base,
					mods,
				})
			})
			.collect()
	}
}

