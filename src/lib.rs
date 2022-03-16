//! Want to have your API documented with OpenAPI? But you dont want to see the
//! trouble with manual yaml or json tweaking? Would like it to be so easy that it would almost
//! be like utopic? Don't worry utoipa is just there to fill this gap. It aims to do if not all then
//! the most of heavy lifting for you enabling you to focus writing the actual API logic instead of
//! documentation. It aims to be *minimal*, *simple* and *fast*. It uses simple proc macros which
//! you can use to annotate your code to have items documented.
//!
//! Utoipa crate provides autogenerated OpenAPI documentation for Rust REST APIs. It treats
//! code first appoach as a first class citizen and simplifies API documentation by providing
//! simple macros for generating the documentation from your code.
//!
//! It also contains Rust types of OpenAPI spec allowing you to write the OpenAPI spec only using
//! Rust if autogeneration is not your flavor or does not fit your purpose.
//!
//! Long term goal of the library is to be the place to go when OpenAPI documentation is needed in Rust
//! codebase.
//!
//! # What's up with the word play?
//!
//! The name comes from words `utopic` and `api` where `uto` is the first three letters of utopic
//! and the `ipa` is api reversed.
//!
//! # Features
//!
//! * **default** Default enabled features are **json**.
//! * **json** Enables **serde_json** what allow to use json values in OpenAPI specification values.
//!   Thus is enabled by default.
//! * **actix_extras** Enhances actix-web intgration with being able to parse some documentation
//!   from actix web macro attributes and types. See [`utoipa::path(...)`][path] for more details.
//! * **debug** Add extra traits such as debug traits to openapi definitions and elsewhere.
//!
//! # Install
//!
//! Add minimal dependency declaration to Cargo.toml.
//! ```text
//! [dependencies]
//! utoipa = "0.1.0.beta7"  
//! ```
//!
//! To enable more features such as use actix framework extras you could define the
//! dependency as follows.
//! ```text
//! [dependencies]
//! utoipa = { version = "0.1.0.beta7", features = ["actix_extras"] }
//! ```
//!
//! **Note!** To use `utoipa` together with Swagger UI you can use the [`utoipa-swagger-ui`][utoipa_swagger] crate.
//!
//! [utoipa_swagger]: <https://docs.rs/utoipa-swagger-ui/>
//!
//! # Examples
//!
//! Create a struct or it could be an enum also. Add `Component` derive macro to it so it can be registered
//! as a component in openapi schema.
//! ```rust
//! use utoipa::Component;
//! #[derive(Component)]
//! struct Pet {
//!    id: u64,
//!    name: String,
//!    age: Option<i32>,
//! }
//! ```
//!
//! Create an handler that would handle your business logic and add `path` proc attribute macro over it.
//! ```rust
//! mod pet_api {
//! #     use utoipa::OpenApi;
//! #     use utoipa::Component;
//! #     
//! #     #[derive(Component)]
//! #     struct Pet {
//! #       id: u64,
//! #       name: String,
//! #       age: Option<i32>,
//! #     }
//!     /// Get pet by id
//!     ///
//!     /// Get pet from database by pet id  
//!     #[utoipa::path(
//!         get,
//!         path = "/pets/{id}"
//!         responses = [
//!             (status = 200, description = "Pet found succesfully", body = Pet),
//!             (status = 404, description = "Pet was not found")
//!         ],
//!         params = [
//!             ("id" = u64, path, description = "Pet database id to get Pet for"),
//!         ]
//!     )]
//!     async fn get_pet_by_id(pet_id: u64) -> Pet {
//!         Pet {
//!             id: pet_id,
//!             age: None,
//!             name: "lightning".to_string(),
//!         }
//!     }
//! }
//! ```
//!
//! Tie the component and the above api to the openapi schema with following `OpenApi` derive proc macro.
//! ```rust
//! # mod pet_api {
//! #     use utoipa::Component;
//! #     
//! #     #[derive(Component)]
//! #     struct Pet {
//! #       id: u64,
//! #       name: String,
//! #       age: Option<i32>,
//! #     }
//! #
//! #     /// Get pet by id
//! #     ///
//! #     /// Get pet from database by pet id  
//! #     #[utoipa::path(
//! #         get,
//! #         path = "/pets/{id}"
//! #         responses = [
//! #             (status = 200, description = "Pet found succesfully", body = Pet),
//! #             (status = 404, description = "Pet was not found")
//! #         ],
//! #         params = [
//! #             ("id" = u64, path, description = "Pet database id to get Pet for"),
//! #         ]
//! #     )]
//! #     async fn get_pet_by_id(pet_id: u64) -> Pet {
//! #         Pet {
//! #             id: pet_id,
//! #             age: None,
//! #             name: "lightning".to_string(),
//! #         }
//! #     }
//! # }
//! # use utoipa::Component;
//! #
//! # #[derive(Component)]
//! # struct Pet {
//! #   id: u64,
//! #   name: String,
//! #   age: Option<i32>,
//! # }
//! # use utoipa::OpenApi;
//! #[derive(OpenApi)]
//! #[openapi(handlers(pet_api::get_pet_by_id), components(Pet))]
//! struct ApiDoc;
//!
//! println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
//! ```
//! # Go beyond the surface
//!
//! * See how to serve OpenAPI doc via Swagger UI check [`utoipa-swagger-ui`][utoipa_swagger] crate for more details.
//! * Browse to [examples](https://github.com/juhaku/utoipa/tree/master/examples) for more comprehensinve examples.
//! * Modify generated OpenAPI at runtime check [`Modify`] trait for more details.
//! * More about OpenAPI security in [security documentation][security].
//!
//! [path]: attr.path.html
//!
//! [security]: openapi/security/index.html

/// Rust implementation of Openapi Spec V3
pub mod openapi;

pub use utoipa_gen::*;

/// Trait for implementing OpenAPI specification in Rust.
///
/// This trait is derivable and can be used with `#[derive]` attribute. The derived implementation
/// will use Cargo provided environment variables to implement the default information. For a details of
/// `#[derive(Component)]` refer to [derive documentation][derive].
///
/// # Examples
///
/// Below is derived example of `OpenApi`.
/// ```rust
/// use utoipa::OpenApi;
/// #[derive(OpenApi)]
/// #[openapi(handlers())]
/// struct OpenApiDoc;
/// ```
///
/// This manual `OpenApi` trait implementation is approximately equal to the above derived one except the derive
/// implementation will by default use the Cargo environment variables to set defaults for *application name,
/// version, application description, license, author name & email*.
///
///```rust
/// struct OpenApiDoc;
///
/// impl utoipa::OpenApi for OpenApiDoc {
///     fn openapi() -> utoipa::openapi::OpenApi {
///         use utoipa::{Component, Path};
///         utoipa::openapi::OpenApi::new(
///             utoipa::openapi::Info::new("application name", "version")
///                 .with_description("application description")
///                 .with_license(utoipa::openapi::License::new("MIT"))
///                 .with_contact(
///                     utoipa::openapi::Contact::new()
///                         .with_name("author name")
///                         .with_email("author email"),
///                 ),
///             utoipa::openapi::path::Paths::new(),
///         )
///         .with_components(utoipa::openapi::Components::new())
///     }
/// }
/// ```
/// [derive]: derive.OpenApi.html
pub trait OpenApi {
    fn openapi() -> openapi::OpenApi;
}

/// Trait for implementing OpenAPI Schema object.
///
/// This trait is deriveable and can be used with `[#derive]` attribute. For a details of
/// `#[derive(Component)]` refer to [derive documentation][derive].
///
/// [derive]: derive.Component.html
///
/// # Examples
///
/// Use `#[derive]` to implement `Component` trait.
/// ```rust
/// # use utoipa::Component;
/// #[derive(Component)]
/// #[component(example = json!({"name": "bob the cat", "id": 1}))]
/// struct Pet {
///     id: u64,
///     name: String,
///     age: Option<i32>,
/// }
/// ```
///
/// Following manual implementation is equal to above derive one.
/// ```rust
/// # struct Pet {
/// #     id: u64,
/// #     name: String,
/// #     age: Option<i32>,
/// # }
/// #
/// impl utoipa::Component for Pet {
///     fn component() -> utoipa::openapi::schema::Component {
///         use utoipa::openapi::ToArray;
///         utoipa::openapi::Object::new()
///             .with_property(
///                 "id",
///                 utoipa::openapi::Property::new(utoipa::openapi::ComponentType::Integer)
///                     .with_format(utoipa::openapi::ComponentFormat::Int64),
///             )
///             .with_required("id")
///             .with_property(
///                 "name",
///                 utoipa::openapi::Property::new(utoipa::openapi::ComponentType::String),
///             )
///             .with_required("name")
///             .with_property(
///                 "age",
///                 utoipa::openapi::Property::new(utoipa::openapi::ComponentType::Integer)
///                     .with_format(utoipa::openapi::ComponentFormat::Int32),
///             )
///             .with_example(serde_json::json!({
///               "name":"bob the cat","id":1
///             }))
///             .into()
///     }
/// }
/// ```
pub trait Component {
    fn component() -> openapi::schema::Component;
}

/// Trait for implementing OpenAPI PathItem object with path.
///
/// This trait is implemented via [`#[utoipa::path(...)]`][derive] attribute macro and there
/// is no need to implement this trait manually.
///
/// # Examples
///
/// Use `#[utoipa::path(..)]` to implement Path trait
/// ```rust
/// # struct Pet {
/// #   id: u64,
/// #   name: String,
/// # }
/// #
/// #
/// /// Get pet by id
/// ///
/// /// Get pet from database by pet database id  
/// #[utoipa::path(
///     get,
///     path = "/pets/{id}"
///     responses = [
///         (status = 200, description = "Pet found succesfully", body = Pet),
///         (status = 404, description = "Pet was not found")
///     ],
///     params = [
///         ("id" = u64, path, description = "Pet database id to get Pet for"),
///     ]
/// )]
/// async fn get_pet_by_id(pet_id: u64) -> Pet {
///     Pet {
///         id: pet_id,
///         name: "lightning".to_string(),
///     }
/// }
/// ```
///
/// Example of what would manual implementation roughly look like of above `#[utoipa::path(...)]` macro.
/// ```rust
/// utoipa::openapi::Paths::new().append(
///         "/pets/{id}",
///         utoipa::openapi::PathItem::new(
///             utoipa::openapi::PathItemType::Get,
///             utoipa::openapi::path::Operation::new()
///                 .with_responses(
///                     utoipa::openapi::Responses::new()
///                         .with_response(
///                             "200",
///                             utoipa::openapi::Response::new("Pet found succesfully").with_content(
///                                 "application/json",
///                                 utoipa::openapi::Content::new(
///                                     utoipa::openapi::Ref::from_component_name("Pet"),
///                                 ),
///                             ),
///                         )
///                         .with_response("404", utoipa::openapi::Response::new("Pet was not found")),
///                 )
///                 .with_operation_id("get_pet_by_id")
///                 .with_deprecated(utoipa::openapi::Deprecated::False)
///                 .with_summary("Get pet by id")
///                 .with_description("Get pet by id\n\nGet pet from database by pet database id\n")
///                 .with_parameter(
///                     utoipa::openapi::path::Parameter::new("id")
///                         .with_in(utoipa::openapi::path::ParameterIn::Path)
///                         .with_deprecated(utoipa::openapi::Deprecated::False)
///                         .with_description("Pet database id to get Pet for")
///                         .with_schema(
///                             utoipa::openapi::Property::new(utoipa::openapi::ComponentType::Integer)
///                                 .with_format(utoipa::openapi::ComponentFormat::Int64),
///                         )
///                         .with_required(utoipa::openapi::Required::True),
///                 )
///                 .with_tag("pet_api"),
///         ),
///     );
/// ```
///
/// [derive]: attr.path.html
pub trait Path {
    fn path() -> &'static str;

    fn path_item(defalt_tag: Option<&str>) -> openapi::path::PathItem;
}

/// Trait that allows OpenApi modification at runtime.
///
/// Implement this trait if you wish to modify the OpenApi at runtime before it is being consumed
/// *(Before `utoipa::OpenApi::openapi()` function returns)*.
/// This is trait can be used to add or change already generated OpenApi spec to alter the generated
/// specification by user defined condition. For example you can add definitions that should be loaded
/// from some configuration at runtime what may not be available during compile time.
///
/// See more about [`OpenApi`][derive] derive at [derive documentation][derive].
///
/// [derive]: derive.OpenApi.html
/// [security_schema]: openapi/security/enum.SecuritySchema.html
///
/// # Examples
///
/// Add custom JWT [`SecuritySchema`][security_schema] to [`OpenApi`][`openapi::OpenApi`].
/// ```rust
/// # use utoipa::{OpenApi, Modify};
/// # use utoipa::openapi::security::{SecuritySchema, Http, HttpAuthenticationType};
/// #[derive(OpenApi)]
/// #[openapi(modifiers(&SecurityAddon))]
/// struct ApiDoc;
///
/// struct SecurityAddon;
///
/// impl Modify for SecurityAddon {
///    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
///        if let Some(components) = openapi.components.as_mut() {
///            components.add_security_schema(
///                "api_jwt_token",
///                SecuritySchema::Http(
///                    Http::new(HttpAuthenticationType::Bearer).with_bearer_format("JWT"),
///                ),
///            )
///        }
///    }
///}
/// ```
pub trait Modify {
    fn modify(&self, openapi: &mut openapi::OpenApi);
}
