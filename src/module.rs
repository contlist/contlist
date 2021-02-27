use crate::application_logic::contact_features::create::CreatorImpl;
use crate::application_logic::contact_features::delete::DelitorImpl;
use crate::application_logic::contact_features::get::GetterImpl as ContactGetterImpl;
use crate::application_logic::contact_features::update::UpdaterImpl as ContactUpdaterImpl;
use crate::application_logic::user_features::get::GetterImpl as UserGetterImpl;
use crate::application_logic::user_features::login::LoginerImpl;
use crate::application_logic::user_features::register::RegistarImpl;
use crate::application_logic::user_features::update::UpdaterImpl as UserUpdateImpl;
use crate::infrastructure::security::argon_hasher::ArgonHasher;
use crate::infrastructure::security::jwt_token_handler::JwtTokenHandler;
use shaku::module;

module! {
    pub MainModule {
        components = [
            ArgonHasher,
            JwtTokenHandler,
        ],
        providers = [
            RegistarImpl,
            LoginerImpl,
            UserGetterImpl,
            UserUpdateImpl,
            CreatorImpl,
            ContactGetterImpl,
            ContactUpdaterImpl,
            DelitorImpl,
        ],
    }
}
