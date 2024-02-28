use listenbrainz::raw::Client;

/// Holds a MBID, and has some useful function to get data from MusicBrainz and listenbrainz
/// THe information is also cached, meaning that some duplicate calls are cancelled
pub struct RecordingMBID{
    MBID: String,
    total_listen_count: Option<usize>
}
