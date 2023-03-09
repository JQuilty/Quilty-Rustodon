mod feed;
mod mastodonnetworkconnection;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);

    //Ensure that there are arguments, otherwise exit.
    if args.len() <= 1 {
        println!("Error: You must specify a URL.");
        std::process::exit(-1);
    }

    //TODO: Check if a valid URL

    let url: &String = &args[1];
    println! {"URL is: {}", url};
    let string = String::from("test");
    let mut feed = feed::MastodonFeed::MastodonFeed::new(string);
}
