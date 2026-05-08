use anyhow::Result;
use std::path::Path;
use tokio::fs::read_dir;

use crate::expander::{Expanded, Expander};
use crate::{node::Node, policy::Policy, rule::Rule, trash_buffer::TrashBuffer};

pub async fn get_nodes(
    path: impl AsRef<Path>,
    expanders: &'static [&'static dyn Expander<Node>],
) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();

    if let Ok(mut dir) = read_dir(path).await {
        while let Some(entry) = dir.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let node = Node::new(entry.path(), Policy::Scan);
            let mut buffer = Vec::new();

            for expander in expanders {
                match expander.expand(&node) {
                    Expanded::Item(item) => buffer.push(item),
                    Expanded::Vec(items) => buffer.extend(items),
                    Expanded::None => (),
                };
            }

            if buffer.is_empty() {
                nodes.push(node);
            } else {
                nodes.extend(buffer);
            }
        }
    }

    Ok(nodes)
}

async fn scan_node(
    node: Node,
    buffer: &mut TrashBuffer,
    rules: &'static [&'static dyn Rule],
) -> Result<()> {
    let mut stack = vec![node];

    while let Some(node) = stack.pop() {
        tracing::debug!("scanning: {:?}", node);
        if let Ok(mut dir) = read_dir(&node.path).await {
            while let Some(entry) = dir.next_entry().await? {
                let path = entry.path();

                for rule in rules {
                    if let Some(category) = rule.check(path.as_path()).await {
                        buffer.push(category, path.clone());
                    }
                }

                if entry.file_type().await?.is_dir() {
                    stack.push(Node::new(path, Policy::Scan));
                }
            }
        }
    }

    Ok(())
}

pub async fn scan(
    path: impl AsRef<Path>,
    expanders: &'static [&'static dyn Expander<Node>],
    rules: &'static [&'static dyn Rule],
) -> Result<TrashBuffer> {
    let mut buffer = TrashBuffer::new();

    for node in get_nodes(path, expanders).await? {
        match node.policy {
            Policy::Collect(category) => {
                tracing::debug!("collected: {:?} to {:?}", node, category);
                buffer.push(category, node.path)
            }
            Policy::Scan => {
                tracing::debug!("scanning: {:?}", node);
                scan_node(node, &mut buffer, rules).await?
            }
        }
    }

    Ok(buffer)
}
