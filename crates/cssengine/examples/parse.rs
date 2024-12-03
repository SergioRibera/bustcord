use cssengine::StyleSheet;

fn main() {
    let css = r#"
        button {
          border-radius: 5px;
        }
        .my-class {
            color: red;
            background-color: blue;
        }

        #super-id {
          color: #000;
        }
    "#;

    let stylesheet = StyleSheet::from_css(css);

    let classes = [
        "button",
        ".my-class",
        "#super-id",
        "#super-id .my-class",
        ".other-class",
    ];

    for class in &classes {
        if let Some(styles) = stylesheet.get_styles(class) {
            println!("{class:?}");
            for style in styles {
                println!("\t{style:?}");
            }
        } else {
            println!("No styles found for the given selector ({class}).");
        }
    }
}
