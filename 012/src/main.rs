use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::Client;
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{Doc, ReadTxn, StateVector, Text, Transact, Update};

// This function handles incoming HTTP requests and applies received updates
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let update = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let update = Update::decode_v1(&update).unwrap();
    {
        let doc = Doc::new();
        let mut txn = doc.transact_mut();
        txn.apply_update(update);
    }
    Ok(Response::new(Body::from("Update applied")))
}

// This function sends updates to other nodes via HTTP
async fn send_update_to_node(node_addr: &str, update: Vec<u8>) {
    let client = Client::new();
    let req = Request::post(node_addr)
        .header("content-type", "application/octet-stream")
        .body(Body::from(update))
        .unwrap();

    if let Err(e) = client.request(req).await {
        eprintln!("Error sending update: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let doc = Doc::new();
    let text = doc.get_or_insert_text("root");

    // Update
    {
        let mut txn = doc.transact_mut();
        text.insert(&mut txn, 0, "micro");
        text.insert(&mut txn, 5, "world");
    } // transaction is automatically committed when dropped

    // Run the HTTP server
    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |_: Request<Body>| async move {
                Ok::<_, Infallible>(Response::new(Body::from(format!(
                    "Received {}!",
                    remote_addr
                ))))
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Start the server
    let server_handle = tokio::spawn(server);

    // Example nodes to send updates to
    let nodes: Vec<String> = vec![
        "http://127.0.0.1:3001".to_string(),
        "http://127.0.0.1:3002".to_string(),
    ];

    // Generate an update and send it to other nodes
    {
        let remote_doc = Doc::new();
        remote_doc.get_or_insert_text("article");
        let remote_timestamp = remote_doc.transact().state_vector().encode_v1();
        // get update with contents not observed by remote_doc
        let update = doc
            .transact()
            .encode_diff_v1(&StateVector::decode_v1(&remote_timestamp).unwrap());
        // apply update on remote doc
        for node in nodes {
            send_update_to_node(&node, update.clone()).await;
        }
    };

    // Keep the server running
    server_handle.await.unwrap();
}
