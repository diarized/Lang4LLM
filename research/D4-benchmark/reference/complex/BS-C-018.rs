#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Processor {
    history: Vec<String>,
    redo_stack: Vec<String>,
    max_history: i64,
    counter: i64,
    prev_values: Vec<i64>,
}

fn proc_new(max_h: i64) -> Processor {
    let h: Vec<String> = vec![];
    let r: Vec<String> = vec![];
    let pv: Vec<i64> = vec![];
    return Processor { history: h, redo_stack: r, max_history: max_h, counter: 0_i64, prev_values: pv };
}

fn apply_command(proc: Processor, cmd: String) -> Processor {
    let parts: Vec<String> = cmd.split(":".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
    let kind: String = parts[(0_i64) as usize].clone();
    let val: i64 = { let __s = format!("{}", parts[(1_i64) as usize].clone()); __s.parse::<i64>().unwrap_or(0) };
    let mut new_counter: i64 = proc.counter.clone();
    if kind == "add".to_string() {
        new_counter = proc.counter + val;
    };
    if kind == "multiply".to_string() {
        new_counter = proc.counter * val;
    };
    if kind == "set".to_string() {
        new_counter = val;
    };
    return Processor { history: proc.history, redo_stack: proc.redo_stack, max_history: proc.max_history, counter: new_counter, prev_values: proc.prev_values };
}

fn proc_execute(proc: Processor, cmd: String) -> Processor {
    let old_val: i64 = proc.counter.clone();
    let proc2: Processor = apply_command(proc.clone(), cmd.clone());
    let mut new_h: Vec<String> = proc2.history.clone();
    let mut new_pv: Vec<i64> = proc2.prev_values.clone();
    new_h.push(cmd.clone());
    new_pv.push(old_val.clone());
    if (new_h.len() as i64) > proc2.max_history {
        let mut trimmed_h: Vec<String> = vec![];
        let mut trimmed_pv: Vec<i64> = vec![];
        let mut i: i64 = 1_i64;
        while i < (new_h.len() as i64) {
            trimmed_h.push(new_h[(i) as usize].clone());
            trimmed_pv.push(new_pv[(i) as usize].clone());
            i = i + 1_i64;
        }
        return Processor { history: trimmed_h, redo_stack: vec![], max_history: proc2.max_history, counter: proc2.counter, prev_values: trimmed_pv };
    };
    return Processor { history: new_h, redo_stack: vec![], max_history: proc2.max_history, counter: proc2.counter, prev_values: new_pv };
}

fn proc_undo(proc: Processor) -> Processor {
    if proc.history.is_empty() {
        return proc;
    };
    let last_idx: i64 = (proc.history.len() as i64) - 1_i64;
    let last_cmd: String = proc.history[(last_idx) as usize].clone();
    let prev_val: i64 = proc.prev_values[(last_idx) as usize].clone();
    let mut new_h: Vec<String> = vec![];
    let mut new_pv: Vec<i64> = vec![];
    let mut i: i64 = 0_i64;
    while i < last_idx {
        new_h.push(proc.history[(i) as usize].clone());
        new_pv.push(proc.prev_values[(i) as usize].clone());
        i = i + 1_i64;
    }
    let mut new_redo: Vec<String> = proc.redo_stack.clone();
    new_redo.push(format!("{}:{}", last_cmd, format!("{}", prev_val)));
    return Processor { history: new_h, redo_stack: new_redo, max_history: proc.max_history, counter: prev_val, prev_values: new_pv };
}

fn proc_redo(proc: Processor) -> Processor {
    if proc.redo_stack.is_empty() {
        return proc;
    };
    let redo_idx: i64 = (proc.redo_stack.len() as i64) - 1_i64;
    let redo_entry: String = proc.redo_stack[(redo_idx) as usize].clone();
    let mut new_redo: Vec<String> = vec![];
    let mut ri: i64 = 0_i64;
    while ri < redo_idx {
        new_redo.push(proc.redo_stack[(ri) as usize].clone());
        ri = ri + 1_i64;
    }
    let parts: Vec<String> = redo_entry.split(":".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
    let cmd: String = format!("{}:{}", parts[(0_i64) as usize].clone(), parts[(1_i64) as usize].clone());
    let old_val: i64 = proc.counter.clone();
    let proc2: Processor = apply_command(proc.clone(), cmd.clone());
    let mut new_h: Vec<String> = proc2.history.clone();
    let mut new_pv: Vec<i64> = proc2.prev_values.clone();
    new_h.push(cmd.clone());
    new_pv.push(old_val.clone());
    return Processor { history: new_h, redo_stack: new_redo, max_history: proc2.max_history, counter: proc2.counter, prev_values: new_pv };
}

fn proc_history_size(proc: Processor) -> i64 {
    return (proc.history.len() as i64);
}

fn proc_redo_size(proc: Processor) -> i64 {
    return (proc.redo_stack.len() as i64);
}

fn print_state(proc: Processor) {
    println!("{}", format!("counter={} hist={} redo={}", format!("{}", proc.counter), format!("{}", proc_history_size(proc.clone())), format!("{}", proc_redo_size(proc.clone()))));
}

fn main() {
    let mut p1: Processor = proc_new(10_i64);
    p1 = proc_execute(p1.clone(), "add:10".to_string());
    print_state(p1.clone());
    p1 = proc_undo(p1.clone());
    print_state(p1.clone());
    p1 = proc_redo(p1.clone());
    print_state(p1.clone());
    let mut p2: Processor = proc_new(10_i64);
    p2 = proc_undo(p2.clone());
    println!("{}", format!("undo empty: counter={}", format!("{}", p2.counter)));
    let mut p3: Processor = proc_new(10_i64);
    p3 = proc_execute(p3.clone(), "add:1".to_string());
    p3 = proc_execute(p3.clone(), "add:1".to_string());
    p3 = proc_undo(p3.clone());
    p3 = proc_execute(p3.clone(), "add:5".to_string());
    println!("{}", format!("after new exec: counter={} redo={}", format!("{}", p3.counter), format!("{}", proc_redo_size(p3.clone()))));
    let mut p4: Processor = proc_new(10_i64);
    p4 = proc_execute(p4.clone(), "add:1".to_string());
    p4 = proc_execute(p4.clone(), "add:2".to_string());
    p4 = proc_execute(p4.clone(), "add:3".to_string());
    p4 = proc_undo(p4.clone());
    p4 = proc_undo(p4.clone());
    println!("{}", format!("after 2 undos: counter={}", format!("{}", p4.counter)));
    p4 = proc_redo(p4.clone());
    p4 = proc_redo(p4.clone());
    p4 = proc_redo(p4.clone());
    println!("{}", format!("after 3 redos: counter={}", format!("{}", p4.counter)));
    let mut p5: Processor = proc_new(3_i64);
    p5 = proc_execute(p5.clone(), "add:1".to_string());
    p5 = proc_execute(p5.clone(), "add:2".to_string());
    p5 = proc_execute(p5.clone(), "add:3".to_string());
    p5 = proc_execute(p5.clone(), "add:4".to_string());
    p5 = proc_execute(p5.clone(), "add:5".to_string());
    println!("{}", format!("hist_size after 5 cmds (max 3): {}", format!("{}", proc_history_size(p5.clone()))));
    println!("{}", format!("counter after 5 adds: {}", format!("{}", p5.counter)));
    p5 = proc_undo(p5.clone());
    p5 = proc_undo(p5.clone());
    p5 = proc_undo(p5.clone());
    p5 = proc_undo(p5.clone());
    println!("{}", format!("counter after 4 undos (only 3 possible): {}", format!("{}", p5.counter)));
    let mut p6: Processor = proc_new(10_i64);
    p6 = proc_execute(p6.clone(), "add:10".to_string());
    p6 = proc_execute(p6.clone(), "set:100".to_string());
    println!("{}", format!("after set:100: counter={}", format!("{}", p6.counter)));
    p6 = proc_undo(p6.clone());
    println!("{}", format!("after undo set: counter={}", format!("{}", p6.counter)));
}

