mod parser;
mod handler;

fn main() {
    let result = parser::args::parse();
    handler::handle(result);
}
