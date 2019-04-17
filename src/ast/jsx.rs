// document: https://facebook.github.io/jsx/


use ast::expression::AssignmentExpression;


// React only.
pub const CREATE_JSX_ELEMENT: &str = "React.createElement";
pub const CREATE_JSX_FRAGMENT: &str = "React.Fragment";

// IdentifierStart
// JSXIdentifier IdentifierPart
// JSXIdentifier NO WHITESPACE OR COMMENT -
pub type JSXIdentifier = String;
pub type JSXAttributes = Vec<JSXAttribute>;
pub type JSXChildren = Vec<JSXChild>;
pub type JSXMemberExpression = Vec<JSXIdentifier>;
// SourceCharacter but not one of {, <, > or }
pub type JSXText = String;
pub type JSXChildExpression = Vec<AssignmentExpression>;


// PrimaryExpression: JSXFragment, JSXElement
#[derive(Debug, PartialEq, Clone)]
pub struct JSXFragment {
    pub children: Option<JSXChildren>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JSXElement {
    SelfClosing(JSXSelfClosingElement),
    Normal(JSXNormalElement)
}

impl JSXElement {
    pub fn is_self_closing(&self) -> bool {
        match *self {
            JSXElement::SelfClosing(_) => true,
            JSXElement::Normal(_) => false,
        }
    }

    pub fn is_element_name_match(&self) -> bool {
        match *self {
            JSXElement::SelfClosing(_) => true,
            JSXElement::Normal(ref elem) => elem.opening.name == elem.closing.name,
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub struct JSXOpeningElement {
    pub name: JSXElementName,
    pub attrs: Option<JSXAttributes>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JSXClosingElement {
    pub name: JSXElementName,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JSXSelfClosingElement {
    pub name: JSXElementName,
    pub attrs: Option<JSXAttributes>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct JSXNormalElement {
    pub opening: JSXOpeningElement,
    pub children: Option<JSXChildren>,
    pub closing: JSXClosingElement,
}


#[derive(Debug, PartialEq, Clone)]
pub enum JSXElementName {
    Identifier(JSXIdentifier),
    NamespacedName(JSXNamespacedName),
    MemberExpression(JSXMemberExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct JSXNamespacedName {
    pub namespace: JSXIdentifier,
    pub name: JSXIdentifier,
}


#[derive(Debug, PartialEq, Clone)]
pub enum JSXAttribute {
    Normal(JSXNormalAttribute),
    Spread(AssignmentExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct JSXNormalAttribute {
    pub name: JSXNormalAttributeName,
    pub init: Option<JSXNormalAttributeInitializer>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JSXNormalAttributeName {
    Identifier(JSXIdentifier),
    NamespacedName(JSXNamespacedName),
}

#[derive(Debug, PartialEq, Clone)]
pub enum JSXNormalAttributeInitializer {
    Identifier(JSXIdentifier),
    Assignment(AssignmentExpression),
    Element(JSXElement),
    Fragment(JSXFragment),
}

#[derive(Debug, PartialEq, Clone)]
pub enum JSXChild {
    Text(JSXText),
    Element(JSXElement),
    ChildExpression(Option<JSXChildExpression>),
}
