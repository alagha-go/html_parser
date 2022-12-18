impl Node {
    fn new(name: String, attributes: Attributes, children: Vec<Document>) -> Self {
        let id = match attributes.get("id") {
            None => None,
            Some(value) => Some(String::from(value))
        };
        let classes = match attributes.get("class") {
            Some(value) => {
                let array: Vec<&str> = value.split(' ').collect();
                let mut hash_set = HashSet::new();
                for value in array {
                    hash_set.insert(String::from(value));
                }
                hash_set
            },
            None => HashSet::new()
        };
        Self{name, id, classes, attributes, children}
    }
}