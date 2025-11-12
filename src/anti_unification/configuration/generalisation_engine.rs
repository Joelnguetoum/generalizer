use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use crossbeam::queue::SegQueue;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generaliser::generaliser::Generaliser;
use crate::anti_unification::rules::rule::Rule;
use crate::terms::term::Term;


//Work in progress
#[allow(dead_code)]
pub struct GeneralisationEngine {
    pub results: Arc<Mutex<Vec<Configuration>>>,
    pub unsolved_configurations: Arc<SegQueue<Configuration>>,
}
#[allow(dead_code)]
impl GeneralisationEngine {
    pub fn init_engine(t1: &Term, t2: &Term) -> Self {
        let init_conf = Configuration::init_conf(t1, t2);
        let queue = Arc::new(SegQueue::new());
        queue.push(init_conf);

        Self {
            results: Arc::new(Mutex::new(Vec::new())),
            unsolved_configurations: queue,
        }
    }

    pub fn run(&mut self, is_constrained_anti_unification: bool, alpuente: bool,verbose:bool) {
        let n_threads = 1; // number of worker threads
        let mut handles = Vec::new();
        let stop_flag = Arc::new(AtomicBool::new(false));

        for _ in 0..n_threads {
            let q = Arc::clone(&self.unsolved_configurations);
            let results = Arc::clone(&self.results);
            let stop = Arc::clone(&stop_flag);

            let handle = thread::spawn(move || {
                while let Some(cfg) = q.pop() {
                    if stop.load(Ordering::Relaxed) {
                        if verbose {
                            println!("thread stopped");
                        }

                        break; // exit thread immediately if stop flag is set
                    }

                    if cfg.active.is_empty() {
                        results.lock().unwrap().push(cfg.clone());

                        if is_constrained_anti_unification {
                            if verbose {
                                println!("Stop signal sent");
                            }

                            stop.store(true, Ordering::Relaxed); // signal all threads to stop
                            break;
                        }
                        continue;
                    }

                    let rules = cfg.get_applicable_rules_first_aut(is_constrained_anti_unification, alpuente);

                    // skip SolveFail configs
                    if rules.contains(&Rule::SolveFail) {
                        continue;
                    }

                    for rule in rules {
                        if verbose {
                            println!("{:?}",rule);
                            println!("{}",cfg);
                        }

                        if stop.load(Ordering::Relaxed) {

                            break; // exit early if stop signal
                        }


                        if let Ok(res_confs) = cfg.apply_rule(rule) {
                            for conf in res_confs {
                                q.push(conf);
                            }
                        }
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all threads to complete
        for h in handles {
            h.join().unwrap();
        }
    }

    pub fn to_generalisers(&self) -> Vec<Generaliser> {
        let results = self.results.lock().unwrap();
        results.iter().map(|c| c.to_generaliser()).collect()
    }
}
