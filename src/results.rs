#[derive(Debug)]
pub struct CountResult {
    pub url: String,
    pub query: String,
    pub count: u32,
    pub error: String,
}


#[derive(Debug)]
pub struct Result {
    pub count_results: Vec<CountResult>
}
