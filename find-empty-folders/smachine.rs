/*
 *                  Cadence Data Soft Pvt. Ltd.
 *
*/

use std::{string::String, fmt::Result};

type SmFtype = fn (&mut Sm, &mut String, &mut String) -> String;

pub struct Sm
{
    pub s_transitions: [[SmFtype; 5]; 2],
    cur_state: STATES,
    last_state: STATES,
}
// pub struct Sm <SmFtype: ?Sized>{
//     cur_state: STATES,
//     last_state: STATES,
//     s_transitions: [[SmFtype; 5]; 5]
// }

pub enum STATES{
    STATE_IDLE,
    STATE_INIT,
    STATE_BUSY,
    STATE_SLEEP,
    STATE_DONE
}

impl Sm {

    pub fn new() -> Self{
        Sm{
            s_transitions: [ [Sm::state_idle,
            Sm::state_busy,
            Sm::state_sleep,
            Sm::state_done,
            Sm::state_exit],
            [Sm::state_idle,
            Sm::state_busy,
            Sm::state_sleep,
            Sm::state_done,
            Sm::state_exit]
            ],
            cur_state: STATES::STATE_IDLE,
            last_state:  STATES::STATE_IDLE

        }


    }

    fn state_idle(&mut self, param1: &mut String, param2: &mut String) -> String{
        for x in 0..10 {
            println!("Idle"); // x: i32
        }
        
        "".to_string()
    }

    fn state_busy(&mut self, param1: &mut String, param2: &mut String) -> String{
        for x in 0..10 {
            println!("Busy"); // x: i32
        }
        
        "".to_string()

    }

    fn state_sleep(&mut self, param1: &mut String, param2: &mut String) -> String{
        for x in 0..10 {
            println!("Sleep"); // x: i32
        }
        
        "".to_string()

    }
    
    fn state_done(&mut self, param1: &mut String, param2: &mut String) -> String{
        for x in 0..10 {
            println!("Done"); // x: i32
        }
        
        "".to_string()

    }

    fn state_exit(&mut self, param1: &mut String, param2: &mut String) -> String{
        for x in 0..10 {
            println!("Exit"); // x: i32
        }
        
        "".to_string()

    }
}
