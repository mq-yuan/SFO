#include "../include/include.hpp"

class CQueue {
public:
    stack<int> s1, s2;
    CQueue() {}

    void appendTail( int value ) {
        s1.push( value );
    }

    int deleteHead() {
        if ( !s2.empty() ) {
            int ans = s2.top();
            s2.pop();
            return ans;
        }

        if ( s1.empty() ) { return -1; }

        while ( !s1.empty() ) {
            s2.push( s1.top() );
            s1.pop();
        }

        int ans = s2.top();
        s2.pop();
        return ans;
    }
};

/**
 * Your CQueue object will be instantiated and called as such:
 * CQueue* obj = new CQueue();
 * obj->appendTail(value);
 * int param_2 = obj->deleteHead();
 */

int main() {
    // ====== Driver Code ======
    vector<int> res;
    CQueue*     cQueue = new CQueue();
    res.push_back( cQueue->deleteHead() );
    cQueue->appendTail( 5 );
    cQueue->appendTail( 2 );
    res.push_back( cQueue->deleteHead() );
    res.push_back( cQueue->deleteHead() );
    PrintUtil::printVector( res );

    return 0;
}
