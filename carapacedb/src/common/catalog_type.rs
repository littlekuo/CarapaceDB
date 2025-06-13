#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum CatalogType {
    Invalid = 0,
    Table = 1,
    Schema = 2,
    TableFunction = 3,
    ScalarFunction = 4,
    View = 5,
    Index = 6,
    UpdatedEntry = 10,
    DeletedEntry = 11,
    PreparedStatement = 12,
    Sequence = 13,
}
