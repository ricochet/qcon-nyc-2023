use bindings::{
    exports::component::rustacean::greeting::Greeting,
};

struct Component;

impl Greeting for Component {
    fn hello(greeting: String) -> String {
        greeting + "\n<🦀>Hello"
    }
}

bindings::export!(Component);
