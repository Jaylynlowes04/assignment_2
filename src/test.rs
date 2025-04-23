mod test {
    use crate::DynamicLinkedList;
    use crate::StaticLinkedList;

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

    #[test]
    fn should_update_element_at_index() {
        let mut list = DynamicLinkedList::new();
        list.insert(5);
        list.insert(10);
        assert!(list.update_element_at_index(1, 15));
        assert_eq!(list.get(1), Some(15));
        assert!(!list.update_element_at_index(5, 99));
    }

    #[test]
    fn test_find() {
        let mut list = DynamicLinkedList::new();
        list.insert(7);
        list.insert(14);
        list.insert(21);
        assert!(list.find(14));
        assert!(!list.find(100));
    }

    #[test] 
    fn should_static_insert_and_get() {
        let mut list = StaticLinkedList::new();
        list.insert("apple".to_string());
        list.insert("banana".to_string());
        list.insert("cherry".to_string());

        assert_eq!(list.get(0), Some("apple".to_string()));
        assert_eq!(list.get(1), Some("banana".to_string()));
        assert_eq!(list.get(2), Some("cherry".to_string()));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn should_static_delete_element() {
        let mut list = StaticLinkedList::new();
        list.insert("a".to_string());
        list.insert("b".to_string());
        list.insert("c".to_string());

        assert!(list.delete_element("b".to_string()));
        assert_eq!(list.get(0), Some("a".to_string()));
        assert_eq!(list.get(1), Some("c".to_string()));
        assert!(!list.delete_element("x".to_string()));
    }
}