use rocket::Route;
use rocket_okapi::swagger_ui::{self, SwaggerUIConfig, UrlObject};

pub fn api() -> impl Into<Vec<Route>> {
    let urls = vec![
        UrlObject::new("User", "../user/openapi.json"),
        UrlObject::new("Contact", "../contact/openapi.json"),
    ];

    let config = SwaggerUIConfig {
        urls,
        ..Default::default()
    };

    swagger_ui::make_swagger_ui(&config)
}
