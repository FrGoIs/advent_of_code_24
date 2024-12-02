pub fn get_distance() -> (i32, i32) {
    let input = include_str!("./inputs.txt");
    let (mut list1, mut list2) = (Vec::new(), Vec::new());
    let mut distance = 0;
    input.lines().collect::<Vec<&str>>().into_iter().for_each(|s| {
        let temp = s.split("   ").collect::<Vec<&str>>();
        list1.push(temp[0].parse::<i32>().unwrap());
        list2.push(temp[1].parse::<i32>().unwrap());
    });

    list1.sort();
    list2.sort();
    for i in 0..list1.len() {
        distance += (list1[i] - list2[i]).abs();
    }
    let similarity = get_similarity(&list1, &list2);
    (distance, similarity)
}

pub fn get_similarity(list1: &[i32], list2: &[i32]) -> i32 {
    let mut similarity_score = 0;

    // list1 y list2 ya están ordenados
    list1.into_iter().for_each(|n| {
        let similar = list2.into_iter().filter(|&m| m == n).collect::<Vec<&i32>>();
        similarity_score += n * similar.len() as i32;
    });

    similarity_score
}