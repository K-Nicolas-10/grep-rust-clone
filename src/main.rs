use std::{env, fs::File, io::BufRead, io::BufReader, path::Path};
fn get_args() -> Result<Vec<String>,String>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("No arguments provided. Try running -- help"));
    }
    Ok(args)
}
fn main() -> std::io::Result<()> {
    
    let args : Vec<String> = match get_args(){
        Ok(some_args) => some_args,
        Err(error) => {
            eprintln!("{}",error);
            std::process::exit(1)
        }
    };
    let path = if args[1].len() > 0{
        Path::new(&args[1])
    } else {
        panic!("No directory selected");
    };
    let searched_word = if args[2].len() > 0 {
        &args[2]
    } else {
        panic!("No word provided to search for");
    };

    let file = File::open(path)?;

    let file_read = BufReader::new(file);

    let mut current_line_index = 0;

    for line in file_read.lines() {
        current_line_index = current_line_index + 1;

        let line = line?;

        for word in line.split(' ') {
            //Debugging line:

            //println!("Comparing [{:?}] to [{:?}]", word, searched_word);

            if &String::from(word) == searched_word {
                println!(
                    "instance of {} on line {}",
                    &searched_word, &current_line_index
                );
            }
        }
    }

    Ok(())
}
