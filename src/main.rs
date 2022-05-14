use std::collections::HashMap;
use actix_web::{web,App,HttpResponse,HttpServer};
use ttr_api::Population;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
        .route("/",web::get().to(index))
        .route("/pop",web::get().to(pop_page))
    });
    println!("Serving on http://localhost:3000...");
    server
    .bind("127.0.0.1:3000").expect("error binding server to address")
    .run().expect("error running server");
}

fn index() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        r#"<head><title>TTR Population!</title></head>
        <body><center><p style=font-family:Impress BT,Arial;font-size:23;>Need to know the current population in Toontown Rewritten? You're in the right place!<br>Check out the button below to see the current population!</p></center><br><br>
        <center><form action="/pop"><button type="submit" style=font-family:mickeykw,Arial;font-size:16;>GET population!</button></form><br><br><br></center></body>"#
    )
}

fn get_population() -> (HashMap<String,u16>,u16) {
    let pop = Population::PopAPI::new(ttr_api::makeclient().unwrap()).unwrap();
    (pop.populationByDistrict,pop.totalPopulation)
}

fn pop_table(pop_dict:HashMap<String,u16>,tot:u16) -> String {
    let mut resp: String = String::from("<table><tr><td colspan=2>Population by District</td></tr>");
    for (k,v) in pop_dict {
        let vals = format!("<tr><td>{}</td><td>{}</td></tr>",k,v);
        resp.push_str(&vals);
    } let lastbox = format!("<td>Total</td><td>{}</td></table>",tot);
    resp.push_str(&lastbox);
    resp
}

fn pop_page() -> HttpResponse {
    let table = get_population();
    let body = pop_table(table.0,table.1);
    HttpResponse::Ok()
    .content_type("text/html")
    .body(format!("<title>Population</title>{}",body))
}