fn main() {
    // process id's
    let processes = vec![1, 2, 3];
    let n = processes.len();

    // Burst time of all processes
    let bt = vec![10, 5, 8];

    // Time quantum
    let quantum = 2;

    find_average_time(processes, n, bt, quantum);
}

// Method to find the waiting time
// for all processes
fn find_waiting_time(
    processes: &Vec<usize>,
    n: usize,
    bt: &Vec<usize>,
    wt: &mut Vec<usize>,
    quantum: usize,
) {
    // Make a copy of burst times bt[] to store remaining burst times.
    let mut rem_bt = vec![0; n];
    for idx in 0..n {
        rem_bt[idx] = bt[idx];
    }

    let mut t = 0; // current time

    // Keep traversing processes in round robin manner until all of them are not done.
    loop {
        let mut done = true;

        // Traverse all processes one by one repeatedly
        for idx in 0..n {
            // if burst time of a process is greater than 0 then only need to process further
            if rem_bt[idx] > 0 {
                // There is a pending process
                done = false;

                if rem_bt[idx] > quantum {
                    // Increase the value of t i.e;
                    // shows how much time a process has been processed
                    t += quantum;

                    // Decrease the burst_time(bt) of current process by quantum
                    rem_bt[idx] -= quantum;
                // if burst time is smaller than or equal to quantum. Last cycle for this process
                } else {
                    // Increase the value of t i.e.
                    // shows how much time a process has been processed
                    t += rem_bt[idx];

                    // Waiting time is current time minus burst time of this process
                    wt[idx] = t - bt[idx];

                    // As the process gets fully executed make its remaining
                    // burst time = 0
                    rem_bt[idx] = 0;
                }
            }
        }

        // If all process are done
        if done {
            break;
        }
    }
}

// Method to calculate turn around time
fn find_turn_around_time(
    processes: &Vec<usize>,
    n: usize,
    bt: &Vec<usize>,
    wt: &Vec<usize>,
    tat: &mut Vec<usize>,
) {
    // calculating turnaround time by adding
    // bt[idx] + wt[idx]
    for idx in 0..n {
        tat[idx] = bt[idx] + wt[idx];
    }
}

// Function to calculate average time
fn find_average_time(processes: Vec<usize>, n: usize, bt: Vec<usize>, quantum: usize) {
    let mut wt = vec![0; n];
    let mut tat = vec![0; n];
    let mut total_wt = 0;
    let mut total_tat = 0;

    // Function to find waiting time of all processes
    find_waiting_time(&processes, n, &bt, &mut wt, quantum);

    // Function to find turn around time of all processes
    find_turn_around_time(&processes, n, &bt, &wt, &mut tat);

    // Display processes along with all details
    println!("Processes  Burst time  Waiting time  Turn around time");

    for idx in 0..n {
        total_wt += wt[idx];
        total_tat += tat[idx];

        println!(
            " {}\t\t{}\t {}\t\t {}",
            processes[idx], bt[idx], wt[idx], tat[idx]
        );
    }

    println!("\nAverage waiting time = {}", total_wt as f64 / n as f64);
    println!("Average turn around time = {}", total_tat as f64 / n as f64);
}
