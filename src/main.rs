use listenbrainz::raw::Client;
use listenbrainz::raw::response::{UserListensListen, UserListensResponse};
use derive_builder::Builder;

pub struct Recording {
    mbid: String
}
#[derive(Clone, Debug, Builder)]
//#[cfg_attr(feature = "builders", derive(Builder))]
pub struct ListenReader {
    #[builder(setter(into, strip_option))]
    user_name: String,
    
    #[builder(setter(into, strip_option))]
    min_ts: Option<i64>,

    #[builder(setter(into, strip_option))]
    max_ts: Option<i64>,

    #[builder(setter(into, strip_option))]
    count: Option<u64>,

    #[builder(setter(into, strip_option))]
    time_range: Option<u64>
}

impl ListenReader {
    /// Update min_ts for the latest listen in the response
    fn update_min_ts(&mut self, responce: &UserListensResponse) {
        self.min_ts = responce.payload.listens.iter().max_by_key(|listen| listen.listened_at).map(|latest_listen| latest_listen.listened_at)
    }

    pub fn next(&mut self, client: Client) -> Result<UserListensResponse, listenbrainz::Error> {
        let responce = client.user_listens(&self.user_name, self.min_ts, self.max_ts, self.count, self.time_range)?;
        self.update_min_ts(&responce);
        Ok(responce)
    }
}

pub struct ListenCollection {
    listens: Vec<UserListensListen>
}

impl ListenCollection {
    pub fn get_listens_with_recording_id<'a>(&'a self, record_id: &str) -> impl Iterator<Item=&UserListensListen> + 'a {
        self.listens.iter().filter(|listen| listen.recording_msid == record_id)
    } 
}

fn main() {
    println!("Hello, world!");

    let client = Client::new();

    let res = client.user_listens("RustyNova", None, None, Some(5), None);

    println!("{:#?}", res);
}
