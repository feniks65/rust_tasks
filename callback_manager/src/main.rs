use std::rc::{Rc, Weak};

type WeakCallback = Weak<dyn Fn(i32)>;

struct CallbackManager {
    callbacks: Vec<WeakCallback>,
}

impl CallbackManager {
    fn add<C>(&mut self, callback: C) -> Rc<dyn Fn(i32)>
    where
        C: Fn(i32) + 'static,
    {
        let shared_ref_callback = Rc::new(callback);
        let return_handle = shared_ref_callback.clone();
        let weak_ref_callback = Rc::downgrade(&shared_ref_callback);
        self.callbacks.push(weak_ref_callback);
        return return_handle;
    }

    fn run_all(&mut self, arg: i32) {
        // clear all hanging callbacks
        let mut callbacks_for_removal = vec![];
        for (pos, callback) in self.callbacks.iter().enumerate() {
            let rc_callback = callback.upgrade();
            if rc_callback.is_some() == false {
                callbacks_for_removal.push(pos);
            }
        }

        for index in callbacks_for_removal {
            self.callbacks.remove(index);
        }

        // fire callbacks
        for (pos, callback) in self.callbacks.iter().enumerate() {
            println!("position {}", pos);
            let rc_callback = callback.upgrade().unwrap();
            rc_callback(arg);
        }
    }
}

fn main() {
    println!("Hello");
    let mut callback_manager = CallbackManager { callbacks: vec![] };
    let expensive_closure = |num| {
        let _ = num + 1;
        println!("Expensive callback closure called with arg {}", num);
        return ();
    };
    /*let _closure_handle = */
    callback_manager.add(expensive_closure);
    callback_manager.run_all(3);
    println!("Bye");
}
