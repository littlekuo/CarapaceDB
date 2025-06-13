
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlterType {
    Invalid = 0,
    AlterTable = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlterTableType {
    Invalid = 0,
    RenameColumn = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlterInfo {
    pub alter_type: AlterType,
}

impl AlterInfo {
    pub fn new(alter_type: AlterType) -> Self {
        AlterInfo { alter_type }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlterTableInfo {
    pub base: AlterInfo,
    pub alter_table_type: AlterTableType,
    pub schema: String,
    pub table: String,
}

impl AlterTableInfo {
    pub fn new(
        alter_table_type: AlterTableType,
        schema: impl Into<String>,
        table: impl Into<String>,
    ) -> Self {
        AlterTableInfo {
            base: AlterInfo::new(AlterType::AlterTable),
            alter_table_type,
            schema: schema.into(),
            table: table.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenameColumnInfo {
    pub base: AlterTableInfo,
    pub name: String,
    pub new_name: String,
}

impl RenameColumnInfo {
    pub fn new(
        schema: impl Into<String>,
        table: impl Into<String>,
        name: impl Into<String>,
        new_name: impl Into<String>,
    ) -> Self {
        RenameColumnInfo {
            base: AlterTableInfo::new(
                AlterTableType::RenameColumn,
                schema.into(),
                table.into(),
            ),
            name: name.into(),
            new_name: new_name.into(),
        }
    }
}
