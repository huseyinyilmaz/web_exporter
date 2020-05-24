use std::fmt;

#[derive(Debug)]
pub struct QueryResult {
    pub url: String,
    pub query: String,
    pub count: u32,
    pub status: u16,
    pub error: bool,
    pub duration: u128,
    pub size: usize,
}

#[derive(Debug)]
pub struct Result {
    pub query_results: Vec<QueryResult>,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut results = Vec::new();
        for result in &self.query_results {
            let str1 = format!(
                "web_exporter_query_count{{url=\"{}\", query=\"{}\", status={}, error={}}} {}",
                result.url,
                result.query,
                result.status,
                if result.error { 1 } else { 0 },
                result.count,
            );
            let str2 = format!(
                "web_exporter_query_duration_milliseconds{{url=\"{}\", query=\"{}\", status={}, error={}}} {}",
                result.url,
                result.query,
                result.status,
                if result.error { 1 } else { 0 },
                result.duration,
            );
            let str3 = format!(
                "web_exporter_query_response_size_bytes{{url=\"{}\", query=\"{}\", status={}, error={}}} {}",
                result.url,
                result.query,
                result.status,
                if result.error { 1 } else { 0 },
                result.size,
            );
            results.push(str1);
            results.push(str2);
            results.push(str3);
        }
        write!(f, "{}", results.join("\n"))
    }
}
