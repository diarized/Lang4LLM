#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Subject {
    update_observers: Vec<String>,
    tick_observers: Vec<String>,
    log: Vec<String>,
}

fn subject_new() -> Subject {
    let uo: Vec<String> = vec![];
    let to_obs: Vec<String> = vec![];
    let lg: Vec<String> = vec![];
    return Subject { update_observers: uo, tick_observers: to_obs, log: lg };
}

fn subscribe(subj: Subject, event: String, obs: String) {
    if event == "update".to_string() {
        subj.update_observers.push(obs.clone());
    };
    if event == "tick".to_string() {
        subj.tick_observers.push(obs.clone());
    };
}

fn unsubscribe(subj: Subject, event: String, obs: String) {
    if event == "update".to_string() {
        let mut new_obs: Vec<String> = vec![];
        let mut i: i64 = 0_i64;
        let mut removed: bool = false;
        while i < (subj.update_observers.len() as i64) {
            if subj.update_observers[(i) as usize].clone() == obs && !removed {
                removed = true;
            } else {
                new_obs.push(subj.update_observers[(i) as usize].clone());
            };
            i = i + 1_i64;
        }
        while (subj.update_observers.len() as i64) > 0_i64 {
            subj.update_observers.pop().unwrap();
        }
        let mut k: i64 = 0_i64;
        while k < (new_obs.len() as i64) {
            subj.update_observers.push(new_obs[(k) as usize].clone());
            k = k + 1_i64;
        }
    };
    if event == "tick".to_string() {
        let mut new_obs2: Vec<String> = vec![];
        let mut i2: i64 = 0_i64;
        let mut removed2: bool = false;
        while i2 < (subj.tick_observers.len() as i64) {
            if subj.tick_observers[(i2) as usize].clone() == obs && !removed2 {
                removed2 = true;
            } else {
                new_obs2.push(subj.tick_observers[(i2) as usize].clone());
            };
            i2 = i2 + 1_i64;
        }
        while (subj.tick_observers.len() as i64) > 0_i64 {
            subj.tick_observers.pop().unwrap();
        }
        let mut k2: i64 = 0_i64;
        while k2 < (new_obs2.len() as i64) {
            subj.tick_observers.push(new_obs2[(k2) as usize].clone());
            k2 = k2 + 1_i64;
        }
    };
}

fn dispatch_observer(subj: Subject, obs_name: String, data: String) {
    if obs_name == "logger".to_string() {
        subj.log.push(format!("logger received: {}", data));
        println!("{}", format!("logger received: {}", data));
    };
    if obs_name == "counter".to_string() {
        subj.log.push(format!("counter received: {}", data));
        println!("{}", format!("counter received: {}", data));
    };
    if obs_name == "auditor".to_string() {
        subj.log.push(format!("auditor received: {}", data));
        println!("{}", format!("auditor received: {}", data));
    };
    if obs_name == "obs1".to_string() {
        subj.log.push(format!("obs1 received: {}", data));
        println!("{}", format!("obs1 received: {}", data));
    };
    if obs_name == "obs2".to_string() {
        subj.log.push(format!("obs2 received: {}", data));
        println!("{}", format!("obs2 received: {}", data));
    };
}

fn notify(subj: Subject, event: String, data: String) {
    if event == "update".to_string() {
        let mut i: i64 = 0_i64;
        while i < (subj.update_observers.len() as i64) {
            dispatch_observer(subj.clone(), subj.update_observers[(i) as usize].clone(), data.clone());
            i = i + 1_i64;
        }
    };
    if event == "tick".to_string() {
        let mut i2: i64 = 0_i64;
        while i2 < (subj.tick_observers.len() as i64) {
            dispatch_observer(subj.clone(), subj.tick_observers[(i2) as usize].clone(), data.clone());
            i2 = i2 + 1_i64;
        }
    };
}

fn main() {
    let mut s1: Subject = subject_new();
    subscribe(s1.clone(), "update".to_string(), "logger".to_string());
    notify(s1.clone(), "update".to_string(), "hello".to_string());
    notify(s1.clone(), "update".to_string(), "world".to_string());
    let mut s2: Subject = subject_new();
    subscribe(s2.clone(), "update".to_string(), "obs1".to_string());
    subscribe(s2.clone(), "update".to_string(), "obs2".to_string());
    subscribe(s2.clone(), "update".to_string(), "logger".to_string());
    notify(s2.clone(), "update".to_string(), "event_data".to_string());
    let mut s3: Subject = subject_new();
    subscribe(s3.clone(), "tick".to_string(), "obs1".to_string());
    subscribe(s3.clone(), "tick".to_string(), "obs2".to_string());
    notify(s3.clone(), "tick".to_string(), "tick1".to_string());
    unsubscribe(s3.clone(), "tick".to_string(), "obs1".to_string());
    notify(s3.clone(), "tick".to_string(), "tick2".to_string());
    let mut s4: Subject = subject_new();
    subscribe(s4.clone(), "update".to_string(), "logger".to_string());
    subscribe(s4.clone(), "tick".to_string(), "counter".to_string());
    notify(s4.clone(), "update".to_string(), "upd1".to_string());
    notify(s4.clone(), "tick".to_string(), "t1".to_string());
    notify(s4.clone(), "update".to_string(), "upd2".to_string());
    let mut s5: Subject = subject_new();
    notify(s5.clone(), "tick".to_string(), "nothing".to_string());
    println!("{}", "no-observer notify: ok".to_string());
    let mut s6: Subject = subject_new();
    subscribe(s6.clone(), "update".to_string(), "auditor".to_string());
    subscribe(s6.clone(), "tick".to_string(), "auditor".to_string());
    unsubscribe(s6.clone(), "update".to_string(), "auditor".to_string());
    notify(s6.clone(), "update".to_string(), "should_not_fire".to_string());
    notify(s6.clone(), "tick".to_string(), "should_fire".to_string());
}

