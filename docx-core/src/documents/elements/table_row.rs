use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::{Delete, Insert, TableCell, TableRowProperty};
use crate::{json_render, render_children, xml_builder::*, Render, TableCellContent};
use crate::{documents::BuildXML, HeightRule};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    pub cells: Vec<TableRowChild>,
    pub has_numbering: bool,
    pub property: TableRowProperty,
}

impl Render for TableRow {
    // Each cell in the row has its own newlines. We need to reconcile the ascii rendering of these across all cells.
    fn render_ascii_json(&self) -> crate::JsonRender {
        let child_ascii: Vec<Vec<_>> = self.cells.iter()
            .map(|c| {
                c.render_ascii().split("\n").map(String::from).collect::<Vec<_>>()
            })
            .collect();

        let Some(max_rows) = child_ascii.iter().map(|c| c.len()).max() else {
            return json_render!("TableRow", "| |");
        };
        
        // For each cell in row, get the i'th row of the cell, and merge with other cells in the row. 
        let ascii_rows: Vec<String> = (0..max_rows).map(|i| {
            let ascii_row = child_ascii.iter().map(|c| c.get(i).cloned().unwrap_or_default()).collect::<Vec<_>>();
            format!("| {} |", ascii_row.join(" | "))
        }).collect::<Vec<_>>();

        json_render!("TableRow", ascii_rows.join("\n"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableRowChild {
    TableCell(TableCell),
}

impl Render for TableRowChild {
    fn render_ascii_json(&self) -> crate::JsonRender {
        match self {
            TableRowChild::TableCell(TableCell { children,.. }) => {
                render_children(children, "\n", "TableCell", &serde_json::Value::Null)
            },
        }
    }
}

impl BuildXML for TableRowChild {
    fn build(&self) -> Vec<u8> {
        match self {
            TableRowChild::TableCell(v) => v.build(),
        }
    }
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> TableRow {
        let property = TableRowProperty::new();
        let has_numbering = cells.iter().any(|c| c.has_numbering);
        let cells = cells.into_iter().map(TableRowChild::TableCell).collect();
        Self {
            cells,
            property,
            has_numbering,
        }
    }

    pub fn grid_after(mut self, grid_after: u32) -> TableRow {
        self.property = self.property.grid_after(grid_after);
        self
    }

    pub fn width_after(mut self, w: f32) -> TableRow {
        self.property = self.property.width_after(w);
        self
    }

    pub fn grid_before(mut self, grid_before: u32) -> TableRow {
        self.property = self.property.grid_before(grid_before);
        self
    }

    pub fn width_before(mut self, w: f32) -> TableRow {
        self.property = self.property.width_before(w);
        self
    }

    pub fn row_height(mut self, h: f32) -> TableRow {
        self.property = self.property.row_height(h);
        self
    }

    pub fn height_rule(mut self, r: HeightRule) -> TableRow {
        self.property = self.property.height_rule(r);
        self
    }

    pub fn delete(mut self, d: Delete) -> TableRow {
        self.property = self.property.delete(d);
        self
    }

    pub fn insert(mut self, i: Insert) -> TableRow {
        self.property = self.property.insert(i);
        self
    }

    pub fn cant_split(mut self) -> TableRow {
        self.property = self.property.cant_split();
        self
    }
}

impl BuildXML for TableRow {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new()
            .open_table_row()
            .add_child(&self.property)
            .add_children(&self.cells);
        b.close().build()
    }
}

impl Serialize for TableRowChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TableRowChild::TableCell(ref r) => {
                let mut t = serializer.serialize_struct("TableCell", 2)?;
                t.serialize_field("type", "tableCell")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{Paragraph, Run};

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_row() {
        let b = TableRow::new(vec![TableCell::new()]).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tr><w:trPr /><w:tc><w:tcPr /><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr></w:p></w:tc></w:tr>"#
        );
    }

    #[test]
    fn test_row_json() {
        let r = TableRow::new(vec![TableCell::new()]);
        assert_eq!(
            serde_json::to_string(&r).unwrap(),
            r#"{"cells":[{"type":"tableCell","data":{"children":[],"property":{"width":null,"borders":null,"gridSpan":null,"verticalMerge":null,"verticalAlign":null,"textDirection":null,"shading":null},"hasNumbering":false}}],"hasNumbering":false,"property":{"gridAfter":null,"widthAfter":null,"gridBefore":null,"widthBefore":null}}"#
        );
    }

    #[test]
    fn test_row_cant_split() {
        let b = TableRow::new(vec![TableCell::new()]).cant_split().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tr><w:trPr><w:cantSplit /></w:trPr><w:tc><w:tcPr /><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr></w:p></w:tc></w:tr>"#
        );
    }

    #[test]
    fn test_multi_line_table_row_render() {
        let row = TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("hello").add_text("twice"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("world"))),
        ]);

        assert_eq!(
            row.render_ascii(),
            "| hello | world |\n| twice |  |"
        )
    }
}
