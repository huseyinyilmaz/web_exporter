use crate::settings;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct QueryResult {
    pub query: String,
    pub count: Option<u32>,
}

#[derive(Debug)]
pub struct TargetResult<'a> {
    pub url: String,
    pub method: &'a settings::TargetMethod,
    pub status: u16,
    pub error: bool,
    pub size: usize,
    pub duration: u128,
    pub query_results: Vec<QueryResult>,
    pub extra_labels: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Result<'a> {
    pub target_results: Vec<TargetResult<'a>>,
}

impl fmt::Display for TargetResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut labels = self.extra_labels.clone();
        labels.insert("url".to_string(), self.url.to_string());
        labels.insert("status".to_string(), self.status.to_string());
        labels.insert("method".to_string(), self.method.to_string());
        labels.insert("error".to_string(), self.error.to_string());
        let base_labels = labels
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        let mut results = Vec::new();
        let duration_stat = format!(
            "web_exporter_response_duration_milliseconds{{{} }} {}",
            base_labels, self.duration,
        );
        let size_stat = format!(
            "web_exporter_response_response_size_bytes{{{} }} {}",
            base_labels, self.size,
        );
        results.push(duration_stat);
        results.push(size_stat);
        for query_result in &self.query_results {
            let query_stat = format!(
                "web_exporter_query_count{{{}, query=\"{}\" }} {}",
                base_labels,
                query_result.query,
                query_result.count.unwrap_or(0),
            );
            results.push(query_stat);
        }
        write!(f, "{}", results.join("\n"))
    }
}

impl fmt::Display for Result<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut results = Vec::new();
        for target_result in &self.target_results {
            results.push(format!("{}", target_result));
        }
        write!(f, "{}", results.join("\n"))
    }
}
