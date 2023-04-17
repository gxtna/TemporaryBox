
use delay_timer::prelude::*;
use crate::minio_client::minio;


pub async fn task_build(){
    let delay_timer_builder = DelayTimerBuilder::default().build();
    delay_timer_builder.insert_task(timer_delete_task().await).expect("deleted task error");
}

async fn timer_delete_task() ->Task{
    println!("{}","jinru");
    let mut task_builder=TaskBuilder::default();
    let body = || async {
        println!("{}","执行完成111");
        minio::delete_object("docker-compose-1681739291372-MOANDAlL.yml").await;
            println!("{}","执行完成")
    };
    task_builder.set_frequency_repeated_by_cron_str("0,10,15,25,50 0/1 * * Jan-Dec * 2020-2100")
    .set_maximum_running_time(10)
    .spawn_async_routine(body).expect("task execute error")
}