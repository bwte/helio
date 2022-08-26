//! Core.

/// Create a new trait for creating a new static instance.
pub trait New {
    /// Return the instance upon creating it.
    fn new() -> Self;
}

/// Create a new trait for creating a new static instance.
pub trait Instance: New + Clone {
    /// Return the instance upon call.
    fn instance() -> Self;
}

/// Creates a safe static instance of the provided type. Get it from the `instance()` method, it will return the same instance.
///
/// This is useful for creating a singleton and using it in multiple places.
///
/// # Returns
/// The instance of the object.
#[macro_export]
macro_rules! create_instance {
    ($obj:ident) => {
        impl Instance for $obj {
            fn instance() -> $obj {
                static mut INSTANCE: *mut $obj = 0 as *mut $obj;

                unsafe {
                    std::sync::Once::new().call_once(|| {
                        let instance = $obj::new();
                        INSTANCE = Box::into_raw(Box::new(instance));
                    });

                    (*INSTANCE).clone()
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Counter {
        counter: i32,
    }

    impl Counter {
        fn say_hello(&self) {
            println!("Hello World!");
        }

        fn add(&mut self, value: i32) {
            self.counter += value;
        }

        fn get(&self) -> i32 {
            self.counter
        }

        fn print(&self) {
            println!("{}", self.counter);
        }
    }

    impl New for Counter {
        fn new() -> Counter {
            Counter { counter: 0 }
        }
    }

    create_instance!(Counter);

    #[test]
    fn count() {
        let mut counter = Counter::instance();
        counter.add(1);
        counter.add(2);
        counter.add(3);
        counter.print();
        assert_eq!(counter.get(), 6);
    }

    #[test]
    fn say_hello() {
        let counter = Counter::instance();
        counter.say_hello();
    }
}
