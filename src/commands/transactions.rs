

use crate::Error;
use serde_derive::Deserialize;
use serde_json::Value;
//use poise::serenity_prelude as serenit;
use serenity::utils::Colour;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Root2 {
    pub waiver_budget: Vec<WaiverBudget>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub transaction_id: String,
    pub status_updated: i64,
    pub status: String,
    pub settings: Value,
    pub roster_ids: Vec<i64>,
    pub metadata: Value,
    pub leg: i64,
    pub drops: Value,
    pub draft_picks: Vec<Value>,
    pub creator: String,
    pub created: i64,
    pub consenter_ids: Vec<i64>,
    pub adds: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WaiverBudget {
    pub sender: i64,
    pub receiver: i64,
    pub amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub username: String,
    pub user_id: String,
    pub display_name: String,
    pub avatar: String,
}




/// Get info on a coin by entering their symbol
#[poise::command(slash_command)]
pub async fn transactions(
    ctx: poise::Context<'_, (), Error>,
    #[description = "What is your league ID?"] league: i128,
) -> Result<(), Error> {
    

    ctx.send(|b| {
        b.embed(|b| 
            b.title("Transactions will now be fetched")
            .description(format!("League {}", league))
            .colour(Colour::DARK_GREEN)
        )
        .ephemeral(false)
    })
    .await?;

    let mut currenttransaction = 0;

    loop {
        let response = reqwest::get(format!("https://api.sleeper.app/v1/league/{}/transactions/1", league))
        .await
        .map_err(|a| a)?
        .error_for_status()
        .map_err(|b| b)?
        .json::<Root>()
        .await
        .map_err(|c| c)?;

        


        let totaltrades = response.len();

        for i in (currenttransaction..totaltrades).rev() {
            let user = reqwest::get(format!("https://api.sleeper.app/v1/user/{}", response[i].creator))
                .await
                .map_err(|a| a)?
                .error_for_status()
                .map_err(|b| b)?
                .json::<User>()
                .await
                .map_err(|c| c)?;




            let date = response[i].created / 1000;
            ctx.send(|b| {
                b.embed(|b| 
                b.title("Transactions:")
                .field("Type", &response[i].type_field, true)
                .field("Status", &response[i].status, true)
                .field("Creator", user.display_name, true)
                .field("$Costs", &response[i].waiver_budget[0].amount, true)
                .field("Send", &response[i].waiver_budget[0].sender, true)
                .field("Received", &response[i].waiver_budget[0].receiver, true)
                .field("Date", format!("<t:{}:f>", date), true)
                .colour(Colour::DARK_GREEN))
            }).await?;

            

        }
        currenttransaction = totaltrades;
        // println!("{currenttransaction}");
        // println!("Checked for updates");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        //panic!("Crashing before anything bad happens");

    }
    


    
    

    

    //ctx.say(response).await?;
    Ok(())
}

