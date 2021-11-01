fn main() {
   let mut args = std::env::args().skip(1);
   let key = args.next().unwrap();
   let value = args.next().unwrap();

   let db = Database::new();
   let contents = format!("{}\t{}\n", key, value);
   std::fs::write("kv.db", contents);
   println!("key:{} Value:{}", key, value);
}

use std::collections::HashMap;
struct Database {
   inner: HashMap<String, String>,
}

impl Database {
   fn new() -> Result<Database, std::io::Error> {
      //? is the syntax for error handling in rust it will return the err
      let contents = std::fs::read_to_string("kv.db")?;
      let mut inner = HashMap::new();
      //here we are iterating over the pointers of contents
      for line in contents.lines() {
         //collect will collect the chunks(of type pointers) and put them in the Vec
         let chunks: Vec<&str> = line.split('\t').collect();
         if chunks.len() != 2 {
            todo!("Return error");
         }
         let key = chunks[0];
         let value = chunks[1];
         inner.insert(key.to_owned(), value.to_owned());
      }
      Ok(Database { inner: inner })
   }
}
