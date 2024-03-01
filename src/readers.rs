use listenbrainz::raw::{response::UserListensResponse, Client};

#[cfg(feature = "builders")]
use derive_builder::Builder;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
#[cfg_attr(feature = "builders", derive(Builder))]
/// Reader for the the User Listens endpoint
pub struct ListenReader {
    #[cfg_attr(feature = "builders", builder(setter(into, strip_option)))]
    /// The name of the target user
    user_name: String,

    #[cfg_attr(feature = "builders", builder(setter(into, strip_option)))]
    /// The UNIX timestamp of the earliest listen to retreive
    min_ts: Option<i64>,

    #[cfg_attr(feature = "builders", builder(setter(into, strip_option)))]
    /// The UNIX timestamp of the latest listen to retreive
    max_ts: Option<i64>,

    #[cfg_attr(
        feature = "builders",
        builder(setter(into, strip_option), default = "Some(999)")
    )]
    /// The number of listen to retreive from the API.
    count: Option<u64>,

    #[cfg_attr(feature = "builders", builder(setter(into, strip_option)))]
    time_range: Option<u64>,
}

impl ListenReader {
    /// Update [`Self::max_ts`] for the latest listen in the response
    fn update_max_ts(&mut self, responce: &UserListensResponse) {
        self.max_ts = responce
            .payload
            .listens
            .iter()
            .min_by_key(|listen| listen.listened_at)
            .map(|latest_listen| latest_listen.listened_at)
    }

    /// Retreive the next page of listens
    pub fn next(&mut self, client: &Client) -> Result<UserListensResponse, listenbrainz::Error> {
        let response =
            client.user_listens(&self.user_name, self.min_ts, self.max_ts, self.count)?;
        self.update_max_ts(&response);
        Ok(response)
    }
}
