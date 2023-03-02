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
    let mut feed = MastodonFeed::new(string);
}

struct MastodonFeed {
    url: String,
}

//Change to vector of posts
impl MastodonFeed {
    fn new(url: String) -> MastodonFeed {
        MastodonFeed {
            url: url, //postvector: Vec<MastodonPost> = Vec::new();
        }
    }

    fn run(&mut self) {
        return;
    }
}

/*
   Basic post. All of these are attributes of a Mastodon post when a GET request is placed. It'll come in as a JSON,
   it will get parsedout and put in this struct to be displayed.

   Done via api/v1/accounts/:id/statuses
*/
struct MastodonPost {
    user_name: String,
    display_name: String,
    locked_state: bool,
    date: String,
    sensitivity: bool,
    post_id: u64,
    uri: String,
    url: String,
    replies_count: u32,
    reblogs_count: u32,
    post_content: String,
}

impl MastodonPost {
    fn new(url: String) -> Self {
        let self::user_name = "testusername";
        display_name = "testname";
        locked_state = False;


    //Parse the JSON
    fn populatePostData(&String: url) {
        println! {"In populatePostData!"};
    }
}