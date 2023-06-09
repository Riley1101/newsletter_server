use crate::helpers::spawn_app;
use wiremock::{Mock,ResponseTemplate};
use wiremock::matchers::{method,path};

#[tokio::test]
async fn subscribe_send_a_confirmation_for_valid_data( ) {
    let app = spawn_app().await; 
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    app.post_subscriptions(body.into()).await;
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data(){
    let app =spawn_app()
       .await;
    let body ="name=le&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    let response = app.post_subscriptions(body.into()).await;
    println!("response: {:?}",response.status());
    let _ = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(200,response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        // Act
        let response = app.post_subscriptions(body.into()).await;
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description 
        );
    }
}

#[tokio::test]
async fn subscribe_returns_400_when_fields_present_but_invalid(){
    let app =spawn_app()
        .await;
    let test_cases = vec![
      ("name=Ursula&email=", "empty email"),
      ("name=Ursula&email=definitely-not-an-email", "invalid email"),
   ];

    for (body, description) in test_cases{
        let response = app.post_subscriptions(body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "api did not return 400 when the payload was {}",description
        )
    }

}

#[tokio::test]
async fn subscribe_returns_400_when_data_missing(){
    let app =spawn_app()
        .await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (body,error_message) in test_cases{
        let response = app.post_subscriptions(body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 when the payload was {}",
            error_message
        );
    }
}
