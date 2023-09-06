#include "../include/include.hpp"

class Solution {
public:
    vector<int> reversePrint( ListNode* head ) {
        recur( head );
        return res;
    }

    void recur( ListNode* head ) {
        if ( head == NULL ) return;
        recur( head->next );
        res.push_back( head->val );
    }

private:
    vector<int> res;
};

int main() {
    // ======= Test Case =======
    ListNode* head = vectorToLinkedList( vector<int>{ 1, 3, 2 } );
    // ====== Driver Code ======
    Solution*   slt = new Solution();
    vector<int> res = slt->reversePrint( head );
    PrintUtil::printVector( res );

    return 0;
}
