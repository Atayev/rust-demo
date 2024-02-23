pub struct RandomNumber {
    pub value: u8, // if dont add "pub" keyword value is private by default

                   // value2: u8, //private
} // all struct is private too you need to add "pub"
impl RandomNumber {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}
