use std::{boxed::Box, collections::VecDeque, vec::Vec};
use std::{vec};
use egui_plot::LegendGrouping::ByName;
use egui_plot::{Legend, Line, Plot, PlotPoints};

const MAXITEMSINITEMVEC:usize = 50;

pub struct Graphing;

pub enum GraphSize {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    Full,
}

impl Into<u32> for GraphSize {
    fn into(self) -> u32 {
        match self {
            GraphSize::XSmall => 100,
            GraphSize::Small => 1000,
            GraphSize::Medium => 2000,
            GraphSize::Large => 4000,
            GraphSize::XLarge => 10_000,
            GraphSize::Full => u32::MAX,
        }
    }
}


#[derive(Debug,Clone)]
pub enum BroadcastMessages {
    /// Remove(index)
    Remove(usize),
    // Update(index,value)
    Update(usize,f32),
    /// Add(value,name)
    Add(Option<VecDeque<f32>>,&'static str)
}

impl BroadcastMessages {
    pub(crate) fn parse_broadcast_message(message:BroadcastMessages,item_vec:&mut Vec<GraphItem>) {
        match message {
            BroadcastMessages::Remove(index) => {
                item_vec.remove(index);
            },
            BroadcastMessages::Update(index, value) => {
                if  MAXITEMSINITEMVEC > item_vec[index].vec.len()  {
                    item_vec[index].vec.push_front(value);
                }
                else {
                    item_vec[index].vec.pop_back(); 
                    item_vec[index].vec.push_front(value);
                }
            },
            BroadcastMessages::Add(items,name) => {
                match items {
                    Some(x) => {
                        item_vec.push(GraphItem { name, vec: x });
                    },
                    None => {
                        item_vec.push(GraphItem { name, vec: VecDeque::new() });
                    },
                }
            },
        }
    }
}

/// Graphing window struct that is visible to the user
struct GraphingWindow {
    broadcast_reader:tokio::sync::broadcast::Receiver<BroadcastMessages>,
    /// this is a list of line points
    /// so like this 
    /// [
    /// [1.0,1.0,1.0]
    /// [1.0,1.0,0.9]
    /// ]
    /// (this is a straight line and another straightish line)
    line_state:Vec<GraphItem>,
    /// whats the highest value you expect to see?
    expected_max_value:u32,
}


/// Queue that is able to write to ItemQueueReader. 
/// 
/// The primary use of this structure is to write information to your graph.
pub struct ItemQueue {
    broadcast_writer:tokio::sync::broadcast::Sender<BroadcastMessages>
}

pub struct ItemQueueReader {
    broadcast_reader:tokio::sync::broadcast::Receiver<BroadcastMessages>
}

pub(crate) struct GraphItem {
    name:&'static str,
    vec:VecDeque<f32>
}


impl ItemQueue {



    /// adds a new data point to a line
    pub async fn add_point_to_line(&self, index:usize,value:f32) {
        self.broadcast_writer.send(BroadcastMessages::Update(index, value)).unwrap();
    } 

    /// Adds a new line to the graph
    pub async fn add_new_line(&self,item:Option<VecDeque<f32>>,name:&'static str) {
        self.broadcast_writer.send(BroadcastMessages::Add(item,name)).unwrap();
    }


    /// delete a line based off the index of it 
    pub async fn delete_line(&self, index:usize) {
        self.broadcast_writer.send(BroadcastMessages::Remove(index)).unwrap();
    }

}

impl Graphing {
    /// configures a window with the default settings and returns it alongside an ItemQueue reader and writer.
    /// 
    /// this function was made to be used alongside `spawn_window()`
    pub fn make_window() -> (
        eframe::NativeOptions,
        ItemQueue,
        ItemQueueReader
    ) {

        

        let (broadcast_writer,broadcast_reader) = tokio::sync::broadcast::channel(32);
        return (
            eframe::NativeOptions::default(),
            ItemQueue { broadcast_writer },
            ItemQueueReader { broadcast_reader }
        )
    }

    /// creates a window.
    /// 
    /// The arguments to this function can be retrieved through Graphing::make_window
    pub fn spawn_window(window:eframe::NativeOptions,broadcast_reader: ItemQueueReader, graph_size:GraphSize){ 
        eframe::run_native("ten80av_rs graph", window,
        Box::new(
            |_| 
            Ok(
                Box::new(
                    GraphingWindow::new(broadcast_reader.broadcast_reader,graph_size)
                )
            ))).unwrap();
    }
    

    
}


impl GraphingWindow {
    fn new(broadcast_reader: tokio::sync::broadcast::Receiver<BroadcastMessages>, graph_size:GraphSize) -> Self {
        Self { broadcast_reader, line_state:vec![], expected_max_value: graph_size.into() }
    }
}

impl eframe::App for GraphingWindow {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.request_repaint();

        // check if the queue has any new messages 
        match self.broadcast_reader.try_recv() {
            Ok(message) => {
                BroadcastMessages::parse_broadcast_message(message, &mut self.line_state);
            },
            Err(_) => {
                // do nothing and do not update the graph
            },
        }

        // is this really something I want to have in a "hotloop"
        let mut points_list:Vec<Line> = Vec::new();
        for points in self.line_state.iter() {
            let temp_points:PlotPoints = (0..points.vec.len())
            .map(|pos| {
                [pos as f64,points.vec[pos] as f64 ]   
            }).collect();

            points_list.push(
                Line::new(points.name, temp_points)
            )
        }

        let legend = Legend::default()
        .title("Sensors")
        .grouping(ByName);

        Plot::new("diddyplot :skull: those who know")
        .allow_axis_zoom_drag(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .default_y_bounds(0.0, self.expected_max_value as f64)
        .default_x_bounds(0.0, MAXITEMSINITEMVEC as f64)
        .legend(legend)
        .show_crosshair(false)
        .x_axis_label("Cycles Past")
        .show(ui, |plot_ui| {
            for i in points_list {
                plot_ui.line(i);
            }
        });

    }
}