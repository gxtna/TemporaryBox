use crate::minio_client::minio_server;
use crate::pg_client::pg;
use crate::utils::time;
use delay_timer::prelude::*;

use super::config::APPCONFIG;

pub async fn task_build() {
    let delay_timer_builder = DelayTimerBuilder::default().build();
    delay_timer_builder
        .insert_task(timer_delete_task().await)
        .expect("deleted task error");
}

async fn timer_delete_task() -> Task {
    let conf = &APPCONFIG.timer;
    let mut task_builder = TaskBuilder::default();
    let body = || async {
        delete_build().await;
    };
    // 每天凌晨12点执行一次 ，超时的删除
    task_builder
        .set_frequency_repeated_by_cron_str(&conf.clone().cron)
        .set_maximum_running_time(10)
        .spawn_async_routine(body)
        .expect("task execute error")
}
async fn delete_build() {
    let box_infos = pg::select_box_info_all().await;
    let date_time = time::get_local_time();
    for info in box_infos {
        let subtract_time = date_time - info.update_time();
        if subtract_time.num_hours() > i64::from(info.storage_time()) {
            minio_server::delete_object(&info.file_remote_name())
                .await
                .expect("delete file error");
            pg::delete_box_info(info.file_remote_name()).await;
        }
    }
}
