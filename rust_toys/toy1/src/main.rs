extern crate find_folder;
extern crate hound;
extern crate portaudio as pa;
extern crate sample;
extern crate iced;

use sample::{signal, Frame, Sample, Signal, ToFrameSliceMut};

use iced::{
    canvas, executor, Application, Canvas, Color, Command, Container, Element,
    Length, Point, Settings, Subscription, Vector,
};



const FRAMES_PER_BUFFER: u32 = 512;
const NUM_CHANNELS: i32 = 1;
const SAMPLE_RATE: f64 = 44_100.0;

pub fn main() {
    //play_wav()
    //synth().unwrap()
    Clock::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

fn play_wav()
{
    // Find and load the wav.
    //let assets = find_folder::Search::ParentsThenKids(5, 5).for_folder("~/").unwrap();
    //let reader = hound::WavReader::open(assets.join("~/001.wav")).unwrap();
    let reader = hound::WavReader::open("./001.wav").unwrap();
    let spec = reader.spec();

    // Read the interleaved samples and convert them to a signal.
    let samples = reader.into_samples::<i16>().filter_map(Result::ok);
    let mut frames = signal::from_interleaved_samples_iter(samples).until_exhausted();

    // Initialise PortAudio.
    let pa = pa::PortAudio::new().unwrap();
    let ch = spec.channels as i32;
    let sr = spec.sample_rate as f64;
    let buffer_len = 0; // use default
    let settings = pa.default_output_stream_settings::<i16>(ch, sr, buffer_len).unwrap();

    // A channel for indicating when playback has completed.
    let (complete_tx, complete_rx) = std::sync::mpsc::channel();

    // Define the callback which provides PortAudio the audio.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, .. }| {
        let buffer: &mut [[i16; 2]] = buffer.to_frame_slice_mut().unwrap();
        for out_frame in buffer {
            match frames.next() {
                Some(frame) => *out_frame = frame,
                None => {
                    complete_tx.send(()).unwrap();
                    return pa::Complete;
                },
            }
        }
        pa::Continue
    };

    // Spawn and start the output stream.
    let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();
    stream.start().unwrap();

    // Block until playback completes.
    complete_rx.recv().unwrap();

    stream.stop().unwrap();
    stream.close().unwrap();
}

fn synth() -> Result<(), pa::Error> {

    // Create a signal chain to play back 1 second of each oscillator at A4.
    let hz = signal::rate(SAMPLE_RATE).const_hz(440.0);
    let one_sec = SAMPLE_RATE as usize;
    let mut waves = hz.clone()
        .sine()
        .take(one_sec)
        .chain(hz.clone().saw().take(one_sec))
        .chain(hz.clone().square().take(one_sec))
        .chain(hz.clone().noise_simplex().take(one_sec))
        .chain(signal::noise(0).take(one_sec))
        .map(|f| f.map(|s| s.to_sample::<f32>() * 0.2));

    // Initialise PortAudio.
    let pa = pa::PortAudio::new().unwrap();
    let settings = pa.default_output_stream_settings::<f32>(
        NUM_CHANNELS,
        SAMPLE_RATE,
        FRAMES_PER_BUFFER,
    ).unwrap();

    // Define the callback which provides PortAudio the audio.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, .. }| {
        let buffer: &mut [[f32; 1]] = buffer.to_frame_slice_mut().unwrap();
        for out_frame in buffer {
            match waves.next() {
                Some(frame) => *out_frame = frame,
                None => return pa::Complete,
            }
        }
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();
    stream.start();

    while let Ok(true) = stream.is_active() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    stream.stop();
    stream.close();

    Ok(())
}

struct Clock {
    now: LocalTime,
    clock: canvas::layer::Cache<LocalTime>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (
            Clock {
                now: chrono::Local::now().into(),
                clock: canvas::layer::Cache::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clock - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time.into();

                if now != self.now {
                    self.now = now;
                    self.clock.clear();
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(500)).map(Message::Tick)
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Canvas::new()
            .width(Length::Units(400))
            .height(Length::Units(400))
            .push(self.clock.with(&self.now));

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LocalTime {
    hour: u32,
    minute: u32,
    second: u32,
}

impl From<chrono::DateTime<chrono::Local>> for LocalTime {
    fn from(date_time: chrono::DateTime<chrono::Local>) -> LocalTime {
        use chrono::Timelike;

        LocalTime {
            hour: date_time.hour(),
            minute: date_time.minute(),
            second: date_time.second(),
        }
    }
}

impl canvas::Drawable for LocalTime {
    fn draw(&self, frame: &mut canvas::Frame) {
        let center = frame.center();
        let radius = frame.width().min(frame.height()) / 2.0;
        let offset = Vector::new(center.x, center.y);

        let clock = canvas::Path::new(|path| path.circle(center, radius));

        frame.fill(
            &clock,
            canvas::Fill::Color(Color::from_rgb8(0x12, 0x93, 0xD8)),
        );

        fn draw_hand(
            n: u32,
            total: u32,
            length: f32,
            offset: Vector,
            path: &mut canvas::path::Builder,
        ) {
            let turns = n as f32 / total as f32;
            let t = 2.0 * std::f32::consts::PI * (turns - 0.25);

            let x = length * t.cos();
            let y = length * t.sin();

            path.line_to(Point::new(x, y) + offset);
        }

        let hour_and_minute_hands = canvas::Path::new(|path| {
            path.move_to(center);
            draw_hand(self.hour, 12, 0.5 * radius, offset, path);

            path.move_to(center);
            draw_hand(self.minute, 60, 0.8 * radius, offset, path)
        });

        frame.stroke(
            &hour_and_minute_hands,
            canvas::Stroke {
                width: 6.0,
                color: Color::WHITE,
                line_cap: canvas::LineCap::Round,
                ..canvas::Stroke::default()
            },
        );

        let second_hand = canvas::Path::new(|path| {
            path.move_to(center);
            draw_hand(self.second, 60, 0.8 * radius, offset, path)
        });

        frame.stroke(
            &second_hand,
            canvas::Stroke {
                width: 3.0,
                color: Color::WHITE,
                line_cap: canvas::LineCap::Round,
                ..canvas::Stroke::default()
            },
        );
    }
}

mod time {
    use iced::futures;

    pub fn every(
        duration: std::time::Duration,
    ) -> iced::Subscription<chrono::DateTime<chrono::Local>> {
        iced::Subscription::from_recipe(Every(duration))
    }

    struct Every(std::time::Duration);

    impl<H, I> iced_native::subscription::Recipe<H, I> for Every
        where
            H: std::hash::Hasher,
    {
        type Output = chrono::DateTime<chrono::Local>;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;

            std::any::TypeId::of::<Self>().hash(state);
            self.0.hash(state);
        }

        fn stream(
            self: Box<Self>,
            _input: futures::stream::BoxStream<'static, I>,
        ) -> futures::stream::BoxStream<'static, Self::Output> {
            use futures::stream::StreamExt;

            async_std::stream::interval(self.0)
                .map(|_| chrono::Local::now())
                .boxed()
        }
    }
}