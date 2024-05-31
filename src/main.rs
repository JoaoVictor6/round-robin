fn main() {
    // process id's
    let processes = vec![1, 2, 3];
    let processes_length = processes.len();

    // Burst time of all processes
    let bt = vec![10, 5, 8];

    // Time quantum
    let quantum = 2;

    find_average_time(processes, processes_length, bt, quantum);
}

// Method to find the waiting time
// for all processes
fn find_waiting_time(
    processes: &Vec<usize>,
    burst_time: &Vec<usize>,
    waiting_time: &mut Vec<usize>,
    quantum: usize,
) {
    let processes_length = processes.len();
    // Make a copy of burst times bt[] to store remaining burst times.
    let mut rem_bt = vec![0; processes_length];
    for idx in 0..processes_length {
        rem_bt[idx] = burst_time[idx];
    }

    let mut t = 0; // current time

    // Keep traversing processes in round robin manner until all of them are not done.
    loop {
        let mut done = true;

        // Traverse all processes one by one repeatedly
        for idx in 0..processes_length {
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
                    waiting_time[idx] = t - burst_time[idx];

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
    burst_time: &Vec<usize>,
    waiting_time: &Vec<usize>,
    turnaround_time: &mut Vec<usize>,
) {
    let processes_length = processes.len();
    // calculating turnaround time by adding
    // bt[idx] + wt[idx]
    for idx in 0..processes_length {
        turnaround_time[idx] = burst_time[idx] + waiting_time[idx];
    }
}

// Function to calculate average time
fn find_average_time(
    processes: Vec<usize>,
    processes_length: usize,
    burst_time: Vec<usize>,
    quantum: usize,
) {
    let mut waiting_time = vec![0; processes_length];
    let mut turnaround_time = vec![0; processes_length];
    let mut total_waiting_time = 0;
    let mut total_turnaround_time = 0;

    // Function to find waiting time of all processes
    find_waiting_time(&processes, &burst_time, &mut waiting_time, quantum);

    // Function to find turn around time of all processes
    find_turn_around_time(&processes, &burst_time, &waiting_time, &mut turnaround_time);

    // Display processes along with all details
    println!("Processes  Burst time  Waiting time  Turn around time");

    for idx in 0..processes_length {
        total_waiting_time += waiting_time[idx];
        total_turnaround_time += turnaround_time[idx];

        println!(
            " {}\t\t{}\t {}\t\t {}",
            processes[idx], burst_time[idx], waiting_time[idx], turnaround_time[idx]
        );
    }

    println!(
        "\nAverage waiting time = {}",
        total_waiting_time as f64 / processes_length as f64
    );
    println!(
        "Average turn around time = {}",
        total_turnaround_time as f64 / processes_length as f64
    );
}
