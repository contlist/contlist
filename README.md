# Contlist

## Architecture

Layers:

* [x] Domain Model
  * domain entities (User, Contact)
  * other types (PhoneNumber, Claims)
* [x] Domain Logic
  * repository traits
  * business logic decisions
* [x] Application Logic
  * features traits (Loginer, ContactCreater) (optional)
  * features (login, register, create contact)
* [x] Infrastructure
  * endpoints
  * security
  * repository implementation

## Conventions

There are some conventions I followed during the development of this project

### Error handling

* each domain object (like User, Contact, e.t.c) must have it's own error type
* all public interface of a domain object must use the same error type on each [layer](#architecture)
* if some [layer](#architecture) needs to have it's own error type, this is fine, but not in the public interface
* when forwarding errors that depend on implementation, the underlying type must be sealed (cannot be accessed directly)
* when forwarding domain object errors, the underlying types must be passed directly and with the `[error(transparent)]` attribute
