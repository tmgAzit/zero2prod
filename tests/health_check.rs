use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use tokio::net::TcpListener;
use zero2prod::startup::run;

pub struct TestApp{
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind the random port");
    let address = format!("http://{}", listener.local_addr().unwrap());
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect to Postgres.");
    
    let pool_for_app = connection_pool.clone();
    tokio::spawn(async move {
        run(listener, connection_pool)
            .await
            .expect("Failed to spawn our app");
    });
    TestApp{
        address,
        db_pool: pool_for_app
    }
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await.address;
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    
    let email = format!("{}@gmail.com", uuid::Uuid::new_v4());
    let body = format!("name=liebe&email={}", urlencoding::encode(&email));

    let res = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions WHERE email = $1", email)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, email);
    assert_eq!(saved.name, "liebe");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    let app_address = spawn_app().await.address;

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

