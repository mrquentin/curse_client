use curse_client::CurseClient;
use std::env;

struct TestContext {
    client: CurseClient,
}

impl Drop for TestContext {
    fn drop(&mut self) {
        println!("Test teardown ...");
    }
}

fn setup() -> TestContext {
    println!("Test setup ...");
    let key = "$2a$10$EnRkD7vrTASVq651KWgFweFxasKtm3kFur30Dw3t6RZ6Me4TiiVGq";
    let client = CurseClient::new(key.to_string()).unwrap();
    TestContext { client }
}

#[actix_rt::test]
async fn test_get_games() {
    let ctx = setup();
    let resp = ctx.client.games().test().await;

    let games = match resp {
        Ok(games) => games,
        Err(e) => panic!("Error: {}", e),
    };

    // assert!(games.len() > 0);
}