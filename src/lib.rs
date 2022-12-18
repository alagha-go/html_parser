mod parser;
pub use parser::*;




#[cfg(test)]
mod tests {
    use static_init::dynamic;

    const HTML: &str = r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8"></meta>
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Document</title>
    </head>
    <body>
        <h1 id="id" class="class">Hello World!!!</h1>
        <h1>Trial @</h1>
        <span id="id" class="class"></span>
        <!-- trial comment -->
    </body>
    </html>"#;

    #[dynamic]
    static DOCUMENT: Document = Dom::new(HTML).documents[0].clone();

    use super::*;
    #[test]
    fn get_all_elements_by_class() {
        let documents = DOCUMENT.get_all_elements_by_class(&String::from("class"));
        assert_eq!(documents.len(), 2);
        assert_eq!((documents[0].name(), documents[1].name()), (Some(&String::from("h1")), Some(&String::from("span"))))
    }

    #[test]
    fn is() {
        let documents = DOCUMENT.get_all_elements_by_name(&String::from("body"));
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].is_element(), true);
        let documents = documents[0].children().unwrap();
        assert_eq!(documents[documents.len()-1].is_comment(), true);
        assert_eq!(documents[0].children().unwrap()[0].is_text(), true)
    }

    #[test]
    fn get_element_by_name() {
        let document = DOCUMENT.get_element_by_name(&String::from("span"), 0).unwrap();
        assert_eq!(document.id(), Some(&String::from("id")));
        assert_eq!(document.class(), Some(&String::from("class")));
    }

    #[test]
    fn text() {
        let document = DOCUMENT.get_element_by_id(0, &String::from("id")).unwrap();
        assert_eq!(document.text(), Some(&String::from("Hello World!!!")))
    }

    #[test]
    fn comment() {
        let document = DOCUMENT.get_element_by_name(&String::from("body"), 0).unwrap();
        assert_eq!(document.comment(), Some(&String::from("trial comment")))
    }

    #[test]
    fn attribute() {
        let documents = DOCUMENT.get_all_elements_by_attribute("charset", &String::from("UTF-8"));
        assert_eq!(documents.len(), 1);
        let document = DOCUMENT.get_element_by_attribute(0, "charset", &String::from("UTF-8")).unwrap();
        assert_eq!(document.name(), Some(&String::from("meta")));
        let document = DOCUMENT.get_element_by_attribute(1, "charset", &String::from("UTF-8"));
        assert_eq!(document, None);
    }
}
