use proto_gen::cab::cab_service_client::CabServiceClient;
use proto_gen::cab::{ GetCabRequest, Location, CabLocationRequest };

mod proto_gen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a channel(connection to a server)
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

    // Creating gRPC client from channel
    let mut client = CabServiceClient::new(channel);

    loop {
        let mut name = String::new();
        let mut latitude = String::new();
        let mut longitude = String::new();

        println!(
            "Welcome to our cab service rpc client.\n
            Enter a to get a cab and b to record a cab.\n
            Entering any other input ends the program"
        );
        let mut service = String::new();

        std::io::stdin().read_line(&mut service).expect("Failed reading response");
    

        if service.trim() == "a" {
            // user requests a cab
            println!("Enter the latitude i.e 60.00N, 45.00S, etc");
            std::io::stdin().read_line(&mut latitude).expect("Failed reading latitude");

    
            println!("Enter the longitude i.e 60.00E, 45.00W, etc");
            std::io::stdin().read_line(&mut longitude).expect("Failed reading longitude");
    
            let mut latitude = latitude.trim().to_string();
            let lat_direction = match latitude.pop().unwrap().to_lowercase().collect::<Vec<_>>()[0] {
                'n' => 0,
                's' => 1,
                _ => { panic!("Invalid direction for latitude"); }
            };
    
            let lat_value = latitude.parse::<f32>()
                .unwrap_or_else(|e| panic!("Error parsing latitude value: {}", e));
            println!("Latitude value : {}", lat_value);
    
            let mut longitude = longitude.trim().to_string();
            let long_direction = match longitude.pop().unwrap().to_lowercase().collect::<Vec<_>>()[0] {
                'e' => 2,
                'w' => 3,
                _ => { panic!("Invalid direction for longitude");}
            };
    
            let long_value = longitude.parse::<f32>()
                .unwrap_or_else(|e| panic!("Error parsing longitude value: {}", e));
            println!("Longitude value : {}", long_value);
            
            // Create and send request
            let request = tonic::Request::new( 
                GetCabRequest {
                    location: Some(Location {
                        latitude: lat_value, 
                        lat_direction: lat_direction,
                        longitude: long_value,
                        long_direction: long_direction,
                    })
                }
            );
    
            let response = client.get_cabs(request).await?.into_inner();
            println!("RESPONSE = {:?}", response);
            break;
        } else if service.trim() == "b" {
            // User records a cab
            println!("Input the cab name: ");
            std::io::stdin().read_line(&mut name).expect("Failed to read name from stdin");
    
            println!("Enter the latitude i.e 60.00N, 45.00S, etc");
            std::io::stdin().read_line(&mut latitude).expect("Failed reading latitude");
    
            println!("Enter the longitude i.e 60.00W, 45.00E, etc");
            std::io::stdin().read_line(&mut longitude).expect("Failed reading longitude");
    
            let lat_direction = match latitude.pop().unwrap() {
                'n' | 'N' => 0,
                's' | 'S' => 1,
                _ => { panic!("Invalid direction for latitude");}
            };
    
            let lat_value = latitude.parse::<f32>()
                .unwrap_or_else(|e| panic!("Error parsing latitude value: {}", e));
            println!("Latitude value : {}", lat_value);
    
            let long_direction = match longitude.pop().unwrap() {
                'e' | 'E' => 2,
                'w' | 'W' => 3,
                _ => { panic!("Invalid direction for longitude");}
            };
    
            let long_value = longitude.parse::<f32>()
                .unwrap_or_else(|e| panic!("Error parsing longitude value: {}", e));
            println!("Longitude value : {}", long_value);
            
    
            // Create and send request
            let request = tonic::Request::new( 
                CabLocationRequest {
                    name: name,
                    location: Some(Location {
                        latitude: lat_value, 
                        lat_direction: lat_direction,
                        longitude: long_value,
                        long_direction: long_direction,
                    })
                }
            );
    
            let response = client.record_cab_location(request).await?.into_inner();
            println!("RESPONSE={:?}", response);
            break;
        } else {
            println!("Ending client..");
            break;
        }
    }

    Ok(())
}

