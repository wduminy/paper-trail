use googletest::prelude::*;

use super::*;

#[gtest]
fn data_crud() {
    let db = DataService::open_temp_h();
    {
        let tx = db.write().unwrap();
        {
            let mut tab = tx.open(T_TEST).unwrap();
            assert_ok!(tab.insert("key-101", &serde_json::json!({"name": "Pete"})));
        }
        assert_ok!(tx.commit());
    }
    let rx = db.read().unwrap();
    let tab = rx.open(T_TEST).unwrap();
    let obj = tab.read_o::<serde_json::Value>("key-101").unwrap().unwrap();
    assert_that!(obj["name"], eq("Pete"))
}

/** A table becomes available only after something is inserted into it */
#[gtest]
fn table_not_found() {
    let db = DataService::open_temp_h();
    let rx = db.read().unwrap();
    let tab = rx.open(T_TEST).unwrap();
    let obj = tab.read_o::<serde_json::Value>("key-101").unwrap();
    assert_true!(obj.is_none());
}

const T_TEST: TableDef = TableDef::new("test_tab");
