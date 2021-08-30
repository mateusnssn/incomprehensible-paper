use std::env;
use rand::Rng;

// Utils
fn range(qnt:u32) -> u32 {
    rand::thread_rng().gen_range(std::ops::Range{start: 0, end: qnt})
}

fn generate_key(qnt:u32) -> Vec<u32> {
    let mut key_vec:Vec<u32> = Vec::new();
    let mut n_for_comparation;
    let mut repeat:bool = true;

    let mut n:u32 = range(qnt);
    key_vec.push(n);
    
    for _i in (std::ops::Range{start: 0, end: qnt - 1}){    
        while repeat{
            n = range(qnt);
            n_for_comparation = n;
            if key_vec.iter().filter(|&n| *n == n_for_comparation).count() == 0{
                key_vec.push(n);
                repeat = false;
            }else{
                repeat = true;
            }
        }repeat = true;
    }
    return key_vec;
}





// Structs

struct Key{
    components:Vec<u32>
}

impl Key{
    fn format_key(&self) -> String {

        let mut string_base:String = String::from("");
        let mut aux;

        enum Last{SingleDigit,  MultipleDigit}
        let mut last_number = Last::MultipleDigit;

        for i in &self.components{
            if i < &10{
                match last_number{
                    Last::SingleDigit => aux = format!("{}", i),
                    Last::MultipleDigit => aux = format!(".{}", i)
                }
                // aux = format!("{}.", i);   
                last_number = Last::SingleDigit;
            }else{
                match last_number{
                    Last::SingleDigit => aux = format!("-{}", i),
                    Last::MultipleDigit => aux = format!("-{}", i)
                }last_number = Last::MultipleDigit;
            }string_base = string_base + String::as_str(&aux);
        }

        
        let final_formatting = &string_base.split_at(string_base.len() - 1);
        if final_formatting.1 == "."{
            return String::from(final_formatting.0)
        }else{
            return string_base
        }
    }
}




struct Data{
    seed:Vec<String>,
    key:String
}

fn receive_data(arguments:Vec<String>) -> Data {

    let mut words:Vec<String> = Vec::new();
    let mut key:String = String::from("");
    let mut received_words:bool = false; // receive true if all words has been passed to words:<Vec>
    let mut is_there_key:bool = false;
    let data:Data;

    for i in arguments{
        if i != "-k" && !received_words{
            words.push(i);
        }else if i == "-k"{
            received_words = true;
            is_there_key = true;
        }else{
            key = String::from(&i);
        }
    }words.remove(0);

    data = Data{
        seed:words,
        key:key
    };
    data
}



// fn key_reader() {}

// fn code(seed:Vec<String>) -> ? {}

// fn decode(seed:Vec<String>, key:Vec<u32>) -> seed:Vec<String> {}





fn handle_commands() {
    let mut arguments: Vec<String> = Vec::new();
    for argument in env::args() {
        arguments.push(String::from(argument));
    }arguments.remove(0);


    if arguments[0] == "new-key"{

        let key:Key = Key{components:generate_key(arguments[1].parse::<u32>().unwrap())};

        println!(";;\t{}", key.format_key());





    }else if arguments[0] == "code"{


        let data = receive_data(arguments);
        
        
        println!("dev - {{seed: {:?}}}", data.seed);
        println!("dev - {{key: {:?}}}", data.key);
        




    }else if arguments[0] == "decode"{
        println!("decode");
    }

}






fn main() {
    handle_commands();
}
