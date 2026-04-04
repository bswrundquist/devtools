use console::style;

pub fn section_header(name: &str) {
    eprintln!("\n{}", style(format!("── {} ──", name)).bold().cyan());
}

pub fn success(msg: &str) {
    eprintln!("  {} {}", style("✓").green(), msg);
}

pub fn skipped(msg: &str) {
    eprintln!("  {} {}", style("·").dim(), msg);
}

pub fn conflict(msg: &str) {
    eprintln!("  {} {}", style("✗").yellow(), msg);
}

pub fn error(msg: &str) {
    eprintln!("  {} {}", style("✗").red(), msg);
}

pub fn info(msg: &str) {
    eprintln!("  {}", style(msg).dim());
}

pub fn summary(report: &crate::sections::SectionReport) {
    let total = report.succeeded.len()
        + report.skipped.len()
        + report.conflicts.len()
        + report.failed.len();
    eprintln!(
        "  {} total: {} ok, {} unchanged, {} conflicts, {} failed",
        style(total).bold(),
        style(report.succeeded.len()).green(),
        style(report.skipped.len()).dim(),
        style(report.conflicts.len()).yellow(),
        style(report.failed.len()).red(),
    );
}
