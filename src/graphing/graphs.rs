use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
use plotters_piston::{draw_piston_window};
use std::{collections::VecDeque, vec::Vec};

const MAXITEMSINITEMVEC:usize = 10;

pub struct Graphing;

#[derive(Debug,Clone)]
enum BroadcastMessages {
    /// Remove(index)
    Remove(usize),
    // Update(index,value)
    Update(usize,f32),
    /// Add(v)
    Add(Option<VecDeque<f32>>)
}

pub struct GrapingWindow {
    fps:u8,
    length:u8,
    window:PistonWindow,
    broadcast_reader:tokio::sync::broadcast::Receiver<BroadcastMessages>
}
/// NOTE: RENAME THIS IDK WHAT ITS GONNA LOOK LIKE RIGHT NOW SO i JUST GAVE IT THE FIRST NAME THAT CAME TO MY MIND 
pub struct ItemQueue {
    broadcast_writer:tokio::sync::broadcast::Sender<BroadcastMessages>
    // queue:Arc<RwLock<Vec<VecDeque<f32>>>>
}


impl ItemQueue {


    pub async fn add_value_to_item(&self, index:usize,value:f32) {
        self.broadcast_writer.send(BroadcastMessages::Update(index, value)).unwrap();
    } 

    /// adds a new VecDequeue to the queue
    pub async fn add_item_to_queue(&self,item:Option<VecDeque<f32>>) {
        self.broadcast_writer.send(BroadcastMessages::Add(item)).unwrap();
    }


    /// im trusting you to use this responsibly
    pub async fn remove_item_from_queue(&self, index:usize) {
        self.broadcast_writer.send(BroadcastMessages::Remove(index)).unwrap();
    }

}

impl Graphing {
    pub fn init_graph(
        fps:u8,
        length:u8,
        fullscreen:bool,
    ) -> (GrapingWindow,ItemQueue) {
        // configure the window
        let mut window:PistonWindow = WindowSettings::new("ten80av_rs graph",[400,400])
            .fullscreen(fullscreen)
            .exit_on_esc(true)
        .build()
        .unwrap();

        let (broadcast_writer,broadcast_reader) = tokio::sync::broadcast::channel(32);

        window.set_max_fps(fps as u64);
        return (
            GrapingWindow {
                fps,
                length,
                window,
                broadcast_reader
            },
            ItemQueue{
                broadcast_writer,
            } 
        )

        

    }

    // so our reader will read data from vectors
    // so how do we update the vectors?
    // maybe have 
    

    // because the window cant be sent between threads with async and shit maybe flip around our thinking?
    // have gathering data be async and reporting it sync
    // so the itemdata stuff would live inside of the spawn window thing. the broadcasts would be used to basically check
    // "hey? is there any more data for me to use?"
    // the user would use the exposed ItemQueue thing to communicate with the spawn_window thing

    pub fn spawn_window(mut window:GrapingWindow,items:&ItemQueue){
        
        let data_points = (window.fps * window.length) as usize;
        let mut local_state: Vec<VecDeque<f32>> = Vec::new();
        // set up the hot loop or whatever you want to call it 

        

        while let Some(_) = draw_piston_window(&mut window.window, |b| {
            let root = b.into_drawing_area();
            root.fill(&WHITE)?;
            
            {
                loop {
                    match window.broadcast_reader.try_recv() {
                        Ok(message) => {
                            Graphing::parse_broadcast_message(message, &mut local_state);
                        },
                        Err(e) => {
                            match e {
                                tokio::sync::broadcast::error::TryRecvError::Empty => {
                                    break;
                                },
                                _ => {
                                    panic!("tokio: {e:?}")
                                }
                            }
                        },
                    }
                }
            }
            
            
            let mut cc = ChartBuilder::on(&root)
                .margin(10)
                .caption("AV Sensor Data. ESC to exit", ("sans serif",30))
                .x_label_area_size(40)
                .y_label_area_size(50)
            .build_cartesian_2d(0..data_points as u32, 0f32..1f32).unwrap();

            cc.configure_mesh()
            .x_label_formatter(&|x| std::format!("{}", -(window.length as f32) + (*x as f32 / window.fps as f32)))
            .y_label_formatter(&|y| std::format!("{}%", (*y * 100.0) as u32))
            .x_labels(15)
            .y_labels(5)
            .x_desc("Seconds")
            .y_desc("Sensor Value")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

            for (index,data) in (0..).zip(local_state.iter()) {
                cc.draw_series(LineSeries::new(
                    (0..).zip(data.iter())
                    .map(|(idx,data_idx)| (idx, *data_idx))
                    , &Palette99::pick(index)
                ))?
                .label(std::format!("Sensor {}",index))
                .legend(move |(x,y)| {
                    Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(index))
                });


            }

            cc.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

            Ok(())
        
        }) {

        };

        panic!("Window closed.")
    }
    

    fn parse_broadcast_message(message:BroadcastMessages,item_vec:&mut Vec<VecDeque<f32>>) {
        match message {
            BroadcastMessages::Remove(index) => {
                item_vec.remove(index);
            },
            BroadcastMessages::Update(index, value) => {
                if  MAXITEMSINITEMVEC > item_vec[index].len()  {
                    item_vec[index].push_front(value);
                }
                else {
                    item_vec[index].pop_back(); 
                    item_vec[index].push_front(value);
                }
            },
            BroadcastMessages::Add(items) => {
                match items {
                    Some(x) => {
                        item_vec.push(x);
                    },
                    None => {
                        item_vec.push(VecDeque::new());
                    },
                }
            },
        }
    }
}


// todo: make all the messages work off of an id rather than an index. I dont trust y'all niggas 