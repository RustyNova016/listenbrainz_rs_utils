use listenbrainz::raw::Client;

pub struct Recording {
    mbid: String
}

pub struct ListenReader {
    curent_page: usize,
    
}

fn main() {
    println!("Hello, world!");

    let client = Client::new();

    let res = client.user_listens("RustyNova", None, None, Some(5), None);

    println!("{:#?}", res);
}
