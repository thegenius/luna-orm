#[derive(Debug)]
pub struct NotImplementError(pub String);
impl std::error::Error for NotImplementError {}
impl std::fmt::Display for NotImplementError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "method {} is not implements", self.0)
    }
}