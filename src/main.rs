extern crate lmdb_zero as lmdb;
extern crate tempdir;

pub fn custom_query(query: &str, path: &str) -> bool {
  let mut q_status = false;
  let query_arr: Vec<_> = query.split(' ').collect();

  let mut env_builder = lmdb::EnvBuilder::new().unwrap();
  env_builder.set_maxdbs(2);
  let env = unsafe {
    env_builder
      .open(path, lmdb::open::Flags::empty(), 0o600)
      .unwrap()
  };

  let db = lmdb::Database::open(
    &env,
    Some("example-db"),
    &lmdb::DatabaseOptions::new(lmdb::db::CREATE),
  )
  .unwrap();
  println!("TX CALL");

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

          {
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
            let txn = lmdb::ReadTransaction::new(&env).unwrap();
            let access = txn.access();

            let get_val: &str = access.get(&db, key).unwrap();
            if(value == get_val){
              q_status = true; 
            }
          {
            println!("Key Vaule pair has inserted.");
          }
        } else {
          println!("Syntax Error at Column 2")
        }
        break;
      }
      "UPDATE" | "update" | "Update" => {
        println!("Update");

        let sliced_query = query
          .replace("UPDATE", "")
          .replace("update", "")
          .replace("Update", "");
        let trimmed = sliced_query.trim();
        let sliced_query_arr: Vec<_> = trimmed.split(' ').collect();
        let db_name = sliced_query_arr[0];
        let key = sliced_query_arr[6];
        let value = sliced_query_arr[14];

        let txn = lmdb::ReadTransaction::new(&env).unwrap();
        let access = txn.access();
        //Fetch by key value
        let fetch_key: &str = access.get(&db, value).unwrap();
        //delete the current key
        //create new key with new val 

        break;
      }
      "DELETE" | "delete" | "Delete" => {
        println!("DELETE");
        let sliced_query = query
          .replace("DELETE", "")
          .replace("delete", "")
          .replace("Delete", "");
        let trimmed = sliced_query.trim();
        let sliced_query_arr: Vec<_> = trimmed.split(' ').collect();
        let db_name = sliced_query_arr[1];
        let key = sliced_query_arr[(sliced_query_arr.len() - 1)];
        let txn = lmdb::ReadTransaction::new(&env).unwrap();
        let access = txn.access();
        let mut cursor = txn.cursor(&db).unwrap();
        cursor.seek_k::<str, str>(&access, key).unwrap(); // get the current cursor

        break;
      }
      _ => println!("Syntax Error at Column 1"),
    }
  }
  q_status
}

#[cfg(test)]
mod tests {
    use super::*;  
    #[test]
    fn is_query_working() {
        
        assert_eq!(true, custom_query(DB_PARAMS.query, DB_PARAMS.path));

    }
}

fn main() {
  let path = DB_PARAMS.path;
  let QUERY_INSERT = DB_PARAMS.query;
  // let QUERY_UPDATE = "UPDATE database_name KEY = XXXXXX | NEW_VALUE = xxxxxx";
  // let QUERY_DELETE = "DELETE FROM database_name KEY = XXXXXX ";
  custom_query(QUERY_INSERT, path);
}
static DB_PARAMS: QueryStruct = QueryStruct  {
    path:   "/home/rajkishor/Desktop/RUST/DB",
    query:  "INSERT IN popa KEY = GOAL | VALUE = FINISH",
};
struct QueryStruct {
    path: &'static str,
    query: &'static str,
}

