use priority_queue::PriorityQueue;

fn main() {
    let mut pq = PriorityQueue::new();
    assert!(pq.is_empty());

    pq.push("Apples", 5);
    pq.push("Bananas", 8);
    pq.push("Strawberries", 23);
    assert_eq!(pq.peek(), Some((&"Strawberries", &23)));

    for (item, priority) in pq.into_sorted_iter() {
        println!("{} {}", priority, item);
    }
}