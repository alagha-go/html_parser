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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Error {
    source: ErrorSource,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B")
    }
}


impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct ErrorSource(String);

impl std::fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ErrorSource{}

impl Error {
    pub fn new(error: String) -> Self {
        Self{source: ErrorSource(error)}
    }
}