mod serial;
use std::time::{Duration, Instant};
use std::fs as fst;
use actix::{prelude::*, Addr, AsyncContext};
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, web::Data, Responder, get};
use actix_web_actors::ws;
/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(r: HttpRequest, stream: web::Payload, srv:Data<Addr<serial::SerialActor>>) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(srv.get_ref().clone()), &r, stream);
    println!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor

struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hex: Addr<serial::SerialActor>,
    hb: Instant,
    /*id: Uuid*/
}
impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

    }
}


/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => self.hex.do_send(
                serial::ClientActorMessage {
                    /*id: self.id,*/ 
                    msg: text.to_string()
                }
            ),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    fn new(serial: Addr<serial::SerialActor>) -> Self {
        Self { 
            hb: Instant::now(),
            /*id: Uuid::new_v4(),*/
            hex: serial
        }
    }
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

#[get("/m")]
async fn hello() -> impl Responder {
    let paths = fst::read_dir("C:/Users/User/wb_server/Music")
    .unwrap()
    .filter_map(|e| e.ok())
    .map(|e| e.path().to_string_lossy().into_owned())
    .collect::<Vec<_>>();
    HttpResponse::Ok().body(paths[0].clone())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    let serial_chat = serial::SerialActor::default().start();
    HttpServer::new(move || {
        App::new()
            .service(hello)
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            // static files
            .service(fs::Files::new("/", "static/").index_file("index.html"))
            .app_data(Data::new(serial_chat.clone()))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
