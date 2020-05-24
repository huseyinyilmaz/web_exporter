use std::fmt;

#[derive(Debug)]
pub struct QueryResult {
    pub query: String,
    pub count: Option<u32>,
}

#[derive(Debug)]
pub struct TargetResult {
    pub url: String,
    pub status: u16,
    pub error: bool,
    pub size: usize,
    pub duration: u128,
    pub query_results: Vec<QueryResult>,
}

#[derive(Debug)]
pub struct Result {
    pub target_results: Vec<TargetResult>,
}

impl fmt::Display for TargetResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut results = Vec::new();
        let duration_stat = format!(
            "web_exporter_response_duration_milliseconds{{url=\"{}\", status={}, error={}}} {}",
            self.url,
            self.status,
            if self.error { 1 } else { 0 },
            self.duration,
        );
        let size_stat = format!(
            "web_exporter_response_response_size_bytes{{url=\"{}\", status={}, error={}}} {}",
            self.url,
            self.status,
            if self.error { 1 } else { 0 },
            self.size,
        );
        results.push(duration_stat);
        results.push(size_stat);
        for query_result in &self.query_results {
            let query_stat = format!(
                "web_exporter_query_count{{url=\"{}\", query=\"{}\", status={}, error={}}} {}",
                self.url,
                query_result.query,
                self.status,
                if self.error { 1 } else { 0 },
                // If we cannot parse selector just return -1
                query_result.count.unwrap_or(0),
            );
            results.push(query_stat);
        }
        write!(f, "{}", results.join("\n"))
    }
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut results = Vec::new();
        for target_result in &self.target_results {
            results.push(format!("{}", target_result));
        }
        write!(f, "{}", results.join("\n"))
    }
}
