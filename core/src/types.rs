/// It can be serialized as a database text field
use crate::db::{DbData, DbDataType};

pub trait TextField: Sized{
    fn from_db_data(data: &Box<dyn DbData>) -> Self;
    fn to_db_data(&self) -> String;
    fn db_field_type() -> DbDataType {
        DbDataType::Text
    }
}

pub trait NullableTextField: Sized {
    fn from_db_data(data: &Box< dyn DbData>) -> Self;
    fn to_db_data(&self) -> Option<String>;
    fn db_field_type() -> DbDataType {
        DbDataType::NullableText
    }
}

/// It can be serailized as a 64-bit integer
pub trait IntegerField: Sized {
    fn from_db_data(data: &Box<dyn DbData>) -> Self;
    fn to_db_data(&self)-> i64;
    fn db_field_type() -> DbDataType {
        DbDataType::Int
    }
}

pub trait NullableIntegerField: Sized {
    fn from_db_data(data: &Box<dyn DbData>) -> Self;
    fn to_db_data(&self)-> Option<i64>;
    fn db_field_type() -> DbDataType {
        DbDataType::NullableInt
    }
}

impl TextField for String {
    fn from_db_data(data: &Box<dyn DbData>) -> String {
        <String as DbData>::from_boxed_db_data(data)
    }
    fn to_db_data(&self) -> String {
        self.to_owned()
    }
}

impl NullableTextField for Option<String> {
    fn from_db_data(data: &Box<dyn DbData>) -> Option<String> {
        <Option<String> as DbData>::from_boxed_db_data(data)
    }
    fn to_db_data(&self) -> Option<String> {
        match self {
            None => None,
            Some(x) => Some(x.to_owned())
        }
    }
}

impl IntegerField for i64 {
    fn from_db_data(data: &Box<dyn DbData>) -> Self {
        <i64 as DbData>::from_boxed_db_data(data)
    }
    fn to_db_data(&self)-> i64 {
        *self
    }
}

impl NullableIntegerField for Option<i64> {
    fn from_db_data(data: &Box< dyn DbData>) -> Self {
        <Option<i64> as DbData>::from_boxed_db_data(data)
    }
    fn to_db_data(&self) -> Option<i64> {
        *self
    }
}

/// Auto increment row ID field. It will be represented as an integer primary key.
/// 
/// A schema can has at most one RowID field.
#[derive(Clone, Copy, Debug)]
pub enum RowID {
    /// A new created object so it doesn't have a row id yet.
    NEW,
    /// The row id retrieved from the database.
    ID(i64)
}

impl RowID {
    pub fn from_db_data(data: &Box<dyn DbData>) -> RowID{
        <RowID as DbData>::from_boxed_db_data(data)
    }
    pub fn to_db_data(&self) -> RowID {
        self.clone()
    }
    pub fn db_field_type() -> DbDataType {
        DbDataType::RowID
    }
}

/// Make the type a data schema in the relational database.
/// 
/// In most cases, you should only use the derive macro to implement this trait.
pub trait Schema: 'static {
    /// the schema name in database
    fn get_schema_name() -> String;
    /// the list of field names and types of this data struct
    fn get_fields() -> Vec<(String, DbDataType)>;
    /// the values of all fields in boxed DbData objects.
    fn get_values(&self) -> Vec<Box<dyn DbData>>;
    /// to create the struct with valeus of all fields in boxed DbData objects
    fn create_with_values(values: Vec<Box<dyn DbData>>) -> Self;

    /// get the name and value of the RowID field.
    /// Return `None` if there is no such field. Panic if there are more than one RowID field.
    fn get_row_id_field(&self) -> Option<(String, RowID)> {
        let fields = Self::get_fields();
        let values = Self::get_values(&self);
        let mut answer = None;
        for i in 0..fields.len() {
            let (field_name, field_type) = &fields.get(i).unwrap();
            if let DbDataType::RowID = field_type{
                if answer.is_none() {
                    let field_value = values.get(i).unwrap();
                    let answer_field_name = field_name.to_owned();
                    let answer_field_value = RowID::from_db_data(field_value);
                    answer = Some((answer_field_name, answer_field_value));
                } else {
                    panic!("Multiple Row ID fields found in {}", Self::get_schema_name());
                }
            }
        }
        return answer;
    }
}