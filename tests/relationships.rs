use chorus::types;

mod common;

#[tokio::test]
async fn test_get_mutual_relationships() {
    let register_schema = types::RegisterSchema::new(
        "integrationtestuser2".to_string(),
        None,
        true,
        None,
        None,
        None,
        Some("2000-01-01".to_string()),
        None,
        None,
        None,
    )
    .unwrap();

    let bundle = common::setup().await;
    let mut belongs_to = bundle.instance;
    let mut user = bundle.user;
    let mut other_user = belongs_to.register_account(&register_schema).await.unwrap();
    let friend_request_schema = types::FriendRequestSendSchema {
        username: user.object.username.clone(),
        discriminator: Some(user.object.discriminator.clone()),
    };
    other_user.send_friend_request(friend_request_schema).await;
    let relationships = user
        .get_mutual_relationships(&other_user.object.id.to_string())
        .await
        .unwrap();
    println!("{:?}", relationships.unwrap());
}
