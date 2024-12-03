use smallvec::SmallVec;

use crate::declaration::Declaration;
use crate::{css_to_rules, Selector};

pub struct StyleSheet<'a> {
    rules: SmallVec<[(Selector<'a>, SmallVec<[Declaration; 32]>); 32]>,
}

impl<'a> StyleSheet<'a> {
    pub const fn new_const() -> Self {
        Self {
            rules: SmallVec::new_const(),
        }
    }

    /// Parse a CSS input string into a `StyleSheet`.
    pub fn from_css(input: &'a str) -> Self {
        let rules = css_to_rules(input)
            .unwrap_or_default()
            .into_iter()
            .map(|rule| {
                let declarations: SmallVec<[Declaration; 32]> = rule
                    .iter_props()
                    .filter_map(Declaration::from_cow)
                    .collect();
                rule.selectors
                    .into_iter()
                    .map(|selector| (selector, declarations.clone()))
                    .collect::<SmallVec<[(Selector, SmallVec<_>); 32]>>()
            })
            .flatten()
            .collect();

        Self { rules }
    }

    /// Get the styles for a given component or class.
    pub fn get_styles(&self, selector: &str) -> Option<SmallVec<[Declaration; 32]>> {
        self.rules
            .iter()
            .find(|(sel, _)| sel.selector.contains(selector) || sel.selector == "*")
            .map(|(_, declarations)| declarations.clone())
    }
}
