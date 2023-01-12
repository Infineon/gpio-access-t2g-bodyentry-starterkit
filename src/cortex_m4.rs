use cortex_m_rt::pre_init;
use cortex_m_rt::{entry};
use cortex_m::delay::Delay;

#[pre_init]
unsafe fn before_main() {
    
}

#[entry]
fn main() -> ! {
	
    let cp = cortex_m::Peripherals::take().unwrap();
    let  syst = cp.SYST;

    let mut _delay = Delay::new(syst,100000000);
    
    loop  {
        //delay.delay_ms(10);
       
    }
}




