use distributed_list::push;
use distributed_list::DistributedList;

static REGISTRY: DistributedList<usize> = DistributedList::root();

push!(REGISTRY, ONE: usize = 1);
push!(REGISTRY, TWO: usize = 2);
push!(REGISTRY, THREE: usize = 3);

#[test]
fn check_elements() {
    let mut elements = REGISTRY.iter().copied().collect::<Vec<_>>();
    elements.sort();
    assert_eq!(elements, vec![1, 2, 3]);
}
