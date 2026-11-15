use serde::{Deserialize, Serialize};

/// Source of diagnostic evidence. Used to keep copy honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceSource {
    /// OS/backend attributed a veto to a process (e.g. Windows CM veto, Event 225).
    OsAttributedVeto,
    /// App verified a process has a file/resource open (e.g. Restart Manager, lsof, /proc).
    VerifiedOpenFile,
    /// Observed but not fully attributed; use cautious copy.
    Observed,
}

/// A single piece of blocker evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockerEvidence {
    pub source: EvidenceSource,
    pub process_name: Option<String>,
    pub pid: Option<u32>,
    /// Process start identity where available (for PID reuse protection).
    pub process_start_id: Option<String>,
    /// Full path only in expanded/redacted-export contexts as appropriate.
    pub file_path: Option<String>,
    pub observed_at_unix_ms: i64,
    pub detail: Option<String>,
}

/// Collected diagnostic evidence for a busy result.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub items: Vec<BlockerEvidence>,
    /// True if diagnostics were bounded/skipped due to permissions, time, or scope.
    pub incomplete: bool,
    pub note: Option<String>,
}

impl DiagnosticReport {
    pub fn has_attribution(&self) -> bool {
        self.items
            .iter()
            .any(|e| e.source == EvidenceSource::OsAttributedVeto)
    }

    pub fn has_verified_open_file(&self) -> bool {
        self.items
            .iter()
            .any(|e| e.source == EvidenceSource::VerifiedOpenFile)
    }
}

/// Redacted export form (no raw paths/process details unless explicitly included).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactedDiagnosticExport {
    pub summary: String,
    pub evidence_count: usize,
    pub incomplete: bool,
    pub sources: Vec<EvidenceSource>,
}

impl From<&DiagnosticReport> for RedactedDiagnosticExport {
    fn from(report: &DiagnosticReport) -> Self {
        Self {
            summary: if report.items.is_empty() {
                "No specific blocker identified".to_string()
            } else {
                format!("{} blocker evidence item(s)", report.items.len())
            },
            evidence_count: report.items.len(),
            incomplete: report.incomplete,
            sources: report.items.iter().map(|e| e.source).collect(),
        }
    }
}