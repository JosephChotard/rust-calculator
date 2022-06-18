use confy::{load, store};
use serde::{Deserialize, Serialize};

/// It tries to load the config from the file, if it fails it creates a default config and stores it in
/// the file
/// Currently panics if it cannot create the config file
/// TODO: Handle panic
/// If an error occurs while loading the config, it will create a default config
/// TODO: If a key is missing, only use the default value for that key and not the default config
///
/// Returns:
///
/// The preferences in the Config struct
pub fn get_config() -> Config {
  let config: Config = match load("com.josephchotard.calculator") {
    Ok(config) => config,
    Err(err) => {
      println!("Error: {}", err);
      let config = Config::default();
      store("com.josephchotard.calculator", &config).expect("Could not store config");
      config
    }
  };
  config
}

/// `Config` is a struct that has the user preferences
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  /// `dark_mode`: Whether or not the user wants to use dark mode.
  pub dark_mode: bool,
}

impl Default for Config {
  /// Default config value
  ///
  /// Returns: a new `Config`
  ///
  /// A new instance of the Settings struct.
  fn default() -> Self {
    let dark_mode = match dark_light::detect() {
      dark_light::Mode::Dark => true,
      dark_light::Mode::Light => false,
    };

    Self {
      dark_mode: dark_mode,
    }
  }
}

impl Config {
  /// It updates the dark mode value in the config and then stores the preferences
  ///
  /// Arguments:
  ///
  /// * `dark_mode`: bool
  pub fn update_dark_mode(&mut self, dark_mode: bool) {
    self.dark_mode = dark_mode;
    store("com.josephchotard.calculator", self).unwrap();
  }
}
