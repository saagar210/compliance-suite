use crate::domain::errors::CoreResult;
use crate::storage::EvidenceItem;

pub fn render_index_md(evidence: &[EvidenceItem]) -> CoreResult<String> {
    let mut out = String::new();
    out.push_str("# Export Index\n\n");
    out.push_str("## Evidence\n\n");

    // stable order
    let mut items: Vec<_> = evidence.iter().collect();
    items.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    for e in items {
        out.push_str(&format!(
            "- `{}` ({}, {} bytes)\n",
            e.relative_path, e.sha256, e.byte_size
        ));
    }

    Ok(out)
}
