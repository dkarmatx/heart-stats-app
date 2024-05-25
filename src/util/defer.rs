pub struct Defer<F: FnOnce()> {
    callee: Option<F>
}

impl<F: FnOnce()> Defer<F> {
    pub fn new(callee: F) -> Self {
        Self { callee: Some(callee) }
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        self.callee.take().map(|f| { f() });
    }
}

macro_rules! defer {
    ( $($tt:tt)* ) => {
        let _deffered = $crate::util::defer::Defer::new(|| { $($tt)* });
    };
}
pub(crate) use defer;
