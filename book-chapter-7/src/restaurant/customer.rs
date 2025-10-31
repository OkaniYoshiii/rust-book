pub struct Customer {
    name: String,
    pub has_ordered: bool,
    pub is_delivered: bool,
    pub table_number: u16,
}

impl Customer {
    pub fn is_waiting(&self) -> bool {
        return self.is_delivered == false && self.has_ordered == true;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}