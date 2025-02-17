# Dht22-embedded-hal
Embassy-rp HAL for DHT22, a temperature and humidity sensor. Utilizes async functions to minimize blocking.

---

## Key Points
1. **Async functions**
   - The program uses Timer function, and a yielding function which has a 1 microsecond delay to minimize time between tasks
   - The time elasped of the data pin at a certain state is measured using Instant::now() function.

2. **No added delays**
   - All necessary delays and timers are added using the timer function.
   - This makes the code completely async.

3. **Timeout handling**
   - Functions for measuring duration at high and low state return an error if the pin does not transition in expected time.
   - If the pin never gets to low, a generic error is returned.

4. **Checksum Error**
   - To ensure the value is accurate, the checksum (last byte) is compared with the expected value (sum of first 4 bytes of received data).
   - An error is returned if they do not match. 

---

### Usage

It is important to use the function as a separate task without external blocking or interruption as it can deter the reading task causing high timeout or generic timeout error readings. This also allows us to use the async programming as it is intended to. 
An example of implemtation async function is shown below. 

'''rust 
#[embassy_executor::task]
async fn dht22_task(mut dht: DHT22<'static>) {
    loop {
        match dht.read().await {
            Ok(reading) => {
                log::info!("DHT22 Reading: Temp = {}, Hum = {}", reading.temp, reading.hum);
            }
            Err(e) => {
                log::warn!("DHT22 read error: {:?}", e);
            }
        }
        Timer::after(Duration::from_secs(2)).await; 
    }
}
'''

In the main function the module can be initalized using a Flex pin and Spawner
'''
let dht = DHT22::new(Flex::new(rp.PIN_22));
spawner.spawn(dht22_task(dht)).unwrap();
'''
