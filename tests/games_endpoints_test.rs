use curse_client::CurseClient;

struct TestContext {
    client: CurseClient,
}

fn setup() -> TestContext {
    let key = "$2a$10$EnRkD7vrTASVq651KWgFweFxasKtm3kFur30Dw3t6RZ6Me4TiiVGq";
    let client = CurseClient::new(key.to_string()).unwrap();
    TestContext { client }
}

#[actix_rt::test]
async fn test_get_games() {
    let ctx = setup();
    let resp = ctx.client.games().get_games().await;
    let games = resp.unwrap();

    assert!(games.len() > 0);
}

#[actix_rt::test]
async fn test_get_game() {
    let ctx = setup();
    let resp = ctx.client.games().get_game(432).await;
    let game = resp.unwrap();

    assert_eq!(game.id, 432);
    assert_eq!(game.name, "Minecraft");
    assert_eq!(game.slug, "minecraft");
}

#[actix_rt::test]
async fn test_get_versions() {
    let ctx = setup();
    let resp = ctx.client.games().get_versions(432).await;
    let versions = resp.unwrap();

    assert!(versions.len() > 0);
}

#[actix_rt::test]
async fn test_get_version_types() {
    let ctx = setup();
    let resp = ctx.client.games().get_version_types(432).await;
    let versions_types = resp.unwrap();

    assert!(versions_types.len() > 0);
}