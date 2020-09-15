use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{ channel, Sender, Receiver };
use std::time::Duration;
use std::sync::{ Arc, Mutex, Condvar };



pub fn producer (sender: Sender<usize>, pair: Arc<(Mutex<bool>, Condvar)>) -> Vec<JoinHandle<()>> {
  // mock 10 threads
  let mut handlers = vec![];
  for i in 0..9 {
    let clone_sender = sender.clone();
    let tid = i;
    let pair_clone = pair.clone();

    let handle = thread::spawn(move || {
      let (lock, cvar) = &*pair_clone;
      let mut flag = lock.lock().unwrap();

      for j in 1..10 {
        let data = [tid.to_string(), j.to_string()].concat().parse::<usize>().unwrap();
        println!("producer: {:?}", data);
        let duration = Duration::from_secs(1);
        thread::sleep(duration);
        clone_sender.send(data);

        if i == 9 && j == 9 {
          *flag = true
        }

        cvar.notify_all();
      }
    });
    handlers.push(handle);
  }

  handlers
}

pub fn consumer(receiver: Receiver<usize>, pair: Arc<(Mutex<bool>, Condvar)>) -> JoinHandle<()> {
  
  let handle = thread::spawn(move || {

    let mut _continue: bool = true;

    while (_continue) {
       _continue = match receiver.recv() {
        Ok(data) => {
          println!("consumer: {:?}", data);
          true
        },
        _ => {
          false
        }
      };
    }

  });

  return handle;
}


#[cfg(test)]
mod thread_ops_tests {
  use super::*;
  
  #[test]
  pub fn test_producer_consumer () {
    // condition variable
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let (tx, rx) = channel();

    let handle = consumer(rx, Arc::clone(&pair));
    let handlers = producer(tx, Arc::clone(&pair2));

    // wait for consumer to finish
    handle.join().unwrap();
    for h in handlers {
      h.join().unwrap();
    }
    println!("[[test end]]")
  }
}
