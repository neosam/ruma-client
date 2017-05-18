//! Crate `ruma_client` is a [Matrix](https://matrix.org/) client library.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![feature(conservative_impl_trait, try_from)]

extern crate futures;
extern crate hyper;
extern crate ruma_api;
pub extern crate ruma_client_api;
extern crate ruma_identifiers;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio_core;
extern crate url;

use std::convert::TryInto;

use futures::{Future, IntoFuture};
use futures::future::FutureFrom;
use hyper::Client as HyperClient;
use hyper::client::HttpConnector;
use ruma_api::Endpoint;
use tokio_core::reactor::Handle;
use url::Url;

pub use error::Error;
pub use session::Session;

mod error;
mod session;

/// A client for the Matrix client-server API.
#[derive(Debug)]
pub struct Client {
    homeserver_url: Url,
    hyper: HyperClient<HttpConnector>,
    session: Option<Session>,
}

impl Client {
    /// Creates a new client for making requests to the given homeserver.
    pub fn new(handle: &Handle, homeserver_url: Url) -> Self {
        Client {
            homeserver_url,
            hyper: HyperClient::configure().keep_alive(true).build(handle),
            session: None,
        }
    }

    /// Makes a request to a Matrix API endpoint.
    pub fn request<E: Endpoint>(
        &self,
        request: <E as Endpoint>::Request,
    ) -> impl Future<Item = E::Response, Error = Error> {
        let cloned_hyper = self.hyper.clone();

        request
            .try_into()
            .map_err(Error::from)
            .into_future()
            .and_then(move |hyper_request| cloned_hyper.request(hyper_request).map_err(Error::from))
            .and_then(
                |hyper_response| E::Response::future_from(hyper_response).map_err(Error::from),
            )
    }
}
