use dom;
use html;

#[test]
fn test_parsing_single_element() {
    // Given an atribute map with the attribute id
    let mut attributes = dom::AttrMap::new();
    attributes.insert("id".to_string(), "test".to_string());

    // and given a div element with the attribute map above
    let expected = dom::make_element(
        "div".to_string(),
        attributes,
        [].to_vec(),
    );

    // When the following HTML string is parsed
    let parsed = html::parse("<div id='test'></div>".to_string());

    // Then it should match the hand crafted element
    assert_eq!(parsed, expected);
}
