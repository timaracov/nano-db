struct Column {
    name: String
}

struct Schema {
    cols: [Column]
}

struct Table {
    schema: Schema,
}

impl Table {
    pub fn select_data() {
    }
}
