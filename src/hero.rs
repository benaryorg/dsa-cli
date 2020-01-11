use crate::error::*;

use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Hero
{
	name: String,
	health: isize,
	endurance: isize,
	astral: isize,
	basevalues: HashMap<BaseValue,isize>,
}

impl std::str::FromStr for Hero
{
	type Err = Error;

	fn from_str(input: &str) -> Result<Self>
	{
		let document = roxmltree::Document::parse(&input).chain_err(|| "xml document could not be parsed")?;

		// get the hero
		let root = document.root_element();
		if ! root.has_tag_name("helden")
		{
			bail!("unknown root element");
		}
		let held = root.first_child()
			.filter(|child| child.has_tag_name("held"))
			.ok_or("root element does not contain held element")?;

		// get the basevalues
		let basevalues = held.children().into_iter()
			.filter(|elem| elem.has_tag_name("eigenschaften"))
			.next()
			.ok_or("hero does not have base values")?;
		let mut basevalues: HashMap<_,isize> = basevalues.children().into_iter()
			.map(|elem|
				{
					let name = elem.attribute("name").unwrap_or("").to_lowercase();
					let base = elem.attribute("value").and_then(|i| i.parse().ok()).unwrap_or(0);
					let modi = elem.attribute("mod").and_then(|i| i.parse().ok()).unwrap_or(0);
					(name,base + modi)
				})
			.collect();
		let health_base = basevalues.remove("lebensenergie").unwrap_or(0);
		let endurance_base = basevalues.remove("ausdauer").unwrap_or(0);
		let astral_base = basevalues.remove("astralenergie").unwrap_or(0);
		let basevalues: HashMap<_,_> = basevalues.into_iter()
			.map(|(k,v)| (k.parse::<BaseValue>().ok(),v))
			.filter(|(k,_)| k.is_some())
			.map(|(k,v)| (k.unwrap(),v))
			.collect();

		use BaseValue::*;

		let hero = Hero
		{
			name: held.attribute("name").ok_or("hero does not have a name")?.into(),
			health:
			{
				let ko = *basevalues.get(&Constitution).unwrap_or(&0);
				let kk = *basevalues.get(&Strength).unwrap_or(&0);
				((ko as f32 * 2.0 + kk as f32) / 2.0).round() as isize + health_base
			},
			endurance:
			{
				let mu = *basevalues.get(&Courage).unwrap_or(&0);
				let ko = *basevalues.get(&Constitution).unwrap_or(&0);
				let ge = *basevalues.get(&Dexterity).unwrap_or(&0);
				((mu as f32 + ko as f32 + ge as f32) / 2.0).round() as isize + endurance_base
			},
			astral:
			{
				let mu = *basevalues.get(&Courage).unwrap_or(&0);
				let int = *basevalues.get(&Intuition).unwrap_or(&0);
				let ch = *basevalues.get(&Charisma).unwrap_or(&0);
				((mu as f32 + int as f32 + ch as f32) / 2.0).round() as isize + astral_base
			},
			basevalues,
		};

		Ok(hero)
	}
}

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Hash,Clone,Copy)]
enum BaseValue
{
	Courage,
	Intelligence,
	Intuition,
	Charisma,
	Prestidigitation,
	Dexterity,
	Constitution,
	Strength,
	SocialStatus,
	MagicResistance,
	Initiative,
	CloseCombat,
	Parry,
	RangedCombat,
}

impl std::str::FromStr for BaseValue
{
	type Err = Error;

	fn from_str(input: &str) -> Result<Self>
	{
		use BaseValue::*;
		match input.to_lowercase().as_str()
		{
			"mu" | "mut" | "courage" => Ok(Courage),
			"kl" | "klugheit" | "intelligence" => Ok(Intelligence),
			"in" | "intuition" => Ok(Intuition),
			"ch" | "charisma" => Ok(Charisma),
			"ff" | "fingerfertigkeit" | "prestidigitation" => Ok(Prestidigitation),
			"ge" | "gewandtheit" | "dexterity" => Ok(Dexterity),
			"ko" | "konstitution" | "constitution" => Ok(Constitution),
			"kk" | "körperkraft" | "strength" => Ok(Strength),
			"gs" | "sozialstatus" | "socialstatus" => Ok(SocialStatus),
			"mr" | "magieresistenz" | "magicrestistance" => Ok(MagicResistance),
			"ini"| "initiative" => Ok(Initiative),
			"at" | "attacke" | "nahkampf" | "combat" | "closecombat" => Ok(CloseCombat),
			"pa" | "parrieren" | "parry" => Ok(Parry),
			"fk" | "fernkampf" | "ranged" | "rangedcombat" => Ok(RangedCombat),
			// TODO: make own error type with values
			_ => bail!("unknown basevalue '{}'", input),
		}
	}
}
