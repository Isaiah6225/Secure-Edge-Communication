use rusqlite::{Connection};

fn main() {
    let conn = Connection::open("device_registry.db").expect("couldn't create db");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS device (mac_address BLOB, pub_key BLOB, nonce INTEGER, enrollment_status TEXT)",
        (),
    ).expect("couldn't execute create command");
    
    let dev_id = "fill_from_node_gen_save_output";
    let dev_pub = "fill_from_set_keys_output"; 

    conn.execute(
        "INSERT INTO device (mac_address, pub_key) VALUES (?1, ?2)",
        (dev_id, dev_pub),
    ).expect("couldn't execute insert command");
}
