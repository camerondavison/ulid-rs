use ulid::Generator;
use std::thread;
use ulid::Ulid;
use std::sync::Arc;
use std::sync::Mutex;

struct AppM {
    generator: Generator,
}

impl AppM {
    fn new() -> AppM {
        AppM{
            generator: Generator::new(),
        }
    }

    fn gen(&mut self) -> Ulid {
        self.generator.generate().unwrap()
    }
}

struct App {
}

impl App {
    fn new() -> App {
        App{}
    }

    fn gen(&mut self) -> Ulid {
        ulid::Ulid::new()
    }
}

fn main() {
    //let app = Arc::new(Mutex::new(App::new())); // works
    let app = Arc::new(Mutex::new(AppM::new())); // how do I get this to work?

    let a = {
        let app = app.clone();
        thread::spawn(move || {
            let mut app = app.lock().unwrap();
            println!("{}", app.gen())
        })
    };

    let b = {
        let app = app.clone();
        thread::spawn(move || {
            let mut app = app.lock().unwrap();
            println!("{}", app.gen())
        })
    };

    a.join();
    b.join();
}
