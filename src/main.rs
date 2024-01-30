use search::WikiSearch;

mod search;

fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;

	let search = WikiSearch::search()?;
	let entry = search.find("de");

	if let Some(entry) = entry {
		println!("::: {} :::", search.title());
		println!("de: {}", entry.name());
	}

	Ok(())
}
