pub struct Channel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub avatar_url: String,
    pub keywords: Vec<String>,
}

impl Channel {
    pub(crate) fn new(
        id: &str,
        name: &str,
        description: &str,
        url: &str,
        avatar_url: &str,
        keywords: Vec<&str>,
    ) -> Self {
        Channel {
            id: String::from(id),
            name: String::from(name),
            description: String::from(description),
            url: String::from(url),
            avatar_url: String::from(avatar_url),
            keywords: keywords.iter().map(|&s| String::from(s)).collect(),
        }
    }
}
