use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
	word: String,
	from: Option<String>,
	into: Option<String>,
	#[arg(short, long, help = "reverse from and into")]
	rev: bool,
}

impl Cli {
	pub fn word(&self) -> &str {
		&self.word
	}

	/// internal from
	fn in_from(&self) -> &str {
		self.from.as_deref().unwrap_or("en")
	}

	pub fn from(&self) -> &str {
		if !self.rev {
			self.in_from()
		} else {
			self.in_to()
		}
	}

	/// internal to
	fn in_to(&self) -> &str {
		self.into.as_deref().unwrap_or("de")
	}

	pub fn to(&self) -> &str {
		if !self.rev {
			self.in_to()
		} else {
			self.in_from()
		}
	}
}
