//! Code for applying CSS styles to the DOM.
//!
//! This is not very interesting at the moment.  It will get much more
//! complicated if I add support for compound selectors.

use crate::css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::console;


/// Map from CSS property names to values.
pub type PropertyMap = HashMap<String, Value>;

/// A node with associated style data.
#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub  children: Vec<StyledNode<'a>>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    /// Return the specified value of a property if it exists, otherwise `None`.
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    /// Return the specified value of property `name`, or property `fallback_name` if that doesn't
    /// exist, or value `default` if neither does.
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }

    /// The value of the `display` property (defaults to inline).
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

/// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
///
/// This finds only the specified values at the moment. Eventually it should be extended to find the
/// computed values too, including inherited values.
pub fn style_tree<'a>(
    root: &'a Node,
    stylesheet: &'a Stylesheet,
    parent: Option<&StyledNode<'a>>,
) -> StyledNode<'a> {
    let mut node = StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(parent, elem, stylesheet),
            NodeType::Text(_) => inherit_rules(parent),
        },
        children: Vec::new()
    };
    node.children =  root
        .children
        .iter()
        .map(|child| style_tree(child, stylesheet, Some(&node)))
        .collect();

    node
}

/// Apply styles to a single element, returning the specified styles.
///
/// To do: Allow multiple UA/author/user stylesheets, and implement the cascade.
fn specified_values(parent: Option<&StyledNode>, elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = inherit_rules(parent);
    let mut rules = matching_rules(elem, stylesheet);

    // Go through the rules from lowest to highest specificity.
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

fn inherit_rules(
    parent: Option<&StyledNode>
) -> PropertyMap {
    let mut values = HashMap::new();
    match parent {
        Some(node) => {
            for (key, val) in node.specified_values.iter() {
                if should_inherit_rule(key) {
                    values.insert(key.clone(), val.clone());
                }
            };
            values
        }
        _ => values,
    }
}

/// A single CSS rule and the specificity of its most specific matching selector.
type MatchedRule<'a> = (Specificity, &'a Rule);

/// Find all CSS rules that match the given element.
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    // For now, we just do a linear scan of all the rules.  For large
    // documents, it would be more efficient to store the rules in hash tables
    // based on tag name, id, class, etc.
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

/// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    // Find the first (most specific) matching selector.
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

/// Selector matching:
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID selector
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check class selectors
    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }

    // We didn't find any non-matching selector components.
    true
}

fn should_inherit_rule(rule: &String) -> bool {
    match rule.as_ref() {
        "color" => true,
        _ => false,
    }
}
