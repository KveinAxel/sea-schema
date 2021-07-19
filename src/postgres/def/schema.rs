#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Schema {
    pub schema: String,
    pub tables: Vec<TableDef>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct TableDef {
    pub info: TableInfo,
    pub columns: Vec<ColumnInfo>,

    pub check_constraints: Vec<Check>,
    pub unique_keys: Vec<Unique>,
    pub references: Vec<References>,

    pub of_type: Option<Type>,
	// TODO:
    // pub inherets: Vec<String>,
}