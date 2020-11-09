use crate::pokemon::{Error, Pokemon};
use iced::{
	button, Align, Application, Button, Column, Command, Container, Element, Image, Length,
	ProgressBar, Row, Text,
};
use inflector::Inflector;

#[derive(Debug)]
pub enum Pokedex {
	Loading,
	Loaded {
		pokemon: Pokemon,
		search: button::State,
	},
}

#[derive(Debug, Clone)]
pub enum Message {
	PokemonFound(Result<Pokemon, Error>),
	Search,
}

impl Application for Pokedex {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Message>) {
		(
			Self::Loading,
			Command::perform(Pokemon::fetch(), Message::PokemonFound),
		)
	}

	fn title(&self) -> String {
		let title = match self {
			Self::Loading => "Loading",
			Self::Loaded { pokemon, .. } => &pokemon.name,
		};
		format!("{} - Pokédex", title)
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::PokemonFound(Ok(pokemon)) => {
				*self = Self::Loaded {
					pokemon,
					search: button::State::new(),
				};
				Command::none()
			}
			Message::PokemonFound(Err(_error)) => Command::none(),
			Message::Search => match self {
				Self::Loading => Command::none(),
				_ => {
					*self = Self::Loading;
					Command::perform(Pokemon::fetch(), Message::PokemonFound)
				}
			},
		}
	}

	fn view(&mut self) -> Element<Message> {
		let content = match self {
			Self::Loading => Column::new()
				.align_items(Align::Center)
				.spacing(20)
				.width(Length::Shrink)
				.push(Text::new("Searching for Pokémon...").size(40))
				.push(
					if cfg!(target_arch = "wasm32") {
						Image::new("resources/pokeball.png")
					} else {
						Image::new(format!(
							"{}/resources/pokeball.png",
							env!("CARGO_MANIFEST_DIR")
						))
					}
					.width(Length::Units(100)),
				),
			Self::Loaded { pokemon, search } => Column::new()
				.padding(20)
				.spacing(50)
				.align_items(Align::End)
				.push(pokemon.view())
				.push(button(search, "Keep searching!").on_press(Message::Search)),
		};
		Container::new(content)
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.into()
	}
}

impl Pokemon {
	/// The main overarching pokemon view.
	fn view(&self) -> Element<Message> {
		let row_name = Row::new()
			.align_items(Align::Center)
			.spacing(30)
			.push(
				Text::new(&self.name.to_title_case())
					.size(30)
					.width(Length::FillPortion(80)),
			)
			.push(self.type_view())
			.push(
				Text::new(format!("#{}", self.id))
					.size(20)
					.color([0.5, 0.5, 0.5]),
			);
		let row_description = Text::new(&self.flavor_text);
		Column::new()
			.spacing(50)
			.push(
				Row::new()
					.spacing(20)
					.align_items(Align::Start)
					.push(
						Column::new()
							.align_items(Align::Center)
							.push(Image::new(self.image.clone()))
							.push(self.physique_view()),
					)
					.push(
						Column::new()
							.spacing(40)
							.push(row_name)
							.push(row_description)
							.align_items(Align::Start),
					),
			)
			.push(self.stats_view())
			.into()
	}

	/// View to display types.
	fn type_view(&self) -> Element<Message> {
		// + ----- + -------- +
		// | water | electric |
		// + ----- + -------- +
		Row::new()
			.align_items(Align::End)
			.spacing(5)
			.push(Text::new(self.types[0].clone()).size(14))
			.push(
				Text::new(if self.types.len() == 2 {
					self.types[1].clone()
				} else {
					"".into()
				})
				.size(14),
			)
			.into()
	}

	/// View to display physique.
	fn physique_view(&self) -> Element<Message> {
		// -------- + -------
		//   height | 1.1m
		//   weight | 100 g
		// -------- + -------
		Row::new()
			.align_items(Align::End)
			.spacing(10)
			.push(
				Column::new()
					.align_items(Align::End)
					.push(Text::new("height:").size(15).color([0.5, 0.5, 0.5]))
					.push(Text::new("weight:").size(15).color([0.5, 0.5, 0.5])),
			)
			.push(
				Column::new()
					.align_items(Align::Start)
					.push(
						Text::new(format!("{:.1}m", self.height_meters))
							.size(15)
							.color([0.5, 0.5, 0.5]),
					)
					.push(
						Text::new(if self.weight_grams >= 1_000.0 {
							format!("{:.1}kg", self.weight_grams / 1_000.0)
						} else {
							format!("{}g", self.weight_grams)
						})
						.size(15)
						.color([0.5, 0.5, 0.5]),
					),
			)
			.into()
	}

	/// View to display stats.
	fn stats_view(&self) -> Element<Message> {
		let bar_height = Length::Units(8);
		let bar_range = 0.0..=200.0;
		let label_size = 12;
		Column::new()
			.align_items(Align::Center)
			.spacing(5)
			.push(
				Column::new()
					.align_items(Align::Center)
					.push(Text::new("hp").size(label_size))
					.push(ProgressBar::new(bar_range.clone(), self.hp).height(bar_height)),
			)
			.push(
				Column::new()
					.align_items(Align::Center)
					.push(Text::new("attack").size(label_size))
					.push(ProgressBar::new(bar_range.clone(), self.attack).height(bar_height)),
			)
			.push(
				Column::new()
					.align_items(Align::Center)
					.push(Text::new("defence").size(label_size))
					.push(ProgressBar::new(bar_range.clone(), self.defense).height(bar_height)),
			)
			.push(
				Column::new()
					.align_items(Align::Center)
					.push(Text::new("special attack").size(label_size))
					.push(ProgressBar::new(bar_range.clone(), self.special_attack).height(bar_height)),
			)
			.push(
				Column::new()
					.align_items(Align::Center)
					.push(Text::new("special defence").size(label_size))
					.push(ProgressBar::new(bar_range.clone(), self.special_defense).height(bar_height)),
			)
			.push(
				Column::new()
					.align_items(Align::Center)
					.push(Text::new("speed").size(label_size))
					.push(ProgressBar::new(bar_range, self.speed).height(bar_height)),
			)
			.into()
	}
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
	Button::new(state, Text::new(text))
		.padding(10)
		.style(style::Button::Primary)
}

mod style {
	use iced::{button, Background, Color, Vector};

	pub enum Button {
		Primary,
	}

	impl button::StyleSheet for Button {
		fn active(&self) -> button::Style {
			button::Style {
				background: Some(Background::Color(match self {
					Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
				})),
				border_radius: 12,
				shadow_offset: Vector::new(1.0, 1.0),
				text_color: Color::WHITE,
				..button::Style::default()
			}
		}
	}
}
