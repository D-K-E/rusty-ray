//! wait group implementation
use std::sync::Condvar;
use std::sync::Mutex;

pub struct WaitGroup {
    cond: Condvar,
    group_size_mutex: Mutex<i16>,
}

impl WaitGroup {
    pub fn new() -> Self {
        Self {
            cond: Condvar::new(),
            group_size_mutex: Mutex::new(0),
        }
    }

    pub fn add_member(&self, delta: i16) {
        let mut group_size = self.group_size_mutex().lock().unwrap();
        let mg = *group_size + delta;
        *group_size = mg;
    }
    pub fn wait(&self) {
        self.cond()
            .wait_while(self.group_size_mutex().lock().unwrap(), |gsize| *gsize > 0);
    }

    pub fn done(&self) {
        let mut m = self.group_size_mutex().lock().unwrap();
        *m = (*m) - 1;
        if (*m) == 0 {
            self.cond().notify_all();
        }
    }

    fn cond(&self) -> &Condvar {
        &self.cond
    }

    fn group_size_mutex(&self) -> &Mutex<i16> {
        &self.group_size_mutex
    }
}
