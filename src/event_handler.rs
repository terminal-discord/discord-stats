use serenity::{model::prelude::*, prelude::*};
use std::cell::RefCell;
use std::sync::Arc;

use crate::store::StatsStore;

pub struct Handler {
    store: StatsStore,
    user: Mutex<RefCell<Option<User>>>,
}

impl Handler {
    pub fn with_store(store: StatsStore) -> Handler {
        Handler {
            store,
            user: Mutex::new(RefCell::new(None)),
        }
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, m: Message) {
        if let Some(ref user) = *self.user.lock().borrow() {
            if user == &m.author {
                self.store.insert_msg(&m)
            }
        }
    }

    fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
        *self.user.lock().borrow_mut() = Some(ready.user.into())
    }
}
