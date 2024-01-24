use async_std::task;
use rcon::{AsyncStdStream, Connection, Error};
use sys_info::mem_info;

async fn run(address: &str, password: &str) -> Result<(), Error> {
    println!("{}, {}", address, password);
    if let Ok(usage) = mem_info() {
        if usage.free < 500000 || usage.swap_free < 2000000 {
            println!("Less than 500mb");
            let mut conn = <Connection<AsyncStdStream>>::builder()
                .connect(address, password)
                .await?;

            let player_num = check_player_num(&mut conn).await?;

            let seconds = match player_num {
                0 => "25",
                _ => "300",
            };

            println!("restarting in {}s..", seconds);

            shutdown_server(&mut conn, seconds).await?;
        } else {
            println!("enough free memory");
        }

        Ok(())
    } else {
        println!("Couldn't get the current memory usage");
        Ok(())
    }
}

async fn check_player_num(conn: &mut Connection<AsyncStdStream>) -> Result<usize, Error> {
    let resp = conn.cmd("ShowPlayers").await?;

    println!("{}", resp);

    let x = resp.lines().count();

    let num = match x {
        0 => 0,
        x => x - 1,
    };

    Ok(num)
}

async fn shutdown_server(conn: &mut Connection<AsyncStdStream>, time: &str) -> Result<(), Error> {
    let cmd = format!(
        "Shutdown {} 'Neustart, weil wir keinen Speicher haben'",
        time
    );
    let resp = conn.cmd(cmd.as_str()).await?;

    println!("{}", resp);
    Ok(())
}

async fn test_check_player_num(address: &str, password: &str) -> Result<(), Error> {
    let mut conn = <Connection<AsyncStdStream>>::builder()
        .connect(address, password)
        .await?;

    let player_num = check_player_num(&mut conn).await?;

    println!("Player num {}", player_num);

    Ok(())
}

fn main() -> Result<(), Error> {
    let address = std::env::args().nth(1).expect("no address given");
    let password = std::env::args().nth(2).expect("no password given");

    let x = task::block_on(test_check_player_num(address.as_str(), password.as_str()));

    if let Ok(usage) = mem_info() {
        println!("{:#?}", usage);
    }
    println!("{:#?}", x);

    //task::block_on(run(address.as_str(), password.as_str()))

    Ok(())
}
