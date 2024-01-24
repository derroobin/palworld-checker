use rcon_client::{AuthRequest, RCONClient, RCONConfig, RCONError, RCONRequest};
use sys_info::mem_info;

fn run(address: String, password: String) -> Result<(), RCONError> {
    if let Ok(usage) = mem_info() {
        if usage.free < 900_000 || usage.swap_free < 2_000_000 {
            println!("Less than 900mb");
            let mut client = RCONClient::new(RCONConfig {
                url: address,
                // Optional
                read_timeout: Some(13),
                write_timeout: Some(37),
            })?;

            let auth_result = client.auth(AuthRequest::new(password))?;
            assert!(auth_result.is_success());

            let player_num = check_player_num(&mut client)?;

            let seconds = match player_num {
                0 => "25",
                _ => "300",
            };

            println!("restarting in {}s..", seconds);

            let _ = shutdown_server(&mut client, seconds);
        } else {
            println!("enough free memory");
        }

        Ok(())
    } else {
        println!("Couldn't get the current memory usage");
        Ok(())
    }
}

fn check_player_num(client: &mut RCONClient) -> Result<usize, RCONError> {
    let resp = client.execute(RCONRequest::new("ShowPlayers".to_string()))?;
    println!("{}", resp.body);

    let x = resp.body.lines().count();

    let num = match x {
        0 => 0,
        x => x - 1,
    };

    Ok(num)
}

fn shutdown_server(client: &mut RCONClient, time: &str) -> Result<(), RCONError> {
    let cmd = format!("Shutdown {} Neustart", time);
    let resp = client.execute(RCONRequest::new(cmd))?;

    println!("{}", resp.body);
    Ok(())
}

fn main() -> Result<(), RCONError> {
    let address = std::env::args()
        .nth(1)
        .expect("no address given")
        .trim()
        .to_string();
    let password = std::env::args()
        .nth(2)
        .expect("no password given")
        .trim()
        .to_string();

    if let Ok(usage) = mem_info() {
        println!("Free memory: {}", usage.free);
    }

    run(address, password)
}
