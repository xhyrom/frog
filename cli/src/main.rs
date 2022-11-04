mod handler;
mod parser;

fn main() {
    let result = parser::args::parse();
    handler::handle(result);
}
