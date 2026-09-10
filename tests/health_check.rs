use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;
use tokio::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind the random port");
    let address = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move {
        zero2prod::startup::run(listener)
            .await
            .expect("Failed to spawn our app");
    });
    address
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    //Creating new clientRequest
    let client = reqwest::Client::new();

    // Response
    let res = client
        .get(format!("{}/check_health", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(res.status().is_success());
    assert_eq!(res.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let app_address = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let res = client
        .post(format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    let app_address = spawn_app().await;

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let res = client
            .post(format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request.");

        assert_eq!(
            422,
            res.status().as_u16(),
            "The API didnot fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

