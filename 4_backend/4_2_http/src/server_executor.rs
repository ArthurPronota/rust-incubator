use axum::{
    extract::{Json, State},
    response::Json as JsonResponse,
} ;

use crate::args ;

use crate::args::Commands;
use crate::common ;
use std::sync::Arc ;

use crate::db::Database ;

/// Обработка полученных команд от клиента
pub async fn handle_commmand(
                    State(state): State<Arc<Database>>,
                    Json(cmd): Json<args::Commands>
                ) 
                ->  JsonResponse<common::Response> 
                //Json<common::Response>
{
    
    match cmd {
        Commands::InitDb => {

            JsonResponse(common::Response::Success("Database objects created successfully.".to_owned()))
            //Json(common::Response::Success("Database objects created successfully.".to_owned()))
        },
    }
}

