use crate::application_logic::user_features::get::Getter;
use crate::application_logic::user_features::login::{AuthData, LoginData, Loginer};
use crate::application_logic::user_features::register::{Registar, RegisterData};
use crate::application_logic::user_features::update::{UpdateData, Updater};
use crate::domain_model::entities::user::User;
use crate::infrastructure::web::current_user::CurrentUser;
use crate::infrastructure::web::error::{Error, Result};
use crate::log::ResultInspect;
use crate::module::MainModule;
use crate::utils;
use rocket::Route;
use rocket_contrib::json::Json;
use shaku_rocket::InjectProvided;

#[post("/register", format = "json", data = "<register_data>")]
fn register(
    register_data: Json<RegisterData>,
    registar: InjectProvided<MainModule, dyn Registar>,
) -> Result<()> {
    // for security reasons, don't log user registration data, the password is specified there

    registar
        .register(register_data.into_inner())
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to register user: {}", e))
}

#[post("/login", format = "json", data = "<login_data>")]
fn login(
    login_data: Json<LoginData>,
    loginer: InjectProvided<MainModule, dyn Loginer>,
) -> Result<Json<AuthData>> {
    // for security reasons, don't log user cridentials, the password is specified there

    loginer
        .login(login_data.into_inner())
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to login user: {}", e))
        .map(Json)
        .inspect(|a| log::debug!("the user logged in successfully: {:#?}", a))
}

#[get("/<username>")]
fn get(username: String, getter: InjectProvided<MainModule, dyn Getter>) -> Result<Json<User>> {
    getter
        .get(username.as_str())
        .map_err(Error::from)
        .map(Json)
        .inspect(|u| log::debug!("user: {:#?}", u))
}

#[put("/", format = "json", data = "<update_data>")]
fn update(
    current_user: Result<CurrentUser>,
    update_data: Json<UpdateData>,
    updater: InjectProvided<MainModule, dyn Updater>,
) -> Result<()> {
    utils::inspect_current_user(&current_user);

    // for security reasons, don't log update data, the password is specified there

    updater
        .update(current_user?.username(), update_data.into_inner())
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to update user: {}", e))
}

pub fn api() -> Vec<Route> {
    routes![register, login, get, update]
}
