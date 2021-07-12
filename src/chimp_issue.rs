pub trait IssueTracker {
    fn search_issues(&self) -> String;
}