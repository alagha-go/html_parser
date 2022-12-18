impl Node {
    pub fn new(name: String, attributes: Attributes, children: Vec<Document>) -> Self {
        Self{name, attributes, children}
    }

    pub fn id<'a>(&'a self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn class<'a>(&'a self) -> Option<&String> {
        self.attributes.get("class")
    }
}