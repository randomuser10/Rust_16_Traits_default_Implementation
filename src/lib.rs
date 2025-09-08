pub trait Summary{
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub author: String,
    pub content: String,
    pub location: String,
}

impl Summary for NewsArticle{
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
        
    }


}