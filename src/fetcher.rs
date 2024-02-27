use listenbrainz::raw::response::UserListenCountPayload;
use musicbrainz_rs::{entity::{artist::{Artist, ArtistSearchQuery, ArtistSearchQueryLuceneQueryBuilder}, recording::Recording, search}, Fetch};


pub enum GroupByType {
    Album,
}

pub struct QueryParameters {
    filter_artists: Vec<Artist>,
    filter_users: Vec<String>,
    group_by: GroupByType
}

impl QueryParameters {
    pub fn search() {
        //let listen_count_of_album = 
    }
}