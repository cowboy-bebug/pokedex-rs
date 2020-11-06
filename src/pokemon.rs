/// Module that contains types to deserialize response into.
mod response {
	use serde::Deserialize;

	#[derive(Clone, Debug, Deserialize)]
	pub struct NamedResponse {
		pub name: String,
	}

	#[derive(Debug, Deserialize)]
	pub struct TypeText {
		pub r#type: NamedResponse,
	}

	#[derive(Debug, Deserialize)]
	pub struct Stat {
		pub base_stat: u8,
		pub stat: NamedResponse,
	}

	#[derive(Debug, Deserialize)]
	pub struct PokemonResponse {
		pub id: u16,
		pub name: String,
		pub types: Vec<TypeText>,
		pub stats: Vec<Stat>,
		pub height: u8,  // in decimeters
		pub weight: u16, // in hectograms
	}

	#[derive(Clone, Debug, Deserialize)]
	pub struct FlavorText {
		pub flavor_text: String,
		pub language: NamedResponse,
	}

	#[derive(Clone, Debug, Deserialize)]
	pub struct SpeciesResponse {
		pub flavor_text_entries: Vec<FlavorText>,
	}
}

/// Contains various information of a pokemon.
#[derive(Debug, Clone)]
pub struct Pokemon {
	pub image: iced::image::Handle,
	pub shiny_image: iced::image::Handle,
	pub id: u16,
	pub name: String,
	pub types: Vec<String>,
	pub flavor_text: String,
	pub height_meters: f32,
	pub weight_grams: f32,
	pub hp: f32,
	pub attack: f32,
	pub defense: f32,
	pub special_attack: f32,
	pub special_defense: f32,
	pub speed: f32,
}

// NOTE: had to implement Default because iced::image::Handle doesn't implement Default
impl Default for Pokemon {
	fn default() -> Self {
		Self {
			image: iced::image::Handle::from_memory(<Vec<u8>>::new()),
			shiny_image: iced::image::Handle::from_memory(<Vec<u8>>::new()),
			id: u16::default(),
			name: String::default(),
			types: Vec::new(),
			flavor_text: String::default(),
			height_meters: f32::default(),
			weight_grams: f32::default(),
			hp: f32::default(),
			attack: f32::default(),
			defense: f32::default(),
			special_attack: f32::default(),
			special_defense: f32::default(),
			speed: f32::default(),
		}
	}
}

/// Wraps various errors.
#[derive(Debug, Clone)]
pub enum Error {
	Reqwest,
	Language,
}

impl From<reqwest::Error> for Error {
	fn from(_: reqwest::Error) -> Error {
		Error::Reqwest
	}
}

enum ImageType {
	Normal,
	Shiny,
}

/// Main implementation block for `Pokemon`.
impl Pokemon {
	/// Total number of pokemons available.
	const TOTAL: u16 = 893;

	/// Fetches the image of a pokemon.
	async fn fetch_image(id: u16, image_type: ImageType) -> Result<iced::image::Handle, Error> {
		let url = format!(
			"https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon{}/{}.png",
			match image_type {
				ImageType::Normal => "",
				ImageType::Shiny => "/shiny",
			},
			id
		);
		#[cfg(not(target_arch = "wasm32"))]
		{
			let bytes = reqwest::get(&url).await?.bytes().await?;
			Ok(iced::image::Handle::from_memory(bytes.as_ref().to_vec()))
		}
		#[cfg(target_arch = "wasm32")]
		Ok(iced::image::Handle::from_path(url))
	}

	/// Fetches response from `/api/v2/pokemon/`.
	async fn fetch_pokemon(id: u16) -> Result<response::PokemonResponse, Error> {
		let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
		Ok(reqwest::get(&url).await?.json().await?)
	}

	/// Fetches response from `/api/v2/pokemon-species/`.
	async fn fetch_species(id: u16) -> Result<response::SpeciesResponse, Error> {
		let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", id);
		Ok(reqwest::get(&url).await?.json().await?)
	}

	/// Fetches from all the endpoints to build `Pokemon`.
	pub async fn fetch() -> Result<Self, Error> {
		let id = {
			use rand::Rng;
			let mut rng = rand::rngs::OsRng::default();
			rng.gen_range(1, Self::TOTAL)
		};
		let (image_resp, shiny_image_resp, pokemon_resp, species_resp) =
			iced::futures::future::try_join4(
				Self::fetch_image(id, ImageType::Normal),
				Self::fetch_image(id, ImageType::Shiny),
				Self::fetch_pokemon(id),
				Self::fetch_species(id),
			)
			.await?;

		println!("/api/v2/pokemon/ response: {:#?}", pokemon_resp);
		println!("/api/v2/pokemon-species/ response: {:#?}", species_resp);

		let mut pokemon = Pokemon::default();
		pokemon.image = image_resp;
		pokemon.shiny_image = shiny_image_resp;
		pokemon.id = pokemon_resp.id;
		pokemon.name = pokemon_resp.name;
		pokemon.types = pokemon_resp
			.types
			.iter()
			.map(|r#type| r#type.r#type.name.clone())
			.collect();
		pokemon.flavor_text = species_resp
			.flavor_text_entries
			.iter()
			.filter(|text| text.language.name == "en")
			.next()
			.ok_or(Error::Language)?
			.flavor_text
			.chars()
			.map(|c| if c.is_control() { ' ' } else { c })
			.collect();
		pokemon.height_meters = pokemon_resp.height as f32 / 10.0;
		pokemon.weight_grams = pokemon_resp.weight as f32 * 100.0;
		for stat in pokemon_resp.stats {
			match stat.stat.name {
				_ if stat.stat.name == "hp" => pokemon.hp = stat.base_stat.into(),
				_ if stat.stat.name == "attack" => pokemon.attack = stat.base_stat.into(),
				_ if stat.stat.name == "defense" => pokemon.defense = stat.base_stat.into(),
				_ if stat.stat.name == "special-attack" => pokemon.special_attack = stat.base_stat.into(),
				_ if stat.stat.name == "special-defense" => pokemon.special_defense = stat.base_stat.into(),
				_ if stat.stat.name == "speed" => pokemon.speed = stat.base_stat.into(),
				_ => {}
			}
		}
		Ok(pokemon)
	}
}
