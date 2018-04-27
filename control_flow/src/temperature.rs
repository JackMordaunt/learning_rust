// C/5  =  (F-32)/9
const ABS_ZERO_CELSIUS: f32 = -273.15;
const ABS_ZERO_FARENHEIT: f32 = -459.67;

// to_farenheight converts from celsius.
// Values less than absolute zero are treated as absolute zero.
pub fn to_farenheit(celsius: f32) -> f32 {
    let mut celsius = celsius;
    if celsius < ABS_ZERO_CELSIUS {
        celsius = ABS_ZERO_CELSIUS;
    }
    ((celsius/5 as f32)*9 as f32)+32 as f32
}

// to_celsius converts from farenheit.
// Values less than absolute zero are treated as absolute zero.
pub fn to_celsius(farenheit: f32) -> f32 {
    let mut farenheit = farenheit;
    if farenheit < ABS_ZERO_FARENHEIT {
        farenheit = ABS_ZERO_FARENHEIT;
    }
    ((farenheit-32 as f32)/9 as f32)*5 as f32
}
