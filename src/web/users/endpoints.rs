use crate::infrastructure::repository::postgres::UserPgRepo;
use crate::domain::user::{self, AuthUser, CurrentUser, LoginUser, RegisterUser, UpdateUser, User};
use crate::log::ResultInspect;
use crate::utils;
use crate::web::error::{Error, Result};
use rocket::Route;
use rocket_contrib::json::Json;

#[post("/register", format = "json", data = "<register_user>")]
fn register(register_user: Json<RegisterUser>, repo: UserPgRepo) -> Result<()> {
    // for security reasons, don't log user registration data, the password is specified there

    user::register_user(register_user.into_inner(), &repo)
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to register user: {}", e))
}

#[post("/login", format = "json", data = "<login_user>")]
fn login(login_user: Json<LoginUser>, repo: UserPgRepo) -> Result<Json<AuthUser>> {
    // for security reasons, don't log user cridentials, the password is specified there

    user::login_user(login_user.into_inner(), &repo)
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to login user: {}", e))
        .map(Json)
        .inspect(|a| log::debug!("the user logged in successfully: {:#?}", a))
}

#[get("/<username>")]
fn get(username: String, repo: UserPgRepo) -> Result<Json<User>> {
    user::get_user(username.as_str(), &repo)
        .map_err(Error::from)
        .map(Json)
        .inspect(|u| log::debug!("user: {:#?}", u))
}

#[put("/", format = "json", data = "<update_user>")]
fn update(
    current_user: Result<CurrentUser>,
    update_user: Json<UpdateUser>,
    repo: UserPgRepo,
) -> Result<()> {
    utils::inspect_current_user(&current_user);

    // for security reasons, don't log update data, the password is specified there

    user::update_user(&current_user?, update_user.into_inner(), &repo)
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to update user: {}", e))
}

pub fn api() -> Vec<Route> {
    routes![register, login, get, update]
}
