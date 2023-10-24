pub mod application;

use once_cell::sync::Lazy;
use application::Application;

// init the application config into a global static variable
pub static APPLICATION_CONFIG : Lazy<Application> = Lazy::new(self::Application::init);