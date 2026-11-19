/*!
Provides the [[DataService]].

This module provides a public facade over
[redb](https://docs.rs/redb/latest/redb/).
Internally, it is not a facade -- we expose `redb` to the crate.
A database file can be viewed with
[redb-view](https://crates.io/crates/redb-view).

In `redb` there can be only one open write transaction and once it is
committed this transaction instance is consumed.
On the other hand many read transactions can be open at the same time.
When a read transaction is opened it read only *committed* data.

The idea of transaction is a powerful concepts -- especially for long running
updates. It therefore makes sense to keep the concept as part of the facade.

The downside of transaction is the consumers that complexity transfers
to consumers.
*/

#[cfg(test)]
mod helpers;
#[cfg(test)]
mod tests;

use redb::{Database, ReadableDatabase as _};
use std::path::Path;

use crate::result::*;

/**
Provide access to persisted data.
*/
pub struct DataService(Database);

impl DataService {
    pub fn open(p: impl AsRef<Path>) -> CoreResult<Self> {
        let db = Database::create(p)?;
        Ok(Self(db))
    }

    pub fn write(&self) -> CoreResult<WriteTransaction> {
        let tx = self.0.begin_write()?;
        Ok(WriteTransaction(tx))
    }

    pub fn read(&self) -> CoreResult<ReadTransaction> {
        let tx = self.0.begin_read()?;
        Ok(ReadTransaction(tx))
    }
}

pub struct ReadTransaction(redb::ReadTransaction);

impl ReadTransaction {
    pub(crate) fn open(&self, def: TableDef) -> CoreResult<BaseReadTable> {
        let r = self.0.open_table(def);
        let it = match r {
            Ok(tab) => BaseReadTable(Some(tab)),
            Err(redb::TableError::TableDoesNotExist(_)) => BaseReadTable(None),
            Err(e) => return Err(e.into()),
        };
        Ok(it)
    }
}

//? is static lifetime good here?
pub type TableDef = redb::TableDefinition<'static, String, String>;
type Table<'t> = redb::Table<'t, String, String>;
type ReadTable = redb::ReadOnlyTable<String, String>;
pub(crate) struct BaseReadTable(Option<ReadTable>);

impl BaseReadTable {
    pub(crate) fn read_o<O: serde::de::DeserializeOwned>(
        &self,
        key: impl Into<String>,
    ) -> CoreResult<Option<O>> {
        if let Some(t) = &self.0 {
            if let Some(s) = t.get_owned(key.into())? {
                let obj: O = serde_json::from_str(&s.value())?;
                return Ok(Some(obj));
            }
        };
        Ok(None)
    }
}

pub struct WriteTransaction(redb::WriteTransaction);

impl WriteTransaction {
    pub(crate) fn open(&self, def: TableDef) -> CoreResult<BaseWriteTable<'_>> {
        let tab = self.0.open_table(def)?;
        Ok(BaseWriteTable(tab))
    }

    pub(crate) fn commit(self) -> CoreResult {
        self.0.commit()?;
        Ok(())
    }
}

pub(crate) struct BaseWriteTable<'t>(Table<'t>);

impl BaseWriteTable<'_> {
    pub(crate) fn insert<O: serde::Serialize>(
        &mut self,
        key: impl Into<String>,
        obj: &O,
    ) -> CoreResult {
        let obj = serde_json::to_value(obj)?;
        self.0.insert(key.into(), obj.to_string())?;
        Ok(())
    }
}

impl From<redb::CommitError> for CoreError {
    fn from(value: redb::CommitError) -> Self {
        let err = format!("A database error occurred. {}", value);
        CoreError::make(err)
    }
}

impl From<redb::DatabaseError> for CoreError {
    fn from(value: redb::DatabaseError) -> Self {
        let err = format!("A database error occurred. {}", value);
        CoreError::make(err)
    }
}

impl From<redb::StorageError> for CoreError {
    fn from(value: redb::StorageError) -> Self {
        let err = format!("A storage error occurred. {}", value);
        CoreError::make(err)
    }
}

impl From<redb::TransactionError> for CoreError {
    fn from(value: redb::TransactionError) -> Self {
        let err = format!("A transaction error occurred. {}", value);
        CoreError::make(err)
    }
}

impl From<redb::TableError> for CoreError {
    fn from(value: redb::TableError) -> Self {
        let err = format!("A table error occurred. {}", value);
        CoreError::make(err)
    }
}
