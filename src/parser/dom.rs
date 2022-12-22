impl Dom {
    pub fn new(html: &str) -> Result<Self> {
        let mut dom = Dom::default();
        let pairs = match HtmlParser::parse(Rule::html, html) {
            Ok(pairs) => pairs,
            Err(err) => return Err(err)?,
        };

        println!("{}", pairs.clone().count());

        
        for pair in pairs {
            println!("{:?}", pair.as_rule());
            match pair.as_rule() {
                Rule::doctype => {
                    dom.domtype = DomType::Document
                },
                Rule::element => {
                    dom.documents.push(Document::Element(pair.into_inner().try_into()?));
                },
                Rule::EOI => {
                    break
                }
                _ => ()
            }
        }

        if dom.documents.len() > 0 && dom.domtype == DomType::Empty {
            dom.domtype = DomType::Fragment;
        }
        Ok(dom)
    }
}