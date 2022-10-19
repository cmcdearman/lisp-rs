pub fn start_repl(env: Option<Env>) {
    let env_rc = Rc::new(RefCell::new(env.unwrap_or_else(default_env)));

    print!("> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        match interpreter::eval_block(
            env_rc.clone(),
            parser::parse(&line.unwrap()).filter_map(|a| a.ok()),
        ) {
            Ok(val) => println!("{}", val),
            Err(e) => println!("{}", e),
        };

        print!("> ");
        io::stdout().flush().unwrap();
    }

    // Properly go to the next line after quitting
    println!();
}