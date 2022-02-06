
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where
    T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 /self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over you quota.");
        } else if percentage_of_max >= 0.9{
            self.messenger.send("Urgent warning: you've used up over 90% of you quta");

        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: you've used up over 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 可变  sent_messages
            self.sent_message.borrow_mut().push(String::from(message));

            // let mut one_borrow = self.sent_message.borrow_mut();
            // let mut two_borrow = self.sent_message.borrow_mut();

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(100);
        // 不可变 sent_messages
        assert_eq!(mock_messenger.sent_message.borrow().len(), 1);
    }
}