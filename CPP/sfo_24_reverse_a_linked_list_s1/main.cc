#include "../include/include.hpp"
#include <cstddef>

class Solution {
public:
    ListNode* reverse_a_linked_list( ListNode* head ) {
        ListNode* res = NULL;
        ListNode* current;
        while ( head ) {
            current       = head;
            head          = head->next;
            current->next = res;
            res           = current;
        }
        return res;
    }
};

int main() {
    // ======= Test Case =======
    ListNode* head = vectorToLinkedList( vector<int>{ 1, 2, 3, 4, 5 } );
    // ====== Driver Code ======
    Solution* slt = new Solution();
    ListNode* res = slt->reverse_a_linked_list( head );
    PrintUtil::printLinkedList( res );

    return 0;
}
