use listenbrainz::raw::Client;

pub type TrackWithScore = (String, usize);

/// Function to find the underrated tracks that an user listens to, but no one else does 
/// We determine underated tracks with 3 criterias:
/// - The percent of total listens the user as contributed to the track. This give 100 points
/// - The place in the user's top 1000. Number 1 gives 100 points, number 1001 give 0.
/// - How many other users listened to it. No other users give 100 points, 10 users give 0.
/// The track vec will be sorted by decressing numbers of points
pub fn find_underated_tracks(user: String) -> Result<Vec<TrackWithScore>, listenbrainz::Error> {
    let track_scores = Vec::new();
    let client = Client::new();

    // We get the top 1000 tracks
    let top_listen_tracks = client.stats_user_releases(&user, 1000, None, None)?.unwrap();

    for recording in top_listen_tracks.payload.recordings.into_iter() {
        if let Some(listen_id) = recording
        
        // We fetch the release group ID as it's the only way to get the user stats of it
        let release_group = ReleaseGroup::fetch().id(recording.recording_mbid);
        
        let listeners = client.stats_release_group_listeners(recording.);  
    }

    todo!()
}