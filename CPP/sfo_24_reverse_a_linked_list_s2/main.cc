#include "../include/include.hpp"

class Solution {
public:
    ListNode* reverseList( ListNode* head ) {
        return recur( head, NULL );
    }

    ListNode* recur( ListNode* cur, ListNode* pre ) {
        if ( cur == NULL ) return pre;
        ListNode* res = recur( cur->next, cur );
        cur->next     = pre;
        return res;
    }
};

int main( int argc, char* argv[] ) {
    ListNode* head = vectorToLinkedList( vector<int>{ 1, 2, 3, 4, 5 } );
    Solution* stl  = new Solution();
    ListNode* res  = stl->reverseList( head );
    PrintUtil::printLinkedList( res );
}
