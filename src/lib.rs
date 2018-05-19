extern crate config;
extern crate reqwest;
extern crate indexmap;
extern crate serde_json;

pub mod reserializer{
    
    //import config constructs 
    use config::Config;
    use config::Source;

    /**
     * Serialize a TOML config consisting of a hierarchy of strings mapped to
     * either strings or arrays 
     * @param config - the TOML configuration to reserialize 
     * @return output - the reserialized string 
     * @preconditions - config must not contain high level mappings, i.e.
     * those outside of a category 
    */
    pub fn reserialize(config: Config) -> String{

        let mut output = String::new();

        //get each category and table of keys to values
        for (category, table) in config.collect().unwrap().iter(){
            //format each category like in a toml
            let category_str = format!("[{}]\n", category);
            output.push_str(&category_str);
            //get each key and value from the table 
            for (entry, value) in table.clone().into_table().unwrap(){
                match value.clone().into_str(){
                    //if we have a string, format it as such
                    Ok(val) => {
                        let entry_str = format!("{} = \"{}\"\n", entry, val);
                        output.push_str(&entry_str);
                    }
                    //otherwise, we have an array as the value
                    Err(_) => {
                        let mut value_vec = Vec::new();
                        //luckily, vectors' {:?} formatting is just what we need
                        for test in value.into_array().unwrap(){
                            value_vec.push(test.into_str().unwrap());
                        }
                        let entry_str = format!("{} = {:?}\n", entry, value_vec);
                        output.push_str(&entry_str);
                    }
                }
            }
            //add a newline before the next category 
            output.push_str("\n");
        }
        output
    }
}

pub mod subparser{

    //import necessary constructs for our module
    use indexmap::IndexMap;
    use reqwest::get;
    use serde_json::{Value, from_str};
    
    /**
     * Get the first result for a search on a subreddit, sortng by new
     * @param sub - the subreddit to search in
     * @param search - the search query
     * @return url_map - a map of the comments link to the post title 
    */
    pub fn get_results(mut sub: String, mut search:String) 
    -> Result<IndexMap<String, String>, String>{

        let mut url_map = IndexMap::new();

        //in case subreddit is missing r/
        if !sub.contains("r/"){
            sub = format!("r/{}", sub);
        }
        //prevent errors with link
        if search.contains(" "){
            search = search.replace(" ", "+");
        }

        let url = format!("https://www.reddit.com/{}/\
            search.json?q={}&sort=new&restrict_sr=on&limit=1", sub, search);

        //get the json text from the Reddit api
        let content = match get(&url).unwrap().text(){
            Ok(content) => content, 
            Err(_) => return Err("Error retrieving webpage".to_string())
        };

        //store this in a serde_json Value object 
        let json: Value = match from_str(&content){
            Ok(json) => json,
            Err(_) => return Err("Error decoding json object, \
            likely due to an invalid subreddit entered".to_string())
        };

        let results = json["data"]["children"].as_array().expect("Could not into array");

        //if there are no children, invalid sub
        if results.len() == 0{
            return Err("Invalid subreddit provided".to_string());
        }

        for result in results{
            //no direct way to get comments url, so we improvise
            let perma = result["data"]["permalink"].as_str().unwrap();
            let link = format!("https://www.reddit.com{}", perma);
            let title = result["data"]["title"].as_str().unwrap();
            url_map.insert(link.to_string(), title.to_string());
        }

        println!("{:#?}", url_map);
        Ok(url_map)
    }
}