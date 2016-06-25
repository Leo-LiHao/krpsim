// @lecorref - github.com/lecorref, @geam - github.com/geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]

extern crate test;
extern crate itertools;
extern crate krpsim;

use self::test::Bencher;

use self::itertools::Itertools;

use self::krpsim::format::stock::ressource::Ressource;
use self::krpsim::format::stock::inventory::Inventory;
use self::krpsim::format::optimize::Optimize;
use self::krpsim::format::operate::process::Process;
use self::krpsim::format::queue::Queue;
use self::krpsim::format::livep::Livep;
use self::krpsim::parser::config::Configuration;

fn get_ressources_from_process(process_list: &Vec<&Process>,
                               ressources: &mut Inventory)
                               -> () {
  process_list.iter().foreach(|process| {
    let mut add_ressource = |ressources_list: &Inventory| -> () {
      ressources_list.iter().foreach(|res| {
        if ressources.iter().find(|tmp| tmp.0 == res.0).is_none() {
          ressources.push(res.0.clone(), Ressource::new(res.0.clone(), 0));
        }
      });
    };
    add_ressource(&process.input);
    add_ressource(&process.output);
  })
}

fn get_optimized_product(opti: &Vec<String>,
                         ressources: &mut Inventory)
                         -> Option<Ressource> {
  match opti.iter().find(|s| ressources.any(s)) {
    Some(s) => {
      if let Some(x) = ressources.get(&s) {
        Some(x.clone())
      } else {
        None
      }
    }
    None => None,
  }
}

fn get_best(prcs: &Vec<(Process, Vec<Process>)>) -> Option<Vec<Process>> {
  match prcs.first() {
    Some(&(_, ref b)) => Some(b.clone()),
    None => None,
  }
}

fn krpsim(config: &mut Configuration, delay: usize) {
  let mut cycle: usize = 0usize;
  let mut done = false;
  let mut process_queue = Queue::new();

  get_ressources_from_process(&config.running.get_process(),
                              &mut config.ressources);
  let mut production: Ressource =
    match get_optimized_product(&config.optimize.stock,
                                &mut config.ressources) {
      Some(a) => a,
      None => panic!("You should optimize the production of at least one ressources!"),
    };
  production.add_quantity(1);
  // println!("{}", production);
  let final_process: Vec<Process> =
    Process::get_producing_process(&production,
                                   &config.running.get_process(),
                                   Vec::new());
  /* optimization(&mut config.process_list, &production); */
    while !done {
      let mut usable_process: Vec<(Process, Vec<Process>)> = Vec::new();

      final_process.iter().foreach(|process| {
        match process.needed_process(&config.running.get_process(),
                                     &config.ressources,
                                     final_process.clone()) {
          Err(_) => {}
          Ok((None, _)) => {
            usable_process.push((process.clone(), vec![process.clone()]))
          }
          Ok((Some(a), _)) => usable_process.push((process.clone(), a)),
        }
      });
      match get_best(&usable_process) {
        Some(a) => {
          a.iter().foreach(|process| {
            config.ressources.sub_from_inventory(&process.input);
            process_queue.add(Livep::new(process.clone(), cycle));
            println!("inventory: {}", config.ressources);
          })
        }
        None => {
          if process_queue.is_empty() {
            println!("Finished at cycle: {}", cycle);
            done = true;
          }
          match process_queue.get_ended_process(cycle) {
            None => cycle += 1,
            Some(livep_vec) => {
              livep_vec.iter().foreach(|ended_process| {
                config.ressources
                  .add_from_inventory(ended_process.destruct());
                println!("inventory: {}", config.ressources);
              });
              if cycle > delay {
                println!("Finished at cycle: {}", cycle);
                done = true;
              }
            }
          }
        }
      }
    }
}

#[bench]
fn bench_krpsim_ikea(b: &mut Bencher) {
  b.iter(|| {
    krpsim(
      &mut Configuration::new("resources/ikea").unwrap(),
      1usize,
    )
  });
}

#[bench]
fn bench_krpsim_inception(b: &mut Bencher) {
  b.iter(|| {
    krpsim(
      &mut Configuration::new("resources/inception").unwrap(),
      1usize,
    )
  });
}

/*#[bench]
fn bench_krpsim_pomme(b: &mut Bencher) {
  b.iter(|| {
    krpsim(
      &mut Configuration::new("resources/pomme").unwrap(),
      1usize,
    )
  });
}*/

#[bench]
fn bench_krpsim_recre(b: &mut Bencher) {
  b.iter(|| {
    krpsim(
      &mut Configuration::new("resources/recre").unwrap(),
      1usize,
    )
  });
}

#[bench]
fn bench_krpsim_simple(b: &mut Bencher) {
  b.iter(|| {
    krpsim(
      &mut Configuration::new("resources/simple").unwrap(),
      1usize,
    )
  });
}

#[bench]
fn bench_krpsim_steak(b: &mut Bencher) {
  b.iter(|| {
    krpsim(
      &mut Configuration::new("resources/steak").unwrap(),
      1usize,
    )
  });
}
