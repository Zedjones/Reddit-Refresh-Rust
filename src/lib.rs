extern crate config;

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
                        let entry_str = format!("{} = {}\n", entry, val);
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
