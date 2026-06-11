pub struct OnDrop<F>
where
    F: FnOnce(),
{
    f: Option<F>,
}
impl<F> OnDrop<F>
where
    F: FnOnce(),
{
    pub fn new(f: F) -> Self {
        Self { f: Some(f) }
    }
}

impl<F> Drop for OnDrop<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            f();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    #[test]
    fn test_case_on_drop() {
        let value = Arc::new(Mutex::new(String::new()));
        let ctx = String::from("hello");
        let guard = OnDrop::new({
            let value = value.clone();
            move || {

                value.lock().unwrap().insert_str(0, &ctx);
                drop(ctx);// consume a ctx
            }
        });


        assert!(value.lock().unwrap().is_empty());

        drop(guard);
        assert_eq!(*value.lock().unwrap(), "hello");

    }
}
