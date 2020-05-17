#[derive(Debug)]
pub struct QueryResult {
    pub url: String,
    pub query: String,
    pub count: u32,
    pub status: u16,
    pub error: bool,
}


#[derive(Debug)]
pub struct Result {
    pub query_results: Vec<QueryResult>
}


impl Result {
    pub fn to_string(&self) -> String {
        let mut results = Vec::new();
        for result in &self.query_results {
            let str1 = format!(
                "web_exporter_query{{url=\"{}\", query=\"{}\", status={}, error={}}} {}",
                result.url,
                result.query,
                result.status,
                if result.error { 1 } else { 0 },
                result.count,
            );
            results.push(String::from(str1));
        }
        return results.join("\n");
    }
}
