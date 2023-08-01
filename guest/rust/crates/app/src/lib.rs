cargo_component_bindings::generate!();

use bindings::App;
// use bindings::gopher;
// use bindings::dino;

use bindings::rustacean;
use bindings::cat;

struct Component;

impl App for Component {
    fn run() -> String {
        let greeting = "App::run() says hi to some critters:";
        // rustacean::hello(&dino::hello(&cat::hello(greeting)))
        rustacean::hello(&cat::hello(greeting))

    }
}
