use i3ipc::reply::{Node, NodeType};
use i3ipc::I3Connection;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut connection = I3Connection::connect()?;
    print!("{}", connection.get_tree()?.custom_pretty(format_node));
    Ok(())
}

fn format_node(node: &Node) -> String {
    let name = node.name.clone().unwrap_or_else(|| "<None>".to_string());
    match node {
        Node {
            nodetype: NodeType::Workspace,
            ..
        } => format!("WORKSPACE: {}", name),
        Node {
            nodetype: NodeType::Output,
            ..
        } => format!("OUTPUT: {}", name),
        Node {
            name: None,
            nodetype: NodeType::Con,
            ..
        } => format!("CONTAINER: {:?}", node.layout),
        _ => name,
    }
}

#[cfg(test)]
mod format_node {
    use super::*;

    use i3ipc::reply::{NodeBorder, NodeLayout};

    fn node() -> Node {
        Node {
            focus: vec![],
            nodes: vec![],
            floating_nodes: vec![],
            id: 1,
            name: None,
            nodetype: NodeType::Con,
            border: NodeBorder::Normal,
            current_border_width: 4,
            layout: NodeLayout::SplitH,
            percent: None,
            rect: (0, 0, 100, 100),
            window_rect: (0, 0, 0, 0),
            deco_rect: (0, 0, 0, 0),
            geometry: (0, 0, 0, 0),
            window: None,
            urgent: false,
            focused: false,
        }
    }

    #[test]
    fn uses_the_name() {
        let mut node = node();
        node.name = Some(format!("foo"));
        assert_eq!(format_node(&node), "foo")
    }

    #[test]
    fn marks_workspaces_as_such() {
        let mut node = node();
        node.name = Some(format!("foo"));
        node.nodetype = NodeType::Workspace;
        assert_eq!(format_node(&node), "WORKSPACE: foo")
    }

    #[test]
    fn marks_outputs_as_such() {
        let mut node = node();
        node.name = Some(format!("foo"));
        node.nodetype = NodeType::Output;
        assert_eq!(format_node(&node), "OUTPUT: foo")
    }

    #[test]
    fn shows_layout_for_unnamed_containers() {
        let mut node = node();
        node.nodetype = NodeType::Con;
        node.layout = NodeLayout::SplitH;
        assert_eq!(format_node(&node), "CONTAINER: SplitH")
    }
}
