#[derive(Debug)]
pub struct QueryResult {
    pub url: String,
    pub query: String,
    pub count: u32,
    pub status: u16,
    pub error: String,
    pub completed: bool,
}


#[derive(Debug)]
pub struct Result {
    pub query_results: Vec<QueryResult>
}
