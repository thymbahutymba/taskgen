use crate::{
    opt::RTAppOpt,
    tasksets::{Task, Taskset},
};
use serde::Serialize;
use std::{collections::HashMap, fs::File, io::prelude::*};

#[derive(Serialize)]
struct TaskTimer {
    period: usize,
    mode: String,
    #[serde(rename = "ref")]
    refs: String,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct JsonTask {
    runtime: usize,
    timer: TaskTimer,
    dl_runtime: usize,
    dl_period: usize,
}

// Convert the taskset's tasks into json tasks that can be serialized as rt-app wants
impl From<&Task> for JsonTask {
    fn from(t: &Task) -> Self {
        JsonTask {
            runtime: (t.c() * 0.95) as usize, // (t.c() * 1_000.0) as usize, // ms -> us
            timer: TaskTimer {
                period: *t.period() as usize, //(t.period() * 1_000.0) as usize, // ms -> us
                refs: "unique".into(),
                mode: "absolute".into(),
            },
            dl_runtime: *t.c() as usize,
            dl_period: *t.period() as usize,
        }
    }
}

#[derive(Serialize)]
struct JsonTaskset(HashMap<String, JsonTask>);

// Convert the Taskset into JsonTaskset that can be serialized as rt-app wants
impl From<&Taskset> for JsonTaskset {
    fn from(t: &Taskset) -> Self {
        let mut json_taskset: HashMap<String, JsonTask> = HashMap::new();
        t.as_ref().iter().enumerate().for_each(|(index, elem)| {
            json_taskset.insert(format!("task{}", index), elem.into());
            ()
        });

        JsonTaskset(json_taskset)
    }
}

// rt-app global configuration json object
#[derive(Serialize)]
struct JsonRtappConfig<'a> {
    duration: usize,
    #[serde(flatten)]
    inner: &'a RTAppOpt,
}

// rt-app Config.json
#[derive(Serialize)]
struct JsonConfig<'a> {
    tasks: JsonTaskset,
    global: JsonRtappConfig<'a>,
}

pub fn create_config_json(t: &Taskset, global_conf: &RTAppOpt, fname: &String) {
    // Hyperperiod from usec to sec
    let hyperperiod = (t.get_hyperperiod() as f64 / 1_000_000.0).ceil() as usize;
    let rt_app_config = JsonRtappConfig {
        duration: (2 * hyperperiod), // seconds
        inner: global_conf,
    };

    let config = JsonConfig {
        tasks: t.into(),
        global: rt_app_config,
    };

    let mut file = File::create(fname).unwrap();
    file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
        .unwrap();
}
