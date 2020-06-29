use crate::db::ContactPgRepo;
use crate::domain::contact::{self, Contact, CreateContact, UpdateContact};
use crate::domain::user::CurrentUser;
use crate::web::error::{Error, Result};
use rocket::Route;
use rocket_contrib::json::Json;

#[post("/", format = "json", data = "<create_contact>")]
fn create(
    current_user: Result<CurrentUser>,
    create_contact: Json<CreateContact>,
    repo: ContactPgRepo,
) -> Result<()> {
    contact::create_contact(&current_user?, create_contact.into_inner(), &repo).map_err(Error::from)
}

#[get("/")]
fn get(current_user: Result<CurrentUser>, repo: ContactPgRepo) -> Result<Json<Vec<Contact>>> {
    contact::get_contacts(&current_user?, &repo)
        .map_err(Error::from)
        .map(Json)
}

#[put("/<id>", format = "json", data = "<update_contact>")]
fn update(
    current_user: Result<CurrentUser>,
    id: i64,
    update_contact: Json<UpdateContact>,
    repo: ContactPgRepo,
) -> Result<()> {
    contact::update_contact(&current_user?, id, update_contact.into_inner(), &repo)
        .map_err(Error::from)
}

#[delete("/<id>")]
fn delete(current_user: Result<CurrentUser>, id: i64, repo: ContactPgRepo) -> Result<()> {
    contact::delete_contact(&current_user?, id, &repo).map_err(Error::from)
}

pub fn api() -> Vec<Route> {
    routes![create, get, update, delete]
}
