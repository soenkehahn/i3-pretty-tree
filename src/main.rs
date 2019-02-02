use i3ipc::reply::{Node, NodeType, Workspaces};
use i3ipc::I3Connection;
use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

type R<A> = Result<A, Box<Error>>;

fn main() -> R<()> {
    let mut connection = I3Connection::connect()?;
    let root = connection.get_tree()?;
    let workspace = find_active_workspace(&connection.get_workspaces()?, &root)?;
    print!(
        "{}",
        tree_to_dot(workspace, |node| limit_text_size(18, format_node(node)))
    );
    Ok(())
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

fn tree_to_dot(root: &Node, node_to_label: fn(node: &Node) -> String) -> String {
    let mut lines = vec!["digraph g {".to_string()];
    for current in root.iter() {
        lines.push(format!(
            "n{} [ label = \"{}\" ];",
            current.id,
            node_to_label(current)
        ));
        for child in current.nodes.iter() {
            lines.push(format!("n{} -> n{:?};", current.id, child.id));
        }
    }
    lines.push("}".to_string());
    format!("{}\n", lines.join("\n"))
}

fn escape_special_chars(text: String) -> String {
    text.chars()
        .map(|char| match char {
            '\"' => r#"\""#.to_string(),
            _ => char.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn format_node(node: &Node) -> String {
    let name = escape_special_chars(node.name.clone().unwrap_or_else(|| "<None>".to_string()));
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

fn limit_text_size(limit: usize, text: String) -> String {
    let text = text.graphemes(true).collect::<Vec<&str>>();
    if limit <= 3 {
        text[..limit].join("")
    } else if text.len() <= limit {
        text.join("")
    } else {
        let snippet_length: f32 = (limit - 3) as f32 / 2.0;
        let prefix_end = snippet_length.ceil() as usize;
        let suffix_start = text.len() - snippet_length as usize;
        format!(
            "{}...{}",
            text[..prefix_end].join(""),
            text[suffix_start..].join("")
        )
    }
}

#[cfg(test)]
mod test {
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

    mod tree_to_dot {
        use super::*;

        #[test]
        fn translates_trees_with_one_element() {
            let mut root = node();
            root.name = Some("root".to_string());
            let dot = tree_to_dot(&root, |node| {
                format!("label: {}", node.name.clone().unwrap())
            });
            assert_eq!(dot, "digraph g {\nn1 [ label = \"label: root\" ];\n}\n");
        }

        fn tree() -> Node {
            let mut grandchild = node();
            grandchild.id = 3;
            grandchild.name = Some("grandchild".to_string());
            let mut child = node();
            child.id = 2;
            child.name = Some("child".to_string());
            child.nodes = vec![grandchild];
            let mut root = node();
            root.id = 1;
            root.name = Some("root".to_string());
            root.nodes = vec![child];
            root
        }

        #[test]
        fn translates_child_nodes() {
            let root = tree();
            let dot = tree_to_dot(&root, |node| {
                format!("label: {}", node.name.clone().unwrap())
            });
            assert!(
                dot.lines()
                    .any(|line| line == "n2 [ label = \"label: child\" ];"),
                dot
            );
        }

        #[test]
        fn translates_edges() {
            let root = tree();
            let dot = tree_to_dot(&root, |node| {
                format!("label: {}", node.name.clone().unwrap())
            });
            assert!(dot.lines().any(|line| line == "n1 -> n2;"), dot);
        }

        #[test]
        fn translates_nested_edges() {
            let root = tree();
            let dot = tree_to_dot(&root, |node| {
                format!("label: {}", node.name.clone().unwrap())
            });
            assert!(dot.lines().any(|line| line == "n2 -> n3;"), dot);
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

        #[test]
        fn escapes_quotes() {
            let mut node = node();
            node.name = Some(format!("\"foo\""));
            assert_eq!(format_node(&node), r#"\"foo\""#)
        }
    }

    mod limit_text_size {
        use super::*;

        #[test]
        fn limits_the_size_to_the_given_length() {
            assert_eq!(limit_text_size(5, "123456".to_string()).len(), 5);
        }

        #[test]
        fn inserts_dots_in_the_middle() {
            assert_eq!(limit_text_size(7, "1234567890".to_string()), "12...90");
        }

        #[test]
        fn works_for_even_limits() {
            assert_eq!(limit_text_size(8, "1234567890".to_string()), "123...90");
        }

        #[test]
        fn does_not_modify_short_enough_strings() {
            assert_eq!(limit_text_size(8, "12345678".to_string()), "12345678");
            assert_eq!(limit_text_size(8, "1234567".to_string()), "1234567");
        }

        #[test]
        fn when_limit_too_small_returns_prefix() {
            assert_eq!(limit_text_size(4, "12345678".to_string()), "1...");
            assert_eq!(limit_text_size(3, "12345678".to_string()), "123");
            assert_eq!(limit_text_size(2, "12345678".to_string()), "12");
        }

        #[test]
        fn handles_unicode_characters_gracefully() {
            let unicode_string = "—3456";
            limit_text_size(5, unicode_string.to_string());
        }
    }
}
