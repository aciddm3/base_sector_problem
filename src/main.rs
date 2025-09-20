use std::{fs::File, time::Instant};
use std::io::Read;

mod sector;
mod utils;
mod dot_generator;

use sector::{Sector, SectorID};
use utils::*;

use crate::dot_generator::generate;
fn main() {
    let start_time = Instant::now();
    let mut file = File::open("graph.json").expect("Не удалось открыть файл с графом");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("пЕчЕнььКО))!");
    let mut a: Vec<Sector> = serde_json::from_str(&data).expect("Не удалось распарсить файл"); 
    
    let task_order = a.len();

    let mut minimum_cost: f64 = a.iter().map(|f| f.cost).sum();
    
    let mut minimum_conf : Vec<bool> = vec![true; a.len()];
    
    let mut benefited : Vec<SectorID>;

    for i in generate_bool_combinations(task_order) {
        for j in 0..task_order {
            a[j].is_based = i[j];
        }
        benefited = vec![0;0];
        for j in a.iter() {
            if j.is_based {
                add_if_not_present(&mut benefited, j.id);
                for k in &j.relations {
                    add_if_not_present(&mut benefited, *k);
                }
            }
        }
        //println!("Конфигурация: {:?}", i.iter().map(|&s| {if s {1} else {0}}).collect::<Vec<_>>());
        if benefited.len() != task_order {
            //println!("Эта конфигурация контролирует не все сектора!");
        } else {
            let cost = total_cost(&a);
            //println!("Эта конфигурация подходит, значение целевой функции: {cost}");
            if minimum_cost > cost {
                minimum_cost = cost;
                minimum_conf = i.clone();
            }
        }
        //println!();
    }
    println!("//Лучшая конфигурация {:?}", minimum_conf.iter().map(|&s| {if s {1} else {0}}).collect::<Vec<_>>());
    println!("//Потребляет {minimum_cost}");
    println!("//dot формат для этой конфигурации:\n");

    for (index, val) in minimum_conf.into_iter().enumerate() {
        a[index].is_based = val
    }
    let before_dot = start_time.elapsed();
    println!("{}", generate(&a));

    println!("//Время работы без генерфции DOT кода: {} микросекунд", before_dot.as_micros());
    println!("//Общее время работы: {} микросекунд", start_time.elapsed().as_micros());

}
