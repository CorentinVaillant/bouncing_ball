use std::{sync::{Arc, Mutex, MutexGuard}, thread::{self, JoinHandle, Thread}, time::{Duration, Instant}};

use super::traits::Physics;

pub struct PhysicsWrapper{
    objects : Vec<Arc<Mutex<dyn Physics>>>
}

impl PhysicsWrapper{
    pub fn empty()->Self{
        Self { objects: vec![] }
    }

    pub fn push<OBJ : Physics+'static>(&mut self,obj:OBJ)->Arc<Mutex<OBJ>>{
        let obj = Arc::new(Mutex::new(obj));
        
        self.objects.push(obj.clone());

        obj
    }
}

unsafe impl Send for PhysicsWrapper {
    
}

impl Physics for PhysicsWrapper {
    fn physics_update(&mut self,dt:f32) {
        for elem in &mut self.objects{
            match elem.lock(){
                Ok(mut obj) => obj.physics_update(dt),
                Err(_) => (),
            }
    }
}
}

pub struct PhysicsFixUpdates{
    fix_dt : f32,
    run : Arc<Mutex<bool>>,
    thread_handle : JoinHandle<()>,
}

impl PhysicsFixUpdates{
    pub fn new(objects :PhysicsWrapper, fix_dt:f32)->Self{

        let run = Arc::new(Mutex::new(true));
        let run_cl = run.clone();
        let fix_thread = thread::spawn(move ||fix_loop(objects, fix_dt, run_cl));

        Self { 
            fix_dt,
            run : run.clone(),
            thread_handle:fix_thread
        }
    }

    pub fn stop_update(&mut self){
        todo!()
    }
}

fn fix_loop(obj : PhysicsWrapper,fix_dt:f32,run :Arc<Mutex<bool>>){
    let mut obj = obj;

    while run.try_lock().map(|e|*e).unwrap_or(false) {
        let begin = Instant::now();

        for obj in &mut obj.objects {
            let _ = obj.try_lock().map(|mut o|o.physics_update(fix_dt));
        }

        let end = Instant::now();

        let sleep_time = end.duration_since(begin).as_secs_f32() - fix_dt;
        thread::sleep(Duration::from_secs_f32(sleep_time));
    }
}