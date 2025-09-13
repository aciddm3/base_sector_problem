use std::fs::File;
use std::io::Read;
use serde_json;
use serde::Deserialize;

type SectorID = u8;

#[derive(Deserialize, Debug)]
struct Sector {
    id : SectorID,
    relations : Vec<SectorID>,
    is_based : bool,
    cost : f64
}

fn total_cost (g : & Vec<Sector> ) -> f64 {
    let mut s = 0.0;
    for i in g {
        if i.is_based {
            s += i.cost
        }
    };
    s
}

fn add_if_not_present<T: PartialEq>(vec: &mut Vec<T>, item: T) {
    if !vec.contains(&item) {
        vec.push(item);
    }
}

fn generate_bool_combinations(n: usize) -> Vec<Vec<bool>> {
    // Общее количество комбинаций равно 2 в степени n.
    // Используем `2_u64.pow(n as u32)` для вычисления.
    let num_combinations = 2_u64.pow(n as u32);
    
    let mut all_combinations = Vec::with_capacity(num_combinations as usize);
    
    // Итерируемся по каждому числу от 0 до 2^n - 1
    for i in 0..num_combinations {
        let mut current_combination = Vec::with_capacity(n);
        let mut temp_i = i;
        
        // Преобразуем число `i` в двоичное представление
        for _ in 0..n {
            // Если остаток от деления на 2 равен 1, то это `true`, иначе `false`
            current_combination.push(temp_i % 2 == 1);
            temp_i /= 2; // Переходим к следующему биту
        }
        
        // Нам нужен обратный порядок (младший бит в конце)
        current_combination.reverse(); 
        
        all_combinations.push(current_combination);
    }
    
    all_combinations
}

fn main() {
    let mut file = File::open("graph.json").expect("Не удалось открыть файл с графом");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("пЕчЕнььКО))!");
    let mut a: Vec<Sector> = serde_json::from_str(&data).expect("Не удалось распарсить файл"); 
    
    let task_order = a.len().clone();

    let mut minimum_cost: f64 = a.iter().map(|f| f.cost).sum();
    
    let mut benefited : Vec<SectorID> = vec![0; 0];
    
    let mut minimum_conf : Vec<bool> = vec![true; a.len()];

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
        println!("Конфигурация: {:?}", i.iter().map(|&s| {if s {1} else {0}}).collect::<Vec<_>>());
        if benefited.len() != task_order {
            println!("Эта конфигурация контролирует не все сектора!");
        } else {
            let cost = total_cost(&a);
            println!("Эта конфигурация подходит, значение целевой функции: {cost}");
            if minimum_cost > cost {
                minimum_cost = cost;
                minimum_conf = i.clone();
            }
        }
        println!();
    }
    println!("Лучшая конфигурация {:?}", minimum_conf.iter().map(|&s| {if s {1} else {0}}).collect::<Vec<_>>());
    println!("Потребляет {minimum_cost}");

}
