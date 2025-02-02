use std::fmt::{self, Display, Formatter};

pub enum FlexEnum {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Display for FlexEnum {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FlexEnum::Row => write!(f, "row"),
            FlexEnum::Column => write!(f, "column"),
            FlexEnum::RowReverse => write!(f, "row-reverse"),
            FlexEnum::ColumnReverse => write!(f, "column-reverse"),
        }
    }
}
