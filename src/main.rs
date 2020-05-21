mod opt;
mod python;
mod rt_app;
mod tasksets;

use crate::{opt::TaskgenOpt, tasksets::TasksetArray};
use structopt::StructOpt;

pub fn gen_tasksets(opt: &TaskgenOpt) -> TasksetArray {
    let mut tasksets = Vec::new();

    let (x, periods) = python::get_with_opt(&opt).unwrap();

    for (i, p) in x.into_iter().zip(periods) {
        let mut c = &i * &p;

        if opt.round_c {
            c = ndarray::arr1(
                &c.into_raw_vec()
                    .iter()
                    .map(|e| e.round())
                    .collect::<Vec<f32>>(),
            );
        }

        tasksets.push(vec![i, (&c / &p), p, c]);
    }

    (&tasksets).into()
}

fn main() -> std::io::Result<()> {
    let opt = opt::Opt::from_args();
    let tasksets = gen_tasksets(&opt.taskgen_options);

    std::fs::create_dir_all("./json")?;

    tasksets.as_ref().iter().enumerate().for_each(|(i, t)| {
        rt_app::create_config_json(t, &opt.rtapp_options, format!("./json/Config{}.json", i))
    });

    Ok(())
}
