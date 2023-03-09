mod feed {

    pub mod MastodonFeed {
        pub struct MastodonFeed {
            url: String,
        }

        //Change to vector of posts
        impl MastodonFeed {
            pub fn new(url: String) -> MastodonFeed {
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

        mod post {
            pub struct MastodonPost {
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

            //TODO get this to do a request, fail whole program if cannot GET
            impl MastodonPost {
                pub fn new(url: String) -> Self {
                    //let self::user_name = "testusername";
                    //MastodonPost::display_name = "testname";
                    //MastodonPost::locked_state = False;
                    Self::populatePostData(url);
                    Self {};
                  //  return Self;
                }

                    //Parse the JSON
                    fn populatePostData(&String: url) {
                        println! {"In populatePostData!"};
                    }
                }
            }
        }
    }
