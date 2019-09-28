extern crate lmdb_zero as lmdb;
extern crate tempdir;

fn custom_query(query: &str, path: &str) {
  let query_arr: Vec<_> = query.split(' ').collect();
  for n in query_arr {
    //converts static str to String
    match n.into() {
      //INSERT QUERY VALIDATION
      "INSERT" | "insert" | "Insert" => {
        let sliced_query = query
          .replace("INSERT", "")
          .replace("insert", "")
          .replace("Insert", "");
        let trimmed = sliced_query.trim();
        let sliced_query_arr: Vec<_> = trimmed.split(' ').collect();
        if (sliced_query_arr[0] == "IN") {
          let db_name = sliced_query_arr[1];
          let key = sliced_query_arr[4];
          let value = sliced_query_arr[8];
          println!("{}", db_name);
          println!("{}", key);
          println!("{}", value);

          println!("Environment created");
          
          let env = unsafe {
            lmdb::EnvBuilder::new()
              .unwrap()
              .open(path, lmdb::open::Flags::empty(), 0o600)
              .unwrap()
              
          };


         // let env = lmdb::EnvBuilder::new().unwrap().set_maxdbs(5);




          println!("Database created");
          
          // Open the database with name db_name
     let db = lmdb::Database::open(
    &env, Some("example-db"), &lmdb::DatabaseOptions::new(
      lmdb::db::CREATE)).unwrap();
          println!("TX CALL");

          {
            // Write some data in a transaction
            println!("Write data");

            let txn = lmdb::WriteTransaction::new(&env).unwrap();
            // An accessor is used to control memory access.
            // NB You can only have one live accessor from a particular transaction
            // at a time. Violating this results in a panic at runtime.
            {
              let mut access = txn.access();
              access
                .put(&db, key, value, lmdb::put::Flags::empty())
                .unwrap();
            }
            // Commit the changes so they are visible to later transactions
            txn.commit().unwrap();
          }

          {
            // Now let's read the data back
            let txn = lmdb::ReadTransaction::new(&env).unwrap();
            let access = txn.access();

            // Get the capital of Latvia. Note that the string is *not* copied; the
            // reference actually points into the database memory, and is valid
            // until the transaction is dropped or the accessor is mutated.
            let capital_of_latvia: &str = access.get(&db, key).unwrap();
            let new_val = "asdf";
            assert_eq!([value, new_val].concat(), capital_of_latvia);
          }
        } else {
          println!("Syntax Error at Column 2")
        }
        break;
      }
      "UPDATE" | "update" | "Update" => {
        println!("Update");
        break;
      }
      "DELETE" | "delete" | "Delete" => {
        println!("DELETE");
        break;
      }
      _ => println!("Syntax Error at Column 1"),
    }
  }
}

fn main() {
  let path = "/home/rajkishor/Desktop/RUST/DB";
  let QUERY_INSERT = "INSERT IN popa KEY = GOAL | VALUE = FINISH";
  custom_query(QUERY_INSERT, path);
}
