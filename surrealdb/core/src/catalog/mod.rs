//! SurrealDB Catalog definitions.
//!
//! The catalog is the collection of definitions (namespaces, databases, tables, fields, indexes,
//! etc) that are used to describe the state of the database.
//!
//! The catalog should be the only structs/enums that are stored physically in the KV Store.
#![warn(missing_docs)]

mod access;
pub mod aggregation;
mod auth;
mod database;
mod module;
mod namespace;
pub mod providers;
mod record;
mod schema;
mod subscription;
mod table;
mod view;

#[cfg(test)]
mod compat;
#[cfg(test)]
mod test;

pub use access::*;
pub use database::*;
pub use module::*;
pub use namespace::*;
pub use record::*;
pub use schema::ApiMethod;
pub use schema::{
	ApiDefinition, DiskAnnParams, Distance, FullTextParams, HnswParams, Scoring, VectorType, *,
};
pub use subscription::*;
pub use table::*;
pub use view::*;
