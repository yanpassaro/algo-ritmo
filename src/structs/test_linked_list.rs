#[cfg(test)]
mod tests {
    use crate::structs::linked_list::LinkedList;

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.head.unwrap().data, 3);
    }

    #[test]
    fn test_print() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        // Como a função print() não retorna nada, podemos testar se ela executa sem pânico
        let result = std::panic::catch_unwind(|| list.print());
        assert!(result.is_ok());
    }
}
