extern crate config;
extern crate reqwest;
extern crate indexmap;
extern crate serde_json;

pub mod reserializer{
    
    use config::Config;
    use config::Source;

    pub fn reserialize(config: Config) -> String{

        let mut output = String::new();

        for (category, table) in config.collect().unwrap().iter(){
            let category_str = format!("[{}]\n", category);
            output.push_str(&category_str);
            for (entry, value) in table.clone().into_table().unwrap(){
                match value.clone().into_str(){
                    Ok(val) => {
                        let entry_str = format!("{} = \"{}\"\n", entry, val);
                        output.push_str(&entry_str);
                    }
                    Err(_) => {
                        let mut value_vec = Vec::new();
                        for test in value.into_array().unwrap(){
                            value_vec.push(test.into_str().unwrap());
                        }
                        let entry_str = format!("{} = {:?}\n", entry, value_vec);
                        output.push_str(&entry_str);
                    }
                }
            }
            output.push_str("\n");
        }
        output
    }
}

pub mod subparser{

    use indexmap::IndexMap;
    use reqwest::get;
    use serde_json::{Value, from_str};
    
    pub fn get_results(mut sub: String, mut search:String) 
    -> Result<IndexMap<String, String>, String>{

        let mut url_map = IndexMap::new();

        if !sub.contains("r/"){
            sub = format!("r/{}", sub);
        }
        if search.contains(" "){
            search = search.replace(" ", "+");
        }

        let url = format!("https://www.reddit.com/{}/\
            search.json?q={}&restrict_sr=on&limit=1", sub, search);

        let content = get(&url).unwrap().text().unwrap();
        let json: Value = from_str(&content).unwrap();
        let results = json["data"]["children"].as_array().expect("Could not into array");

        if results.len() == 0{
            return Err("Invalid subreddit provided".to_string());
        }

        for result in results{
            let link = result["data"]["url"].as_str().unwrap();
            let title = result["data"]["title"].as_str().unwrap();
            url_map.insert(link.to_string(), title.to_string());
        }

        println!("{:#?}", url_map);
        Ok(url_map)
    }

}

#[cfg(test)]
mod tests{

    use super::subparser::get_results;

    #[test]
    fn subparser_results(){
        get_results("mechanicalkeyboards".to_string(), "Planck".to_string()).unwrap();
        println!("dog");
    }
}
