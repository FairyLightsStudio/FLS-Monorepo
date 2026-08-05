package main

import "fmt"

// 定义状态
type State string

const (
	StateClosed State = "closed"
	StateOpen   State = "open"
	StateLocked State = "locked"
)

// 定义事件
type Event string

const (
	EventOpen   Event = "open"
	EventClose  Event = "close"
	EventLock   Event = "lock"
	EventUnlock Event = "unlock"
)

type FSM struct {
	currentState State
	transitions  map[State]map[Event]State
}

func NewFSM(initialState State) *FSM {
	return &FSM{
		currentState: initialState,
		transitions:  make(map[State]map[Event]State),
	}
}

// 添加状态转换规则
func (f *FSM) AddTransition(state State, event Event, nextState State) {
	if f.transitions[state] == nil {
		f.transitions[state] = make(map[Event]State)
	}
	f.transitions[state][event] = nextState
}

// 处理事件
func (f *FSM) HandleEvent(event Event) {
	if nextState, ok := f.transitions[f.currentState][event]; ok {
		fmt.Printf("Transitioning from %s to %s on event %s\n", f.currentState, nextState, event)
		f.currentState = nextState
	} else {
		fmt.Printf("No transition from %s on event %s\n", f.currentState, event)
	}
}

// 获取当前状态
func (f *FSM) CurrentState() State {
	return f.currentState
}

func main() {
	fsm := NewFSM(StateClosed)

	// 定义状态转换规则
	fsm.AddTransition(StateClosed, EventOpen, StateOpen)
	fsm.AddTransition(StateOpen, EventClose, StateClosed)
	fsm.AddTransition(StateClosed, EventLock, StateLocked)
	fsm.AddTransition(StateLocked, EventUnlock, StateClosed)

	// 处理事件
	fsm.HandleEvent(EventOpen)   // Transitioning from closed to open on event open
	fsm.HandleEvent(EventClose)  // Transitioning from open to closed on event close
	fsm.HandleEvent(EventLock)   // Transitioning from closed to locked on event lock
	fsm.HandleEvent(EventUnlock) // Transitioning from locked to closed on event unlock

	fmt.Printf("Current state: %s\n", fsm.CurrentState()) // Current state: closed
}
