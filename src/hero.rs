use crate::error::*;

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Hero
{
	name: String,
}

impl std::str::FromStr for Hero
{
	type Err = Error;

	fn from_str(input: &str) -> Result<Self>
	{
		let document = roxmltree::Document::parse(&input).chain_err(|| "xml document could not be parsed")?;

		let root = document.root_element();
		if ! root.has_tag_name("helden")
		{
			bail!("unknown root element");
		}
		let held = root.first_child()
			.filter(|child| child.has_tag_name("held"))
			.ok_or("root element does not contain held element")?;
		let name = held.attribute("name")
			// TODO: figure out whether to return a default name
			.ok_or("hero does not have a name")?;

		let hero = Hero
		{
			name: name.into(),
		};

		Ok(hero)
	}
}

