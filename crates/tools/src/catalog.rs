//! The tool catalogue: ids, labels, shortcuts and Tools-panel groups (InDesign's order).

use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ToolInfo {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
    pub icon: &'static str,
    /// Original English usage help, also exposed by `tool.list` and `ui.tool.list`.
    /// Empty when no behavior-checked description has been written yet.
    pub description: &'static str,
}

const fn t(id: &'static str, label: &'static str, shortcut: Option<&'static str>, icon: &'static str) -> ToolInfo {
    ToolInfo { id, label, shortcut, icon, description: "" }
}

const fn explained(tool: ToolInfo, description: &'static str) -> ToolInfo {
    ToolInfo { description, ..tool }
}

/// Tools-panel groups in order (the first tool of each group shows by default). A `None` group
/// marks a separator.
///
/// In two columns InDesign lays some groups out tool by tool (Content Collector and Content
/// Placer side by side): see [`SPLIT_IN_TWO_COLUMNS`].
pub const TOOL_GROUPS: &[&[ToolInfo]] = &[
    &[explained(
        t("selection", "Selection Tool", Some("V"), "tool-selection"),
        "Select and move whole objects. Use Direct Selection to edit points or content inside a frame.",
    )],
    &[explained(
        t("directSelection", "Direct Selection Tool", Some("A"), "tool-direct"),
        "Select points or content inside a frame. Drag to change the selected part while keeping the frame in place.",
    )],
    &[explained(
        t("page", "Page Tool", Some("Shift+P"), "tool-page"),
        "Click to select a page. Alt-drag its handles to keep a new size; a plain drag previews the layout and springs back. Double-click for exact dimensions.",
    )],
    &[explained(
        t("gap", "Gap Tool", Some("U"), "tool-gap"),
        "Drag a gap between objects to move it. Objects on both sides resize as the gap moves.",
    )],
    &[
        explained(
            t("contentCollector", "Content Collector Tool", Some("B"), "tool-collector"),
            "Click an object to collect a copy. Use Content Placer to place collected objects elsewhere.",
        ),
        explained(
            t("contentPlacer", "Content Placer Tool", Some("B"), "tool-placer"),
            "Click to place the next collected object. Collect an object first with Content Collector.",
        ),
    ],
    &[],
    &[
        explained(t("type", "Type Tool", Some("T"), "tool-type"), "Click inside text to edit it, or drag on empty canvas to draw a text frame."),
        explained(
            t("typeOnPath", "Type on a Path Tool", Some("Shift+T"), "tool-type-path"),
            "Click an empty path to put text on it, then use the Type Tool to enter the text.",
        ),
        explained(
            t("verticalType", "Vertical Type Tool", None, "tool-type-vertical"),
            "Click to edit vertical text, or drag on empty canvas to create a vertical text frame.",
        ),
        explained(
            t("verticalTypeOnPath", "Vertical Type on a Path Tool", None, "tool-type-path"),
            "This tool is not implemented in this build. Choose another tool from the Tools panel.",
        ),
    ],
    &[explained(t("line", "Line Tool", Some("\\"), "tool-line"), "Drag to draw a line. Hold Shift to constrain its angle.")],
    &[
        explained(
            t("pen", "Pen Tool", Some("P"), "tool-pen"),
            "Click to add corners, or drag to make curves. Click the first point to close the path.",
        ),
        explained(
            t("addAnchor", "Add Anchor Point Tool", Some("="), "tool-pen-add"),
            "Click a path segment to add an anchor point. Use Direct Selection to move individual points.",
        ),
        explained(t("deleteAnchor", "Delete Anchor Point Tool", Some("-"), "tool-pen-delete"), "Click an anchor point to remove it from the path."),
        explained(
            t("convertDirection", "Convert Direction Point Tool", Some("Shift+C"), "tool-anchor"),
            "Click an anchor point to change between a corner and a smooth curve.",
        ),
    ],
    &[
        explained(t("pencil", "Pencil Tool", Some("N"), "tool-pencil"), "Drag to draw a freehand path. Release to finish the stroke."),
        explained(t("smooth", "Smooth Tool", None, "tool-smooth"), "Drag along a selected path to smooth its points."),
        explained(t("erase", "Erase Tool", None, "tool-erase"), "Drag across a selected path to erase part of it."),
    ],
    &[
        explained(
            t("rectangleFrame", "Rectangle Frame Tool", Some("F"), "tool-rect-frame"),
            "Drag to create an empty frame for a graphic. The frame and the image inside it can be resized separately.",
        ),
        explained(
            t("ellipseFrame", "Ellipse Frame Tool", None, "tool-ellipse-frame"),
            "Drag to create an elliptical frame for a graphic. Hold Shift for a circular frame.",
        ),
        explained(
            t("polygonFrame", "Polygon Frame Tool", None, "tool-polygon-frame"),
            "Drag to create a polygonal frame for a graphic. Use Polygon Settings to change its sides.",
        ),
    ],
    &[
        explained(t("rectangle", "Rectangle Tool", Some("M"), "tool-rect"), "Drag to draw a rectangle. Hold Shift to draw a square."),
        explained(t("ellipse", "Ellipse Tool", Some("L"), "tool-ellipse"), "Drag to draw an ellipse. Hold Shift to draw a circle."),
        explained(
            t("polygon", "Polygon Tool", None, "tool-polygon"),
            "Drag to draw a polygon. Use Polygon Settings to change its sides or star inset.",
        ),
    ],
    &[],
    &[explained(
        t("scissors", "Scissors Tool", Some("C"), "tool-scissors"),
        "Click a selected path to cut it at that point. The remaining pieces can be selected separately.",
    )],
    &[
        explained(
            t("freeTransform", "Free Transform Tool", Some("E"), "tool-free-transform"),
            "This tool is not implemented in this build. Choose another tool from the Tools panel.",
        ),
        explained(t("rotate", "Rotate Tool", Some("R"), "tool-rotate"), "Select an object, then drag around its center to rotate it."),
        explained(t("scale", "Scale Tool", Some("S"), "tool-scale"), "Select an object, then drag to resize it."),
        explained(t("shear", "Shear Tool", Some("O"), "tool-shear"), "Select an object, then drag to slant its shape."),
    ],
    &[explained(
        t("gradientSwatch", "Gradient Swatch Tool", Some("G"), "tool-gradient"),
        "Select an object with a gradient fill, then drag to set the gradient direction and length.",
    )],
    &[explained(
        t("gradientFeather", "Gradient Feather Tool", Some("Shift+G"), "tool-gradient-feather"),
        "Select an object, then drag to set how its gradient transparency fades.",
    )],
    &[],
    &[explained(t("note", "Note Tool", None, "tool-note"), "This tool is not implemented in this build. Choose another tool from the Tools panel.")],
    &[
        explained(
            t("colorTheme", "Color Theme Tool", Some("Shift+I"), "tool-color-theme"),
            "Click a placed image to preview a palette from its colors. Add swatches from the preview separately.",
        ),
        explained(
            t("eyedropper", "Eyedropper Tool", Some("I"), "tool-eyedropper"),
            "Click to sample a color or object attributes. Select the destination object first when applying the sample.",
        ),
        explained(
            t("measure", "Measure Tool", Some("K"), "tool-measure"),
            "This tool is not implemented in this build. Choose another tool from the Tools panel.",
        ),
    ],
    &[explained(t("hand", "Hand Tool", Some("H"), "tool-hand"), "Drag to view another part of the document. The page and objects stay in place.")],
    &[explained(t("zoom", "Zoom Tool", Some("Z"), "tool-zoom"), "Click to zoom in, or Alt-click to zoom out. The page size stays the same.")],
];

/// Groups (by their first tool) whose tools get a button each when the Tools panel has two
/// columns, as InDesign's does.
pub const SPLIT_IN_TWO_COLUMNS: &[&str] = &["contentCollector"];

pub fn tool_info(id: &str) -> Option<ToolInfo> {
    TOOL_GROUPS.iter().flat_map(|g| g.iter()).find(|t| t.id == id).copied()
}

/// Tool for a single-key shortcut (`"T"`, `"Shift+P"`).
pub fn tool_for_shortcut(key: &str) -> Option<&'static str> {
    TOOL_GROUPS.iter().flat_map(|g| g.iter()).find(|t| t.shortcut == Some(key)).map(|t| t.id)
}
