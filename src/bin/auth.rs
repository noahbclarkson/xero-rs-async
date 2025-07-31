use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use tiny_http::{Response, Server};
use url::Url;
use uuid::Uuid;
use xero_rs_async::client::XeroClient;

/// A utility to perform the interactive OAuth 2.0 authorization flow.
/// It starts a temporary local web server to catch the redirect from Xero,
/// exchanges the authorization code for a token set, saves it to a file,
/// and then lists the tenants the app is connected to.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load configuration from .env
    dotenvy::dotenv().expect("Failed to load .env file. Make sure it exists at the project root.");
    let client_id = env::var("XERO_CLIENT_ID").expect("XERO_CLIENT_ID must be set.");
    let client_secret = env::var("XERO_CLIENT_SECRET").expect("XERO_CLIENT_SECRET must be set.");
    let redirect_uri_str = env::var("XERO_REDIRECT_URI").expect("XERO_REDIRECT_URI must be set.");
    let token_cache_path = env::var("TOKEN_CACHE_PATH").expect("TOKEN_CACHE_PATH must be set.");
    let rate_limit_cache_path =
        env::var("RATE_LIMIT_CACHE_PATH").expect("RATE_LIMIT_CACHE_PATH must be set.");

    // 2. Initialize the RateLimiter and Xero Client
    use std::sync::Arc;
    use xero_rs_async::rate_limiter::RateLimiter;

    let rate_limiter = Arc::new(RateLimiter::new(PathBuf::from(rate_limit_cache_path)).await?);
    let xero_client = XeroClient::new(
        client_id.clone(),
        client_secret,
        redirect_uri_str.clone(),
        PathBuf::from(token_cache_path),
        rate_limiter,
    )
    .await?;

    // 3. Generate and display the authorization URL
    let scopes = [
        "openid",
        "profile",
        "email",
        "accounting.transactions",
        "accounting.settings",
        "accounting.contacts",
        "accounting.attachments",
        "files",
        "assets",
        "accounting.reports.read",
        "accounting.reports.tenninetynine.read",
        "accounting.journals.read",
        "accounting.budgets.read",
        "offline_access",
    ];
    let state = "12345";
    let auth_url = xero_client
        .token_manager
        .get_authorization_url(&scopes, state);

    println!("\n✅ Step 1: Your browser will now open for Xero authorization.");
    println!(
        "If it doesn't, please manually visit this URL:\n\n{auth_url}\n"
    );
    webbrowser::open(&auth_url).expect("Failed to open web browser.");

    // 4. Start a local server to listen for the callback
    let server_addr = "localhost:80";
    let server = Server::http(server_addr).unwrap_or_else(|_| panic!("Failed to start server on {server_addr}. Make sure no other services (like a web server) are using port 80 and that you have administrator privileges to run this command."));
    println!(
        "✅ Step 2: Waiting for authorization callback on {redirect_uri_str} ..."
    );
    println!("Note: If you get an error page from Xero, please check that:");
    println!(
        "  - The redirect URI in your Xero app configuration exactly matches: {redirect_uri_str}"
    );
    println!("  - All the scopes requested are enabled for your Xero app");
    println!("  - Your Xero app is set to 'Web App' type (not Public/Mobile)");

    let received_params = Arc::new(Mutex::new(None));
    let received_params_clone = received_params.clone();

    if let Some(request) = server.incoming_requests().next() {
        let url = Url::parse(&format!("http://localhost{}", request.url())).unwrap();
        let params: HashMap<String, String> = url.query_pairs().into_owned().collect();

        let response_html = if params.contains_key("code") {
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Xero Authorization Success</title>
    <style>
        body { background: #f4f8fb; font-family: 'Segoe UI', Arial, sans-serif; color: #222; margin: 0; padding: 0; }
        .container { max-width: 420px; margin: 60px auto; background: #fff; border-radius: 12px; box-shadow: 0 4px 24px rgba(0,0,0,0.09); padding: 36px 32px 28px 32px; text-align: center; }
        .success-icon { font-size: 54px; color: #27ae60; margin-bottom: 18px; }
        h1 { margin: 0 0 12px 0; font-size: 2.1em; font-weight: 600; }
        p { font-size: 1.15em; margin-bottom: 0; }
        .footer { margin-top: 30px; font-size: 0.95em; color: #888; }
    </style>
</head>
<body>
    <div class="container">
        <div class="success-icon">✅</div>
        <h1>Authorization Complete</h1>
        <p>Your Xero authorization code was received.<br>You may now close this tab and return to your application.</p>
        <div class="footer">Thank you for using Xero OAuth Utility</div>
    </div>
</body>
</html>"#
        } else {
            "<h1>Error</h1><p>Could not get authorization code from Xero.</p>"
        };

        let response = Response::from_string(response_html).with_header(
            "Content-Type: text/html"
                .parse::<tiny_http::Header>()
                .unwrap(),
        );

        request.respond(response).unwrap();
        *received_params_clone.lock().unwrap() = Some(params);
    }

    let params = received_params
        .lock()
        .unwrap()
        .take()
        .ok_or("Failed to receive callback from server thread")?;

    // 5. Parse the authorization code and state
    let code = params
        .get("code")
        .ok_or("Authorization code not found in callback URL")?;
    let received_state = params
        .get("state")
        .ok_or("State not found in callback URL")?;

    if received_state != state {
        return Err("State mismatch! CSRF attack may be in progress.".into());
    }

    println!("\nAuthorization code received successfully.");

    // 6. Exchange the code for a token set
    println!("✅ Step 3: Exchanging code for tokens...");
    match xero_client.token_manager.exchange_code(code).await {
        Ok(_) => {
            println!("\n🎉 Success! Tokens have been received and saved to 'xero_token.json'.");

            // 7. Fetch and display connected tenants
            println!("\n✅ Step 4: Fetching connected tenants...");
            fetch_and_display_connections(&xero_client).await?;
        }
        Err(e) => {
            eprintln!("\n❌ Error exchanging code for tokens: {e}");
        }
    }

    Ok(())
}

/// Struct to deserialize the response from the /connections endpoint.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Connection {
    tenant_id: Uuid,
    tenant_type: String,
    tenant_name: Option<String>,
}

/// Fetches the list of connected tenants and displays them.
async fn fetch_and_display_connections(
    xero_client: &XeroClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let access_token = xero_client.token_manager.get_access_token().await?;
    let http_client = reqwest::Client::new();

    let response = http_client
        .get("https://api.xero.com/connections")
        .bearer_auth(access_token)
        .header("Accept", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        let connections: Vec<Connection> = response.json().await?;
        if connections.is_empty() {
            println!("No tenants are currently connected to this app.");
        } else {
            println!("\nAuthorized Tenants:");
            println!("{:-<65}", "");
            println!(
                "{:<38} | {:<12} | Name",
                "Tenant ID", "Type"
            );
            println!("{:-<65}", "");
            for conn in connections {
                println!(
                    "{:<38} | {:<12} | {}",
                    conn.tenant_id,
                    conn.tenant_type,
                    conn.tenant_name.unwrap_or_else(|| "N/A".to_string())
                );
            }
            println!("{:-<65}", "");
            println!("\n💡 Tip: Copy a Tenant ID from the list above and set it as `XERO_TENANT_ID` in your .env file to run tests against that organization.");
        }
    } else {
        eprintln!(
            "❌ Error fetching connections: {} - {}",
            response.status(),
            response.text().await?
        );
    }
    Ok(())
}