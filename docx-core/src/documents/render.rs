

/// [ `Render` ] defines how documents, and its elements can be rendered into different formats.  
pub trait Render {

    /// Provide a ASCII representation of the element includable in a plain text output (e.g. console).
    fn render_ascii(&self) -> String {
        self.render_ascii_json().ascii
    }

    /// Provide a minimal JSON representation of the element.
    fn render_ascii_json(&self) -> JsonRender;
}

#[derive(Default)]
pub struct JsonRender {
    pub r#type: String,
    pub ascii: String,
    pub children: Vec<JsonRender>,
    pub properties: serde_json::Value,
}

/// Shorthand to make a [`JsonRender`] without any properties.
/// 
/// Example
/// ```rust
/// let json = json_render!("Text", "Hello");
/// ```
#[macro_export]
macro_rules! json_render {
    ($type:expr, $ascii:expr) => {
        crate::documents::render::JsonRender {
            r#type: $type.to_string(),
            ascii: $ascii.to_string(),
            children: vec![],
            properties: serde_json::Value::Null,
        }
    };
    ($type:expr, $ascii:expr, $children:expr) => {
        crate::documents::render::JsonRender {
            r#type: $type.to_string(),
            ascii: $ascii.to_string(),
            children: $children,
            properties: serde_json::Value::Null,
        }
    };
    ($type:expr, $ascii:expr, $children:expr, $properties:expr) => {
        crate::documents::render::JsonRender {
            r#type: $type.to_string(),
            ascii: $ascii.to_string(),
            children: $children,
            properties: $properties,
        }
    };
}

/// Shorthand to make a [`JsonRender`] with properties for a element with children that implement [`Render`].
pub fn render_children<T: Render>(children: &[T], children_join: &str, r#type: &str, properties: &serde_json::Value) -> JsonRender {
    let children_json: Vec<_> = children.iter().map(|c| c.render_ascii_json()).collect();
    let ascii: String = children_json.iter().map(|c| c.ascii.clone()).collect::<Vec<_>>().join(children_join);

    JsonRender {
        r#type: r#type.to_string(),
        children: children_json,
        ascii,
        properties: properties.clone(),
    }
}