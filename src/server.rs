use tonic::{transport::Server, Request, Response, Status};
use proto_gen::cab::cab_service_server::{CabService, CabServiceServer};
use proto_gen::cab::{Cab, Location, CabLocationRequest, CabLocationResponse, GetCabRequest, GetCabResponse};

mod proto_gen;

#[derive(Default)]
pub struct Cabs {}

// We implement rpc for our example_service defined in .proto
#[tonic::async_trait]
impl CabService for Cabs {
    async fn record_cab_location(&self, _request:Request<CabLocationRequest>) -> Result<Response<CabLocationResponse>,Status> {
        Ok(Response::new(CabLocationResponse {
            // hardcode accepted as true
            accepted: true,
        }))
    }

    async fn get_cabs(&self, _request:Request<GetCabRequest>) -> Result<Response<GetCabResponse>, Status> {
        let cab_one = Cab {
            name: String::from("toyota_camry2003"),
            location: Some(Location {
                latitude: 60.00,
                lat_direction: 0,
                longitude: 30.00,
                long_direction: 3
            }),
        };

        let cab_two = Cab {
            name: String::from("mike's_sienna_2003"),
            location: Some(Location {
                latitude: -120.00,
                lat_direction: 1,
                longitude: 45.00,
                long_direction: 4
            }),
        };

        let cabs = vec![cab_one, cab_two];

        Ok(Response::new(GetCabResponse {
            // the cabs gotten
            cabs: cabs
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // Creating our service
    let service = Cabs::default();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CabServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}