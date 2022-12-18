pub type Attributes = HashMap<String, String>;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Dom {
    pub documents: Vec<Document>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Document {
    Element(Node),
    Text(String),
    Comment(String)
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Node {
    pub name: String,
    pub attributes: Attributes,
    pub children: Vec<Document>
}


pub enum Action<T> {
    Call(T),
    Handle(T)
}


#[derive(Parser)]
#[grammar = "./src/parser/rules.pest"]
pub struct Grammar;