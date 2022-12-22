#[derive(Parser)]
#[grammar = "./src/parser/grammar.pest"]
struct HtmlParser;


type Attributes = HashMap<String, String>;
type Documents = Vec<Document>;

#[derive(Default, Clone, PartialEq)]
pub enum Document {
    Element(Element),
    Text(String),
    Comment(String),
    #[default]
    None
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Element {
    pub name: String,
    pub attributes: Attributes,
    pub children: Documents,
}

#[derive(Debug, Default, Clone)]
pub struct Dom {
    pub domtype: DomType,
    pub documents: Documents
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum DomType {
    #[default]
    Empty,
    Document,
    Fragment
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action<T> {
    Call(T),
    Handle(T)
}