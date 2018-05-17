extern crate config;

pub mod reserializer{
    
    use config::Config;
    use config::Source;

    pub fn reserialize(config: Config) {

        for (category, table) in config.collect().unwrap().iter(){
            for (entry, value) in table.clone().into_table().unwrap(){
                match value.clone().into_str(){
                    Ok(val) => println!("{}.{}: {}", category, entry, val),
                    Err(_) => {
                        for test in value.into_array().unwrap(){
                            println!("{}.{}: {}", category, entry, test)
                        }
                    }
                }
            }
        }
    }
}
