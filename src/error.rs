//! Error chain created using [error-chain](https://crates.io/crates/error-chain).
//!
//! It can be chained with the very same crate if desired for error handling.

use error_chain::error_chain;

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

