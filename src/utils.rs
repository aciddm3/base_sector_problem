use crate::sector;

pub fn total_cost (g : & Vec<sector::Sector> ) -> f64 {
    let mut s = 0.0;
    for i in g {
        if i.is_based {
            s += i.cost
        }
    };
    s
}

pub fn add_if_not_present<T: PartialEq>(vec: &mut Vec<T>, item: T) {
    if !vec.contains(&item) {
        vec.push(item);
    }
}

pub fn generate_bool_combinations(n: usize) -> Vec<Vec<bool>> {
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