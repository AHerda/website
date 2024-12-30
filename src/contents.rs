use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DataList {
    pub name: String,
    data: Vec<DataEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataEntry {
    pub id: u32,
    pub name: String,
    pub date: Option<Date>,
    pub cost: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: Option<u32>,
}

impl DataList {
    pub fn new(name: String, data: Vec<DataEntry>) -> Self {
        Self {
            name,
            data,
        }
    }

    pub fn add_entry(&mut self, entry: DataEntry) {
        self.data.push(entry);
    }

    pub fn get_entry(&self, id: u32) -> Option<&DataEntry> {
        self.data.iter().find(|entry| entry.id == id)
    }

    pub fn get_data(&self) -> &Vec<DataEntry> {
        &self.data
    }

    pub fn remove_entry(&mut self, id: u32) {
        self.data.retain(|entry| entry.id != id);
    }

    pub fn update_entry(&mut self, entry: DataEntry) {
        if let Some(index) = self.data.iter().position(|e| e.id == entry.id) {
            self.data[index] = entry;
        }
    }
}

impl DataEntry {
    pub fn new(id: u32, name: String, date: Option<Date>, cost: f64) -> Self {
        Self {
            id,
            name,
            date,
            cost,
        }
    }
}

impl Date {
    pub fn new(day: u32, month: u32, year: Option<u32>) -> Self {
        Self {
            day,
            month,
            year,
        }
    }
}
