error_chain! {
	links
	{
	}

	foreign_links
	{
		Io(::std::io::Error);
		XmlParser(::roxmltree::Error);
	}

	errors
	{
	}
}

