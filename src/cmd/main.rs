use kaleidoscope;

fn main() {
    let tokens = kaleidoscope::lexer::tokenize("def foo(x, y) x + y; extern bar;");
    println!("{:?}", tokens);
}
