extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use std::collections::BTreeMap;

use nickel::Nickel;
use nickel::HttpRouter;

use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

struct Contact {
    name: String,
    email: String,
}

impl ToJson for Contact {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("name".to_string(), self.name.to_json());
        map.insert("email".to_string(), self.email.to_json());

        return Json::Object(map);
    }
}

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware!{
        "Index"
    });

    server.get("/contacts", middleware!{
        "ContactsList"
    });
    server.post("/contacts", middleware!{
        "ContactsCreate"
    });
    server.get("/contacts/:id", middleware!{
        "ContactsShow"
    });
    server.put("/contacts/:id", middleware!{
        "ContactsUpdate"
    });
    server.delete("/contacts/:id", middleware!{
        "ContactsDelete"
    });

    server.listen("127.0.0.1:3000");
}
