#include "../include/include.hpp"

// ===== Solution Code =====
class Solution {
public:
    vector<int> reversePrint( ListNode* head ) {
        while ( head != NULL ) {
            s1.push( head->val );
            head = head->next;
        }
        vector<int> res;
        while ( !s1.empty() ) {
            res.push_back( s1.top() );
            s1.pop();
        }
        return res;
    }

private:
    stack<int> s1;
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
