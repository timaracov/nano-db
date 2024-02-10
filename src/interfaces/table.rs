struct Column {
    name: String
}

struct Schema {
    cols: [Column]
}

pub struct Table {
    schema: Schema,
}

impl Table {
    pub fn select_data() {}
}
