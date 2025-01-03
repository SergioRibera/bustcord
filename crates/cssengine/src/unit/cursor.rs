#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum CursorIcon {
    /// Normal cursor icon, whatever that is.
    Default,

    /// Show no cursor
    None,

    // ------------------------------------
    // Links and status:
    /// A context menu is available
    ContextMenu,

    /// Question mark
    Help,

    /// Pointing hand, used for e.g. web links
    PointingHand,

    /// Shows that processing is being done, but that the program is still interactive.
    Progress,

    /// Not yet ready, try later.
    Wait,

    // ------------------------------------
    // Selection:
    /// Hover a cell in a table
    Cell,

    /// For precision work
    Crosshair,

    /// Text caret, e.g. "Click here to edit text"
    Text,

    /// Vertical text caret, e.g. "Click here to edit vertical text"
    VerticalText,

    // ------------------------------------
    // Drag-and-drop:
    /// Indicated an alias, e.g. a shortcut
    Alias,

    /// Indicate that a copy will be made
    Copy,

    /// Omnidirectional move icon (e.g. arrows in all cardinal directions)
    Move,

    /// Can't drop here
    NoDrop,

    /// Forbidden
    NotAllowed,

    /// The thing you are hovering can be grabbed
    Grab,

    /// You are grabbing the thing you are hovering
    Grabbing,

    // ------------------------------------
    /// Something can be scrolled in any direction (panned).
    AllScroll,

    // ------------------------------------
    // Resizing in two directions:
    /// Horizontal resize `-` to make something wider or more narrow (left to/from right)
    ResizeHorizontal,

    /// Diagonal resize `/` (right-up to/from left-down)
    ResizeNeSw,

    /// Diagonal resize `\` (left-up to/from right-down)
    ResizeNwSe,

    /// Vertical resize `|` (up-down or down-up)
    ResizeVertical,

    // ------------------------------------
    // Resizing in one direction:
    /// Resize something rightwards (e.g. when dragging the right-most edge of something)
    ResizeEast,

    /// Resize something down and right (e.g. when dragging the bottom-right corner of something)
    ResizeSouthEast,

    /// Resize something downwards (e.g. when dragging the bottom edge of something)
    ResizeSouth,

    /// Resize something down and left (e.g. when dragging the bottom-left corner of something)
    ResizeSouthWest,

    /// Resize something leftwards (e.g. when dragging the left edge of something)
    ResizeWest,

    /// Resize something up and left (e.g. when dragging the top-left corner of something)
    ResizeNorthWest,

    /// Resize something up (e.g. when dragging the top edge of something)
    ResizeNorth,

    /// Resize something up and right (e.g. when dragging the top-right corner of something)
    ResizeNorthEast,

    // ------------------------------------
    /// Resize a column
    ResizeColumn,

    /// Resize a row
    ResizeRow,

    // ------------------------------------
    // Zooming:
    /// Enhance!
    ZoomIn,

    /// Let's get a better overview
    ZoomOut,
}
