

/// [ `Render` ] defines how documents, and its components can be rendered into different formats.  
pub trait Render {

    /// Provide a ASCII representation of the component includable in a plain text output (e.g. console).
    fn render_ascii(&self) -> Vec<u8> {
        self.render_ascii_json().ascii
    }

    /// Provide a minimal JSON representation of the component.
    fn render_ascii_json(&self) -> JsonRender;
}

#[derive(Default)]
pub struct JsonRender {
    pub r#type: String,
    pub ascii: Vec<u8>,
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
        JsonRender {
            r#type: $type.to_string(),
            ascii: $ascii.to_string().as_bytes().to_vec(),
            properties: serde_json::Value::Null,
        }
    };
}

/// Shorthand to make a [`JsonRender`] with properties for a component with children that implement [`Render`].
/// Does not include children properties in the resultant [`JsonRender`].
pub fn render_children<T: Render>(children: &[T], r#type: &str, properties: &serde_json::Value) -> JsonRender {
    let children_ascii: Vec<u8> = children
        .iter()
        .flat_map(|c| c.render_ascii())
        .collect();

    JsonRender {
        r#type: r#type.to_string(),
        ascii: children_ascii,
        properties: properties.clone(),
    }
}