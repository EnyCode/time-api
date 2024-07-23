#[macro_use] extern crate rocket;
//use rocket::http::Status;
use chrono::{Utc, TimeZone};
use chrono_tz::US::Eastern;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TimeResponse {
  datetime: String
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Status {
  online: bool,
  routes: Vec<&'static str>,
}

#[get("/et")]
fn eastern_time() -> Json<TimeResponse> {
    let utc = Utc::now().naive_utc();
    let eastern = Eastern.from_utc_datetime(&utc);
    Json(TimeResponse { datetime: eastern.to_rfc3339() } )
}

#[get("/")]
fn index() -> Json<Status> {
  Json(Status {
    online: true,
    routes: vec!["/et"]
  })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, eastern_time])
}

