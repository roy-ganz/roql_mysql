
use crate::error::ToqlMySqlError;
use toql::sql_builder::select_stream::Select;

/// Newtype for mysql database row
/// This allows to implement the conversion traits for basic data
/// without violating the orphan rule
pub struct Row(pub mysql::Row);


impl toql::from_row::FromRow<Row, ToqlMySqlError> for bool {
 fn from_row<'a, I>(
        row: &Row,
        i: &mut usize,
        iter: &mut I,
    ) -> std::result::Result<Option<bool>, ToqlMySqlError>
    where
        I: Iterator<Item = &'a Select>,
    {
         if iter . next() . unwrap_or(& Select :: None) != & Select :: None {
            let x = row.0.get_opt(*i).unwrap().map_err(mysql::error::Error::from)?;
            *i += 1;
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }
}

impl toql::from_row::FromRow<Row, ToqlMySqlError> for u16 {
 fn from_row<'a, I>(
        row: &Row,
        i: &mut usize,
        iter: &mut I,
    ) -> std::result::Result<Option<u16>, ToqlMySqlError>
    where
        I: Iterator<Item = &'a Select>,
    {
         if iter . next() . unwrap_or(& Select :: None) != & Select :: None {
            let x = row.0.get_opt(*i).unwrap().map_err(mysql::error::Error::from)?;
            *i += 1;
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }
}

impl toql::from_row::FromRow<Row, ToqlMySqlError> for u64 {
 fn from_row<'a, I>(
        row: &Row,
        i: &mut usize,
        iter: &mut I,
    ) -> std::result::Result<Option<u64>, ToqlMySqlError>
    where
        I: Iterator<Item = &'a Select>,
    {
         if iter . next() . unwrap_or(& Select :: None) != & Select :: None {
            let x = row.0.get_opt(*i).unwrap().map_err(mysql::error::Error::from)?;
            *i += 1;
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }
}

impl toql::from_row::FromRow<Row, ToqlMySqlError> for String {
 fn from_row<'a, I>(
        row: &Row,
        i: &mut usize,
        iter: &mut I,
    ) -> std::result::Result<Option<String>, ToqlMySqlError>
    where
        I: Iterator<Item = &'a Select>,
    {
         if iter . next() . unwrap_or(& Select :: None) != & Select :: None {
            let x = row.0.get_opt(*i).unwrap().map_err(mysql::error::Error::from)?;
            *i += 1;
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }
}