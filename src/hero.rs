use crate::error::*;

use error_chain::bail;

use std::collections::HashMap;

/// Public container for the hero attributes.
///
/// The fields themselves are documented below.
/// All fields are public to allow for easy data access.
/// *Hero* implements *std::str::FromStr* so `parse()` can be called on a `str` containing the xml export of the [Heldensoftware](https://www.helden-software.de/).
///
/// # Examples
///
/// ```
/// use dsa::Hero;
///
/// let hero = "<helden>…</helden>".parse::<Hero>();
/// assert!(hero.is_err());
/// ```
#[derive(Debug,Clone,Default)]
pub struct Hero
{
	/// Hero's name, e.g. Elvenor Elvington
	pub name: String,
	/// Maximum health as per base+(KO+KO+KK)/2
	pub health: isize,
	/// Maximum stamina as per base+(MU+KO+GE)/2
	pub stamina: isize,
	/// Maximum astral points as per base+(MU+IN+CH)/2
	pub astral: isize,
	/// The basic attributes (MU, KL, etc.)
	pub qualities: HashMap<Quality,isize>,
	/// All skills documented in the xml, as a map of name to skill-level and attributes to roll on
	pub skills: HashMap<String,(isize,[Quality;3])>,
}

impl std::str::FromStr for Hero
{
	type Err = Error;

	fn from_str(input: &str) -> Result<Self>
	{
		let document = roxmltree::Document::parse(input).chain_err(|| "xml document could not be parsed")?;

		// get the hero
		let root = document.root_element();
		if ! root.has_tag_name("helden")
		{
			bail!("unknown root element");
		}
		let held = root.children()
			.find(|child| child.has_tag_name("held"))
			.ok_or("root element does not contain held element")?;

		// get the qualities
		let qualities = held.children()
			.find(|elem| elem.has_tag_name("eigenschaften"))
			.ok_or("hero does not have base values")?;
		let mut qualities: HashMap<_,isize> = qualities.children()
			.map(|elem|
				{
					let name = elem.attribute("name").unwrap_or("").to_lowercase();
					let base = elem.attribute("value").and_then(|i| i.parse().ok()).unwrap_or(0);
					let modi = elem.attribute("mod").and_then(|i| i.parse().ok()).unwrap_or(0);
					(name,base + modi)
				})
			.collect();
		let health_base = qualities.remove("lebensenergie").unwrap_or(0);
		let stamina_base = qualities.remove("ausdauer").unwrap_or(0);
		let astral_base = qualities.remove("astralenergie").unwrap_or(0);
		let qualities: HashMap<_,_> = qualities.into_iter()
			.filter_map(|(k,v)|
			{
				k.parse::<Quality>()
					.map(|k| (k,v))
					.ok()
			})
			.collect();

		use Quality::*;

		let skills: HashMap<_,_> = held.children()
			.filter(|elem| elem.has_tag_name("talentliste") || elem.has_tag_name("zauberliste"))
			.flat_map(|elem| elem.children())
			.map(|elem| -> Result<(String,(isize,[Quality;3]))>
				{
					let name = elem.attribute("name").unwrap_or("").to_lowercase();
					let value = elem.attribute("value").and_then(|i| i.parse().ok()).unwrap_or(0);
					let probe = elem.attribute("probe").ok_or("probe not parsable")?;
					let probe = probe.trim().trim_start_matches('(').trim_end_matches(')');
					let probe = probe.split('/').map(|quality| quality.parse::<Quality>()).collect::<Result<Vec<_>>>()?;
					if probe.len() != 3
					{
						bail!("skill does not have three 'probe'");
					}
					Ok((name,(value,[probe[0],probe[1],probe[2]])))
				})
			.filter_map(Result::ok)
			.collect();

		let hero = Hero
		{
			name: held.attribute("name").ok_or("hero does not have a name")?.into(),
			health:
			{
				let ko = *qualities.get(&Constitution).unwrap_or(&0);
				let kk = *qualities.get(&Strength).unwrap_or(&0);
				((ko as f32 * 2.0 + kk as f32) / 2.0).round() as isize + health_base
			},
			stamina:
			{
				let mu = *qualities.get(&Courage).unwrap_or(&0);
				let ko = *qualities.get(&Constitution).unwrap_or(&0);
				let ge = *qualities.get(&Agility).unwrap_or(&0);
				((mu as f32 + ko as f32 + ge as f32) / 2.0).round() as isize + stamina_base
			},
			astral:
			{
				let mu = *qualities.get(&Courage).unwrap_or(&0);
				let int = *qualities.get(&Intuition).unwrap_or(&0);
				let ch = *qualities.get(&Charisma).unwrap_or(&0);
				((mu as f32 + int as f32 + ch as f32) / 2.0).round() as isize + astral_base
			},
			qualities,
			skills,
		};

		Ok(hero)
	}
}

/// Base values of a Hero.
///
/// The type implements *std::str::FromStr* and individual items are annotated with the possible values they are parsed from.
/// Parsing is case insensitive, that is the to-be-parsed string is downcased before matching.
///
/// # Examples
///
/// ```
/// # use dsa::{Quality, Hero};
/// let hero = Hero::default();
/// let int_fromstr = "KluGHeiT".parse().unwrap();
/// assert_eq!(Quality::Wisdom,int_fromstr);
/// let int = hero.qualities.get(&int_fromstr).unwrap_or(&0);
/// assert_eq!(0, *int);
/// ```
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Hash,Clone,Copy)]
pub enum Quality
{
	/// MU, Mut, courage
	Courage,
	/// KL, Klugheit, wisdom
	Wisdom,
	/// IN, Intuition
	Intuition,
	/// CH, Charisma
	Charisma,
	/// FF, Fingerfertigkeit, dexterity
	Dexterity,
	/// GE, Gewandtheit, agility
	Agility,
	/// KO, Konstitution, constitution
	Constitution,
	/// KK, Körperkraft, strength
	Strength,
	/// GS, Sozialstatus, socialstatus
	SocialStatus,
	/// MR, Magieresistenz, magicrestistance
	MagicResistance,
	/// INI, Initiative
	Initiative,
	/// AT, Attacke, nahkampf, combat, closecombat
	CloseCombat,
	/// PA, Parrieren, parry
	Parry,
	/// FK, Fernkampf, ranged, rangedcombat
	RangedCombat,
}

impl std::str::FromStr for Quality
{
	type Err = Error;

	fn from_str(input: &str) -> Result<Self>
	{
		use Quality::*;
		match input.to_lowercase().as_str()
		{
			"mu" | "mut" | "courage" => Ok(Courage),
			"kl" | "klugheit" | "wisdom" => Ok(Wisdom),
			"in" | "intuition" => Ok(Intuition),
			"ch" | "charisma" => Ok(Charisma),
			"ff" | "fingerfertigkeit" | "dexterity" => Ok(Dexterity),
			"ge" | "gewandtheit" | "agility" => Ok(Agility),
			"ko" | "konstitution" | "constitution" => Ok(Constitution),
			"kk" | "körperkraft" | "strength" => Ok(Strength),
			"gs" | "sozialstatus" | "socialstatus" => Ok(SocialStatus),
			"mr" | "magieresistenz" | "magicrestistance" => Ok(MagicResistance),
			"ini"| "initiative" => Ok(Initiative),
			"at" | "attacke" | "nahkampf" | "combat" | "closecombat" => Ok(CloseCombat),
			"pa" | "parrieren" | "parry" => Ok(Parry),
			"fk" | "fernkampf" | "ranged" | "rangedcombat" => Ok(RangedCombat),
			// TODO: make own error type with values
			_ => bail!("unknown quality '{}'", input),
		}
	}
}

