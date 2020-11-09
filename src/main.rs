mod pokedex;
mod pokemon;
use iced::Application;
use pokedex::Pokedex;

fn main() {
	let mut settings = iced::Settings::default();
	settings.window.size = (540, 600);
	Pokedex::run(settings);
}
