use crate::application_logic::contact_features::create::CreatorImpl;
use crate::application_logic::contact_features::delete::DelitorImpl;
use crate::application_logic::contact_features::get::GetterImpl as ContactGetterImpl;
use crate::application_logic::contact_features::update::UpdaterImpl as ContactUpdaterImpl;
use crate::application_logic::user_features::get::GetterImpl as UserGetterImpl;
use crate::application_logic::user_features::login::LoginerImpl;
use crate::application_logic::user_features::register::RegistarImpl;
use crate::application_logic::user_features::update::UpdaterImpl as UserUpdateImpl;
use crate::infrastructure::repository::connection::R2D2Connection;
use crate::infrastructure::repository::pool::R2D2Pool;
use crate::infrastructure::repository::postgres::ContactPgRepo;
use crate::infrastructure::repository::postgres::UserPgRepo;
use crate::infrastructure::security::argon_hasher::ArgonHasher;
use crate::infrastructure::security::jwt_token_handler::JwtTokenHandler;
use diesel::PgConnection;
use shaku::module;

module! {
    pub MainModule {
        components = [
            R2D2Pool<PgConnection>,
            ArgonHasher,
            JwtTokenHandler,
        ],
        providers = [
            R2D2Connection<PgConnection>,
            UserPgRepo,
            ContactPgRepo,
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
