use bindings::App;
use bindings::gopher;
use bindings::rustacean;

struct Component;

impl App for Component {
    fn run() -> String {
        let greeting = "App::run() says hi to some critters:";
        rustacean::hello(&gopher::hello(greeting))
    }
}

bindings::export!(Component);
