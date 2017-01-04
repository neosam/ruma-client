//! Crate ruma_client is a [Matrix](https://matrix.org/) client library.

#![feature(try_from)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

extern crate hyper;
extern crate ruma_client_api;
extern crate ruma_identifiers;
extern crate serde;
extern crate serde_json;
extern crate url;

use std::convert::TryInto;

use hyper::client::{Client as Hyper, IntoUrl};
use hyper::method::Method as HyperMethod;
use ruma_client_api::{Endpoint, Method, supported_versions, r0};
use url::{Url, Host};
use std::io::Read;
use ruma_identifiers::UserId;
use std::convert::TryFrom;

pub use error::Error;
pub use session::Session;
pub use response::Response;

mod error;
mod response;
mod session;

/// A client for the Matrix client-server API.
#[derive(Debug)]
pub struct Client {
    homeserver_url: Url,
    hyper: Hyper,
    session: Option<Session>,
}

trait IntoHyperMethod {
    fn into_hyper(self) -> HyperMethod;
}

impl IntoHyperMethod for Method {
    fn into_hyper(self) -> HyperMethod {
        match self {
            Method::Delete => HyperMethod::Delete,
            Method::Get => HyperMethod::Get,
            Method::Put => HyperMethod::Put,
            Method::Post => HyperMethod::Post,
        }
    }
}

impl Client {
    /// Creates a new client for making requests to the given homeserver.
    ///
    /// # Errors
    ///
    /// Returns an error if the given homserver URL cannot be parsed as a URL.
    pub fn new<U>(homeserver_url: U) -> Result<Self, Error> where U: IntoUrl {
        Ok(Client {
            homeserver_url: homeserver_url.into_url()?,
            hyper: Hyper::new(),
            session: None,
        })
    }

    /// Get the versions of the Matrix client-server specification supported by the homeserver.
    pub fn get_supported_versions(&self)
    -> Result<Response<<supported_versions::Endpoint as Endpoint>::Response>, Error> {
        let response = self.hyper.request(
            supported_versions::Endpoint::method().into_hyper(),
            self.homeserver_url.join(&supported_versions::Endpoint::request_path(()))?,
        ).send()?;

        Ok(response.try_into()?)
    }

    /// Register as guest and set the session attributes.
    pub fn guest_session(&mut self) -> Result<(), Error> {
        // TODO: Should be handled by the ruma_client_apt
        let full_url = format!("{}?kind=guest",
            r0::account::register::Endpoint::request_path(()));
        let mut response = self.hyper
            .request(r0::account::register::Endpoint::method().into_hyper(),
                self.homeserver_url.join(&full_url)?)
            .body("{}")
            .send()?;
        let mut response_str = String::new();
        response.read_to_string(&mut response_str)?;
        let user_session: r0::account::register::Response = serde_json::from_str(&response_str)?;
        let user_id = UserId::try_from(&user_session.user_id)?;
        self.session = Some(Session::new(user_session.access_token,
            Host::Domain(user_session.home_server),
        user_id));
        Ok(())
    }
}
