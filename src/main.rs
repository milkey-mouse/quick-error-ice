use quick_error::quick_error;

quick_error! {
    pub enum AReferenceInQuickError {
        CrashesRustc(s: &str) {
            display("hello, {}!", s)
        }
    }
}

fn main() {
    let world = "world";
    
    AReferenceInQuickError::CrashesRustc(world);
}
