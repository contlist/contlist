use crate::infrastructure::repository::postgres::ContactPgRepo;
use crate::domain::contact::{self, Contact, CreateContact, UpdateContact};
use crate::domain::user::CurrentUser;
use crate::log::ResultInspect;
use crate::utils;
use crate::web::error::{Error, Result};
use rocket::Route;
use rocket_contrib::json::Json;

#[post("/", format = "json", data = "<create_contact>")]
fn create(
    current_user: Result<CurrentUser>,
    create_contact: Json<CreateContact>,
    repo: ContactPgRepo,
) -> Result<Json<i64>> {
    utils::inspect_current_user(&current_user);

    log::debug!("contact data: {:#?}", create_contact);

    contact::create_contact(&current_user?, create_contact.into_inner(), &repo)
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to create contact: {}", e))
        .map(Json)
        .inspect(|id| log::debug!("created contact id: {:#?}", id))
}

#[get("/")]
fn get(current_user: Result<CurrentUser>, repo: ContactPgRepo) -> Result<Json<Vec<Contact>>> {
    utils::inspect_current_user(&current_user);

    contact::get_contacts(&current_user?, &repo)
        .map_err(Error::from)
        .inspect_err(|e| log::error!("failed to collect user's contacts: {}", e))
        .map(Json)
        .inspect(|_| log::debug!("user's contacts was successfully collected"))
}

#[put("/<id>", format = "json", data = "<update_contact>")]
fn update(
    current_user: Result<CurrentUser>,
    id: i64,
    update_contact: Json<UpdateContact>,
    repo: ContactPgRepo,
) -> Result<()> {
    utils::inspect_current_user(&current_user);

    log::debug!("update contact data: {:#?}", update_contact);

    contact::update_contact(&current_user?, id, update_contact.into_inner(), &repo)
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to update contact: {}", e))
        .inspect(|_| log::debug!("the contact was successfully updated"))
}

#[delete("/<id>")]
fn delete(current_user: Result<CurrentUser>, id: i64, repo: ContactPgRepo) -> Result<()> {
    utils::inspect_current_user(&current_user);

    contact::delete_contact(&current_user?, id, &repo)
        .map_err(Error::from)
        .inspect_err(|e| log::debug!("failed to delete contact: {}", e))
        .inspect(|_| log::debug!("the contact was successfully deleted"))
}

pub fn api() -> Vec<Route> {
    routes![create, get, update, delete]
}
