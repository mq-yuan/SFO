#include "../include/include.hpp"
#include <vector>

class MinStack {
public:
    stack<int> stack1;
    stack<int> stack2;
    /** initialize your data structure here. */
    MinStack() {}

    void push( int x ) {
        stack1.push( x );
        if ( stack2.empty() || x <= stack2.top() ) { stack2.push( x ); }
    }

    void pop() {
        int tmp = stack1.top();
        stack1.pop();
        if ( tmp == stack2.top() ) { stack2.pop(); }
    }

    int top() {
        return stack1.top();
    }

    int min() {
        return stack2.top();
    }
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack* obj = new MinStack();
 * obj->push(x);
 * obj->pop();
 * int param_3 = obj->top();
 * int param_4 = obj->min();
 */
int main( int argc, char* argv[] ) {
    vector<int> res;
    MinStack*   obj = new MinStack();
    obj->push( -2 );
    obj->push( 0 );
    obj->push( -3 );
    res.push_back( obj->min() );
    obj->pop();
    res.push_back( obj->top() );
    res.push_back( obj->min() );
    PrintUtil::printVector( res );
}
