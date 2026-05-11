use {
    crate::expanders::{Expanded, Expander},
    crate::trash::TrashEntry,
    crate::{node::Node, policy::Policy, rules::Rule},
    anyhow::Result,
    async_stream::stream,
    futures_lite::Stream,
    std::path::Path,
    tokio::fs::read_dir,
};

pub async fn index_nodes(
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

pub fn scan_node(
    node: Node,
    rules: &'static [&'static dyn Rule],
) -> impl Stream<Item = TrashEntry> {
    stream! {
        let mut stack = vec![node];

        'dir: while let Some(node) = stack.pop() {
           let path = node.path;
           for rule in rules {
               if let Some(category) = rule.check(&path.as_path()).await {
                   yield TrashEntry(category, path.clone());
                   continue 'dir;
               }
           }
           if let Ok(mut dir) = read_dir(&path).await {
               'file: while let Some(entry) = dir.next_entry().await.unwrap_or(None) {
                   let path = entry.path();
                   for rule in rules {
                       if let Some(category) = rule.check(&path.as_path()).await {
                           yield TrashEntry(category, path.clone());
                           continue 'file;
                       }
                   }
                   if entry.file_type().await.map(|ft| ft.is_dir()).unwrap_or(false) {
                       stack.push(Node::new(path, Policy::Scan));
                   }
               }
           }
        }
    }
}
