#[derive(Debug, Clone, PartialEq, Eq)]
enum Priority {
	Low,
	Medium,
	High,
}

#[derive(Debug, Clone)]
struct Task {
    name: String,
    priority: Priority,
}

impl Task {
    fn new(name: &str, priority: Priority) -> Self {
        Task {
            name: name.to_string(),
            priority,
        }
    }
}


struct PriorityIterator<'a> {
    tasks: &'a [Task],
    current_priority: Priority, 
    index: usize,
}

impl<'a> PriorityIterator<'a> {
    fn new(tasks: &'a [Task], current_priority: Priority) -> Self {
        Self {
            tasks, current_priority, index: 0,
        }
    }
}

impl<'a> Iterator for PriorityIterator<'a> {
    type Item = &'a Task;
    fn next(&mut self) -> Option<Self::Item> {
    
        loop {
            if let Some(pos) = self.tasks[self.index..].iter().position(|task| task.priority == self.current_priority) {
                self.index += pos + 1;
               return Some(&self.tasks[self.index - 1])
                
            } else {
                self.current_priority = match self.current_priority {
                            Priority::High => Priority::Medium,
                            Priority::Medium => Priority::Low,
                            Priority::Low => return None, // No more priorities left
                        };
                self.index = 0;        
            }
        }
    }
}


fn execute_task(task: &Task) {
    println!("Executing {:?}: {:?}", task.name, task.priority);
}

/*
fn execute_tasks_by_priority(all_tasks: &[Task], start_priority: Priority) {

    let mut current_priority = start_priority;
    let mut index = 0;
    loop {
        if let Some(pos) = all_tasks[index..].iter().position(|task| task.priority == current_priority) {
            index += pos + 1;
            execute_task(&all_tasks[index - 1]);
            
        } else {
            current_priority = match current_priority {
                        Priority::High => Priority::Medium,
                        Priority::Medium => Priority::Low,
                        Priority::Low => break, // No more priorities left
                    };
            index =0;        
        }
    }
}
*/

fn main() {
    let all_tasks = vec![
        Task::new("Laundry", Priority::Low),
        Task::new("Emails", Priority::High),
        Task::new("Homework", Priority::Medium),
        Task::new("Cleaning", Priority::Medium),
        Task::new("Taxes", Priority::High),
    ];
    
    println!("Execute tasks by priority!");
   
    let prioritized_tasks = PriorityIterator::new(&all_tasks, Priority::High);

    for task in prioritized_tasks {
        execute_task(task);
    }
 
}



