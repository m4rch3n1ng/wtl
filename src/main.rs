use self::{cli::Cli, search::WikiSearch};
use clap::Parser;
use owo_colors::OwoColorize;

mod cli;
mod search;

fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;

	let cli = Cli::parse();
	let word = cli.word();
	let from = cli.from();
	let into = cli.to();

	println!(
		"{} {} {} {}",
		"translating from".italic().dimmed(),
		from,
		"into".italic().dimmed(),
		into
	);

	let search = WikiSearch::search(word, from)?;
	let entry = search.find(into);

	if let Some(entry) = entry {
		println!("::: {} :::", search.title());
		println!("{}: {}", cli.to(), entry.name());
	} else {
		println!(
			"{} {} {} {}",
			"no".italic().dimmed(),
			cli.to(),
			"translation found for".italic().dimmed(),
			search.title()
		)
	}

	Ok(())
}
