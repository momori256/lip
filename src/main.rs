use std::io;

use lip::repl;

fn main() -> io::Result<()> {
    repl::run(&mut io::stdin().lock(), &mut io::stdout())
}
