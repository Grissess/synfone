pub struct Pitch {
    MIDI(f32),  // MIDI format--semitones in 12-TET, 60 is middle C
    Freq(f32),  // Actual frequency in Hz
}

impl Pitch { //Defines methods on Pitch
    fn to_midi(&self) -> Pitch {
        match *self {
            Pitch::MIDI(x) => Pitch::MIDI(x),
            Pitch::Freq(x) => 
                Pitch::MIDI(12.0 * (x / 440.0).log2() + 69.0),
        }
    }

    fn to_freq(&self) -> Pitch {
        match *self {
            Pitch::MIDI(x) =>
                Pitch::Freq(440.0 * (2.0).powf((x - 69.0) / 12.0)),
            Pitch::Freq(x) => Pitch::Freq(x),
        }
    }
}

pub struct Event {
    pub pitch: Pitch,  // Activation pitch
    pub ampl: f32,     // Amplitude, 0.0-1.0
    pub time: f32,     // Time of activation, in seconds
    pub dur: f32,      // Duration of activation, in seconds
    pub bar: i32,      // Measure of the piece, if the format specifies it; otherwise, -1
    pub beat: f32,     // Beat within the bar, as determined by a time signature, if the format includes it
}

impl Event {
    fn new(pitch: Pitch, ampl: f32, time: f32, dur: f32) -> Event {
        Event::new(pitch, ampl, time, dur, -1, 0.0)
    }

    fn new(pitch: Pitch, ampl: f32, time: f32, dur: f32, bar: i32, beat: f32) -> Event {
        Event {
            pitch: pitch,
            ampl: ampl,
            time: time,
            dur: dur,
            bar: bar,
            beat: beat,
        }
    }
}
