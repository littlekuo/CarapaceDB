use std::{sync::Weak};

use crate::{common::buffered_file_writer::BufferedFileWriter, core::database::DuckDB};


/// The WriteAheadLog (WAL) is a log that is used to provide durability. Prior
/// to committing a transaction it writes the changes the transaction made to
/// the database to the log, which can then be replayed upon startup in case the
/// server crashes or is shut down.
pub struct WriteAheadLog {
    pub initialized: bool,
    database: Weak<DuckDB>,
    writer: Box<BufferedFileWriter>,
}
