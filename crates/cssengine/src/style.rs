use smallvec::SmallVec;

use crate::declaration::Declaration;
use crate::{css_to_rules, Selector};

#[cfg(feature = "tailwind_colors")]
use crate::{TAILWIND_COLORS, TAILWIND_NAME_COLORS};

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
    pub fn get_styles(&mut self, selector: &'a str) -> Option<SmallVec<[Declaration; 32]>> {
        if let Some((_, declarations)) = self
            .rules
            .iter()
            .find(|(sel, _)| sel.selector.contains(selector))
        {
            return Some(declarations.clone());
        }

        #[cfg(feature = "tailwind_colors")]
        {
            let mut generated_declarations = SmallVec::<[Declaration; 32]>::new();

            for token in selector.split_whitespace() {
                let Some(class) = token.strip_prefix('.') else {
                    continue;
                };

                for prefix in ["bg", "text", "border", "outline"] {
                    let Some(class) = class.strip_prefix(&format!("{prefix}-")) else {
                        continue;
                    };
                    let class = class.to_ascii_lowercase();
                    let Some((name, tone_str)) = class.split_once('-') else {
                        continue;
                    };

                    if !TAILWIND_NAME_COLORS.contains(&name) {
                        continue;
                    }
                    let Ok(tone) = tone_str.parse::<usize>() else {
                        continue;
                    };
                    let Some(color) = TAILWIND_COLORS
                        .get(format!("{name}-{tone}").as_str())
                        .cloned()
                    else {
                        continue;
                    };

                    let declaration = match prefix {
                        "bg" => Declaration::BackgroundColor(color),
                        "text" => Declaration::Color(color),
                        "border" => Declaration::BorderColor(color),
                        "outline" => Declaration::OutlineColor(color),
                        _ => continue,
                    };
                    generated_declarations.push(declaration);
                }
            }

            if !generated_declarations.is_empty() {
                self.rules
                    .push((Selector::from(selector), generated_declarations.clone()));
                return Some(generated_declarations);
            }
        }

        None
    }
}
