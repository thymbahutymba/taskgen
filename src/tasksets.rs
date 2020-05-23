use getset::Getters;
use ndarray::Array1;
use num::integer::Integer;
use std::iter::FromIterator;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Task {
    i: f32,
    util: f32,
    period: f32,
    c: f32,
}

impl From<&[f32; 4]> for Task {
    fn from(&[i, util, c, period]: &[f32; 4]) -> Self {
        Task { i, util, c, period }
    }
}

#[derive(Debug)]
pub struct Taskset(Vec<Task>);

impl Taskset {
    pub fn get_hyperperiod(&self) -> usize {
        self.0
            .iter()
            .map(|elem| elem.period)
            .fold(1, |hyp, p| hyp.lcm(&(p as usize)))
    }
}

impl AsRef<[Task]> for Taskset {
    fn as_ref(&self) -> &[Task] {
        &self.0
    }
}

impl From<&Vec<Array1<f32>>> for Taskset {
    fn from(t: &Vec<Array1<f32>>) -> Self {
        let mut tset: Vec<Task> = Vec::new();
        for i in 0..t[0].shape()[0] {
            tset.push(Task {
                i: t[0][i],
                util: t[1][i],
                period: t[2][i],
                c: t[3][i],
            });
        }

        Taskset(tset)
    }
}

impl<'a> FromIterator<&'a str> for Taskset {
    fn from_iter<T: IntoIterator<Item = &'a str>>(taskset_str: T) -> Self {
        Taskset(
            taskset_str
                .into_iter()
                .filter(|t| !(*t).is_empty())
                .map(|t| {
                    let task = (*t)
                        .split(" ")
                        .map(|e| e.parse().unwrap())
                        .enumerate()
                        .fold([0.0; 4], |mut acc, (i, x)| {
                            acc[i] = x;
                            acc
                        });
                    Task::from(&task)
                })
                .collect::<Vec<Task>>(),
        )
    }
}

#[derive(Debug)]
pub struct TasksetArray(Vec<Taskset>);

impl TasksetArray {
    pub fn new(ta: Vec<Taskset>) -> Self {
        TasksetArray(ta)
    }
}

impl AsRef<[Taskset]> for TasksetArray {
    fn as_ref(&self) -> &[Taskset] {
        &self.0
    }
}

impl From<&Vec<Vec<Array1<f32>>>> for TasksetArray {
    fn from(ts: &Vec<Vec<Array1<f32>>>) -> Self {
        TasksetArray(
            ts.iter()
                .map(|elem| Taskset::from(elem))
                .collect::<Vec<Taskset>>(),
        )
    }
}
