pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }   
    }
    
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning, you used up 90%");
        } else if percentage_of_max >= 0.7 {
            self.messenger.send("Warning, you used up 70%");
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Here, if we don't use RefCell, then we have borrowed an immutable Messenger
            // and thus can't update the internal value.
            // Another option is to change the argument &self to &mut self. However, this 
            // doesn't match Messenger trait's send() definition. 
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    // There is a counter for each RefCell(). When borrow_mut() is called, the RefMut<T>
    // counter increase by 1. Similar, when borrow() is called, the Ref<T> counter will
    // increase by 1. The counter will decrease by 1 during exiting the scope. At the 
    // same time, the runtime will only allow at most 1 RefMut<T> exists, or several 
    // RefMut<T> exist.
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
