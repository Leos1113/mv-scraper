#[derive(Debug)]
pub struct Forum {
    pub title: String,
    pub link: String,
    pub description: String,
}

impl Forum {
    pub fn new(title: String, link: String, description: String) -> Forum {
        Forum {
            title,
            link,
            description,
        }
    }
}
