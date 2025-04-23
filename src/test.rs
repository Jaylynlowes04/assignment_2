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

    #[test]
    fn should_delete_element() {
        let mut list = DynamicLinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);

        assert!(list.delete_element(2));
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(3));
        assert!(!list.delete_element(99));
    }

    #[test]
    fn should_delete_at_index() {
        let mut list = DynamicLinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        assert!(list.delete_at_index(1));
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(3));
        assert!(!list.delete_at_index(5));
    }

    #[test] 
    fn should_update_element() {
        let mut list = DynamicLinkedList::new();
        list.insert(100);
        list.insert(200);
        assert!(list.update_element(100, 150));
        assert_eq!(list.get(0), Some(150));
        assert!(!list.update_element(999, 111));
    }
}