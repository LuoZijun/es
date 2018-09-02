use ast::IdentifierName;
use ast::expression::AssignmentExpression;


pub type BindingIdentifier = String;


#[derive(Debug, PartialEq, Clone)]
pub enum BindingPattern {
    Object(Box<ObjectBindingPattern>),
    Array(Box<ArrayBindingPattern>),
}


// Object
#[derive(Debug, PartialEq, Clone)]
pub enum PropertyName {
    // IdentifierName
    Identifier(IdentifierName),
    // StringLiteral
    String(String),
    // NumericLiteral
    Numeric(f64),
    Computed(AssignmentExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SingleNameBinding {
    pub identifier: BindingIdentifier,
    pub initializer: Option<AssignmentExpression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BindingProperty {
    SingleNameBinding(SingleNameBinding),
    KeyValue {
        key: PropertyName,
        value: BindingElement,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingPattern {
    pub properties: Vec<BindingProperty>,
    pub rest_property: Option<BindingIdentifier>,
}



// Array
#[derive(Debug, PartialEq, Clone)]
pub enum BindingElement {
    SingleNameBinding(SingleNameBinding),
    BindingPattern {
        pattern: BindingPattern,
        initializer: Option<AssignmentExpression>,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BindingRestElement {
    Identifier(BindingIdentifier),
    Pattern(BindingPattern),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrayBindingPatterItem {
    Elision, // Empty
    Element(BindingElement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayBindingPattern {
    pub elems: Vec<ArrayBindingPatterItem>,
    pub rest_property: Option<BindingRestElement>,
}
