use std::error::Error;
use std::str::FromStr;
use std::convert::TryInto;
use stargate_grpc::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = StargateClient::builder()
    .uri("https://e2ba1542-362d-41bb-b920-96640808b4a6-us-east-1.apps.astra.datastax.com/stargate")?
    .auth_token(AuthToken::from_str("AstraCS:BuqyCeZeifJbzkCnysKZkvsD:618a706865e0a147a6f80e39544309e8b21bcd610f67ae5e7686708e502c705d")?)                                         
    .tls(Some(client::default_tls_config()?))   // optional
    .connect()
    .await?;

    let query = Query::builder()
    .consistency(Consistency::LocalQuorum)
    // comment out `yeet` and add it to the query `yeet.users` and it will work
    .keyspace("yeet")
    .query("select firstname, lastname from users")
    .build();     

    let response = client.execute_query(query).await?; 
    let result_set: ResultSet = response.try_into()?; 

    for row in result_set.rows {
        let (firstname, lastname): (String, String) = row.try_into()?;
        println!("{} {}", firstname, lastname);
    }

    Ok(())
}
