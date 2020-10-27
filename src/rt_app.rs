use crate::{
    opt::RTAppOpt,
    tasksets::{Task, Taskset},
};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*},
};

// Period as H_TIMES * hyperperiod
const H_TIMES: usize = 2;
const RUNTIME: f32 = 0.99;

#[derive(Serialize, Deserialize)]
pub struct TaskTimer {
    period: usize,
    mode: String,
    #[serde(rename = "ref")]
    refs: String,
}

#[derive(Serialize, Deserialize, Getters)]
#[serde(rename_all = "kebab-case")]
#[getset(get = "pub")]
pub struct JsonTask {
    runtime: usize,
    timer: TaskTimer,
    dl_runtime: usize,
    dl_period: usize,
    delay: usize,
}

// Convert the taskset's tasks into json tasks that can be serialized as rt-app wants
impl From<&Task> for JsonTask {
    fn from(t: &Task) -> Self {
        JsonTask {
            runtime: (t.c() * RUNTIME) as usize, // (t.c() * 1_000.0) as usize, // ms -> us
            timer: TaskTimer {
                period: *t.period() as usize, //(t.period() * 1_000.0) as usize, // ms -> us
                refs: "unique".into(),
                mode: "absolute".into(),
            },
            dl_runtime: *t.c() as usize,
            dl_period: *t.period() as usize,
            delay: 500000,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonTaskset(pub HashMap<String, JsonTask>);

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
#[derive(Serialize, Deserialize)]
struct JsonRtappConfig {
    duration: usize,
    #[serde(flatten)]
    inner: RTAppOpt,
}

// rt-app Config.json
#[derive(Serialize, Deserialize)]
struct JsonConfig {
    tasks: JsonTaskset,
    global: JsonRtappConfig,
}

pub fn create_config_json(t: &Taskset, global_conf: RTAppOpt, fname: &String) {
    // Hyperperiod from usec to sec
    let hyperperiod = (t.get_hyperperiod() as f64 / 1_000_000.0).ceil() as usize;
    let rt_app_config = JsonRtappConfig {
        duration: (H_TIMES * hyperperiod), // seconds
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

pub fn write_back_config_json(json: &String) {
    let json_content: JsonConfig = serde_json::from_str(json).unwrap();
    let taskset: Taskset = (&json_content.tasks).into();
    let task_array: Vec<Vec<f32>> = (&taskset).into();
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b' ')
        .from_writer(io::stdout());

    task_array
        .into_iter()
        .for_each(|t| wtr.serialize(t).unwrap());

    wtr.flush().unwrap();
}
