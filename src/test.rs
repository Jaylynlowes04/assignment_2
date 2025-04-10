mod test {
    use crate::DynamicLinkedList;


    #[test]
    fn should_insert_and_get() {
        let mut list = DynamicLinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));
    }

    #[test]
    fn should_insert_at_index() {
        let mut list = DynamicLinkedList::new();
        list.insert(1);
        list.insert(3);
        list.insert_at_index(1, 2);

        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));
    }
}