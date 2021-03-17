# Contlist

A REST-Api beckend for [the Contlist project][Contlist project] writen using Rust with the [Rocket][Rocket] and the [Diesel][Diesel] frameworks.

## Contlist project

It is a service that allows users to store and group their contacts (phone numbers, emails, etc)

## Api specification

You can get the OpenApi specification for each resource at the `http://<host>/<resource>/openapi.json` or you can get the full Api specification in swagger UI at the `http://<host>/swagger`.

## Features

* JWT authentication
* CRUD for users (without Delete for now)
* CRUD for contacts

## Architecture

Layers:

* [x] Domain Model
  * domain entities (User, Contact)
  * other types (PhoneNumber, Claims)
* [x] Domain Logic
  * repository traits (UserRepo, ContactRepo)
  * infrastructure traits (TokenHandler, Hasher)
  * business logic decisions
* [x] Application Logic
  * features traits (Loginer, ContactCreater) (optional)
  * features (login, register, create contact)
* [x] Infrastructure
  * endpoints
  * security
  * repository implementation

## Conventions

There are some conventions I followed during the development of this project.

### Error handling

* each domain object (like User, Contact, e.t.c) must have it's own error type
* all public interface of a domain object must use the same error type on each [layer](#architecture)
* if some [layer](#architecture) needs to have it's own error type, this is fine, but not in the public interface
* when forwarding errors that depend on implementation, the underlying type must be sealed (cannot be accessed directly)
* when forwarding domain object errors, the underlying types must be passed directly and with the `[error(transparent)]` attribute

## TODO

* Implement JWT refresh token
* Groups of contacts
* Favorite contacts
* Admin and moderator roles

## Issues

Bugs & Issues should be reported at [GitHub][issues].

[Contlist project]: https://github.com/contlist
[Rocket]: https://rocket.rs/
[Diesel]: https://diesel.rs/
[issues]: https://github.com/contlist/contlist/issues
