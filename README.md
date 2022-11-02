# Key Value Event Driven State Machine

This shows how to implement an [Event Driven State Machine](https://github.com/titanclass/edfsm) that manages a Key Value store.  
We call this a KV State Machine or _KVSM_.

All updates to the KV store are implemented as EDFSM commands. So `Insert(k, v)` adds a new value `v` with key `k`. `Delete(k)` deletes it.  

Things get more interesting with updates. We do not supply new value `v1` to replace an existing `v0`. 
Instead, each value in the store is controlled by its own state machine logic.  

The command `Execute(k, c)` delivers a sub-command `c` to the value with key `k`. 
Then: `(e, v1) = step(v0, c, h)` where `e` is an event to be logged and side effects can be produced via
the handler `h`.

In other words, a KVSM decorates another EDFSM that controls its values.  The Take a look at the code.

## Limitations

The implementation uses a `std` rust `HashMap` which is cloned for each update.  A functional data structure would be more efficient.
Alternatively, interior mutability could be used provided the EDFSM `on_exit` hook is not required. 
