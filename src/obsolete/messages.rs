use crate::contents::DataEntry;

pub enum Messages {
    AddEntry(DataEntry),
    RemoveEntry(u32),
    UpdateEntry(DataEntry),
}
