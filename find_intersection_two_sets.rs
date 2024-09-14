fn intersect_two_sets(short_set: &[i32], long_set: &[i32]) -> Vec<i32> {
    // Создаём массив фиксированного размера для хранения результатов
    let mut result = vec![0; short_set.len()];
    let mut count = 0; // Счётчик найденных пересечений

    // Проходим по каждому элементу короткого множества
    for i in 0..short_set.len() {
         // Сравниваем его с каждым элементом длинного множества
        for j in 0..long_set.len() {
            // Если элементы совпадают
            if short_set[i] == long_set[j] {
                result[count] = short_set[i]; // Записываем совпадающий элемент в результат
                count += 1; // Увеличиваем счётчик
                break; // Выходим из внутреннего цикла, так как совпадение найдено
            }
        }
    }
    
    // Удаляем пустые элементы и выводим результат
    result.truncate(count);
    result  
}

// Функция для нахождения пересечения двух множеств
fn find_intersection_two_sets(set1: &[i32], set2: &[i32]) -> Vec<i32> {
    // Находим короткое множество и вызываем функцию
    if set1.len() <= set2.len() {
        intersect_two_sets(set1, set2)
    } else {
        intersect_two_sets(set2, set1)
    }
}

fn main() {
    let set1 = vec![1, 2, 3, 4];
    let set2 = vec![3, 4, 5, 6, 7];

    // Находим пересечение множеств
    let intersection = find_intersection_two_sets(&set1, &set2);

    // Выводим результат
    println!("Пересечение: {:?}", intersection);
}
