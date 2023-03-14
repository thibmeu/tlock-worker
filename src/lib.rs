use serde_json::json;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("POST /encrypt/:round to encrypt to a specfic round on fastnet
POST /decrypt to decrypt

File should be under 10MiB in size"))
        .post_async("/encrypt/:round", |mut req, ctx| async move {
            let chain = drand_core::chain::Chain::new("https://drand.cloudflare.com/dbd506d6ef76e5f386f41c651dcb808c5bcbd75471cc4eafa3f4df7ad4e4c493");
            let client = drand_core::http_chain_client::HttpChainClient::new(chain, None);
            let info = client.chain().info().await.unwrap();
            let round = match ctx.param("round") {
                Some(r) => r.parse::<u64>().unwrap(),
                None => return Response::error("round invalid", 400),
            };

            let src = req.bytes().await?;

            let mut armored = tlock_age::armor::ArmoredWriter::wrap_output(vec![]).unwrap();
            tlock_age::encrypt(
                &mut armored,
                src.as_slice(),
                &info.hash(),
                &info.public_key(),
                round,
            )
            .unwrap();
            let encrypted = armored.finish().unwrap();
            Response::from_bytes(encrypted)
        })
        .post_async("/decrypt", |mut req, ctx| async move {
            let chain = drand_core::chain::Chain::new("https://drand.cloudflare.com/dbd506d6ef76e5f386f41c651dcb808c5bcbd75471cc4eafa3f4df7ad4e4c493");
            let client = drand_core::http_chain_client::HttpChainClient::new(chain, Some(drand_core::chain::ChainOptions::new(false, true, None)));
            let info = client.chain().info().await.unwrap();

            let src = req.bytes().await?;
            let round = tlock_age::decrypt_header(src.clone().as_slice()).unwrap().round();

            // let signature = client.get(round).await.unwrap().signature();

            let mut decrypted = vec![];
            // signature for round 1000
            let signature = hex::decode("b09eacd45767c4d58306b98901ad0d6086e2663766f3a4ec71d00cf26f0f49eaf248abc7151c60cf419c4e8b37e80412").unwrap();
            tlock_age::decrypt(
                &mut decrypted,
                src.as_slice(),
                &info.hash(),
                &signature,
            )
            .unwrap();
            Response::from_bytes(decrypted)
        })
        .run(req, env)
        .await
}
