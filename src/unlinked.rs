use std::fs::read;

use crate::ListenReaderBuilder;

pub fn get_all_unlinked_of_user(username: String) -> Vec<UserListensListen> {
    let builder = ListenReaderBuilder::default();
    builder.user_name(username);
    let reader = builder.build().unwrap();

    let unlinked = vec![];
    loop {
        let page = reader.next(client).unwrap();

        for listen in page.payload.listens.into_iter() {
            if listen.
        }
    }
}