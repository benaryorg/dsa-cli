use error_chain::error_chain;
pub use error_chain::bail;

error_chain! {
	links
	{
	}

	foreign_links
	{
		Io(::std::io::Error);
		XmlParser(::roxmltree::Error);
		NumberParsing(::std::num::ParseIntError);
		CommandLineParsing(::clap::Error);
		LineEditing(::rustyline::error::ReadlineError);
	}

	errors
	{
	}
}

