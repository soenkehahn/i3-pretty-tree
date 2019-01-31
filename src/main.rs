use i3ipc::reply::{Node, NodeType, Workspaces};
use i3ipc::I3Connection;
use std::error::Error;

type R<A> = Result<A, Box<Error>>;

fn main() -> R<()> {
    let mut connection = I3Connection::connect()?;
    let root = connection.get_tree()?;
    let workspace = find_active_workspace(&connection.get_workspaces()?, &root)?;
    print!("{}", workspace.custom_pretty(format_node));
    Ok(())
}

fn format_node(node: &Node) -> String {
    let name = node.name.clone().unwrap_or_else(|| "<None>".to_string());
    match node {
        Node {
            nodetype: NodeType::Root,
            ..
        } => "ROOT".to_string(),
        Node {
            nodetype: NodeType::Output,
            ..
        } => format!("OUTPUT: {}", name),
        Node {
            nodetype: NodeType::Workspace { .. },
            ..
        } => format!("WORKSPACE: {}", name),
        Node {
            name: None,
            nodetype: NodeType::Con,
            ..
        } => format!("CONTAINER: {:?}", node.layout),
        _ => name,
    }
}

fn find_active_workspace<'a>(workspaces: &Workspaces, root: &'a Node) -> R<&'a Node> {
    let active_workspace = workspaces
        .workspaces
        .iter()
        .find(|workspace| workspace.focused)
        .ok_or("no focused workspace found")?;
    let active_workspace_node = root
        .iter()
        .find(|node| match node.nodetype {
            NodeType::Workspace { num } => num == active_workspace.num,
            _ => false,
        })
        .ok_or_else(|| format!("workspace with num {} not found", active_workspace.num))?;
    Ok(active_workspace_node)
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

    mod format_node {
        use super::*;

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
            node.nodetype = NodeType::Workspace { num: 0 };
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
        fn marks_root_as_such() {
            let mut node = node();
            node.name = Some(format!("root"));
            node.nodetype = NodeType::Root;
            assert_eq!(format_node(&node), "ROOT")
        }

        #[test]
        fn shows_layout_for_unnamed_containers() {
            let mut node = node();
            node.nodetype = NodeType::Con;
            node.layout = NodeLayout::SplitH;
            assert_eq!(format_node(&node), "CONTAINER: SplitH")
        }
    }

    mod find_active_workspace {
        use super::*;
        use i3ipc::reply::Workspace;

        fn workspace() -> Workspace {
            Workspace {
                num: 1,
                name: "workspace name".to_string(),
                visible: true,
                focused: false,
                urgent: false,
                rect: (0, 0, 10, 10),
                output: "some output".to_string(),
            }
        }

        #[test]
        fn returns_the_active_workspace() {
            let workspaces = {
                let mut active: Workspace = workspace();
                active.num = 1;
                active.name = "active workspace".to_string();
                active.focused = true;
                let mut inactive: Workspace = workspace();
                inactive.num = 2;
                inactive.name = "inactive workspace".to_string();
                Workspaces {
                    workspaces: vec![active, inactive],
                }
            };
            let root = {
                let mut active: Node = node();
                active.nodetype = NodeType::Workspace { num: 1 };
                active.name = Some("active workspace".to_string());
                let mut inactive: Node = node();
                inactive.nodetype = NodeType::Workspace { num: 2 };
                inactive.name = Some("inactive workspace".to_string());
                let mut root: Node = node();
                root.nodes = vec![active, inactive];
                root.nodetype = NodeType::Root;
                root
            };
            let active_workspace: &Node = find_active_workspace(&workspaces, &root).unwrap();
            assert_eq!(active_workspace.name, Some("active workspace".to_string()));
        }
    }
}
