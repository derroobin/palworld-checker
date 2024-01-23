use async_std::task;
use rcon::{AsyncStdStream, Connection, Error};
use sys_info::mem_info;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

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

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    let mut sched = JobScheduler::new().await?;
    let address = std::env::args().nth(1).expect("no address given");
    let password: String = std::env::args().nth(2).expect("no password given");

    sched
        .add(Job::new("11 */10 * * * *", move |_uuid, _l| {
            let _ = task::block_on(run(address.as_str(), password.as_str()));
        })?)
        .await?;

    // Add code to be run during/after shutdown
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    sched.start().await?;
    tokio::time::sleep(core::time::Duration::from_secs(1200)).await;

    Ok(())
}
