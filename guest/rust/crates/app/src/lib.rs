use bindings::App;
use bindings::gopher;
use bindings::rustacean;
use bindings::dino;

struct Component;

impl App for Component {
    fn run() -> String {
        let greeting = "App::run() says hi to some critters:";
        dino::hello(&rustacean::hello(&gopher::hello(greeting)))
    }
}

bindings::export!(Component);
