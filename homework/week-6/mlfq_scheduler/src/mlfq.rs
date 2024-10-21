// src/mlfq.rs
use std::cmp::min;
#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        // TODO: Implement this function
        // Add the process to the appropriate queue based on its priority
        // Ensure the priority is within the valid range (0 to num_levels - 1)

        // Iris's Logic - we first check that the priority exists. The priority must 
        // be greater than or equal to 0 and less than the number of levels. Otherwise 
        // we do not not add the process.
        if process.priority >= 0 && process.priority < self.num_levels {
            self.queues[process.priority].push(process);
        }
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // TODO: Implement this function
        // Execute the process for its time quantum or until completion
        

        let mut current_process = self.queues[queue_index][0].clone();

        // Update remaining_time, total_executed_time, and current_time

        // Iris's logic, we first need to get the time quanta for the queue, then
        // we need to determine which is shorter, the remining time the process has to run
        // or the time quanta. For which ever is less, we run it for that time. 
        // if the process still requires more running time, then we move the process
        // to a lesser priority queue, just one less. Then we update the priority

        let time_quanta_for_queue = self.time_quanta[queue_index];
        let time_process_ran_for = min(time_quanta_for_queue, current_process.remaining_time);
        current_process.total_executed_time += time_process_ran_for;
        current_process.remaining_time -= time_process_ran_for;
        //self.current_time += time_process_ran_for;
        self.update_time(time_process_ran_for);

        if current_process.remaining_time > 0 {
            // Move the process to a lower priority queue if it doesn't complete
            
            if queue_index < self.num_levels - 1  {
                current_process.priority = queue_index + 1;
                self.queues[queue_index].remove(0);
                self.queues[queue_index + 1].push(current_process);
                
            }

        }
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // TODO: Implement this function
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0

        // Iris's logic, we go through each queue then we drain all the processes of that queue
        // and push them to the first priority queue, level 0
        // before adding the process to queue 0, we update its prority
        for priority_queue_i in 1..self.num_levels {
            let drained_processes: Vec<_> = self.queues[priority_queue_i].drain(..).collect();
    
            for mut process in drained_processes {
                process.priority = 0;  // Reset the priority to 0
                self.queues[0].push(process);
            }
        }
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}