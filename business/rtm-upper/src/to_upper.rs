pub trait ToUpper {
    fn as_uppercase(&self) -> String;
}

impl ToUpper for str {
    fn as_uppercase(&self) -> String {
        self.to_uppercase()
    }
}

impl ToUpper for String {
    fn as_uppercase(&self) -> String {
        self.to_uppercase()
    }
}
