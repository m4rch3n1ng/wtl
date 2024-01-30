use search::WikiSearch;

mod search;

fn main() {
	let search = WikiSearch::search();
	println!("search {:?}", search);
}
