use stretch::node;

use crate::ui::core::{Number, Size, Style};

/// The visual requirements of a [`Widget`] and its children.
///
/// When there have been changes and the [`Layout`] needs to be recomputed, the
/// runtime obtains a [`Node`] by calling [`Widget::node`].
///
/// [`Style`]: struct.Style.html
/// [`Widget`]: widget/trait.Widget.html
/// [`Node`]: struct.Node.html
/// [`Widget::node`]: widget/trait.Widget.html#tymethod.node
/// [`Layout`]: struct.Layout.html
pub struct Node(pub(super) node::Node);

impl Node {
    /// Creates a new [`Node`] with the given [`Style`] and children.
    ///
    /// [`Node`]: struct.Node.html
    /// [`Style`]: struct.Style.html
    pub fn new(style: Style, children: Vec<Node>) -> Node {
        Node(node::Node::new(
            style.0,
            children.iter().map(|c| &c.0).collect(),
        ))
    }

    /// Creates a new [`Node`] with the given [`Style`] and a measure function.
    ///
    /// You should use this when your [`Widget`] can adapt its contents to the
    /// size of its container. The measure function will receive the container
    /// [`Size`] as a parameter and must compute the size of the [`Node`] inside
    /// the given bounds.
    ///
    /// This type of node cannot have any children.
    ///
    /// [`Node`]: struct.Node.html
    /// [`Style`]: struct.Style.html
    /// [`Widget`]: widget/trait.Widget.html
    /// [`Size`]: struct.Size.html
    pub fn with_measure<F>(style: Style, measure: F) -> Node
    where
        F: 'static + Fn(Size<Number>) -> Size<f32>,
    {
        Node(node::Node::new_leaf(
            style.0,
            Box::new(move |size| Ok(measure(size))),
        ))
    }
}
