use listenbrainz::raw::Client;
use listenbrainz::raw::response::{UserListensListen, UserListensResponse};
use derive_builder::Builder;

use crate::unlinked::get_all_unlinked_of_user;

//pub mod fetcher;
//pub mod underrated_tracks;
//pub mod mbid_searcher;
//pub mod database;
//pub mod underated_release_groups;
pub mod unlinked;

#[derive(Clone, Debug, Builder)]
//#[cfg_attr(feature = "builders", derive(Builder))]
pub struct ListenReader {
    #[builder(setter(into, strip_option))]
    user_name: String,
    
    #[builder(setter(into, strip_option), default)]
    min_ts: Option<i64>,

    #[builder(setter(into, strip_option), default)]
    max_ts: Option<i64>,

    #[builder(setter(into, strip_option), default = "Some(990)")]
    count: Option<u64>,

    #[builder(setter(into, strip_option), default)]
    time_range: Option<u64>
}

impl ListenReader {
    /// Update min_ts for the latest listen in the response
    fn update_max_ts(&mut self, responce: &UserListensResponse) {
        self.max_ts = responce.payload.listens.iter().min_by_key(|listen| listen.listened_at).map(|latest_listen| latest_listen.listened_at)
    }

    pub fn next(&mut self, client: &Client) -> Result<UserListensResponse, listenbrainz::Error> {
        let responce = client.user_listens(&self.user_name, self.min_ts, self.max_ts, self.count, self.time_range)?;
        self.update_max_ts(&responce);
        Ok(responce)
    }
}

pub struct ListenCollection {
    listens: Vec<UserListensListen>
}

impl ListenCollection {
    //pub fn get_listens_with_recording_id<'a>(&'a self, record_id: &str) -> impl Iterator<Item=&UserListensListen> + 'a {
    //    self.listens.iter().filter(|listen| listen.recording_msid == record_id)
    //}
}

fn main() {
    println!("Hello, world!");

    let client = Client::new();

    let res = get_all_unlinked_of_user("rustynova");


    let mut display_strings: Vec<String> = res.into_iter().map(|listen| {
        format!("{} - {} | https://listenbrainz.org/user/RustyNova/?max_ts={}", listen.track_metadata.track_name, listen.track_metadata.artist_name, listen.listened_at + 1)
    }).collect();

    display_strings.sort();

    println!("{:#?}", display_strings);
    println!("Total: {}", display_strings.len())
}
//378f8667-9206-49b7-8248-22fd56595370