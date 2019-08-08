fn main() {
    let groups = aws::logs::describe_log_groups("/prod-bj/ops");

    println!("{:?}", groups);
}
