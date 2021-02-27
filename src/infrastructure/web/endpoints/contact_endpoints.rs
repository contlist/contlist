use crate::application_logic::contact_features::create::{CreateData, Creator};
use crate::application_logic::contact_features::delete::Delitor;
use crate::application_logic::contact_features::get::Getter;
use crate::application_logic::contact_features::update::{UpdateData, Updater};
use crate::domain_logic::repository::ContactRepo;
use crate::domain_model::entities::contact::Contact;
use crate::infrastructure::web::current_user::CurrentUser;
use crate::infrastructure::web::error::{Error, Result};
use crate::log::ResultInspect;
use crate::module::MainModule;
use crate::utils;
use rocket::Route;
use rocket_contrib::json::Json;
use shaku_rocket::InjectProvided;

#[post("/", format = "json", data = "<create_data>")]
fn create(
    current_user: Result<CurrentUser>,
    create_data: Json<CreateData>,
    creator: InjectProvided<MainModule, dyn Creator>,
) -> Result<Json<i64>> {
    utils::inspect_current_user(&current_user);

    log::debug!("contact data: {:#?}", create_data);

    creator
        .create(current_user?.username(), create_data.into_inner())
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to create contact: {}", e))
        .map(Json)
        .inspect(|id| log::debug!("created contact id: {:#?}", id))
}

#[get("/")]
fn get(
    current_user: Result<CurrentUser>,
    getter: InjectProvided<MainModule, dyn Getter>,
) -> Result<Json<Vec<Contact>>> {
    utils::inspect_current_user(&current_user);

    getter
        .list(current_user?.username())
        .map_err(Error::from)
        .inspect_err(|e| log::error!("failed to collect user's contacts: {}", e))
        .map(Json)
        .inspect(|_| log::debug!("user's contacts was successfully collected"))
}

#[put("/<id>", format = "json", data = "<update_data>")]
fn update(
    current_user: Result<CurrentUser>,
    id: i64,
    update_data: Json<UpdateData>,
    updater: InjectProvided<MainModule, dyn Updater>,
) -> Result<()> {
    utils::inspect_current_user(&current_user);

    log::debug!("update contact data: {:#?}", update_data);

    updater
        .update(current_user?.username(), id, update_data.into_inner())
        .map_err(Error::from)
        .inspect_err(|e| log::warn!("failed to update contact: {}", e))
        .inspect(|_| log::debug!("the contact was successfully updated"))
}

#[delete("/<id>")]
fn delete(
    current_user: Result<CurrentUser>,
    id: i64,
    delitor: InjectProvided<MainModule, dyn Delitor>,
) -> Result<()> {
    utils::inspect_current_user(&current_user);

    delitor
        .delete(current_user?.username(), id)
        .map_err(Error::from)
        .inspect_err(|e| log::debug!("failed to delete contact: {}", e))
        .inspect(|_| log::debug!("the contact was successfully deleted"))
}

pub fn api() -> Vec<Route> {
    routes![create, get, update, delete]
}
