use crate::{
    models::navigation::NavigationInfo, services::openroute_api_service::OpenRouteApiService,
    tools::env_reader::get_env, AppState,
};
use actix_web::{
    web::{self, Bytes},
    HttpResponse, Responder,
};

pub struct NavigationDependencies {
    openroute_api_service: OpenRouteApiService,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(NavigationDependencies {
        openroute_api_service: OpenRouteApiService::new(
            get_env().get("OPEN_SERVICE_API_BASE_URL").unwrap(),
            get_env().get("OPEN_ROUTES_SERVICE_API_KEY").unwrap(),
        ),
    }))
    .service(
        web::scope("/navigation")
            .route("", web::get().to(get_navigation_details))
    );
    // NOTE: to customize error response
    // .app_data(web::JsonConfig::default().error_handler(|e, r| {
    //     match e {
    //         JsonPayloadError::Deserialize(err) => {
    //             // Customize your error message here
    //             JsonPayloadError::Deserialize(JsonError::custom("Custom Error")).into()
    //         }
    //         _ => e.into()
    //     }
    // }));
}


async fn get_navigation_details(
    info: web::Json<NavigationInfo>,
    state: web::Data<NavigationDependencies>,
) -> impl Responder {
    let service = &state.openroute_api_service;
    let response = service
        .get_navigation(&info.profile, &info.start, &info.end)
        .await;
    let result = response.unwrap_or(Bytes::new());

    HttpResponse::Ok().body(result)
}
