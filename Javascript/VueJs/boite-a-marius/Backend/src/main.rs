#[tokio::main]
async fn main() {
    warp::serve(warp::fs::dir("assets"))
        .run(([0, 0, 0, 0], 3030))
        .await;
}