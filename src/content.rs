pub struct WebContent {
    pub h1_content: Vec<String>,
    pub a_content: Vec<String>
}

impl WebContent {
    pub fn new() -> Self {
        WebContent {
            h1_content: Vec::new(),
            a_content: Vec::new()
        }
    }
}


impl From<(Vec<String>, Vec<String>)> for WebContent {
    //first is h, second is a
    fn from(value: (Vec<String>, Vec<String>)) -> Self {
        WebContent {
            h1_content: value.0,
            a_content: value.1
        }
    }
}