use std::any::{Any, TypeId};

use listenbrainz::{raw::Client};
use musicbrainz_rs::{entity::{recording::Recording, release::Release, release_group::{ReleaseGroup, ReleaseGroupPrimaryType}}, Fetch};

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
    let top_listen_tracks = client.stats_user_recordings(&user, 1000, None, None)?.unwrap();

    for recording in top_listen_tracks.payload.recordings.into_iter() {
        if let Some(listen_id) = recording
        
        // We fetch the release group ID as it's the only way to get the user stats of it
        let release_group = ReleaseGroup::fetch().id(recording.recording_mbid);
        
        let listeners = client.stats_release_group_listeners(recording.);  
    }

    todo!()
}

/// Tries to find the single version of a recording
fn get_single_releases_of_recording(recording_id: String) -> Result<(), musicbrainz_rs::Error> {
    let recording = Recording::fetch().id(&recording_id).execute()?;
    let releases = recording.releases.unwrap(); 

    let mut best_releases: Vec<Release> = releases.iter().filter(|release| release.release_group.unwrap().primary_type.unwrap() == ReleaseGroupPrimaryType::Single).cloned().collect(); 

    if best_releases.len() == 0 {
        best_releases.push(
            releases
                .iter()
                .min_by_key(|release|{ 
                    release.media
                    .iter()
                    .map(|medias|{ 
                        medias
                            .iter()
                            .map(|media| {
                                media.track_count
                            })
                            .sum()
                    })
                    .sum()
                }).unwrap()
                .clone()
        )
    }

    recording.releases.unwrap().iter().map(|release| release.release_group).filter(|release| release)

    Ok(())
}