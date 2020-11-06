mod pokedex;
mod pokemon;
use iced::Application;
use pokedex::Pokedex;

fn main() {
	let mut settings = iced::Settings::default();
	settings.window.size = (600, 540);
	settings.window.resizable = true;
	Pokedex::run(settings);
}
