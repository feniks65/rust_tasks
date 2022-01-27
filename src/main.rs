use std::boxed::Box;

type BoxedCallback = Box<dyn Fn(i32)>;

struct CallbackManager {
    callbacks: Vec<BoxedCallback>,
}

impl CallbackManager {
    fn add<C>(&mut self, callback: C)
    where
        C: Fn(i32) + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    fn run_all(&mut self, arg: i32) {
        for (pos, callback) in self.callbacks.iter().enumerate() {
            println!("position {}", pos);
            callback(arg);
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut callback_manager = CallbackManager { callbacks: vec![] };
    let expensive_closure = |num| {
        let _ = num + 1;
        println!("Expensive callback closure called with arg {}", num);
        return ();
    };
    callback_manager.add(expensive_closure);
    callback_manager.run_all(3);
    println!("Bye, world!");
}
