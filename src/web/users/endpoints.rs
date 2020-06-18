use super::error::{Error, Result};
use crate::db::UserPgRepo;
use crate::domain::user::{self, AuthUser, CurrentUser, LoginUser, RegisterUser, UpdateUser, User};
use rocket::Route;
use rocket_contrib::json::Json;

#[post("/register", format = "json", data = "<register_user>")]
fn register(register_user: Json<RegisterUser>, repo: UserPgRepo) -> Result<()> {
    user::register_user(&register_user, &repo)
        .map_err(Error::from)
        .map(|_| ())
}

#[post("/login", format = "json", data = "<login_user>")]
fn login(login_user: Json<LoginUser>, repo: UserPgRepo) -> Result<Json<AuthUser>> {
    user::login_user(&login_user, &repo)
        .map_err(Error::from)
        .map(Json)
}

#[get("/<username>")]
fn get(username: String, repo: UserPgRepo) -> Result<Json<User>> {
    user::get_user(username.as_str(), &repo)
        .map_err(Error::from)
        .map(Json)
}

#[post("/update", format = "json", data = "<update_user>")]
fn update(
    current_user: CurrentUser,
    update_user: Json<UpdateUser>,
    repo: UserPgRepo,
) -> Result<()> {
    user::update_user(current_user.username.as_str(), &update_user, &repo)
        .map_err(Error::from)
        .map(|_| ())
}

pub fn api() -> Vec<Route> {
    routes![register, login, get, update]
}
