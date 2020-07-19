mod opt;
mod python;
mod rt_app;
mod tasksets;

use crate::{
    opt::TaskgenOpt,
    tasksets::{Taskset, TasksetArray},
};
use num::integer::Integer;
use std::{fs::File, io::prelude::*, iter::FromIterator};
use structopt::StructOpt;

pub fn gen_tasksets(opt: &TaskgenOpt) -> TasksetArray {
    let mut tasksets = Vec::new();
    let mut nsets = opt.nsets;

    let mut new_opt = opt.clone();
    new_opt.nsets = 1;

    while nsets != 0 {
        let (x, periods) = python::get_with_opt(&new_opt).unwrap();
        let (i, p) = (x.first().unwrap(), periods.first().unwrap());

        let mut c = i * p;

        if opt.round_c {
            c = ndarray::arr1(
                &c.into_raw_vec()
                    .iter()
                    .map(|e| e.round())
                    .collect::<Vec<f32>>(),
            );
        }

        if c.iter().any(|x| *x < 1_500.0)
            || p.iter().fold(1, |hyp, p| hyp.lcm(&(*p as usize))) as f64 / 1_000_000.0 > 4.0
        {
            continue;
        }

        nsets -= 1;
        let u = &c / p;
        tasksets.push(vec![i.clone(), u, p.clone(), c]);
    }

    (&tasksets).into()
}

fn main() -> std::io::Result<()> {
    let opt = opt::Opt::from_args();

    let tasksets = if let Some(path) = &opt.from_file {
        /* Create tasksets from file */
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        TasksetArray::new(
            contents
                .split("\n\n")
                .map(|ts| Taskset::from_iter(ts.split("\n")))
                .collect::<Vec<Taskset>>(),
        )
    } else {
        gen_tasksets(&opt.taskgen_options)
    };

    std::fs::create_dir_all("./json")?;

    tasksets.as_ref().iter().enumerate().for_each(|(i, t)| {
        rt_app::create_config_json(
            t,
            &opt.rtapp_options,
            &format!(
                "./json/Config{}_{}t_{:.1}u.json",
                i, opt.taskgen_options.n, opt.taskgen_options.util as f32
            ),
        )
    });

    Ok(())
}
